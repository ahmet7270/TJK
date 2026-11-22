//! Mevcut koşuların adlarını program CSV'lerinden doldurur (şema 3, kosu.kosu_adi).
//!
//! Kullanım:
//!   tjk_ayristirici kosu-adi KLASOR --db CALISMA.sqlite [--cikti RAPOR.json]
//!
//! Kurallar:
//!   * Eşleşme dosya adından değil İÇERİKTEN yapılır: ilk satırdaki tarih + hipodrom → gun_id, başlıktaki koşu numarası → kosu_id.
//!   * Programın SHA-256'sı, o günün veri tabanındaki program kaynağıyla (yaris_gunu.program_kaynak_id) aynı olmalıdır;
//!     farklıysa o gün YAZILMAZ (KAYNAK_SHA_FARKLI). Ek güvence: başlıktaki saat veri tabanındaki saatle aynı olmalıdır.
//!   * Adsız koşuda alan NULL bırakılır. Başka hiçbir alan ve tablo değiştirilmez; satır eklenmez, silinmez.
//!   * Tüm güncellemeler tek işlemdedir; tekrar çalıştırmak aynı sonucu verir.

use crate::json::{obj, Json};
use crate::sqlite::{Db, Deger};
use crate::{ayristir, hipodrom_slug, kodlama, sha256};
use std::path::PathBuf;

const KULLANIM: &str = "Kullanım: tjk_ayristirici kosu-adi KLASOR --db CALISMA.sqlite [--cikti RAPOR.json]";

pub fn calistir(args: &[String]) -> Result<(), String> {
    let (mut klasor, mut db_yol, mut cikti) = (None, None, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--db" => db_yol = it.next().cloned(),
            "--cikti" => cikti = it.next().cloned(),
            o if o.starts_with("--") => return Err(format!("bilinmeyen seçenek: {o}\n{KULLANIM}")),
            _ if klasor.is_none() => klasor = Some(a.clone()),
            _ => return Err(KULLANIM.into()),
        }
    }
    let (Some(klasor), Some(db_yol)) = (klasor, db_yol) else { return Err(KULLANIM.into()) };
    if !std::path::Path::new(&db_yol).exists() {
        return Err(format!("veri tabanı yok: {db_yol}"));
    }
    let db = Db::ac(&db_yol, false)?;
    let v = db.tek_int("PRAGMA user_version", &[])?;
    if v != 3 {
        return Err(format!("şema sürümü {v}; kosu-adi şema 3 ister (önce: tjk_ayristirici goc --kaynak ESKI --hedef YENI)"));
    }

    let mut yollar: Vec<PathBuf> = std::fs::read_dir(&klasor)
        .map_err(|e| format!("{klasor} okunamadı: {e}"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x.eq_ignore_ascii_case("csv")))
        .collect();
    yollar.sort();

    let mut gunler = Vec::new();
    let mut toplam = (0usize, 0usize, 0usize); // (adlı, adsız, atlanan koşu)
    db.calistir("BEGIN IMMEDIATE;")?;
    let sonuc: Result<(), String> = (|| {
        let mut guncelle = db.hazirla("UPDATE kosu SET kosu_adi = ?1 WHERE kosu_id = ?2")?;
        for y in &yollar {
            let dosya = y.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
            let bytes = std::fs::read(y).map_err(|e| format!("{dosya} okunamadı: {e}"))?;
            let (metin, _) = kodlama::coz(&bytes);
            let Ok(g) = ayristir::gun_ayristir(&metin) else { continue };
            let program_mi = g.kosular.first().is_some_and(|k| k.kolonlar.iter().any(|c| c == "Son 6 Yarış"));
            if !program_mi {
                continue; // sonuç dosyaları koşu adı için kullanılmaz
            }
            let gun_id = format!("{}-{}", g.tarih.replace('-', ""), hipodrom_slug(&g.hipodrom));
            let sha = sha256::hex(&bytes);
            let (_, r) = db.sorgu(
                "SELECT kd.sha256 FROM yaris_gunu y JOIN kaynak_dosya kd ON kd.kaynak_id = y.program_kaynak_id WHERE y.gun_id = ?1",
                &[Deger::Text(gun_id.clone())],
            )?;
            let durum = match r.first() {
                None => "VERITABANINDA_YOK",
                Some(x) if x[0].metin() != sha => "KAYNAK_SHA_FARKLI",
                Some(_) => "ESLESTI",
            };
            let mut kosular = Vec::new();
            if durum == "ESLESTI" {
                for k in &g.kosular {
                    let kosu_id = format!("{gun_id}-K{:02}", k.kosu_no);
                    let (_, r) = db.sorgu("SELECT saat FROM kosu WHERE kosu_id = ?1", &[Deger::Text(kosu_id.clone())])?;
                    let kd = match r.first() {
                        None => "KOSU_VERITABANINDA_YOK",
                        Some(x) if x[0].metin() != k.saat => "SAAT_FARKLI",
                        Some(_) if k.kosu_adi.is_empty() => "ADSIZ_BOS_BIRAKILDI",
                        Some(_) => "AD_YAZILDI",
                    };
                    match kd {
                        "AD_YAZILDI" => {
                            guncelle.yurut(&[Deger::Text(k.kosu_adi.clone()), Deger::Text(kosu_id.clone())])?;
                            toplam.0 += 1;
                        }
                        "ADSIZ_BOS_BIRAKILDI" => {
                            guncelle.yurut(&[Deger::Null, Deger::Text(kosu_id.clone())])?;
                            toplam.1 += 1;
                        }
                        _ => toplam.2 += 1,
                    }
                    println!("  {kosu_id:<24} {kd:<22} {}", k.kosu_adi);
                    kosular.push(obj(vec![
                        ("kosu_id", kosu_id.into()),
                        ("durum", kd.into()),
                        ("kosu_adi", if k.kosu_adi.is_empty() { Json::Null } else { (&k.kosu_adi).into() }),
                        ("program_kaynak_satir", k.baslik_satir.into()),
                    ]));
                }
            }
            println!("{gun_id:<20} {durum:<20} {dosya}");
            gunler.push(obj(vec![
                ("gun_id", gun_id.into()),
                ("program_dosyasi", dosya.into()),
                ("program_sha256", sha.into()),
                ("durum", durum.into()),
                ("kosular", Json::Arr(kosular)),
            ]));
        }
        Ok(())
    })();
    match sonuc {
        Ok(()) => db.calistir("COMMIT;")?,
        Err(e) => {
            let _ = db.calistir("ROLLBACK;");
            return Err(format!("koşu adı doldurma geri alındı: {e}"));
        }
    }
    println!("Özet: adı yazılan {} | adsız (NULL) {} | atlanan {}", toplam.0, toplam.1, toplam.2);
    if let Some(c) = cikti {
        let j = obj(vec![
            ("mod", "KOSU_ADI_DOLDURMA".into()),
            ("veritabani", db_yol.into()),
            ("adi_yazilan", toplam.0.into()),
            ("adsiz_null", toplam.1.into()),
            ("atlanan", toplam.2.into()),
            ("gunler", Json::Arr(gunler)),
        ]);
        std::fs::write(&c, j.guzel()).map_err(|e| format!("{c} yazılamadı: {e}"))?;
    }
    Ok(())
}
