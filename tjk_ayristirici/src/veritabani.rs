//! Bir yarış gününü SQLite veri tabanına TEK İŞLEMDE aktarır.
//! * Gün önce silinir (ON DELETE CASCADE) sonra yeniden yazılır: aynı girdiyle tekrar aktarım kayıt sayısını değiştirmez.
//! * Hata olursa ROLLBACK: o güne ait kısmi kayıt kalmaz; daha önce aktarılmış sürüm korunur.
//! * Test için yapay kesinti: TJK_TEST_KESINTI_SONRA=N ortam değişkeni N. eklemeden sonra hata üretir.

use crate::json::Json;
use crate::sqlite::{Db, Deger};
use crate::{kosu_altililari, GunVerisi};
use std::cell::Cell;

pub const SEMA: &str = include_str!("sema.sql");

pub struct Ozet {
    pub odeme: usize,
    pub kosu: usize,
    pub katilim: usize,
    pub sonuc: usize,
    pub altili: usize,
    pub ayak: usize,
    pub gecerli: usize,
    pub agf: usize,
}

struct Sayac {
    n: Cell<u64>,
    sinir: Option<u64>,
}

impl Sayac {
    fn ekle(&self) -> Result<(), String> {
        self.n.set(self.n.get() + 1);
        match self.sinir {
            Some(s) if self.n.get() > s => Err(format!("TEST: yapay kesinti ({s} eklemeden sonra)")),
            _ => Ok(()),
        }
    }
}

static BOS: Json = Json::Null;

fn j<'a>(v: &'a Json, k: &str) -> &'a Json {
    match v {
        Json::Obj(a) => a.iter().find(|(x, _)| x == k).map(|(_, y)| y).unwrap_or(&BOS),
        _ => &BOS,
    }
}

fn d(v: &Json) -> Deger {
    match v {
        Json::Null => Deger::Null,
        Json::Bool(b) => Deger::Int(*b as i64),
        Json::Int(i) => Deger::Int(*i),
        Json::Float(f) => Deger::Real(*f),
        Json::Str(s) => Deger::Text(s.clone()),
        other => Deger::Text(other.guzel()),
    }
}

fn al(s: &[(&'static str, Json)], k: &str) -> Deger {
    s.iter().find(|(x, _)| *x == k).map(|(_, v)| d(v)).unwrap_or(Deger::Null)
}

pub const SEMA_SURUMU: i64 = 4;

/// Şema sürüm 1 → 2: bahis_sonucu yeniden kurulur (ödeme alanları NULL olabilir; sonuç durumu, devir, ham ibare),
/// bahis_uyarisi eklenir, altili'ya sonuç durumu ve devir eklenir. Hiçbir satır silinmez.
/// v1 ayrıştırıcısı yalnızca sayısal ödemeyi okuduğundan eski bahis satırlarının hepsi ODENDI'dir.
const GOC_1_2: &str = "
CREATE TABLE bahis_sonucu_yeni (
    kosu_id         TEXT NOT NULL REFERENCES kosu(kosu_id) ON DELETE CASCADE,
    sira            INTEGER NOT NULL,
    oyun            TEXT NOT NULL,
    kombinasyon_ham TEXT NOT NULL,
    tutar_ham       TEXT,
    tutar_tl        REAL,
    teselli_ham     TEXT,
    sonuc_kaynak_satir INTEGER NOT NULL,
    sonuc_durumu    TEXT NOT NULL DEFAULT 'ODENDI' CHECK (sonuc_durumu IN ('ODENDI','DEVIR')),
    devir_ham       TEXT,
    devir_tl        REAL,
    ham_ibare       TEXT,
    PRIMARY KEY (kosu_id, sira)
);
INSERT INTO bahis_sonucu_yeni (kosu_id, sira, oyun, kombinasyon_ham, tutar_ham, tutar_tl, teselli_ham, sonuc_kaynak_satir, sonuc_durumu)
    SELECT kosu_id, sira, oyun, kombinasyon_ham, tutar_ham, tutar_tl, teselli_ham, sonuc_kaynak_satir, 'ODENDI' FROM bahis_sonucu;
DROP TABLE bahis_sonucu;
ALTER TABLE bahis_sonucu_yeni RENAME TO bahis_sonucu;
CREATE TABLE IF NOT EXISTS bahis_uyarisi (
    kosu_id         TEXT NOT NULL REFERENCES kosu(kosu_id) ON DELETE CASCADE,
    sira            INTEGER NOT NULL,
    ham_metin       TEXT NOT NULL,
    sonuc_kaynak_satir INTEGER NOT NULL,
    PRIMARY KEY (kosu_id, sira)
);
ALTER TABLE altili ADD COLUMN sonuc_durumu TEXT;
ALTER TABLE altili ADD COLUMN devir_tl REAL;
UPDATE altili SET sonuc_durumu = 'ODENDI' WHERE odeme_tl IS NOT NULL;
PRAGMA user_version = 2;
";

/// Şema sürüm 2 → 3: kosu tablosuna kosu_adi eklenir (adsız koşularda NULL). Hiçbir satır silinmez, değiştirilmez.
/// Mevcut koşuların adları `kosu-adi` komutuyla program CSV'lerinden (tarih, hipodrom, koşu no) doldurulur.
const GOC_2_3: &str = "
ALTER TABLE kosu ADD COLUMN kosu_adi TEXT;
PRAGMA user_version = 3;
";

/// Şema sürüm 3 → 4. Hiçbir mevcut satır silinmez ya da değiştirilmez; yeni alanlar eski kayıtlarda NULL kalır
/// (tahmin edilmez). Kayıtlar ham dosyalardan yeniden aktarılınca dolar.
///   * kosu: planlanan/gerçekleşen tarih-saat-pist, tarih durumu, erteleme notu, gerçek sonuç kaynağı
///   * altili_ayak_gecerli: neden kısıtına ORTAK_BIRINCI (SQLite kısıt değiştiremediği için tablo yeniden kurulur,
///     satırlar aynen kopyalanır)
///   * altili_odeme, altili_odeme_ayak: altılı ↔ kazanan kombinasyon ödemeleri
///   * görünümler: v_kaynak_izi koşunun gerçek sonuç dosyasını gösterir; v_bahis_kaynak_izi, v_altili_odeme eklenir
const GOC_3_4: &str = "
ALTER TABLE kosu ADD COLUMN planlanan_tarih TEXT;
ALTER TABLE kosu ADD COLUMN planlanan_saat TEXT;
ALTER TABLE kosu ADD COLUMN planlanan_pist TEXT;
ALTER TABLE kosu ADD COLUMN gerceklesen_tarih TEXT;
ALTER TABLE kosu ADD COLUMN gerceklesen_saat TEXT;
ALTER TABLE kosu ADD COLUMN gerceklesen_pist TEXT;
ALTER TABLE kosu ADD COLUMN tarih_durumu TEXT CHECK (tarih_durumu IN ('PLANLANDIGI_GUN','ERTELENDI','SONUC_YOK'));
ALTER TABLE kosu ADD COLUMN ertelenme_notu TEXT;
ALTER TABLE kosu ADD COLUMN sonuc_kaynak_id INTEGER REFERENCES kaynak_dosya(kaynak_id);
DROP VIEW IF EXISTS v_altili_gecerli;
DROP VIEW IF EXISTS v_kaynak_izi;
CREATE TABLE altili_ayak_gecerli_yeni (
    altili_id       TEXT NOT NULL,
    ayak_no         INTEGER NOT NULL,
    program_at_no   INTEGER NOT NULL,
    katilim_id      TEXT REFERENCES katilim(katilim_id) ON DELETE CASCADE,
    neden           TEXT NOT NULL CHECK (neden IN ('BIRINCI','ORTAK_BIRINCI','KOSMAZ','EKURI','ACIKLANMADI')),
    PRIMARY KEY (altili_id, ayak_no, program_at_no),
    FOREIGN KEY (altili_id, ayak_no) REFERENCES altili_ayak(altili_id, ayak_no) ON DELETE CASCADE
);
INSERT INTO altili_ayak_gecerli_yeni (altili_id, ayak_no, program_at_no, katilim_id, neden)
    SELECT altili_id, ayak_no, program_at_no, katilim_id, neden FROM altili_ayak_gecerli;
DROP TABLE altili_ayak_gecerli;
ALTER TABLE altili_ayak_gecerli_yeni RENAME TO altili_ayak_gecerli;
CREATE TABLE IF NOT EXISTS altili_odeme (
    altili_id       TEXT NOT NULL REFERENCES altili(altili_id) ON DELETE CASCADE,
    sira            INTEGER NOT NULL,
    kombinasyon_ham TEXT NOT NULL,
    odeme_ham       TEXT,
    odeme_tl        REAL,
    sonuc_durumu    TEXT NOT NULL CHECK (sonuc_durumu IN ('ODENDI','DEVIR')),
    devir_tl        REAL,
    bahis_kosu_id   TEXT NOT NULL,
    bahis_sira      INTEGER NOT NULL,
    PRIMARY KEY (altili_id, sira),
    UNIQUE (bahis_kosu_id, bahis_sira),
    FOREIGN KEY (bahis_kosu_id, bahis_sira) REFERENCES bahis_sonucu(kosu_id, sira) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS altili_odeme_ayak (
    altili_id       TEXT NOT NULL,
    odeme_sira      INTEGER NOT NULL,
    ayak_no         INTEGER NOT NULL CHECK (ayak_no BETWEEN 1 AND 6),
    program_at_no   INTEGER NOT NULL,
    PRIMARY KEY (altili_id, odeme_sira, ayak_no, program_at_no),
    FOREIGN KEY (altili_id, odeme_sira) REFERENCES altili_odeme(altili_id, sira) ON DELETE CASCADE,
    FOREIGN KEY (altili_id, ayak_no) REFERENCES altili_ayak(altili_id, ayak_no) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS ix_kosu_sonuc_kaynak ON kosu(sonuc_kaynak_id);
CREATE INDEX IF NOT EXISTS ix_altili_odeme_ayak ON altili_odeme_ayak(altili_id, ayak_no);
CREATE VIEW v_kaynak_izi AS
SELECT k.katilim_id,
       kp.dosya_adi AS program_dosyasi, kp.sha256 AS program_sha256, k.program_kaynak_satir,
       ks.dosya_adi AS sonuc_dosyasi,   ks.sha256 AS sonuc_sha256,   s.sonuc_kaynak_satir
FROM katilim k
JOIN kosu ko       ON ko.kosu_id = k.kosu_id
JOIN yaris_gunu g  ON g.gun_id = ko.gun_id
JOIN kaynak_dosya kp ON kp.kaynak_id = g.program_kaynak_id
JOIN kaynak_dosya ks ON ks.kaynak_id = COALESCE(ko.sonuc_kaynak_id, g.sonuc_kaynak_id)
LEFT JOIN sonuc s  ON s.katilim_id = k.katilim_id;
CREATE VIEW IF NOT EXISTS v_bahis_kaynak_izi AS
SELECT b.kosu_id, b.sira, b.oyun, b.kombinasyon_ham, b.tutar_ham, b.tutar_tl, b.sonuc_durumu,
       ks.dosya_adi AS sonuc_dosyasi, ks.sha256 AS sonuc_sha256, b.sonuc_kaynak_satir
FROM bahis_sonucu b
JOIN kosu ko       ON ko.kosu_id = b.kosu_id
JOIN yaris_gunu g  ON g.gun_id = ko.gun_id
JOIN kaynak_dosya ks ON ks.kaynak_id = COALESCE(ko.sonuc_kaynak_id, g.sonuc_kaynak_id);
CREATE VIEW IF NOT EXISTS v_altili_odeme AS
SELECT o.altili_id, a.oyun_adi, o.sira, o.kombinasyon_ham, o.odeme_ham, o.odeme_tl, o.sonuc_durumu, o.devir_tl,
       o.bahis_kosu_id, o.bahis_sira, bk.sonuc_dosyasi, bk.sonuc_kaynak_satir, bk.sonuc_sha256
FROM altili_odeme o
JOIN altili a ON a.altili_id = o.altili_id
JOIN v_bahis_kaynak_izi bk ON bk.kosu_id = o.bahis_kosu_id AND bk.sira = o.bahis_sira;
CREATE VIEW v_altili_gecerli AS
SELECT g.altili_id, g.ayak_no, ko.kosu_no, g.program_at_no, k.at_isim_ham, g.neden, g.katilim_id
FROM altili_ayak_gecerli g
JOIN altili_ayak a ON a.altili_id = g.altili_id AND a.ayak_no = g.ayak_no
JOIN kosu ko ON ko.kosu_id = a.kosu_id
LEFT JOIN katilim k ON k.katilim_id = g.katilim_id;
PRAGMA user_version = 4;
";

/// Şemayı kurar ya da günceller. Kurulu ve güncel veri tabanında dosyaya yazmaz.
fn sema_kur(db: &Db) -> Result<(), String> {
    match db.tek_int("PRAGMA user_version", &[])? {
        0 => db.calistir(SEMA),
        1 => {
            goc_uygula(db, GOC_1_2)?;
            goc_uygula(db, GOC_2_3)?;
            goc_uygula(db, GOC_3_4)
        }
        2 => {
            goc_uygula(db, GOC_2_3)?;
            goc_uygula(db, GOC_3_4)
        }
        3 => goc_uygula(db, GOC_3_4),
        SEMA_SURUMU => Ok(()),
        v => Err(format!("bilinmeyen şema sürümü {v} (bu program {SEMA_SURUMU} bekliyor)")),
    }
}

/// Bir göç adımını tek işlemde uygular.
fn goc_uygula(db: &Db, adim: &str) -> Result<(), String> {
    db.calistir("BEGIN IMMEDIATE;")?;
    match db.calistir(adim) {
        Ok(()) => db.calistir("COMMIT;"),
        Err(e) => {
            let _ = db.calistir("ROLLBACK;");
            Err(format!("şema göçü geri alındı: {e}"))
        }
    }
}

/// Veri tabanını yerinde güncel şemaya getirir. Dönüş: (eski sürüm, yeni sürüm)
pub fn goc(yol: &str) -> Result<(i64, i64), String> {
    let db = Db::ac(yol, false)?;
    let eski = db.tek_int("PRAGMA user_version", &[])?;
    sema_kur(&db)?;
    Ok((eski, db.tek_int("PRAGMA user_version", &[])?))
}

pub fn aktar(yol: &str, g: &GunVerisi) -> Result<Ozet, String> {
    let db = Db::ac(yol, false)?;
    sema_kur(&db)?;
    let sayac = Sayac {
        n: Cell::new(0),
        sinir: std::env::var("TJK_TEST_KESINTI_SONRA").ok().and_then(|v| v.parse().ok()),
    };
    db.calistir("BEGIN IMMEDIATE;")?;
    match aktar_ic(&db, g, &sayac) {
        Ok(o) => {
            db.calistir("COMMIT;")?;
            Ok(o)
        }
        Err(e) => {
            let _ = db.calistir("ROLLBACK;");
            Err(format!("{} aktarımı geri alındı: {e}", g.gun_id))
        }
    }
}

/// Toplu aktarımda bir günün kaderi.
pub enum Karar {
    /// Veri tabanında yoktu, aktarıldı.
    Yeni(Ozet),
    /// Aynı program+sonuç SHA-256 ve aynı ayrıştırıcı sürümü zaten var: hiçbir şey yazılmadı.
    AtlandiAyni,
    /// Girdiler aynı, ayrıştırıcı sürümü farklı: yeniden aktarıldı.
    SurumFarki { eski_surum: String, ozet: Ozet },
    /// Girdi değişmiş (SHA farklı), yeni veri TAMAM ya da eski kayıt zaten TAMAM değildi: güncellendi.
    DegismisGuncellendi { eski_program_sha: String, eski_sonuc_sha: String, ozet: Ozet },
    /// Mevcut kayıt TAMAM, yeni veri INCELEME_GEREKLI: mevcut kayıt KORUNDU, hiçbir şey yazılmadı.
    KorunduInceleme { neden: &'static str, eski_program_sha: String, eski_sonuc_sha: String },
}

impl Karar {
    pub fn kod(&self) -> &'static str {
        match self {
            Karar::Yeni(_) => "YENI_AKTARILDI",
            Karar::AtlandiAyni => "ATLANDI_AYNI_GIRDI_AYNI_SURUM",
            Karar::SurumFarki { .. } => "SURUM_FARKI_YENIDEN_AKTARILDI",
            Karar::DegismisGuncellendi { .. } => "DEGISMIS_GIRDI_GUNCELLENDI",
            Karar::KorunduInceleme { .. } => "DEGISMIS_GIRDI_INCELEME_GEREKLI_MEVCUT_TAMAM_KORUNDU",
        }
    }
}

/// Mevcut kaydı denetleyerek aktarır. Karar ve yazma aynı işlemin içindedir (BEGIN IMMEDIATE).
pub fn aktar_kosullu(yol: &str, g: &GunVerisi) -> Result<Karar, String> {
    let db = Db::ac(yol, false)?;
    sema_kur(&db)?;
    let sayac = Sayac {
        n: Cell::new(0),
        sinir: std::env::var("TJK_TEST_KESINTI_SONRA").ok().and_then(|v| v.parse().ok()),
    };
    db.calistir("BEGIN IMMEDIATE;")?;
    let sonuc = (|| -> Result<(Karar, bool), String> {
        let (_, mevcut) = db.sorgu(
            "SELECT g.gun_durumu, kp.sha256, ks.sha256, g.ayristirici_surumu FROM yaris_gunu g
               JOIN kaynak_dosya kp ON kp.kaynak_id = g.program_kaynak_id
               JOIN kaynak_dosya ks ON ks.kaynak_id = g.sonuc_kaynak_id
             WHERE g.gun_id = ?",
            &[g.gun_id.into()],
        )?;
        let Some(m) = mevcut.first() else {
            return Ok((Karar::Yeni(aktar_ic(&db, g, &sayac)?), true));
        };
        let (durum, psha, ssha, surum) = (m[0].metin(), m[1].metin(), m[2].metin(), m[3].metin());
        let ayni_girdi = psha == g.p.sha && ssha == g.s.sha;
        if ayni_girdi && surum == crate::AYRISTIRICI_SURUMU {
            return Ok((Karar::AtlandiAyni, false));
        }
        if durum == "TAMAM" && g.gun_durumu == "INCELEME_GEREKLI" {
            let neden = if ayni_girdi { "SURUM_FARKI" } else { "DEGISMIS_GIRDI" };
            return Ok((Karar::KorunduInceleme { neden, eski_program_sha: psha, eski_sonuc_sha: ssha }, false));
        }
        let ozet = aktar_ic(&db, g, &sayac)?;
        Ok((
            if ayni_girdi {
                Karar::SurumFarki { eski_surum: surum, ozet }
            } else {
                Karar::DegismisGuncellendi { eski_program_sha: psha, eski_sonuc_sha: ssha, ozet }
            },
            true,
        ))
    })();
    match sonuc {
        Ok((k, yazildi)) => {
            db.calistir(if yazildi { "COMMIT;" } else { "ROLLBACK;" })?;
            Ok(k)
        }
        Err(e) => {
            let _ = db.calistir("ROLLBACK;");
            Err(format!("{} aktarımı geri alındı: {e}", g.gun_id))
        }
    }
}

fn aktar_ic(db: &Db, g: &GunVerisi, sayac: &Sayac) -> Result<Ozet, String> {
    let gun_id = g.gun_id;
    // ---- kaynak dosyalar
    let kaynak = |k: &crate::Kaynak, rol: &str| -> Result<i64, String> {
        db.hazirla("INSERT OR IGNORE INTO kaynak_dosya(sha256, dosya_adi, rol, kodlama) VALUES (?,?,?,?)")?
            .yurut(&[(&k.sha).into(), (&k.ad).into(), rol.into(), (&k.kodlama).into()])?;
        db.tek_int("SELECT kaynak_id FROM kaynak_dosya WHERE sha256 = ?", &[(&k.sha).into()])
    };
    let pk = kaynak(g.p, "program")?;
    let sk = kaynak(g.s, "sonuc")?;
    // Birleştirilmiş sonuçta her koşunun gerçek dosyası (ek sonuç dosyaları dahil): sha256 -> kaynak_id
    let mut kaynak_idleri: Vec<(String, i64)> = vec![(g.s.sha.clone(), sk)];
    for (_, ad, sha, _, kodlama) in &g.s.kosu_kaynagi {
        if kaynak_idleri.iter().any(|x| x.0 == *sha) {
            continue;
        }
        db.hazirla("INSERT OR IGNORE INTO kaynak_dosya(sha256, dosya_adi, rol, kodlama) VALUES (?,?,?,?)")?
            .yurut(&[sha.into(), ad.into(), "sonuc".into(), kodlama.into()])?;
        let id = db.tek_int("SELECT kaynak_id FROM kaynak_dosya WHERE sha256 = ?", &[sha.into()])?;
        kaynak_idleri.push((sha.clone(), id));
    }

    // ---- günü sil (CASCADE) ve yeniden yaz
    db.hazirla("DELETE FROM yaris_gunu WHERE gun_id = ?")?.yurut(&[gun_id.into()])?;
    db.hazirla(
        "INSERT INTO yaris_gunu(gun_id, tarih, hipodrom, yaris_gunu_no, gun_durumu, program_kaynak_id, sonuc_kaynak_id, ayristirici_surumu)
         VALUES (?,?,?,?,?,?,?,?)",
    )?
    .yurut(&[
        gun_id.into(),
        (&g.p.gun.tarih).into(),
        (&g.p.gun.hipodrom).into(),
        g.p.gun.yaris_gunu_no.into(),
        g.gun_durumu.into(),
        pk.into(),
        sk.into(),
        crate::AYRISTIRICI_SURUMU.into(),
    ])?;
    sayac.ekle()?;
    {
        let mut st = db.hazirla("INSERT INTO inceleme_nedeni(gun_id, sira, neden) VALUES (?,?,?)")?;
        for (i, n) in g.inceleme.iter().enumerate() {
            st.yurut(&[gun_id.into(), (i + 1).into(), n.into()])?;
            sayac.ekle()?;
        }
    }

    // ---- koşular
    let mut ozet = Ozet { odeme: 0, kosu: 0, katilim: 0, sonuc: 0, altili: 0, ayak: 0, gecerli: 0, agf: 0 };
    {
        let mut st = db.hazirla(
            "INSERT INTO kosu(kosu_id, gun_id, kosu_no, saat, kosu_cinsi, yas_grubu, kilo_sartlari, mesafe_m, pist, rekor_derece,
               ikramiye_1_tl, ikramiye_2_tl, ikramiye_3_tl, ikramiye_4_tl, ikramiye_5_tl,
               yetistirici_primi_1_tl, yetistirici_primi_2_tl, yetistirici_primi_3_tl, yetistirici_primi_4_tl, yetistirici_primi_5_tl,
               kosu_bahisleri, program_kaynak_satir, kosu_adi,
               planlanan_tarih, planlanan_saat, planlanan_pist, gerceklesen_tarih, gerceklesen_saat, gerceklesen_pist,
               tarih_durumu, ertelenme_notu, sonuc_kaynak_id)
             VALUES (?,?,?,?,?,?,?,?,?,?, ?,?,?,?,?, ?,?,?,?,?, ?,?,?, ?,?,?,?,?,?, ?,?,?)",
        )?;
        for k in &g.p.gun.kosular {
            let kosu_id = format!("{gun_id}-K{:02}", k.kosu_no);
            let bahisler = g
                .prog
                .iter()
                .find(|r| r.kosu_no == k.kosu_no)
                .map(|r| al(&r.satir, "kosu_bahisleri"))
                .unwrap_or(Deger::Null);
            let mut p: Vec<Deger> = vec![
                kosu_id.into(),
                gun_id.into(),
                k.kosu_no.into(),
                (&k.saat).into(),
                (&k.kosu_cinsi).into(),
                (&k.yas_grubu).into(),
                (&k.kilo_sartlari).into(),
                k.mesafe_m.into(),
                (&k.pist).into(),
                (&k.rekor_derece).into(),
            ];
            p.extend(k.ikramiye.iter().map(|x| Deger::from(*x)));
            p.extend(k.yetistirici_primi.iter().map(|x| Deger::from(*x)));
            p.push(bahisler);
            p.push(k.baslik_satir.into());
            // Adsız koşuda NULL (alan boş bırakılır)
            p.push(if k.kosu_adi.is_empty() { Deger::Null } else { (&k.kosu_adi).into() });
            // Şema 4: planlanan / gerçekleşen (kosu_tarihleri.csv ile aynı satır); boş değer NULL
            let t = g.kosu_tarihleri.iter().find(|r| al(r, "kosu_no").int() == Some(k.kosu_no as i64));
            let tf = |a: &str| -> Deger {
                match t.map(|r| al(r, a)) {
                    Some(Deger::Text(x)) if x.is_empty() => Deger::Null,
                    Some(v) => v,
                    None => Deger::Null,
                }
            };
            for a in ["planlanan_tarih", "planlanan_saat", "pist_program", "gerceklesen_tarih", "gerceklesen_saat", "pist_sonuc", "tarih_durumu", "ertelenme_notu"] {
                p.push(tf(a));
            }
            let sha = tf("sonuc_kaynak_sha256").metin();
            p.push(kaynak_idleri.iter().find(|x| x.0 == sha).map_or(Deger::Null, |x| x.1.into()));
            st.yurut(&p)?;
            sayac.ekle()?;
            ozet.kosu += 1;
        }
    }

    // ---- at (geçici kimlik) + katılım
    {
        let mut at_ekle = db.hazirla(
            "INSERT OR IGNORE INTO at(gecici_anahtar, at_adi, baba, anne) VALUES (?,?,?,?)",
        )?;
        let mut kat = db.hazirla(
            "INSERT INTO katilim(katilim_id, kosu_id, at_id, program_at_no, at_isim_ham, isim_kodlari, yas_ham, yas,
               kilo_ham, kilo, kilo_ek, jokey_ham, jokey, jokey_ap_ibaresi, sahip, antrenor, start_ham, start_no, start_ek,
               h, son6_ham, kgs, s20, en_iyi_derece_ham, en_iyi_derece_sn, kosmaz_ibaresi_programda, ekuri_grubu, agf_ham,
               jokey_oncesi_dogrulandi, kilo_oncesi_dogrulandi, kosmaz_zamani_dogrulandi, program_kaynak_satir)
             VALUES (?,?,?,?,?,?,?,?, ?,?,?,?,?,?,?,?,?,?,?, ?,?,?,?,?,?,?,?,?, ?,?,?,?)",
        )?;
        for r in g.prog {
            let s = &r.satir;
            let baba = al(s, "baba").metin().trim().to_string();
            let anne = al(s, "anne").metin().trim().to_string();
            let anahtar = format!("{}|{baba}|{anne}", r.at_adi);
            at_ekle.yurut(&[anahtar.clone().into(), (&r.at_adi).into(), baba.into(), anne.into()])?;
            let at_id = db.tek_int("SELECT at_id FROM at WHERE gecici_anahtar = ?", &[anahtar.into()])?;
            let alanlar = [
                "kayit_id", "kosu_id", "", "program_at_no", "at_isim_ham", "isim_kodlari", "yas_ham", "yas", "kilo_ham", "kilo",
                "kilo_ek", "jokey_ham", "jokey", "jokey_ap_ibaresi", "sahip", "antrenor", "start_ham", "start_no", "start_ek",
                "h", "son6_ham", "kgs", "s20", "en_iyi_derece_ham", "en_iyi_derece_sn", "kosmaz_ibaresi_programda",
                "ekuri_grubu", "agf_ham", "jokey_oncesi_dogrulandi", "kilo_oncesi_dogrulandi", "kosmaz_zamani_dogrulandi",
                "kaynak_satir",
            ];
            let p: Vec<Deger> = alanlar.iter().map(|a| if a.is_empty() { at_id.into() } else { al(s, a) }).collect();
            kat.yurut(&p)?;
            sayac.ekle()?;
            ozet.katilim += 1;
        }
    }

    // ---- sonuç
    {
        let mut st = db.hazirla(
            "INSERT INTO sonuc(katilim_id, sonuc_liste_sirasi, bitis_sirasi, kosu_durumu, derece_ham, derece_sn, ganyan_oran,
               fark_ham, eslesme_durumu, capraz_kontrol_farklari, sonuc_kaynak_satir) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        )?;
        for x in g.son {
            st.yurut(&[
                (&x.kayit_id).into(),
                x.liste_sirasi.into(),
                x.bitis.into(),
                (&x.durum).into(),
                (&x.derece_ham).into(),
                x.derece_sn.into(),
                x.ganyan.into(),
                (&x.fark).into(),
                (&x.eslesme_durumu).into(),
                (&x.capraz).into(),
                x.kaynak_satir.into(),
            ])?;
            sayac.ekle()?;
            ozet.sonuc += 1;
        }
    }

    // ---- bahis sonuçları ve GANYAN kontrolü
    {
        let mut st = db.hazirla(
            "INSERT INTO bahis_sonucu(kosu_id, sira, oyun, kombinasyon_ham, tutar_ham, tutar_tl, teselli_ham, sonuc_kaynak_satir,
               sonuc_durumu, devir_ham, devir_tl, ham_ibare)
             VALUES (?,?,?,?,?,?,?,?,?,?,?,?)",
        )?;
        let mut uy = db.hazirla("INSERT INTO bahis_uyarisi(kosu_id, sira, ham_metin, sonuc_kaynak_satir) VALUES (?,?,?,?)")?;
        let bos_null = |s: &String| -> Deger { if s.is_empty() { Deger::Null } else { s.into() } };
        for kb in g.bahis_kosu {
            let kosu_id = format!("{gun_id}-K{:02}", kb.kosu_no);
            for (i, b) in kb.bahisler.iter().enumerate() {
                st.yurut(&[
                    (&kosu_id).into(),
                    (i + 1).into(),
                    (&b.oyun).into(),
                    (&b.kombinasyon_ham).into(),
                    bos_null(&b.tutar_ham),
                    b.tutar_tl.into(),
                    (&b.teselli_ham).into(),
                    kb.satir_no.into(),
                    b.sonuc_durumu.into(),
                    bos_null(&b.devir_ham),
                    b.devir_tl.into(),
                    (&b.ham_ibare).into(),
                ])?;
                sayac.ekle()?;
            }
            for (i, h) in kb.okunamayan.iter().enumerate() {
                uy.yurut(&[(&kosu_id).into(), (i + 1).into(), h.into(), kb.satir_no.into()])?;
                sayac.ekle()?;
            }
        }
        let mut gk = db.hazirla("INSERT INTO ganyan_kontrol(kosu_id, ganyan_numaralari, birinciler, durum) VALUES (?,?,?,?)")?;
        let liste = |v: &Json| match v {
            Json::Arr(a) => a.iter().map(|x| x.hucre()).collect::<Vec<_>>().join(","),
            _ => String::new(),
        };
        for x in g.ganyan {
            let kn = j(x, "kosu_no").hucre();
            gk.yurut(&[
                format!("{gun_id}-K{:0>2}", kn).into(),
                liste(j(x, "ganyan_numaralari")).into(),
                liste(j(x, "birinci_program_numaralari")).into(),
                d(j(x, "durum")),
            ])?;
            sayac.ekle()?;
        }
    }

    // ---- altılı, ayak, geçerli numaralar
    let mut aktarilan_altililar: Vec<String> = Vec::new();
    {
        let mut alt = db.hazirla(
            "INSERT INTO altili(altili_id, gun_id, oyun_adi, baslangic_kosu_no, kombinasyon_ham, odeme_ham, odeme_tl,
               program_kaynak_satir, sonuc_kaynak_satir, sonuc_durumu, devir_tl) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        )?;
        let mut ayk = db.hazirla("INSERT INTO altili_ayak(altili_id, ayak_no, kosu_id) VALUES (?,?,?)")?;
        let mut gec = db.hazirla(
            "INSERT INTO altili_ayak_gecerli(altili_id, ayak_no, program_at_no, katilim_id, neden) VALUES (?,?,?,?,?)",
        )?;
        for a in g.altililar {
            let altili_id = j(a, "altili_id").hucre();
            alt.yurut(&[
                (&altili_id).into(),
                gun_id.into(),
                d(j(a, "oyun_adi")),
                d(j(a, "baslangic_kosusu")),
                d(j(a, "kombinasyon_ham")),
                d(j(a, "odeme_ham")),
                d(j(a, "odeme_tl")),
                d(j(j(a, "baslangic_kaniti"), "satir")),
                d(j(j(a, "odeme_kaniti"), "satir")),
                d(j(a, "sonuc_durumu")),
                d(j(a, "devir_tl")),
            ])?;
            sayac.ekle()?;
            ozet.altili += 1;
            aktarilan_altililar.push(altili_id.clone());
            let Json::Arr(ayaklar) = j(a, "ayaklar") else { continue };
            for ay in ayaklar {
                let ayak_no = d(j(ay, "ayak"));
                let kosu_id = j(ay, "kosu_id").hucre();
                ayk.yurut(&[(&altili_id).into(), ayak_no.clone(), (&kosu_id).into()])?;
                sayac.ekle()?;
                ozet.ayak += 1;
                let birinci = j(j(ay, "birinci"), "program_at_no").hucre().parse::<i64>().ok();
                // Doğrulanmış ortak birincilik (altili.json → ayak.ortak_birinciler; yalnızca 2+ birincide yazılır)
                let ortak: Vec<i64> = match j(ay, "ortak_birinciler") {
                    Json::Arr(v) => v.iter().filter_map(|x| j(x, "program_at_no").hucre().parse().ok()).collect(),
                    _ => Vec::new(),
                };
                let Json::Arr(gecerli) = j(ay, "gecerli_numaralar") else { continue };
                for n in gecerli.iter().filter_map(|x| if let Json::Int(i) = x { Some(*i) } else { None }) {
                    let katilim_id = format!("{kosu_id}-A{n:02}");
                    let r = g.prog.iter().find(|r| r.kayit_id == katilim_id);
                    let kosmaz_sonuc = g.son.iter().any(|x| x.kayit_id == katilim_id && x.durum == "KOSMAZ");
                    // Öncelik: birinci > birinciyle aynı eküri (programdaki ibareyle belgeli) > koşmaz (H2 hipotezi doğrulanmamış).
                    // Ayrıntılı açıklama (altili.json) da aynı sırayla eküriyi önce denetler.
                    let neden = if ortak.len() > 1 && ortak.contains(&n) {
                        "ORTAK_BIRINCI"
                    } else if Some(n) == birinci {
                        "BIRINCI"
                    } else if birinci.is_some_and(|b| r.is_some_and(|r| r.ekuri.contains(&b) && r.ekuri.contains(&n))) {
                        "EKURI"
                    } else if r.is_some_and(|r| r.kosmaz) || kosmaz_sonuc {
                        "KOSMAZ"
                    } else {
                        "ACIKLANMADI"
                    };
                    gec.yurut(&[
                        (&altili_id).into(),
                        ayak_no.clone(),
                        n.into(),
                        r.map(|_| katilim_id.clone()).into(),
                        neden.into(),
                    ])?;
                    sayac.ekle()?;
                    ozet.gecerli += 1;
                }
            }
        }
    }

    // ---- altılı ödemeleri: her kazanan kombinasyon, son ayak bahis satırına doğrudan bağlı
    {
        let mut od = db.hazirla(
            "INSERT INTO altili_odeme(altili_id, sira, kombinasyon_ham, odeme_ham, odeme_tl, sonuc_durumu, devir_tl, bahis_kosu_id, bahis_sira)
             VALUES (?,?,?,?,?,?,?,?,?)",
        )?;
        let mut oa = db.hazirla("INSERT INTO altili_odeme_ayak(altili_id, odeme_sira, ayak_no, program_at_no) VALUES (?,?,?,?)")?;
        for a in g.altililar {
            let altili_id = j(a, "altili_id").hucre();
            let ayak_sayisi = match j(a, "ayaklar") {
                Json::Arr(v) => v.len(),
                _ => 0,
            };
            let son_kosu: Option<u32> = match j(a, "ayak_kosulari") {
                Json::Arr(v) => v.last().and_then(|x| x.hucre().parse().ok()),
                _ => None,
            };
            // Çok ödemeli (ortak birincilik) altılıda kazanan_kombinasyonlar; tek ödemelide altılının kendi alanları.
            let tekil = [a.clone()];
            let odemeler: &[Json] = match j(a, "kazanan_kombinasyonlar") {
                Json::Arr(v) if !v.is_empty() => v,
                _ if !matches!(j(a, "odeme_kaniti"), Json::Null) => &tekil,
                _ => &[],
            };
            for (i, o) in odemeler.iter().enumerate() {
                let ibare = j(j(o, "odeme_kaniti"), "ibare").hucre();
                let komb = j(o, "kombinasyon_ham").hucre();
                let kb = son_kosu.and_then(|kn| g.bahis_kosu.iter().find(|k| k.kosu_no == kn));
                let bahis_sira = kb
                    .and_then(|k| k.bahisler.iter().position(|b| b.ham_ibare == ibare && b.kombinasyon_ham == komb))
                    .map(|x| x + 1)
                    .ok_or_else(|| format!("{altili_id}: ödeme '{ibare}' son ayak bahis satırında bulunamadı"))?;
                let bahis_kosu_id = format!("{gun_id}-K{:02}", son_kosu.unwrap_or(0));
                od.yurut(&[
                    (&altili_id).into(),
                    (i + 1).into(),
                    komb.into(),
                    d(j(o, "odeme_ham")),
                    d(j(o, "odeme_tl")),
                    d(j(o, "sonuc_durumu")),
                    d(j(o, "devir_tl")),
                    bahis_kosu_id.into(),
                    bahis_sira.into(),
                ])?;
                sayac.ekle()?;
                ozet.odeme += 1;
                if let Json::Arr(ayaklar) = j(o, "kombinasyon_listesi") {
                    for (ai, ay) in ayaklar.iter().enumerate().take(ayak_sayisi.min(6)) {
                        if let Json::Arr(nolar) = ay {
                            for n in nolar.iter().filter_map(|x| if let Json::Int(v) = x { Some(*v) } else { None }) {
                                oa.yurut(&[(&altili_id).into(), (i + 1).into(), (ai + 1).into(), n.into()])?;
                                sayac.ekle()?;
                            }
                        }
                    }
                }
            }
        }
    }

    // ---- AGF gözlemleri (altılıya yalnızca eşleşme belirliyse bağlanır)
    {
        let mut st = db.hazirla(
            "INSERT INTO agf_gozlem(katilim_id, deger_sirasi, agf_yuzde, agf_sira, altili_id, eslesme_dayanagi, zaman_dogrulandi)
             VALUES (?,?,?,?,?,?,?)",
        )?;
        for r in g.prog {
            let bu = kosu_altililari(g.tanimlar, r.kosu_no);
            let tek = (r.agf_degerler.len() == 1 && bu.len() == 1)
                .then(|| g.tanimlar[bu[0]].altili_id.clone())
                .filter(|id| aktarilan_altililar.contains(id));
            for (i, (y, s)) in r.agf_degerler.iter().enumerate() {
                let dayanak = if tek.is_some() { "TEK_ALTILI_CIKARIM" } else { "DOGRULANMADI" };
                st.yurut(&[
                    (&r.kayit_id).into(),
                    (i + 1).into(),
                    (*y).into(),
                    (*s).into(),
                    tek.clone().into(),
                    dayanak.into(),
                    al(&r.satir, "agf_oncesi_dogrulandi"),
                ])?;
                sayac.ekle()?;
                ozet.agf += 1;
            }
        }
    }

    // ---- sahipsiz kayıtları temizle
    db.calistir("DELETE FROM at WHERE at_id NOT IN (SELECT at_id FROM katilim);")?;
    db.calistir(
        "DELETE FROM kaynak_dosya WHERE kaynak_id NOT IN
           (SELECT program_kaynak_id FROM yaris_gunu UNION SELECT sonuc_kaynak_id FROM yaris_gunu
            UNION SELECT sonuc_kaynak_id FROM kosu WHERE sonuc_kaynak_id IS NOT NULL);",
    )?;
    Ok(ozet)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEMA_V3: &str = include_str!("../tests/veri/sema_v3.sql");

    fn gecici(ad: &str) -> String {
        let p = std::env::temp_dir().join(format!("tjk_vt_{}_{ad}.sqlite", std::process::id()));
        let _ = std::fs::remove_file(&p);
        p.to_string_lossy().into_owned()
    }

    /// (tablo/görünüm adı, tür, sütun adları) — şema karşılaştırması için
    fn sema_ozeti(db: &Db) -> Vec<(String, String, Vec<String>)> {
        let (_, adlar) = db
            .sorgu("SELECT name, type FROM sqlite_master WHERE type IN ('table','view','index') AND name NOT LIKE 'sqlite_%' ORDER BY name", &[])
            .unwrap();
        adlar
            .iter()
            .map(|r| {
                let ad = r[0].metin();
                let (_, k) = db.sorgu(&format!("SELECT name FROM pragma_table_info('{ad}') ORDER BY name"), &[]).unwrap();
                (ad, r[1].metin(), k.iter().map(|x| x[0].metin()).collect())
            })
            .collect()
    }

    fn v3_ornek(yol: &str) {
        let db = Db::ac(yol, false).unwrap();
        db.calistir(SEMA_V3).unwrap();
        db.calistir(
            "INSERT INTO kaynak_dosya VALUES (1,'aaa','p.csv','program','UTF-8'),(2,'bbb','s.csv','sonuc','UTF-8');
             INSERT INTO yaris_gunu VALUES ('20260101-X','2026-01-01','X',1,'TAMAM',1,2,'0.11.0');
             INSERT INTO kosu(kosu_id, gun_id, kosu_no, saat, pist, program_kaynak_satir) VALUES ('20260101-X-K01','20260101-X',1,'18:00','Kum',2);
             INSERT INTO at(at_id, gecici_anahtar, at_adi) VALUES (1,'A||','A');
             INSERT INTO katilim(katilim_id, kosu_id, at_id, program_at_no, kosmaz_ibaresi_programda, jokey_oncesi_dogrulandi,
                kilo_oncesi_dogrulandi, kosmaz_zamani_dogrulandi, program_kaynak_satir) VALUES ('20260101-X-K01-A01','20260101-X-K01',1,1,0,0,0,0,5);
             INSERT INTO sonuc(katilim_id, kosu_durumu, eslesme_durumu, sonuc_kaynak_satir, bitis_sirasi) VALUES ('20260101-X-K01-A01','BITIRDI','TEKIL_ESLESME',5,1);
             INSERT INTO bahis_sonucu(kosu_id, sira, oyun, kombinasyon_ham, tutar_ham, tutar_tl, sonuc_kaynak_satir) VALUES ('20260101-X-K01',1,'GANYAN','1','1,50 TL',1.5,9);
             INSERT INTO altili(altili_id, gun_id, oyun_adi, baslangic_kosu_no, kombinasyon_ham, odeme_tl, program_kaynak_satir, sonuc_durumu)
                VALUES ('20260101-X-A1','20260101-X','6''LI GANYAN',1,'1/1/1/1/1/1',100.0,3,'ODENDI');
             INSERT INTO altili_ayak VALUES ('20260101-X-A1',1,'20260101-X-K01');
             INSERT INTO altili_ayak_gecerli VALUES ('20260101-X-A1',1,1,'20260101-X-K01-A01','BIRINCI');",
        )
        .unwrap();
        assert_eq!(db.tek_int("PRAGMA user_version", &[]).unwrap(), 3);
    }

    #[test]
    fn v3_ten_v4_e_goc_kayitlari_korur_tahmin_etmez() {
        let yol = gecici("goc");
        v3_ornek(&yol);
        assert_eq!(goc(&yol).unwrap(), (3, 4));
        let db = Db::ac(&yol, false).unwrap();
        // eski satırlar aynen
        for (t, n) in [("kaynak_dosya", 2), ("yaris_gunu", 1), ("kosu", 1), ("katilim", 1), ("sonuc", 1), ("bahis_sonucu", 1), ("altili", 1), ("altili_ayak_gecerli", 1)] {
            assert_eq!(db.tek_int(&format!("SELECT COUNT(*) FROM {t}"), &[]).unwrap(), n, "{t}");
        }
        assert_eq!(db.sorgu("SELECT neden FROM altili_ayak_gecerli", &[]).unwrap().1[0][0].metin(), "BIRINCI");
        assert_eq!(db.sorgu("SELECT saat, pist FROM kosu", &[]).unwrap().1[0].iter().map(|x| x.metin()).collect::<Vec<_>>(), ["18:00", "Kum"]);
        // yeni alanlar tahmin edilmez: NULL
        assert_eq!(
            db.tek_int(
                "SELECT COUNT(*) FROM kosu WHERE planlanan_tarih IS NULL AND planlanan_saat IS NULL AND planlanan_pist IS NULL
                   AND gerceklesen_tarih IS NULL AND gerceklesen_saat IS NULL AND gerceklesen_pist IS NULL
                   AND tarih_durumu IS NULL AND ertelenme_notu IS NULL AND sonuc_kaynak_id IS NULL",
                &[]
            )
            .unwrap(),
            1
        );
        assert_eq!(db.tek_int("SELECT COUNT(*) FROM altili_odeme", &[]).unwrap(), 0);
        // görünümler çalışıyor; eski kayıtta sonuç kaynağı günün dosyasına düşer
        assert_eq!(db.sorgu("SELECT sonuc_dosyasi FROM v_kaynak_izi", &[]).unwrap().1[0][0].metin(), "s.csv");
        assert_eq!(db.sorgu("SELECT sonuc_dosyasi FROM v_bahis_kaynak_izi", &[]).unwrap().1[0][0].metin(), "s.csv");
        assert_eq!(db.tek_int("SELECT COUNT(*) FROM v_altili_gecerli", &[]).unwrap(), 1);
        // yeni kısıt: ORTAK_BIRINCI kabul, bilinmeyen neden ret
        db.calistir("INSERT INTO altili_ayak_gecerli VALUES ('20260101-X-A1',1,2,NULL,'ORTAK_BIRINCI');").unwrap();
        assert!(db.calistir("INSERT INTO altili_ayak_gecerli VALUES ('20260101-X-A1',1,3,NULL,'YOK');").is_err());
        // bütünlük
        assert_eq!(db.sorgu("PRAGMA foreign_key_check", &[]).unwrap().1.len(), 0);
        assert_eq!(db.sorgu("PRAGMA integrity_check", &[]).unwrap().1[0][0].metin(), "ok");
        // ikinci göç: değişiklik yok
        drop(db);
        assert_eq!(goc(&yol).unwrap(), (4, 4));
    }

    #[test]
    fn bos_kurulum_ve_goc_ayni_semayi_uretir() {
        let (a, b) = (gecici("bos"), gecici("goc2"));
        let db = Db::ac(&a, false).unwrap();
        sema_kur(&db).unwrap();
        assert_eq!(db.tek_int("PRAGMA user_version", &[]).unwrap(), 4);
        v3_ornek(&b);
        goc(&b).unwrap();
        let db2 = Db::ac(&b, false).unwrap();
        assert_eq!(sema_ozeti(&db), sema_ozeti(&db2));
    }
}
