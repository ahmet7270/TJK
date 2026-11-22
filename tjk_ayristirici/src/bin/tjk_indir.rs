//! Aylık toplu arşivleme aracı: TJK günlük program/sonuç CSV adres listesi, kaldığı yerden devam eden indirme,
//! içerikten doğrulama, mevcut `toplu` komutuyla aktarım ve aylık rapor.
//!
//! Kullanım (tek komut, kendi bilgisayarınızda):
//!   tjk_indir hepsi --ay 2026-03 --db tjk_calisma_s3.sqlite --cikti mart [--mevcut ESKI_CSV_KLASORU] [--takvim resmi_takvim.tsv]
//! Ayrı adımlar:
//!   tjk_indir liste  --ay 2026-03 --cikti indirme_listesi.tsv [--mevcut KLASOR] [--gunler 9,10]
//!   tjk_indir indir  --liste indirme_listesi.tsv --klasor indirilen [--bekle-ms 2000] [--yok-yeniden]
//!   tjk_indir rapor  --ay 2026-03 --db DB --liste indirme_listesi.tsv --klasor indirilen --cikti aylik_rapor.tsv [--takvim T]
//!
//! Kurallar:
//!   * Adresler URL kalıbından ÜRETİLİR (KALIPTAN_URETILDI). Yalnızca indirilip içeriği (ilk satırdaki tarih ve hipodrom,
//!     kolon başlığındaki rol) beklenenle aynı çıkan dosya ICERIK_DOGRULANDI olur.
//!   * İndirmeden önce medya-cdn.tjk.org/robots.txt okunur. Yol yasaklıysa ya da robots.txt 404 dışında bir hatayla
//!     okunamazsa hiçbir veri isteği yapılmaz. Kaynak 403/429 dönerse araç durur; engel aşılmaya çalışılmaz.
//!   * İstekler arasında bekleme vardır (varsayılan 2 sn). HTTPS isteği sistemdeki `curl` ile yapılır.
//!   * Her sonuç `indirme_durumu.tsv`'ye anında yazılır; yeniden çalıştırınca tamamlananlar atlanır.
//!   * 404 (adreste dosya yok) "yarış yok" demek DEĞİLDİR; resmî takvim olmadan ay "tamamlandı" sayılmaz.

#[path = "../ayristir.rs"]
#[allow(dead_code)]
mod ayristir;
#[path = "../kodlama.rs"]
#[allow(dead_code)]
mod kodlama;
#[path = "../sha256.rs"]
#[allow(dead_code)]
mod sha256;
#[path = "../sqlite.rs"]
#[allow(dead_code)]
mod sqlite;

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

const KOK: &str = "https://medya-cdn.tjk.org";
const ROBOTS: &str = "https://medya-cdn.tjk.org/robots.txt";
const UA: &str = "tjk-arsiv-araci/0.1 (kisisel arsiv; istekler arasi bekleme; robots.txt'e uyar)";
/// TJK'nın yurt içi hipodromları (adres ve içerikteki yazımla)
const HIPODROMLAR: [&str; 10] =
    ["Adana", "Ankara", "Antalya", "Bursa", "Diyarbakır", "Elazığ", "İstanbul", "İzmir", "Kocaeli", "Şanlıurfa"];
const ROLLER: [(&str, &str); 2] = [("program", "GunlukYarisProgrami"), ("sonuc", "GunlukYarisSonuclari")];
const LISTE_BASLIK: &str = "tarih\thipodrom\trol\turl\tyerel_dosya\turl_durumu\tsha256\tkanit";
const DURUM_BASLIK: &str = "zaman_sn\ttarih\thipodrom\trol\turl\tsonuc\thttp\tsha256\tayrinti";

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let r = match a.first().map(String::as_str) {
        Some("liste") => liste_komutu(&a[1..]),
        Some("indir") => indir_komutu(&a[1..]).map(|_| ()),
        Some("rapor") => rapor_komutu(&a[1..]),
        Some("hepsi") => hepsi(&a[1..]),
        _ => Err("Kullanım: tjk_indir hepsi|liste|indir|rapor ... (ayrıntı için kaynak başındaki açıklama)".into()),
    };
    if let Err(e) = r {
        eprintln!("HATA: {e}");
        std::process::exit(1);
    }
}

// ------------------------------------------------------------------ yardımcılar

fn secenekler(a: &[String]) -> Result<HashMap<String, String>, String> {
    let mut m = HashMap::new();
    let mut it = a.iter();
    while let Some(x) = it.next() {
        let k = x.strip_prefix("--").ok_or_else(|| format!("beklenmeyen argüman: {x}"))?;
        if matches!(k, "yok-yeniden") {
            m.insert(k.to_string(), "1".to_string());
        } else {
            m.insert(k.to_string(), it.next().ok_or_else(|| format!("--{k} bir değer bekliyor"))?.clone());
        }
    }
    Ok(m)
}
fn gerekli<'a>(m: &'a HashMap<String, String>, k: &str) -> Result<&'a str, String> {
    m.get(k).map(String::as_str).ok_or_else(|| format!("--{k} gerekli"))
}

fn slug(h: &str) -> String {
    h.chars()
        .map(|c| match c {
            'İ' | 'ı' | 'i' => 'I',
            'Ş' | 'ş' => 'S',
            'Ğ' | 'ğ' => 'G',
            'Ü' | 'ü' => 'U',
            'Ö' | 'ö' => 'O',
            'Ç' | 'ç' => 'C',
            c => c.to_ascii_uppercase(),
        })
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn yuzde_kodla(s: &str) -> String {
    s.bytes().map(|b| if b.is_ascii_alphanumeric() || b == b'-' || b == b'.' { (b as char).to_string() } else { format!("%{b:02X}") }).collect()
}

fn ayin_gunleri(ay: &str) -> Result<Vec<(String, String, String)>, String> {
    let (y, m) = ay.split_once('-').ok_or("--ay YYYY-AA biçiminde olmalı")?;
    let (yi, mi): (i32, u32) = (y.parse().map_err(|_| "yıl")?, m.parse().map_err(|_| "ay")?);
    let artik = (yi % 4 == 0 && yi % 100 != 0) || yi % 400 == 0;
    let n = match mi {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if artik => 29,
        2 => 28,
        _ => return Err("geçersiz ay".into()),
    };
    // (ISO tarih, dd.mm.yyyy, yyyymmdd)
    Ok((1..=n).map(|d| (format!("{yi:04}-{mi:02}-{d:02}"), format!("{d:02}.{mi:02}.{yi:04}"), format!("{yi:04}{mi:02}{d:02}"))).collect())
}

/// Dosyayı içerikten doğrular: (tarih ISO, hipodrom, rol)
fn icerik(bytes: &[u8]) -> Result<(String, String, &'static str), String> {
    let (metin, _) = kodlama::coz(bytes);
    let g = ayristir::gun_ayristir(&metin)?;
    let k = g.kosular.first().ok_or("koşu yok")?;
    let has = |x: &str| k.kolonlar.iter().any(|c| c == x);
    let rol = match (has("Son 6 Yarış"), has("Derece") && has("Ganyan") && has("Fark")) {
        (true, false) => "program",
        (false, true) => "sonuc",
        _ => return Err("kolon başlıklarından rol çıkarılamadı".into()),
    };
    Ok((g.tarih.clone(), g.hipodrom.clone(), rol))
}

#[derive(Clone)]
struct Satir {
    tarih: String,
    hipodrom: String,
    rol: String,
    url: String,
    yerel: String,
    url_durumu: String,
    sha: String,
    kanit: String,
}

fn liste_oku(yol: &str) -> Result<Vec<Satir>, String> {
    let t = std::fs::read_to_string(yol).map_err(|e| format!("{yol}: {e}"))?;
    t.lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() != 8 {
                return Err(format!("liste satırı 8 alan değil: {l}"));
            }
            Ok(Satir {
                tarih: c[0].into(),
                hipodrom: c[1].into(),
                rol: c[2].into(),
                url: c[3].into(),
                yerel: c[4].into(),
                url_durumu: c[5].into(),
                sha: c[6].into(),
                kanit: c[7].into(),
            })
        })
        .collect()
}

// ------------------------------------------------------------------ liste

fn liste_uret(ay: &str, gunler: Option<&str>, mevcut: Option<&str>) -> Result<Vec<Satir>, String> {
    let secili: Option<Vec<u32>> = gunler.map(|g| g.split(',').filter_map(|x| x.trim().parse().ok()).collect());
    // Elde olan ham CSV'ler içerikten doğrulanır: (tarih, hipodrom-slug, rol) → (dosya adı, sha)
    let mut elde: HashMap<(String, String, String), (String, String)> = HashMap::new();
    if let Some(k) = mevcut {
        let mut yollar: Vec<PathBuf> = std::fs::read_dir(k).map_err(|e| format!("{k}: {e}"))?.filter_map(|e| e.ok().map(|e| e.path())).collect();
        yollar.sort();
        for y in yollar.iter().filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("csv"))) {
            let b = std::fs::read(y).map_err(|e| e.to_string())?;
            if let Ok((t, h, r)) = icerik(&b) {
                let ad = y.file_name().unwrap().to_string_lossy().into_owned();
                elde.insert((t, slug(&h), r.to_string()), (ad, sha256::hex(&b)));
            }
        }
    }
    let mut v = Vec::new();
    for (i, (iso, nokta, kisa)) in ayin_gunleri(ay)?.into_iter().enumerate() {
        if secili.as_ref().is_some_and(|s| !s.contains(&(i as u32 + 1))) {
            continue;
        }
        let yil = &iso[..4];
        for h in HIPODROMLAR {
            for (rol, klasor) in ROLLER {
                let url = format!("{KOK}/raporftp/TJKPDF/{yil}/{iso}/CSV/{klasor}/{nokta}-{}-{klasor}-TR.csv", yuzde_kodla(h));
                let yerel = format!("{kisa}-{}-{rol}.csv", slug(h));
                let (durum, sha, kanit) = match elde.get(&(iso.clone(), slug(h), rol.to_string())) {
                    Some((ad, sha)) => ("ICERIK_DOGRULANDI".to_string(), sha.clone(), format!("elde olan ham dosya: {ad}")),
                    None => ("KALIPTAN_URETILDI".to_string(), String::new(), String::new()),
                };
                v.push(Satir { tarih: iso.clone(), hipodrom: h.into(), rol: rol.into(), url, yerel, url_durumu: durum, sha, kanit });
            }
        }
    }
    Ok(v)
}

fn liste_yaz(yol: &str, v: &[Satir]) -> Result<(), String> {
    let mut s = String::from(LISTE_BASLIK);
    s.push('\n');
    for r in v {
        s.push_str(&[&r.tarih, &r.hipodrom, &r.rol, &r.url, &r.yerel, &r.url_durumu, &r.sha, &r.kanit].map(|x| x.replace('\t', " ")).join("\t"));
        s.push('\n');
    }
    std::fs::write(yol, s).map_err(|e| format!("{yol}: {e}"))
}

fn liste_komutu(a: &[String]) -> Result<(), String> {
    let m = secenekler(a)?;
    let v = liste_uret(gerekli(&m, "ay")?, m.get("gunler").map(String::as_str), m.get("mevcut").map(String::as_str))?;
    liste_yaz(gerekli(&m, "cikti")?, &v)?;
    let d = v.iter().filter(|r| r.url_durumu == "ICERIK_DOGRULANDI").count();
    println!("İndirme listesi: {} adres ({} içerik doğrulanmış, {} URL kalıbından üretilmiş)", v.len(), d, v.len() - d);
    Ok(())
}

// ------------------------------------------------------------------ indir

fn curl(url: &str, hedef: &Path) -> Result<String, String> {
    let prog = std::env::var("TJK_INDIR_CURL").unwrap_or_else(|_| "curl".into());
    let o = Command::new(&prog)
        .args(["-sS", "-L", "--max-time", "60", "-A", UA, "-o"])
        .arg(hedef)
        .args(["-w", "%{http_code}", url])
        .output()
        .map_err(|e| format!("'{prog}' çalıştırılamadı ({e}); curl kurulu olmalı"))?;
    let kod = String::from_utf8_lossy(&o.stdout).trim().to_string();
    if kod.is_empty() || kod == "000" {
        return Ok(format!("000:{}", String::from_utf8_lossy(&o.stderr).trim()));
    }
    Ok(kod)
}

/// robots.txt: yol izinli mi? (izinli, açıklama)
fn robots_izni(klasor: &Path, yol: &str) -> Result<(bool, String), String> {
    let gecici = klasor.join(".robots.txt.part");
    let kod = curl(ROBOTS, &gecici)?;
    let govde = std::fs::read_to_string(&gecici).unwrap_or_default();
    let _ = std::fs::remove_file(&gecici);
    match kod.as_str() {
        "200" => {}
        "404" | "410" => return Ok((true, format!("robots.txt yok (HTTP {kod}); kural tanımlanmamış kabul edildi"))),
        k if k.starts_with("000") => return Ok((false, format!("robots.txt okunamadı — bağlantı kurulamadı: {k}"))),
        k => return Ok((false, format!("robots.txt okunamadı (HTTP {k}); izin belirsiz, indirme yapılmadı"))),
    }
    // User-agent: * grubundaki Allow/Disallow; en uzun eşleşme kazanır
    let (mut grupta, mut en) = (false, (0usize, true));
    for l in govde.lines().map(|l| l.split('#').next().unwrap_or("").trim()) {
        let Some((k, v)) = l.split_once(':') else { continue };
        let (k, v) = (k.trim().to_ascii_lowercase(), v.trim());
        match k.as_str() {
            "user-agent" => grupta = v == "*",
            "disallow" if grupta && !v.is_empty() && yol.starts_with(v) && v.len() >= en.0 => en = (v.len(), false),
            "allow" if grupta && !v.is_empty() && yol.starts_with(v) && v.len() >= en.0 => en = (v.len(), true),
            _ => {}
        }
    }
    Ok((en.1, if en.1 { "robots.txt bu yola izin veriyor".into() } else { "robots.txt bu yolu yasaklıyor".into() }))
}

fn durum_oku(yol: &Path) -> HashMap<String, (String, String)> {
    let mut m = HashMap::new();
    if let Ok(t) = std::fs::read_to_string(yol) {
        for l in t.lines().skip(1) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() >= 8 {
                m.insert(c[4].to_string(), (c[5].to_string(), c[7].to_string()));
            }
        }
    }
    m
}

fn simdi() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Dönüş: (yeni doğrulanan dosya sayısı, durdurma nedeni)
fn indir(liste: &[Satir], klasor: &Path, bekle_ms: u64, yok_yeniden: bool) -> Result<(usize, Option<String>), String> {
    std::fs::create_dir_all(klasor).map_err(|e| e.to_string())?;
    let red = klasor.join("reddedilen");
    let durum_yol = klasor.join("indirme_durumu.tsv");
    if !durum_yol.exists() {
        std::fs::write(&durum_yol, format!("{DURUM_BASLIK}\n")).map_err(|e| e.to_string())?;
    }
    let onceki = durum_oku(&durum_yol);
    let yaz = |r: &Satir, sonuc: &str, http: &str, sha: &str, ayr: &str| -> Result<(), String> {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().append(true).open(&durum_yol).map_err(|e| e.to_string())?;
        writeln!(f, "{}\t{}\t{}\t{}\t{}\t{sonuc}\t{http}\t{sha}\t{}", simdi(), r.tarih, r.hipodrom, r.rol, r.url, ayr.replace(['\t', '\n'], " "))
            .map_err(|e| e.to_string())
    };
    let bekleyen: Vec<&Satir> = liste.iter().filter(|r| r.url_durumu != "ICERIK_DOGRULANDI").collect();
    if bekleyen.is_empty() {
        return Ok((0, None));
    }
    let (izin, acik) = robots_izni(klasor, "/raporftp/TJKPDF/")?;
    println!("robots.txt: {acik}");
    if !izin {
        return Ok((0, Some(format!("İNDİRME YAPILMADI: {acik}"))));
    }
    let mut yeni = 0;
    for r in bekleyen {
        let hedef = klasor.join(&r.yerel);
        // Kaldığı yerden devam: geçerli dosya varsa ya da daha önce 404 alındıysa atla
        if hedef.exists() {
            if let Ok(b) = std::fs::read(&hedef) {
                if icerik(&b).is_ok_and(|(t, h, ro)| t == r.tarih && slug(&h) == slug(&r.hipodrom) && ro == r.rol) {
                    continue;
                }
            }
        }
        if !yok_yeniden && onceki.get(&r.url).is_some_and(|(s, _)| s == "URL_YOK") {
            continue;
        }
        let part = klasor.join(format!("{}.part", r.yerel));
        let kod = curl(&r.url, &part)?;
        std::thread::sleep(std::time::Duration::from_millis(bekle_ms));
        match kod.as_str() {
            "200" => {
                let b = std::fs::read(&part).map_err(|e| e.to_string())?;
                let sha = sha256::hex(&b);
                match icerik(&b) {
                    Ok((t, h, ro)) if t == r.tarih && slug(&h) == slug(&r.hipodrom) && ro == r.rol => {
                        std::fs::rename(&part, &hedef).map_err(|e| e.to_string())?;
                        yaz(r, "ICERIK_DOGRULANDI", &kod, &sha, &format!("içerik: {t} {h} {ro}"))?;
                        yeni += 1;
                    }
                    Ok((t, h, ro)) => {
                        std::fs::create_dir_all(&red).map_err(|e| e.to_string())?;
                        std::fs::rename(&part, red.join(&r.yerel)).map_err(|e| e.to_string())?;
                        yaz(r, "REDDEDILDI_ICERIK_UYUSMUYOR", &kod, &sha, &format!("beklenen {} {} {}, içerik {t} {h} {ro}", r.tarih, r.hipodrom, r.rol))?;
                    }
                    Err(e) => {
                        std::fs::create_dir_all(&red).map_err(|e| e.to_string())?;
                        std::fs::rename(&part, red.join(&r.yerel)).map_err(|e| e.to_string())?;
                        yaz(r, "REDDEDILDI_BICIM", &kod, &sha, &e)?;
                    }
                }
            }
            "404" | "410" => {
                let _ = std::fs::remove_file(&part);
                yaz(r, "URL_YOK", &kod, "", "adreste dosya yok (yarış olmadığı anlamına gelmez)")?;
            }
            "403" | "429" => {
                let _ = std::fs::remove_file(&part);
                yaz(r, "KAYNAK_ENGELI", &kod, "", "kaynak isteği reddetti; araç durduruldu")?;
                return Ok((yeni, Some(format!("Kaynak HTTP {kod} döndü ({}); engel aşılmaya çalışılmadı, indirme durduruldu.", r.url))));
            }
            k if k.starts_with("000") => {
                let _ = std::fs::remove_file(&part);
                yaz(r, "BAGLANTI_HATASI", "000", "", k)?;
                return Ok((yeni, Some(format!("Bağlantı kurulamadı ({k}); indirme durduruldu. Yeniden çalıştırınca kaldığı yerden devam eder."))));
            }
            k => {
                let _ = std::fs::remove_file(&part);
                yaz(r, "HATA", k, "", "beklenmeyen HTTP kodu; sonraki çalıştırmada yeniden denenir")?;
            }
        }
    }
    Ok((yeni, None))
}

fn indir_komutu(a: &[String]) -> Result<usize, String> {
    let m = secenekler(a)?;
    let liste = liste_oku(gerekli(&m, "liste")?)?;
    let bekle = m.get("bekle-ms").map(|x| x.parse().unwrap_or(2000)).unwrap_or(2000);
    let (n, dur) = indir(&liste, Path::new(gerekli(&m, "klasor")?), bekle, m.contains_key("yok-yeniden"))?;
    println!("Yeni doğrulanan dosya: {n}");
    if let Some(d) = dur {
        println!("{d}");
        return Err(d);
    }
    Ok(n)
}

// ------------------------------------------------------------------ rapor

fn rapor(ay: &str, db: &str, liste: &[Satir], klasor: Option<&Path>, takvim: Option<&str>, cikti: &str) -> Result<String, String> {
    let durum = klasor.map(|k| durum_oku(&k.join("indirme_durumu.tsv"))).unwrap_or_default();
    // DB: bu ayın hipodrom-günleri
    let d = sqlite::Db::ac(db, true)?;
    let onek = ay.replace('-', "");
    let (_, satirlar) = d.sorgu(
        "SELECT tarih, hipodrom, gun_durumu, (SELECT group_concat(neden, ' | ') FROM inceleme_nedeni i WHERE i.gun_id = y.gun_id) FROM yaris_gunu y WHERE gun_id LIKE ?1 ORDER BY 1, 2",
        &[sqlite::Deger::Text(format!("{onek}%"))],
    )?;
    let mut dbde: BTreeMap<(String, String), (String, String)> = BTreeMap::new();
    for r in satirlar {
        dbde.insert((r[0].metin(), slug(&r[1].metin())), (r[2].metin(), r[3].metin()));
    }
    let disarida = d.tek_int("SELECT COUNT(*) FROM yaris_gunu WHERE gun_id NOT LIKE ?1", &[sqlite::Deger::Text(format!("{onek}%"))])?;
    // Resmî takvim (verildiyse): tarih<TAB>hipodrom satırları
    let resmi: Option<Vec<(String, String)>> = match takvim {
        Some(t) => Some(
            std::fs::read_to_string(t)
                .map_err(|e| format!("{t}: {e}"))?
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.starts_with("tarih"))
                .filter_map(|l| l.split_once('\t').map(|(a, b)| (a.trim().to_string(), b.trim().to_string())))
                .collect(),
        ),
        None => None,
    };
    let mut s = String::from("tarih\thipodrom\tresmi_takvimde\tdb_durumu\tprogram_dosyasi\tsonuc_dosyasi\tsinif\tneden\n");
    let mut sayac: BTreeMap<&str, usize> = BTreeMap::new();
    let dosya_durumu = |r: &Satir| -> String {
        if r.url_durumu == "ICERIK_DOGRULANDI" {
            return "ICERIK_DOGRULANDI(elde)".into();
        }
        durum.get(&r.url).map(|(s, _)| s.clone()).unwrap_or_else(|| "INDIRILMEDI(KALIPTAN_URETILDI)".into())
    };
    let mut anahtarlar: Vec<(String, String)> = Vec::new();
    for r in liste {
        let k = (r.tarih.clone(), r.hipodrom.clone());
        if !anahtarlar.contains(&k) {
            anahtarlar.push(k);
        }
    }
    for (t, h) in &anahtarlar {
        let p = liste.iter().find(|r| &r.tarih == t && &r.hipodrom == h && r.rol == "program").map(dosya_durumu).unwrap_or_default();
        let so = liste.iter().find(|r| &r.tarih == t && &r.hipodrom == h && r.rol == "sonuc").map(dosya_durumu).unwrap_or_default();
        let resmide = resmi.as_ref().map(|v| v.iter().any(|(a, b)| a == t && slug(b) == slug(h)));
        let db_d = dbde.get(&(t.clone(), slug(h)));
        let dogru = |x: &str| x.starts_with("ICERIK_DOGRULANDI");
        let (sinif, neden) = match (db_d, resmide) {
            (Some((g, n)), _) if g == "INCELEME_GEREKLI" => ("INCELEMEDE", n.clone()),
            (Some(_), _) => ("MEVCUT", String::new()),
            (None, Some(false)) => continue,
            (None, _) if dogru(&p) && dogru(&so) => ("CIFT_VAR_AKTARILMADI", "doğrulanmış çift var ama veri tabanında yok".into()),
            (None, Some(true)) => ("EKSIK", format!("resmî takvimde var; program: {p}; sonuç: {so}")),
            (None, None) if p == "URL_YOK" && so == "URL_YOK" => ("ARSIVDE_YOK_TAKVIM_DOGRULANMADI", "iki adres de 404; yarış yok denemez".into()),
            (None, None) if dogru(&p) || dogru(&so) => ("TEK_ROL", format!("program: {p}; sonuç: {so}")),
            (None, None) => ("DOGRULANAMADI", format!("program: {p}; sonuç: {so}")),
        };
        *sayac.entry(sinif).or_default() += 1;
        let resmi_s = match resmide {
            Some(true) => "EVET",
            Some(false) => "HAYIR",
            None => "TAKVIM_VERILMEDI",
        };
        let db_s = db_d.map(|(g, _)| g.as_str()).unwrap_or("YOK");
        s.push_str(&format!("{t}\t{h}\t{resmi_s}\t{db_s}\t{p}\t{so}\t{sinif}\t{}\n", neden.replace('\t', " ")));
    }
    std::fs::write(cikti, &s).map_err(|e| format!("{cikti}: {e}"))?;
    let mut ozet = format!("Ay {ay}: ");
    match &resmi {
        Some(v) => ozet.push_str(&format!("resmî takvimde beklenen {} hipodrom-günü; ", v.len())),
        None => ozet.push_str("resmî takvim VERİLMEDİ — beklenen sayı belirlenemedi, ay tamamlandı denemez; "),
    }
    ozet.push_str(&sayac.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(", "));
    ozet.push_str(&format!("; ay dışındaki pilot günler (sayıma katılmadı): {disarida}"));
    if let Some(v) = &resmi {
        let tamam = v.iter().all(|(t, h)| dbde.get(&(t.clone(), slug(h))).is_some_and(|(g, _)| g == "TAMAM"));
        ozet.push_str(if tamam { "; AY TAMAMLANDI (resmî listedeki her gün TAMAM)" } else { "; AY TAMAMLANMADI" });
    }
    Ok(ozet)
}

fn rapor_komutu(a: &[String]) -> Result<(), String> {
    let m = secenekler(a)?;
    let liste = liste_oku(gerekli(&m, "liste")?)?;
    let o = rapor(gerekli(&m, "ay")?, gerekli(&m, "db")?, &liste, m.get("klasor").map(Path::new), m.get("takvim").map(String::as_str), gerekli(&m, "cikti")?)?;
    println!("{o}");
    Ok(())
}

// ------------------------------------------------------------------ hepsi

fn hepsi(a: &[String]) -> Result<(), String> {
    let m = secenekler(a)?;
    let (ay, db, cikti) = (gerekli(&m, "ay")?, gerekli(&m, "db")?, PathBuf::from(gerekli(&m, "cikti")?));
    let klasor = cikti.join("indirilen");
    std::fs::create_dir_all(&klasor).map_err(|e| e.to_string())?;
    let liste_yol = cikti.join("indirme_listesi.tsv");
    let mevcut = m.get("mevcut").map(String::as_str);
    let liste = liste_uret(ay, m.get("gunler").map(String::as_str), mevcut)?;
    liste_yaz(liste_yol.to_str().unwrap(), &liste)?;
    println!("1) İndirme listesi: {} adres -> {}", liste.len(), liste_yol.display());
    let bekle = m.get("bekle-ms").map(|x| x.parse().unwrap_or(2000)).unwrap_or(2000);
    let (n, dur) = indir(&liste, &klasor, bekle, m.contains_key("yok-yeniden"))?;
    println!("2) İndirme: {n} yeni doğrulanmış dosya{}", dur.as_ref().map(|d| format!(" — {d}")).unwrap_or_default());
    // Elde olan doğrulanmış dosyalar aktarım klasörüne KOPYALANIR (asılları değişmez)
    if let Some(k) = mevcut {
        for r in liste.iter().filter(|r| r.url_durumu == "ICERIK_DOGRULANDI") {
            if let Some(ad) = r.kanit.strip_prefix("elde olan ham dosya: ") {
                let h = klasor.join(&r.yerel);
                if !h.exists() {
                    std::fs::copy(Path::new(k).join(ad), &h).map_err(|e| e.to_string())?;
                }
            }
        }
    }
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let toplu = exe.with_file_name(format!("tjk_ayristirici{}", std::env::consts::EXE_SUFFIX));
    let o = Command::new(&toplu)
        .args(["toplu", klasor.to_str().unwrap(), "--db", db, "--ay", ay, "--cikti", cikti.join("aktarim").to_str().unwrap()])
        .output()
        .map_err(|e| format!("{} çalıştırılamadı: {e}", toplu.display()))?;
    std::fs::write(cikti.join("aktarim.txt"), &o.stdout).map_err(|e| e.to_string())?;
    let ozet = String::from_utf8_lossy(&o.stdout).lines().filter(|l| l.starts_with("Özet")).collect::<Vec<_>>().join(" ");
    println!("3) Aktarım: {ozet}");
    let r = rapor(ay, db, &liste, Some(&klasor), m.get("takvim").map(String::as_str), cikti.join("aylik_rapor.tsv").to_str().unwrap())?;
    println!("4) {r}");
    if let Some(d) = dur {
        return Err(d);
    }
    Ok(())
}
