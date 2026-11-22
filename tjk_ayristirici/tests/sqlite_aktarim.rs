//! Şema 4 SQLite aktarımı: gerçek ham CSV'lerle (`../kontrol/ham`) uçtan uca. Geçici veritabanı kullanılır.

use std::path::{Path, PathBuf};
use std::process::Command;

fn ham(ad: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../kontrol/ham").join(ad).to_string_lossy().into_owned()
}

fn gecici(ad: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("tjk_sqlt_{}_{ad}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn aktar(db: &Path, cikti: &Path, args: &[&str]) {
    let o = Command::new(env!("CARGO_BIN_EXE_tjk_ayristirici"))
        .args(&args[..2])
        .arg(cikti)
        .args(&args[2..])
        .arg("--sqlite")
        .arg(db)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
}

/// tjk_sorgu çıktısını satır/hücre listesine çevirir (başlık ve ayraç satırı atlanır).
fn sorgu(db: &Path, sql: &str) -> Vec<Vec<String>> {
    let o = Command::new(env!("CARGO_BIN_EXE_tjk_sorgu")).arg(db).arg(sql).output().unwrap();
    assert!(o.status.success(), "{sql}: {}", String::from_utf8_lossy(&o.stderr));
    let t = String::from_utf8_lossy(&o.stdout).into_owned();
    t.lines()
        .skip(3)
        .take_while(|l| !l.starts_with('('))
        .map(|l| l.split(" | ").map(|x| x.trim().to_string()).collect())
        .collect()
}

fn tek(db: &Path, sql: &str) -> String {
    sorgu(db, sql)[0][0].clone()
}

fn butunluk(db: &Path) {
    assert_eq!(sorgu(db, "PRAGMA foreign_key_check").len(), 0, "yabancı anahtar ihlali");
    assert_eq!(tek(db, "PRAGMA integrity_check"), "ok");
    assert_eq!(tek(db, "PRAGMA user_version"), "4");
}

#[test]
fn antalya_31_mart_a2_iki_odeme_kombinasyonlara_bagli() {
    let d = gecici("ant");
    let db = d.join("t.sqlite");
    aktar(&db, &d.join("c"), &[&ham("20260331-ANTALYA-program.csv"), &ham("20260331-ANTALYA-sonuc.csv")]);
    butunluk(&db);
    // Son ayakta 5 ile biten kombinasyon 2.093,81; 6 ile biten 1.686,12 (altili_odeme_ayak üzerinden doğrudan)
    let r = sorgu(
        &db,
        "SELECT oa.program_at_no, o.kombinasyon_ham, o.odeme_ham, o.odeme_tl, b.ham_ibare FROM altili_odeme o
           JOIN altili_odeme_ayak oa ON oa.altili_id=o.altili_id AND oa.odeme_sira=o.sira AND oa.ayak_no=6
           JOIN bahis_sonucu b ON b.kosu_id=o.bahis_kosu_id AND b.sira=o.bahis_sira
          WHERE o.altili_id='20260331-ANTALYA-A2' ORDER BY oa.program_at_no",
    );
    assert_eq!(r.len(), 2);
    assert_eq!(r[0][..4], ["5", "7/8/1/1/8/5", "2.093,81 TL", "2093.81"]);
    assert_eq!(r[1][..4], ["6", "7/8/1/1/8/6", "1.686,12 TL", "1686.12"]);
    assert_eq!(r[0][4], "2. 6'LI GANYAN(7/8/1/1/8/5) :2.093,81 TL");
    // Ortak birincilik geçerlilik nedeninde
    let g = sorgu(&db, "SELECT program_at_no, neden FROM v_altili_gecerli WHERE altili_id='20260331-ANTALYA-A2' AND ayak_no=6 ORDER BY 1");
    assert_eq!(g, vec![vec!["5".to_string(), "ORTAK_BIRINCI".into()], vec!["6".to_string(), "ORTAK_BIRINCI".into()]]);
    // Tek ödemeli A1: eski altili alanları aynen + altili_odeme'de tek satır
    assert_eq!(sorgu(&db, "SELECT kombinasyon_ham, odeme_tl FROM altili WHERE altili_id='20260331-ANTALYA-A1'")[0], ["5,7/2,3/7/8/1/1", "1787.18"]);
    assert_eq!(tek(&db, "SELECT COUNT(*) || '/' || SUM(odeme_tl) FROM altili_odeme WHERE altili_id='20260331-ANTALYA-A1'"), "1/1787.18");
    assert_eq!(tek(&db, "SELECT COUNT(*) FROM v_altili_gecerli WHERE neden='ORTAK_BIRINCI' AND altili_id='20260331-ANTALYA-A1'"), "0");
}

#[test]
fn adana_15_mart_kosu_kaynaklari_ve_bilinmeyen_odemeler() {
    let d = gecici("ada15");
    let db = d.join("t.sqlite");
    let args = [
        ham("20260315-ADANA-program.csv"),
        ham("20260315-ADANA-sonuc.csv"),
        "--ek-sonuc".into(),
        ham("20260316-ADANA-sonuc.csv"),
        "--ertelenme-notu".into(),
        "TJK duyuru 52071 (test)".into(),
    ];
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    aktar(&db, &d.join("c"), &a);
    butunluk(&db);
    let k = sorgu(
        &db,
        "SELECT ko.kosu_no, ko.planlanan_tarih, ko.gerceklesen_tarih, ko.tarih_durumu, kd.dosya_adi
           FROM kosu ko JOIN kaynak_dosya kd ON kd.kaynak_id=ko.sonuc_kaynak_id ORDER BY ko.kosu_no",
    );
    assert_eq!(k.len(), 8);
    for r in &k {
        let n: u32 = r[0].parse().unwrap();
        let (gt, dur, dosya) = if n <= 2 {
            ("2026-03-15", "PLANLANDIGI_GUN", "20260315-ADANA-sonuc.csv")
        } else {
            ("2026-03-16", "ERTELENDI", "20260316-ADANA-sonuc.csv")
        };
        assert_eq!((r[1].as_str(), r[2].as_str(), r[3].as_str(), r[4].as_str()), ("2026-03-15", gt, dur, dosya), "{r:?}");
    }
    // Katılım ve bahis kaynak izleri gerçek dosyayı gösterir
    assert_eq!(tek(&db, "SELECT sonuc_dosyasi FROM v_kaynak_izi WHERE katilim_id='20260315-ADANA-K02-A01'"), "20260315-ADANA-sonuc.csv");
    assert_eq!(tek(&db, "SELECT sonuc_dosyasi FROM v_kaynak_izi WHERE katilim_id='20260315-ADANA-K08-A01'"), "20260316-ADANA-sonuc.csv");
    assert_eq!(
        tek(&db, "SELECT COUNT(*) FROM v_bahis_kaynak_izi WHERE kosu_id >= '20260315-ADANA-K03' AND sonuc_dosyasi <> '20260316-ADANA-sonuc.csv'"),
        "0"
    );
    // Bilinmeyen iki ödeme: satır yok / NULL; sıfır ya da iade yok
    assert_eq!(tek(&db, "SELECT COUNT(*) FROM altili_odeme WHERE altili_id='20260315-ADANA-A1'"), "0");
    assert_eq!(sorgu(&db, "SELECT odeme_tl, sonuc_durumu FROM altili WHERE altili_id='20260315-ADANA-A1'")[0], ["NULL", "NULL"]);
    assert_eq!(tek(&db, "SELECT COUNT(*) FROM bahis_sonucu WHERE kosu_id='20260315-ADANA-K03' AND oyun='GANYAN'"), "0");
    assert_eq!(tek(&db, "SELECT COUNT(*) FROM bahis_sonucu WHERE tutar_tl=0"), "0");
    // 2. altılı: tek ödeme 19.387,81, 16.03 dosyasındaki satıra bağlı
    assert_eq!(
        sorgu(&db, "SELECT odeme_tl, sonuc_dosyasi FROM v_altili_odeme WHERE altili_id='20260315-ADANA-A2'")[0],
        ["19387.81", "20260316-ADANA-sonuc.csv"]
    );
    assert_eq!(tek(&db, "SELECT odeme_tl FROM altili WHERE altili_id='20260315-ADANA-A2'"), "19387.81");
}

#[test]
fn ikinci_aktarim_kayitlari_degistirmez() {
    let d = gecici("iki");
    let db = d.join("t.sqlite");
    let gunler = [
        ("20260331-ANTALYA-program.csv", "20260331-ANTALYA-sonuc.csv"),
        ("20260321-ISTANBUL-program.csv", "20260321-ISTANBUL-sonuc.csv"),
    ];
    let tablolar = ["kaynak_dosya", "kosu", "katilim", "sonuc", "bahis_sonucu", "altili", "altili_odeme", "altili_odeme_ayak", "altili_ayak_gecerli"];
    let dokum = || tablolar.iter().map(|t| sorgu(&db, &format!("SELECT * FROM {t} ORDER BY 1,2"))).collect::<Vec<_>>();
    for (p, s) in gunler {
        aktar(&db, &d.join("c"), &[&ham(p), &ham(s)]);
    }
    let ilk = dokum();
    for (p, s) in gunler {
        aktar(&db, &d.join("c"), &[&ham(p), &ham(s)]);
    }
    assert_eq!(ilk, dokum());
    butunluk(&db);
}
