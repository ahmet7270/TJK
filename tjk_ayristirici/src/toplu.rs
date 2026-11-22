//! Toplu aktarım: bir klasördeki ham TJK CSV'lerini İÇERİKLERİNE göre (ilk satırdaki tarih ve hipodrom,
//! kolon başlıklarındaki rol) program–sonuç çiftlerine ayırır; envanter çıkarır; çalışma kopyasına gün gün aktarır.
//!
//! Kullanım:
//!   tjk_ayristirici toplu KLASOR --envanter [--db VERITABANI] [--ay YYYY-AA] [--cikti KLASOR]
//!   tjk_ayristirici toplu KLASOR --db CALISMA.sqlite [--arsivden ARSIV.sqlite] --cikti KLASOR [--ay YYYY-AA]
//!
//! Kurallar:
//!   * Dosya adı (yükleme öneki dahil) eşleşmede kullanılmaz; yalnızca raporlanır.
//!   * Aynı (tarih, hipodrom, rol) için içeriği farklı iki aday varsa seçim yapılmaz: ÇAKIŞMA.
//!     İçeriği aynı (SHA-256 aynı) kopyalar çakışma sayılmaz, raporlanır.
//!   * Dosyası olmayan günler için yarış yapılıp yapılmadığı çıkarılmaz; kapsama oranı resmî takvim olmadan hesaplanmaz.
//!   * --arsivden verilirse arşiv yalnızca okunur; çalışma kopyası yoksa arşivden kopyalanır, varsa üzerine yazılmaz.

use crate::json::{obj, Json};
use crate::kodlama;
use crate::sha256;
use crate::sqlite::Db;
use crate::veritabani::{self, Karar};
use crate::{ayristir, gunu_isle, hipodrom_slug, yukle, Secenekler, AYRISTIRICI_SURUMU};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const KULLANIM: &str = "Kullanım:\n  tjk_ayristirici toplu KLASOR --envanter [--db VERITABANI] [--ay YYYY-AA] [--cikti KLASOR]\n  \
tjk_ayristirici toplu KLASOR --db CALISMA.sqlite [--arsivden ARSIV.sqlite] --cikti KLASOR [--ay YYYY-AA]";
const KAPSAMA_NOTU: &str = "Kapsama oranı hesaplanmadı: resmî TJK yarış takvimi bu çalıştırmaya verilmedi. \
Dosyası olmayan bir gün için yarış yapılıp yapılmadığı klasörden çıkarılmaz.";

struct Secim {
    klasor: String,
    envanter: bool,
    db: Option<String>,
    arsivden: Option<String>,
    ay: Option<String>,
    cikti: Option<String>,
}

#[derive(Clone)]
struct Aday {
    dosya: String,
    yol: PathBuf,
    sha: String,
    rol: &'static str,
    tarih: String,
    hipodrom: String,
    yaris_gunu_no: u32,
}

#[derive(Default)]
struct Grup {
    program: Vec<Aday>,
    sonuc: Vec<Aday>,
}

fn secim_oku(a: &[String]) -> Result<Secim, String> {
    let mut s = Secim { klasor: String::new(), envanter: false, db: None, arsivden: None, ay: None, cikti: None };
    let mut it = a.iter();
    let mut poz = Vec::new();
    while let Some(x) = it.next() {
        let mut deger = |ad: &str| it.next().cloned().ok_or_else(|| format!("{ad} bir değer bekliyor\n{KULLANIM}"));
        match x.as_str() {
            "--envanter" => s.envanter = true,
            "--db" => s.db = Some(deger(x)?),
            "--arsivden" => s.arsivden = Some(deger(x)?),
            "--ay" => s.ay = Some(deger(x)?),
            "--cikti" => s.cikti = Some(deger(x)?),
            "-h" | "--help" => return Err(KULLANIM.into()),
            o if o.starts_with("--") => return Err(format!("bilinmeyen seçenek: {o}\n{KULLANIM}")),
            _ => poz.push(x.clone()),
        }
    }
    if poz.len() != 1 {
        return Err(KULLANIM.into());
    }
    s.klasor = poz.remove(0);
    if let Some(ay) = &s.ay {
        let ok = ay.len() == 7 && ay.as_bytes()[4] == b'-' && ay[..4].bytes().all(|b| b.is_ascii_digit())
            && ay[5..].bytes().all(|b| b.is_ascii_digit());
        if !ok {
            return Err(format!("--ay YYYY-AA biçiminde olmalı: {ay}"));
        }
    }
    if !s.envanter && (s.db.is_none() || s.cikti.is_none()) {
        return Err(format!("Aktarım için --db ve --cikti gerekli (yalnızca envanter için --envanter).\n{KULLANIM}"));
    }
    Ok(s)
}

/// Dosyanın rolünü içerikten çıkarır: program kolonlarında "Son 6 Yarış", sonuç kolonlarında "Derece;Ganyan;Fark".
fn rol_bul(g: &ayristir::Gun) -> Option<&'static str> {
    let k = g.kosular.first()?;
    let has = |x: &str| k.kolonlar.iter().any(|c| c == x);
    match (has("Son 6 Yarış"), has("Derece") && has("Ganyan") && has("Fark")) {
        (true, false) => Some("program"),
        (false, true) => Some("sonuc"),
        _ => None,
    }
}

fn tara(klasor: &str) -> Result<(Vec<Aday>, Vec<(String, String)>), String> {
    let mut yollar: Vec<PathBuf> = std::fs::read_dir(klasor)
        .map_err(|e| format!("{klasor} okunamadı: {e}"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x.eq_ignore_ascii_case("csv")))
        .collect();
    yollar.sort();
    let mut adaylar = Vec::new();
    let mut taninmayan = Vec::new();
    for y in yollar {
        let dosya = y.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let bytes = std::fs::read(&y).map_err(|e| format!("{} okunamadı: {e}", y.display()))?;
        let (metin, _) = kodlama::coz(&bytes);
        match ayristir::gun_ayristir(&metin) {
            Ok(g) => match rol_bul(&g) {
                Some(rol) => adaylar.push(Aday {
                    dosya,
                    yol: y.clone(),
                    sha: sha256::hex(&bytes),
                    rol,
                    tarih: g.tarih.clone(),
                    hipodrom: g.hipodrom.clone(),
                    yaris_gunu_no: g.yaris_gunu_no,
                }),
                None => taninmayan.push((dosya, "kolon başlıklarından rol (program/sonuç) çıkarılamadı".into())),
            },
            Err(e) => taninmayan.push((dosya, format!("TJK günlük CSV biçiminde değil: {e}"))),
        }
    }
    Ok((adaylar, taninmayan))
}

/// Aynı SHA-256'lı adayları tek adaya indirger; (tekil adaylar, aynı içerik kopyalarının adları)
fn tekille(v: &[Aday]) -> (Vec<Aday>, Vec<String>) {
    let mut tek: Vec<Aday> = Vec::new();
    let mut kopya = Vec::new();
    for a in v {
        match tek.iter().find(|t| t.sha == a.sha) {
            Some(t) => kopya.push(format!("{} = {} (aynı içerik)", a.dosya, t.dosya)),
            None => tek.push(a.clone()),
        }
    }
    (tek, kopya)
}

fn aday_json(v: &[Aday]) -> Json {
    Json::Arr(
        v.iter()
            .map(|a| obj(vec![("dosya", (&a.dosya).into()), ("sha256", (&a.sha).into()), ("yaris_gunu_no", a.yaris_gunu_no.into())]))
            .collect(),
    )
}

/// Veri tabanındaki mevcut durum (salt okunur): YOK / AYNI / DEGISMIS / SURUM_FARKI
fn db_durumu(db: Option<&Db>, gun_id: &str, psha: &str, ssha: &str) -> Result<String, String> {
    let Some(db) = db else { return Ok("VERITABANI_VERILMEDI".into()) };
    let (_, r) = db.sorgu(
        "SELECT g.gun_durumu, kp.sha256, ks.sha256, g.ayristirici_surumu FROM yaris_gunu g
           JOIN kaynak_dosya kp ON kp.kaynak_id = g.program_kaynak_id
           JOIN kaynak_dosya ks ON ks.kaynak_id = g.sonuc_kaynak_id WHERE g.gun_id = ?",
        &[gun_id.into()],
    )?;
    Ok(match r.first() {
        None => "VERITABANINDA_YOK".into(),
        Some(m) if m[1].metin() == psha && m[2].metin() == ssha && m[3].metin() == AYRISTIRICI_SURUMU => {
            format!("AYNI_GIRDI_AYNI_SURUM (mevcut: {}) → atlanacak", m[0].metin())
        }
        Some(m) if m[1].metin() == psha && m[2].metin() == ssha => {
            format!("SURUM_FARKI (mevcut sürüm {}, mevcut durum {})", m[3].metin(), m[0].metin())
        }
        Some(m) => format!("DEGISMIS_GIRDI (mevcut durum {})", m[0].metin()),
    })
}

fn dosya_sha(yol: &str) -> Option<String> {
    std::fs::read(yol).ok().map(|b| sha256::hex(&b))
}

fn calisma_kopyasi(sec: &Secim, notlar: &mut Vec<String>) -> Result<(), String> {
    let (Some(db), Some(arsiv)) = (&sec.db, &sec.arsivden) else { return Ok(()) };
    let arsiv_k = std::fs::canonicalize(arsiv).map_err(|e| format!("arşiv bulunamadı ({arsiv}): {e}"))?;
    if let Ok(db_k) = std::fs::canonicalize(db) {
        if db_k == arsiv_k {
            return Err("Çalışma veri tabanı arşivin kendisi olamaz: --db ile --arsivden aynı dosya.".into());
        }
        notlar.push(format!("Çalışma kopyası zaten var, üzerine yazılmadı: {db}"));
        return Ok(());
    }
    for ek in ["-journal", "-wal"] {
        if Path::new(&format!("{arsiv}{ek}")).exists() {
            return Err(format!("Arşivin yanında {ek} dosyası var (yarım işlem olabilir); kopyalanmadı."));
        }
    }
    std::fs::copy(arsiv, db).map_err(|e| format!("çalışma kopyası oluşturulamadı: {e}"))?;
    notlar.push(format!("Çalışma kopyası arşivden oluşturuldu: {arsiv} → {db}"));
    Ok(())
}

pub fn calistir(args: &[String]) -> Result<(), String> {
    let sec = secim_oku(args)?;
    let (adaylar, taninmayan) = tara(&sec.klasor)?;
    let mut gruplar: BTreeMap<(String, String), Grup> = BTreeMap::new();
    let mut ay_disi = 0usize;
    for a in adaylar {
        if sec.ay.as_ref().is_some_and(|ay| !a.tarih.starts_with(ay.as_str())) {
            ay_disi += 1;
            continue;
        }
        let g = gruplar.entry((a.tarih.clone(), a.hipodrom.clone())).or_default();
        if a.rol == "program" { g.program.push(a) } else { g.sonuc.push(a) }
    }

    let mut notlar: Vec<String> = Vec::new();
    let arsiv_once = sec.arsivden.as_deref().and_then(dosya_sha);
    if !sec.envanter {
        calisma_kopyasi(&sec, &mut notlar)?;
    }
    // Envanterde veri tabanı yalnızca okunur ve dosya yoksa oluşturulmaz.
    let db_oku = match &sec.db {
        Some(y) if Path::new(y).exists() => Some(Db::ac(y, true)?),
        _ => None,
    };

    println!("== TOPLU {} | klasör: {}", if sec.envanter { "ENVANTER" } else { "AKTARIM" }, sec.klasor);
    if let Some(ay) = &sec.ay {
        println!("Ay filtresi: {ay} (filtre dışında kalan aday dosya: {ay_disi})");
    }
    for n in &notlar {
        println!("{n}");
    }
    let mut gun_json = Vec::new();
    let mut sayim: BTreeMap<String, usize> = BTreeMap::new();
    for ((tarih, hipodrom), g) in &gruplar {
        let gun_id = format!("{}-{}", tarih.replace('-', ""), hipodrom_slug(hipodrom));
        let (pt, pk) = tekille(&g.program);
        let (st, sk) = tekille(&g.sonuc);
        let mut kopyalar = pk;
        kopyalar.extend(sk);
        let durum = if pt.len() > 1 || st.len() > 1 {
            "CAKISMA"
        } else if pt.is_empty() {
            "PROGRAM_EKSIK"
        } else if st.is_empty() {
            "SONUC_EKSIK"
        } else if pt[0].yaris_gunu_no != st[0].yaris_gunu_no {
            "CAKISMA_YARIS_GUNU_NO"
        } else {
            "HAZIR"
        };
        let mut satir = format!("{tarih} {hipodrom:<10} {gun_id:<22} {durum:<22}");
        let mut alanlar = vec![
            ("tarih", tarih.into()),
            ("hipodrom", hipodrom.into()),
            ("gun_id", (&gun_id).into()),
            ("envanter_durumu", durum.into()),
            ("program_adaylari", aday_json(&pt)),
            ("sonuc_adaylari", aday_json(&st)),
            ("ayni_icerik_kopyalari", kopyalar.clone().into()),
        ];
        if durum == "HAZIR" {
            let dbd = db_durumu(db_oku.as_ref(), &gun_id, &pt[0].sha, &st[0].sha)?;
            satir.push_str(&format!(" DB: {dbd}"));
            alanlar.push(("veritabani_durumu", dbd.into()));
        }
        if durum.starts_with("CAKISMA") {
            for a in pt.iter().chain(st.iter()) {
                satir.push_str(&format!("\n      aday {:<7} {} sha={}", a.rol, a.dosya, &a.sha[..16]));
            }
            satir.push_str("\n      → seçim yapılmadı, aktarılmadı");
        }
        for k in &kopyalar {
            satir.push_str(&format!("\n      kopya: {k}"));
        }

        // ---- aktarım
        if !sec.envanter {
            let karar_kodu: String;
            if durum == "HAZIR" {
                let db = sec.db.clone().unwrap();
                let cikti = Path::new(sec.cikti.as_ref().unwrap()).join(&gun_id).to_string_lossy().into_owned();
                let gs = Secenekler {
                    program: pt[0].yol.to_string_lossy().into_owned(),
                    sonuc: st[0].yol.to_string_lossy().into_owned(),
                    cikti,
                    kontrol_komb: vec![],
                    kontrol_odeme: vec![],
                    ilgili_url: None,
                    ilgili_not: None,
                    sqlite: None,
                    ek_sonuc: vec![],
                    ertelenme_notu: None,
                };
                let sonuc = (|| -> Result<(Karar, String), String> {
                    let p = yukle(&gs.program)?;
                    let s = yukle(&gs.sonuc)?;
                    gunu_isle(&gs, p, s, vec![], false, |gv| Ok((veritabani::aktar_kosullu(&db, gv)?, gv.gun_durumu.to_string())))
                })();
                match sonuc {
                    Ok((k, yeni_durum)) => {
                        karar_kodu = k.kod().to_string();
                        let mut ek: Vec<(&str, Json)> =
                            vec![("karar", k.kod().into()), ("yeni_verinin_gun_durumu", yeni_durum.clone().into())];
                        match &k {
                            Karar::Yeni(o) | Karar::SurumFarki { ozet: o, .. } | Karar::DegismisGuncellendi { ozet: o, .. } => {
                                ek.push(("katilim", o.katilim.into()));
                                ek.push(("altili", o.altili.into()));
                            }
                            _ => {}
                        }
                        match &k {
                            Karar::DegismisGuncellendi { eski_program_sha, eski_sonuc_sha, .. } => {
                                ek.push(("eski_program_sha256", eski_program_sha.into()));
                                ek.push(("eski_sonuc_sha256", eski_sonuc_sha.into()));
                            }
                            Karar::KorunduInceleme { neden, eski_program_sha, eski_sonuc_sha } => {
                                ek.push(("koruma_nedeni", (*neden).into()));
                                ek.push(("mevcut_program_sha256", eski_program_sha.into()));
                                ek.push(("mevcut_sonuc_sha256", eski_sonuc_sha.into()));
                            }
                            Karar::SurumFarki { eski_surum, .. } => ek.push(("eski_ayristirici_surumu", eski_surum.into())),
                            _ => {}
                        }
                        satir.push_str(&format!("\n      AKTARIM: {} (yeni veri: {yeni_durum})", k.kod()));
                        alanlar.push(("aktarim", obj(ek)));
                    }
                    Err(e) => {
                        karar_kodu = "HATA_GERI_ALINDI".into();
                        satir.push_str(&format!("\n      AKTARIM: HATA_GERI_ALINDI — {e}"));
                        alanlar.push(("aktarim", obj(vec![("karar", "HATA_GERI_ALINDI".into()), ("hata", e.into())])));
                    }
                }
            } else {
                karar_kodu = "AKTARILMADI".into();
                satir.push_str("\n      AKTARIM: AKTARILMADI (çift hazır değil)");
                alanlar.push(("aktarim", obj(vec![("karar", "AKTARILMADI".into())])));
            }
            *sayim.entry(format!("aktarim:{karar_kodu}")).or_default() += 1;
        }
        *sayim.entry(format!("envanter:{durum}")).or_default() += 1;
        println!("{satir}");
        gun_json.push(obj(alanlar));
    }
    for (d, neden) in &taninmayan {
        println!("TANINMAYAN dosya: {d} — {neden}");
    }
    drop(db_oku);
    let arsiv_sonra = sec.arsivden.as_deref().and_then(dosya_sha);
    println!("Özet: {}", sayim.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(", "));
    println!("{KAPSAMA_NOTU}");
    if let (Some(o), Some(s)) = (&arsiv_once, &arsiv_sonra) {
        println!("Arşiv SHA-256 önce/sonra: {}… / {}… ({})", &o[..16], &s[..16], if o == s { "DEĞİŞMEDİ" } else { "DEĞİŞTİ!" });
    }

    let rapor = obj(vec![
        ("mod", (if sec.envanter { "ENVANTER" } else { "AKTARIM" }).into()),
        ("klasor", (&sec.klasor).into()),
        ("ay_filtresi", sec.ay.clone().into()),
        ("veritabani", sec.db.clone().into()),
        ("arsiv", sec.arsivden.clone().into()),
        ("arsiv_sha256_once", arsiv_once.clone().into()),
        ("arsiv_sha256_sonra", arsiv_sonra.clone().into()),
        ("ayristirici_surumu", AYRISTIRICI_SURUMU.into()),
        ("notlar", notlar.into()),
        ("kapsama_notu", KAPSAMA_NOTU.into()),
        ("gunler", Json::Arr(gun_json)),
        (
            "taninmayan_dosyalar",
            Json::Arr(taninmayan.iter().map(|(d, n)| obj(vec![("dosya", d.into()), ("neden", n.into())])).collect()),
        ),
        ("ozet", Json::Obj(sayim.iter().map(|(k, v)| (k.clone(), Json::Int(*v as i64))).collect())),
    ]);
    if let Some(c) = &sec.cikti {
        std::fs::create_dir_all(c).map_err(|e| format!("{c} oluşturulamadı: {e}"))?;
        let ad = if sec.envanter { "envanter.json" } else { "toplu_rapor.json" };
        std::fs::write(Path::new(c).join(ad), rapor.guzel()).map_err(|e| format!("{ad} yazılamadı: {e}"))?;
    }
    if arsiv_once.is_some() && arsiv_once != arsiv_sonra {
        return Err("Arşiv dosyası değişti! (beklenmeyen durum)".into());
    }
    Ok(())
}
