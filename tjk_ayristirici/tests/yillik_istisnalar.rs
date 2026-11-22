//! 2025–2026 yıllık kapsamda bulunan istisnalar (girdi: `../kontrol/ham/AY/` altındaki doğrulanmış ham CSV'ler).
//! Ham dosyalar yalnızca okunur; olumsuz senaryolar geçici kopyalarla sınanır.

use std::path::{Path, PathBuf};
use std::process::Command;

fn ham(ay: &str, ad: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../kontrol/ham").join(ay).join(ad)
}

fn gecici(ad: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("tjk_ytest_{}_{ad}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn calistir(ad: &str, p: &Path, s: &Path) -> (i32, String, String) {
    let k = gecici(ad);
    let o = Command::new(env!("CARGO_BIN_EXE_tjk_ayristirici")).arg(p).arg(s).arg(k.join("cikti")).output().unwrap();
    let j = std::fs::read_to_string(k.join("cikti/altili.json")).unwrap_or_default();
    (o.status.code().unwrap_or(-1), String::from_utf8_lossy(&o.stdout).into(), j)
}

fn turet(ay: &str, kaynak: &str, eski: &str, yeni: &str, test: &str) -> PathBuf {
    let t = String::from_utf8(std::fs::read(ham(ay, kaynak)).unwrap()).unwrap();
    assert_eq!(t.matches(eski).count(), 1, "türetme ifadesi tek olmalı: {eski}");
    let y = gecici(&format!("{test}_girdi")).join(kaynak);
    std::fs::write(&y, t.replacen(eski, yeni, 1)).unwrap();
    y
}

/// 02.02.2026 Bursa 3. koşu: GANYAN(1) ve GANYAN(7) aynı tutar; 7 birinci, 1 eküri eşi (3. sırada). Ölü yarış değil.
#[test]
fn bursa_2_subat_ekuri_ganyani_ortak_birincilik_sayilmaz() {
    let (kod, out, j) = calistir("b0202", &ham("2026-02", "20260202-BURSA-program.csv"), &ham("2026-02", "20260202-BURSA-sonuc.csv"));
    assert_eq!(kod, 0, "{out}");
    assert!(j.contains("\"gun_durumu\": \"TAMAM\""), "{out}");
    assert!(!j.contains("ORTAK_BIRINCILIK_DOGRULANAMADI"));
    assert!(!j.contains("\"ortak_birincilikler\""), "eküri ödemesi ortak birincilik olarak yazılmamalı");
    assert!(j.contains("EKURI_ILE_ACIKLANDI"), "{j}");
}

#[test]
fn bursa_16_subat_ekuri_ganyani_ortak_birincilik_sayilmaz() {
    let (kod, out, j) = calistir("b0216", &ham("2026-02", "20260216-BURSA-program.csv"), &ham("2026-02", "20260216-BURSA-sonuc.csv"));
    assert_eq!(kod, 0, "{out}");
    assert!(j.contains("\"gun_durumu\": \"TAMAM\""), "{out}");
    assert!(j.contains("EKURI_ILE_ACIKLANDI"));
}

/// Tutarlar farklıysa eküri açıklaması geçmez: gün incelemede kalır.
#[test]
fn ekuri_ganyani_tutarlar_farkliysa_incelemede_kalir() {
    let s = turet("2026-02", "20260202-BURSA-sonuc.csv", "GANYAN(1) :3,45 TL", "GANYAN(1) :3,55 TL", "b0202_tutar");
    let (_, out, j) = calistir("b0202_tutar", &ham("2026-02", "20260202-BURSA-program.csv"), &s);
    assert!(j.contains("\"gun_durumu\": \"INCELEME_GEREKLI\""), "{out}");
    assert!(j.contains("ORTAK_BIRINCILIK_DOGRULANAMADI: 3. koşu"));
}

/// Numaralar programda aynı eküride değilse eküri açıklaması geçmez.
#[test]
fn ekuri_ganyani_eküri_ibaresi_yoksa_incelemede_kalir() {
    let p = turet("2026-02", "20260202-BURSA-program.csv", "[(1)ÇELİK TOYNAK,(7)KIZIL KUYRUK,] eküridir.", "", "b0202_ekurisiz");
    let (_, out, j) = calistir("b0202_ekurisiz", &p, &ham("2026-02", "20260202-BURSA-sonuc.csv"));
    assert!(j.contains("\"gun_durumu\": \"INCELEME_GEREKLI\""), "{out}");
    assert!(j.contains("ORTAK_BIRINCILIK_DOGRULANAMADI: 3. koşu"));
}
