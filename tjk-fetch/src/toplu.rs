//! Mart çalışma listesini sırayla indirir, her CSV'yi içerikten doğrular (mevcut
//! `tjk_ayristirici` kaynaklarındaki ayrıştırıcıyla), iki dosyası doğrulanan çifti
//! `tjk_ayristirici` aracıyla birlikte işler ve raporu yazar. SQLite kullanılmaz.
//!
//! Durdurma: HTTP 403, 429, 502, 503, 504 veya bağlantı kurulamaması (curl 000).
//! Yeniden başlatma: durum dosyasında ICERIK_DOGRULANDI olan ve SHA-256'sı diskteki
//! dosyayla hâlâ eşleşen CSV yeniden indirilmez.

#[path = "../../tjk_ayristirici/src/ayristir.rs"]
#[allow(dead_code)]
mod ayristir;
#[path = "../../tjk_ayristirici/src/kodlama.rs"]
#[allow(dead_code)]
mod kodlama;

use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

const ROBOTS: &str = "https://medya-cdn.tjk.org/robots.txt";
const UA: &str = "tjk-arsiv-araci/0.1 (kisisel arsiv; istekler arasi bekleme; robots.txt'e uyar)";
const DURUM_BASLIK: &str = "zaman_sn\ttarih\thipodrom\trol\turl\thttp\tbayt\tsha256\tdogrulama\tayrinti";
const DURDUR: [&str; 5] = ["403", "429", "502", "503", "504"];

#[derive(Clone)]
struct Satir {
    tarih: String,
    hipodrom: String,
    rol: String,
    url: String,
    yerel: String,
    /// Listede içeriği doğrulanmış olarak işaretli mevcut ham dosya: (dosya adı, sha256)
    mevcut: Option<(String, String)>,
}

#[derive(Clone, Default)]
struct Dosya {
    http: String,
    bayt: String,
    sha: String,
    dogrulama: String,
    ayrinti: String,
    yeniden: bool,
}

fn dur(msg: String) -> ! {
    eprintln!("DUR: {msg}");
    exit(6)
}

fn hex(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
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

/// İçerikten (tarih ISO, hipodrom, rol). Rol kuralı `tjk_indir` ile aynıdır.
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

/// (http kodu, curl hata metni)
fn curl(url: &str, hedef: &Path) -> (String, String) {
    let o = Command::new("curl")
        .args(["-sS", "-L", "--max-time", "60", "-A", UA, "-o"])
        .arg(hedef)
        .args(["-w", "%{http_code}", url])
        .output()
        .unwrap_or_else(|e| dur(format!("curl çalıştırılamadı: {e}")));
    let kod = String::from_utf8_lossy(&o.stdout).trim().to_string();
    let err = String::from_utf8_lossy(&o.stderr).trim().to_string();
    let kod = if kod.is_empty() { "000".into() } else { kod };
    let err = if o.status.success() { err } else { format!("curl çıkış kodu {:?}: {err}", o.status.code()) };
    (kod, err)
}

fn secenekler(a: &[String]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let mut it = a.iter();
    while let Some(x) = it.next() {
        let k = x.strip_prefix("--").unwrap_or_else(|| dur(format!("beklenmeyen argüman {x}")));
        m.insert(k.to_string(), it.next().unwrap_or_else(|| dur(format!("--{k} değer bekliyor"))).clone());
    }
    m
}

/// altili.json içindeki "inceleme_nedenleri" dizisini okur; koda göre gruplar:
/// "KOD ×n (ilk: ...)"; birden çok kod "; " ile ayrılır.
fn neden_ozeti(json: &str) -> String {
    let Some(i) = json.find("\"inceleme_nedenleri\"") else { return String::new() };
    let Some(j) = json[i..].find('[') else { return String::new() };
    let mut it = json[i + j + 1..].chars();
    let mut items: Vec<String> = Vec::new();
    'dis: loop {
        // sonraki dizgeye ya da dizinin sonuna ilerle
        loop {
            match it.next() {
                Some('"') => break,
                Some(']') | None => break 'dis,
                _ => {}
            }
        }
        let mut s = String::new();
        loop {
            match it.next() {
                Some('\\') => match it.next() {
                    Some('n') => s.push(' '),
                    Some(c) => s.push(c),
                    None => break 'dis,
                },
                Some('"') => break,
                Some(c) => s.push(c),
                None => break 'dis,
            }
        }
        items.push(s);
    }
    let mut gruplar: Vec<(String, usize, String)> = Vec::new();
    for s in items {
        let kod = s.split(':').next().unwrap_or("").trim().to_string();
        match gruplar.iter_mut().find(|g| g.0 == kod) {
            Some(g) => g.1 += 1,
            None => gruplar.push((kod, 1, s.clone())),
        }
    }
    gruplar.iter().map(|(k, n, ilk)| format!("{k} ×{n} (ilk: {ilk})")).collect::<Vec<_>>().join("; ")
}

fn simdi() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn run(a: &[String]) {
    let m = secenekler(a);
    let g = |k: &str| m.get(k).cloned().unwrap_or_else(|| dur(format!("--{k} gerekli")));
    let (liste, ham, durum_yol, ayristirici, cikti, rapor) =
        (g("liste"), PathBuf::from(g("ham")), PathBuf::from(g("durum")), g("ayristirici"), PathBuf::from(g("cikti")), g("rapor"));
    // --atla isteğe bağlı: verilirse listede tam bir çift (2 satır) atlanır
    let atla = m.get("atla").cloned();
    let hata_yol = g("hata");
    let beklenen_cift: usize = g("beklenen-cift").parse().unwrap();
    let bekle_ms: u64 = m.get("bekle-ms").map_or(2000, |x| x.parse().unwrap());
    let (atla_t, atla_h) = match &atla {
        Some(a) => a.split_once('|').unwrap_or_else(|| dur("--atla 'TARİH|HİPODROM' olmalı".into())),
        None => ("", ""),
    };
    let beklenen_atlanan = if atla.is_some() { 2 } else { 0 };

    // ---- 1) çalışma listesi ve sayım kontrolü
    let t = std::fs::read_to_string(&liste).unwrap_or_else(|e| dur(format!("{liste}: {e}")));
    let mut lines = t.lines();
    let head: Vec<&str> = lines.next().unwrap_or("").split('\t').collect();
    let ix = |n: &str| head.iter().position(|c| *c == n).unwrap_or_else(|| dur(format!("{liste}: '{n}' sütunu yok")));
    let (it, ih, ir, iu, iy) = (ix("tarih"), ix("hipodrom"), ix("rol"), ix("url"), ix("yerel_dosya"));
    let opt = |n: &str| head.iter().position(|c| *c == n);
    let (id, ish, ik) = (opt("url_durumu"), opt("sha256"), opt("kanit"));
    let tum: Vec<Satir> = lines
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            let al = |i: Option<usize>| i.and_then(|i| c.get(i)).copied().unwrap_or("");
            let mevcut = (al(id) == "ICERIK_DOGRULANDI")
                .then(|| al(ik).strip_prefix("elde olan ham dosya: ").map(|ad| (ad.to_string(), al(ish).to_string())))
                .flatten()
                .filter(|(_, sha)| sha.len() == 64);
            Satir { tarih: c[it].into(), hipodrom: c[ih].into(), rol: c[ir].into(), url: c[iu].into(), yerel: c[iy].into(), mevcut }
        })
        .collect();
    let atlanan: Vec<&Satir> = tum.iter().filter(|s| s.tarih == atla_t && s.hipodrom == atla_h).collect();
    let is: Vec<Satir> = tum.iter().filter(|s| !(s.tarih == atla_t && s.hipodrom == atla_h)).cloned().collect();
    let mut ciftler: BTreeMap<(String, String), BTreeMap<String, Satir>> = BTreeMap::new();
    for s in &is {
        if ciftler.entry((s.tarih.clone(), s.hipodrom.clone())).or_default().insert(s.rol.clone(), s.clone()).is_some() {
            dur(format!("{} {} {}: aynı rol birden fazla", s.tarih, s.hipodrom, s.rol));
        }
    }
    let urller: BTreeSet<&String> = is.iter().map(|s| &s.url).collect();
    println!(
        "Liste: {} satır; atlanan {} {} ({} satır); kalan {} çift, {} satır, {} benzersiz URL",
        tum.len(),
        atla_t,
        atla_h,
        atlanan.len(),
        ciftler.len(),
        is.len(),
        urller.len()
    );
    let eksik_rol: Vec<String> = ciftler
        .iter()
        .filter(|(_, r)| r.len() != 2 || !r.contains_key("program") || !r.contains_key("sonuc"))
        .map(|(k, r)| format!("{} {} roller={:?}", k.0, k.1, r.keys().collect::<Vec<_>>()))
        .collect();
    if atlanan.len() != beklenen_atlanan || ciftler.len() != beklenen_cift || is.len() != 2 * beklenen_cift || urller.len() != is.len() || !eksik_rol.is_empty() {
        dur(format!(
            "sayılar tutmuyor: atlanan satır {} (beklenen {beklenen_atlanan}), çift {} (beklenen {beklenen_cift}), satır {} (beklenen {}), benzersiz URL {} ; rol sorunları: {eksik_rol:?}",
            atlanan.len(),
            ciftler.len(),
            is.len(),
            2 * beklenen_cift,
            urller.len()
        ));
    }

    // ---- önceki durum (yeniden başlatma)
    std::fs::create_dir_all(&ham).unwrap();
    std::fs::create_dir_all(&cikti).unwrap();
    let red = cikti.join("reddedilen");
    if !durum_yol.exists() {
        std::fs::write(&durum_yol, format!("{DURUM_BASLIK}\n")).unwrap();
    }
    let mut onceki: HashMap<String, Vec<String>> = HashMap::new();
    for l in std::fs::read_to_string(&durum_yol).unwrap().lines().skip(1) {
        let c: Vec<String> = l.split('\t').map(str::to_string).collect();
        if c.len() == 10 {
            onceki.insert(c[4].clone(), c); // son kayıt geçerli
        }
    }
    let kaydet = |s: &Satir, d: &Dosya| {
        let mut f = std::fs::OpenOptions::new().append(true).open(&durum_yol).unwrap();
        writeln!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            simdi(),
            s.tarih,
            s.hipodrom,
            s.rol,
            s.url,
            d.http,
            d.bayt,
            d.sha,
            d.dogrulama,
            d.ayrinti.replace(['\t', '\n', '\r'], " ")
        )
        .unwrap();
    };

    // ---- robots.txt
    let mut erisim_hatasi: Option<String> = None;
    let rb = cikti.join(".robots.txt");
    let (rk, re) = curl(ROBOTS, &rb);
    let govde = std::fs::read_to_string(&rb).unwrap_or_default();
    let robots_notu = match rk.as_str() {
        "404" | "410" => format!("HTTP {rk}: robots.txt yok, kural tanımlı değil (RFC 9309: izinli)"),
        "200" => {
            let (mut grupta, mut en) = (false, (0usize, true));
            for l in govde.lines().map(|l| l.split('#').next().unwrap_or("").trim()) {
                let Some((k, v)) = l.split_once(':') else { continue };
                let (k, v) = (k.trim().to_ascii_lowercase(), v.trim());
                let yol = "/raporftp/TJKPDF/";
                match k.as_str() {
                    "user-agent" => grupta = v == "*",
                    "disallow" if grupta && !v.is_empty() && yol.starts_with(v) && v.len() >= en.0 => en = (v.len(), false),
                    "allow" if grupta && !v.is_empty() && yol.starts_with(v) && v.len() >= en.0 => en = (v.len(), true),
                    _ => {}
                }
            }
            if !en.1 {
                erisim_hatasi = Some("robots.txt /raporftp/TJKPDF/ yolunu yasaklıyor; hiçbir veri isteği yapılmadı".into());
            }
            format!("HTTP 200, /raporftp/TJKPDF/ izinli={}", en.1)
        }
        k => {
            erisim_hatasi = Some(format!("robots.txt okunamadı: HTTP {k} {re}\n{govde}"));
            format!("HTTP {k}")
        }
    };
    println!("robots.txt: {robots_notu}");

    // ---- 2-3) çiftler
    let mut sonuc: Vec<((String, String), String, String, BTreeMap<String, Dosya>)> = Vec::new();
    let mut ilk_istek = true;
    for (k, roller) in &ciftler {
        if erisim_hatasi.is_some() {
            sonuc.push((k.clone(), "HENUZ_DENENMEDI".into(), String::new(), BTreeMap::new()));
            continue;
        }
        let mut dosyalar: BTreeMap<String, Dosya> = BTreeMap::new();
        let mut yollar: BTreeMap<&str, PathBuf> = BTreeMap::new();
        for rol in ["program", "sonuc"] {
            let s = &roller[rol];
            let hedef = ham.join(&s.yerel);
            yollar.insert(rol, hedef.clone());
            // depoda zaten olan ham dosya: listedeki SHA-256 hâlâ eşleşiyor ve içerik (tarih, hipodrom, rol) doğruysa indirilmez
            if let Some((ad, sha)) = &s.mevcut {
                let yol = ham.join(ad);
                if let Ok(b) = std::fs::read(&yol) {
                    let ic = icerik(&b);
                    let uygun = matches!(&ic, Ok((t, h, ro)) if *t == s.tarih && slug(h) == slug(&s.hipodrom) && *ro == s.rol);
                    if hex(&b) == *sha && uygun {
                        println!("  {} {} {:<7} depodaki ham dosya: {ad} ({} bayt, SHA-256 eşleşti)", s.tarih, s.hipodrom, rol, b.len());
                        dosyalar.insert(
                            rol.into(),
                            Dosya {
                                http: "-".into(),
                                bayt: b.len().to_string(),
                                sha: sha.clone(),
                                dogrulama: "ICERIK_DOGRULANDI".into(),
                                ayrinti: format!("depodaki ham dosya yeniden kullanıldı: {ad} (SHA-256 eşleşti)"),
                                yeniden: true,
                            },
                        );
                        yollar.insert(rol, yol);
                        continue;
                    }
                }
            }
            // yeniden kullan: önceki kayıt doğrulanmış ve SHA hâlâ eşleşiyor
            if let (Some(c), Ok(b)) = (onceki.get(&s.url), std::fs::read(&hedef)) {
                if c[8] == "ICERIK_DOGRULANDI" && c[7] == hex(&b) {
                    dosyalar.insert(
                        rol.into(),
                        Dosya { http: c[5].clone(), bayt: b.len().to_string(), sha: c[7].clone(), dogrulama: c[8].clone(), ayrinti: c[9].clone(), yeniden: true },
                    );
                    continue;
                }
            }
            if !ilk_istek {
                std::thread::sleep(std::time::Duration::from_millis(bekle_ms));
            }
            ilk_istek = false;
            let part = cikti.join(format!("{}.part", s.yerel));
            let (kod, err) = curl(&s.url, &part);
            let b = std::fs::read(&part).unwrap_or_default();
            let mut d = Dosya { http: kod.clone(), bayt: b.len().to_string(), sha: if b.is_empty() { String::new() } else { hex(&b) }, ..Default::default() };
            if kod == "000" || DURDUR.contains(&kod.as_str()) {
                d.dogrulama = "ERISIM_HATASI".into();
                d.ayrinti = format!("HTTP {kod}; {err}; yanıt: {}", String::from_utf8_lossy(&b).chars().take(2000).collect::<String>());
                erisim_hatasi = Some(format!("{} {} {}\nURL: {}\nHTTP: {kod}\ncurl: {err}\nYanıt gövdesi:\n{}", s.tarih, s.hipodrom, rol, s.url, String::from_utf8_lossy(&b)));
                let _ = std::fs::remove_file(&part);
                kaydet(s, &d);
                dosyalar.insert(rol.into(), d);
                break;
            }
            match kod.as_str() {
                "200" => match icerik(&b) {
                    Ok((t, h, ro)) if t == s.tarih && slug(&h) == slug(&s.hipodrom) && ro == s.rol => {
                        std::fs::rename(&part, &hedef).unwrap();
                        d.dogrulama = "ICERIK_DOGRULANDI".into();
                        d.ayrinti = format!("içerik: {t} {h} {ro}");
                    }
                    Ok((t, h, ro)) => {
                        std::fs::create_dir_all(&red).unwrap();
                        std::fs::rename(&part, red.join(&s.yerel)).unwrap();
                        d.dogrulama = "ICERIK_UYUSMUYOR".into();
                        d.ayrinti = format!("beklenen {} {} {}, içerik {t} {h} {ro}", s.tarih, s.hipodrom, s.rol);
                    }
                    Err(e) => {
                        std::fs::create_dir_all(&red).unwrap();
                        std::fs::rename(&part, red.join(&s.yerel)).unwrap();
                        d.dogrulama = "BICIM_HATASI".into();
                        d.ayrinti = e;
                    }
                },
                "404" | "410" => {
                    let _ = std::fs::remove_file(&part);
                    d.dogrulama = "URL_YOK".into();
                    d.ayrinti = "adreste dosya yok".into();
                }
                _ => {
                    let _ = std::fs::remove_file(&part);
                    d.dogrulama = "BEKLENMEYEN_HTTP".into();
                    d.ayrinti = format!("HTTP {kod} {err}");
                }
            }
            println!("  {} {} {:<7} HTTP {} {:>6} bayt {}", s.tarih, s.hipodrom, rol, d.http, d.bayt, d.dogrulama);
            kaydet(s, &d);
            dosyalar.insert(rol.into(), d);
        }

        // çift durumu
        let dog = |r: &str| dosyalar.get(r).map(|d| d.dogrulama.as_str()).unwrap_or("");
        let (durum, neden) = if dosyalar.values().any(|d| d.dogrulama == "ERISIM_HATASI") {
            ("INCELEME_GEREKLI".to_string(), "erişim hatasıyla yarıda kaldı".to_string())
        } else if let Some(d) = dosyalar.iter().find(|(_, d)| !matches!(d.dogrulama.as_str(), "ICERIK_DOGRULANDI" | "URL_YOK")) {
            ("INCELEME_GEREKLI".into(), format!("{}: {} ({})", d.0, d.1.dogrulama, d.1.ayrinti))
        } else if dosyalar.values().any(|d| d.dogrulama == "URL_YOK") {
            let eks: Vec<&str> = ["program", "sonuc"].into_iter().filter(|r| dog(r) == "URL_YOK").collect();
            ("EKSIK".into(), format!("CSV yok (HTTP 404): {}", eks.join(", ")))
        } else {
            // birlikte ayrıştır
            let od = cikti.join(format!("{}-{}", k.0, slug(&k.1)));
            let o = Command::new(&ayristirici)
                .arg(&yollar["program"])
                .arg(&yollar["sonuc"])
                .arg(&od)
                .output()
                .unwrap_or_else(|e| dur(format!("{ayristirici}: {e}")));
            let out = String::from_utf8_lossy(&o.stdout).to_string();
            std::fs::create_dir_all(&od).ok();
            std::fs::write(od.join("ayristirici_cikti.txt"), format!("{out}\n--- stderr ---\n{}", String::from_utf8_lossy(&o.stderr))).ok();
            let gd = out.lines().find_map(|l| l.strip_prefix("GÜN DURUMU")).map(|l| l.trim_start_matches([' ', ':']).trim().to_string());
            if o.status.success() && gd.as_deref() == Some("TAMAM") {
                ("TAMAMLANDI".into(), String::new())
            } else {
                let json = std::fs::read_to_string(od.join("altili.json")).unwrap_or_default();
                let nedenler = neden_ozeti(&json);
                let err = String::from_utf8_lossy(&o.stderr).trim().to_string();
                ("INCELEME_GEREKLI".into(), format!("ayrıştırıcı gün durumu {}: {nedenler} {err}", gd.unwrap_or_default()).trim().to_string())
            }
        };
        println!("{} {}: {durum} {neden}", k.0, k.1);
        sonuc.push((k.clone(), durum, neden, dosyalar));
    }

    if let Some(h) = &erisim_hatasi {
        std::fs::write(&hata_yol, format!("{h}\n")).unwrap();
        eprintln!("ERİŞİM HATASI, indirme durduruldu:\n{h}");
    }

    // ---- 4) rapor
    let say = |d: &str| sonuc.iter().filter(|x| x.1 == d).count();
    let (ta, ig, ek, hd) = (say("TAMAMLANDI"), say("INCELEME_GEREKLI"), say("EKSIK"), say("HENUZ_DENENMEDI"));
    assert_eq!(ta + ig + ek + hd, beklenen_cift);
    let mut r = String::new();
    r += "# Son veri testi\n\n";
    r += &format!(
        "- **Durum:** {}\n- **Çalışma:** Mart 2026 toplu indirme, `{liste}`{}\n",
        if erisim_hatasi.is_some() { "ERİŞİM HATASIYLA DURDU" } else { "TAMAMLANDI (liste sonuna kadar işlendi)" },
        if atla.is_some() { format!(" ({atla_t} {atla_h} önceden doğrulandığı için atlandı)") } else { String::new() }
    );
    r += "- **Araçlar:** `tjk-fetch toplu` (curl ile HTTPS; içerik doğrulaması `tjk_ayristirici/src/ayristir.rs` ile), birlikte ayrıştırma `tjk_ayristirici`\n";
    r += "- **SQLite:** kullanılmadı\n";
    r += &format!("- **Ön kontrol:** {} çift, {} satır, {} benzersiz URL; her çiftte bir program ve bir sonuç\n", ciftler.len(), is.len(), urller.len());
    r += &format!("- **robots.txt:** {robots_notu}\n\n");
    r += &format!("## Özet (bu çalışmadaki {beklenen_cift} çift)\n\n| Durum | Çift |\n|---|---|\n");
    r += &format!("| Tamamlandı | {ta} |\n| İnceleme gerekli | {ig} |\n| Eksik | {ek} |\n| Henüz denenmedi | {hd} |\n| **Toplam** | **{}** |\n\n", ta + ig + ek + hd);
    let yeniden = sonuc.iter().flat_map(|x| x.3.values()).filter(|d| d.yeniden).count();
    r += &format!("Önceki çalışmadan SHA-256'sı eşleşerek yeniden kullanılan dosya: {yeniden}\n\n");
    if let Some(h) = &erisim_hatasi {
        r += &format!("## Erişim hatası (tam mesaj, ayrıca `kontrol/son_hata.txt`)\n\n```\n{h}\n```\n\n");
    }
    r += "## Çiftler\n\n| # | Tarih | Hipodrom | Durum | Gerekçe |\n|---|---|---|---|---|\n";
    for (i, (k, d, n, _)) in sonuc.iter().enumerate() {
        r += &format!("| {} | {} | {} | {} | {} |\n", i + 1, k.0, k.1, d, n.replace('|', "\\|"));
    }
    r += "\n## Dosyalar\n\n| Tarih | Hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | Doğrulama |\n|---|---|---|---|---|---|---|---|\n";
    for (k, _, _, ds) in &sonuc {
        for (rol, d) in ds {
            let url = &ciftler[k][rol].url;
            r += &format!(
                "| {} | {} | {} | {} | {} | {} | `{}` | {}{} |\n",
                k.0,
                k.1,
                rol,
                url,
                d.http,
                d.bayt,
                d.sha,
                d.dogrulama,
                if d.yeniden { " (yeniden kullanıldı)" } else { "" }
            );
        }
    }
    r += &format!(
        "\nDoğrulanan ham CSV'ler `{}` altında (`yerel_dosya` adlarıyla). Ayrıntılı kayıt: `{}`.\n",
        ham.display(),
        durum_yol.display()
    );
    std::fs::write(&rapor, r).unwrap();
    println!("\nÖZET: tamamlandı {ta}, inceleme gerekli {ig}, eksik {ek}, henüz denenmedi {hd} (toplam {})", ta + ig + ek + hd);
    if erisim_hatasi.is_some() {
        exit(3);
    }
}
