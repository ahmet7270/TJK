//! Mart 2026 istisnaları için uçtan uca regresyon testleri. Girdi: depodaki doğrulanmış ham CSV'ler
//! (`../kontrol/ham`). Ham dosyalar okunur, asla değiştirilmez; olumsuz senaryolar geçici kopyalarla sınanır.

use std::path::{Path, PathBuf};
use std::process::Command;

fn ham(ad: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../kontrol/ham").join(ad)
}

fn gecici(ad: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("tjk_test_{}_{ad}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

struct Cikti {
    kod: i32,
    stdout: String,
    stderr: String,
    klasor: PathBuf,
}

impl Cikti {
    fn oku(&self, ad: &str) -> String {
        std::fs::read_to_string(self.klasor.join(ad)).unwrap_or_default()
    }
    fn json(&self) -> String {
        self.oku("altili.json")
    }
    /// CSV'den (kayıt sütunu = değer) olan satırın istenen sütununu döner.
    fn hucre(&self, dosya: &str, anahtar_sutun: &str, anahtar: &str, sutun: &str) -> String {
        let t = csv(&self.oku(dosya));
        let b = &t[0];
        let ia = b.iter().position(|x| x == anahtar_sutun).unwrap();
        let is = b.iter().position(|x| x == sutun).unwrap_or_else(|| panic!("{sutun} sütunu yok"));
        t.iter().skip(1).find(|r| r[ia] == anahtar).map(|r| r[is].clone()).unwrap_or_else(|| panic!("{anahtar} yok"))
    }
    fn satirlar(&self, dosya: &str) -> Vec<Vec<String>> {
        csv(&self.oku(dosya))
    }
}

fn csv(t: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let (mut satir, mut alan, mut tirnak) = (Vec::new(), String::new(), false);
    let mut it = t.chars().peekable();
    while let Some(c) = it.next() {
        match (c, tirnak) {
            ('"', true) if it.peek() == Some(&'"') => {
                alan.push('"');
                it.next();
            }
            ('"', true) => tirnak = false,
            ('"', false) if alan.is_empty() => tirnak = true,
            (',', false) => satir.push(std::mem::take(&mut alan)),
            ('\r', false) => {}
            ('\n', false) => {
                satir.push(std::mem::take(&mut alan));
                out.push(std::mem::take(&mut satir));
            }
            (c, _) => alan.push(c),
        }
    }
    out
}

fn calistir(ad: &str, args: &[&Path], ek: &[&str]) -> Cikti {
    let klasor = gecici(ad);
    let o = Command::new(env!("CARGO_BIN_EXE_tjk_ayristirici"))
        .args(args)
        .arg(klasor.join("cikti"))
        .args(ek)
        .output()
        .unwrap();
    Cikti {
        kod: o.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&o.stdout).into(),
        stderr: String::from_utf8_lossy(&o.stderr).into(),
        klasor: klasor.join("cikti"),
    }
}

/// Ham dosyanın bir metin değişikliği uygulanmış geçici kopyası (ham dosya değişmez).
fn turet(kaynak: &str, eski: &str, yeni: &str, test: &str) -> PathBuf {
    let b = std::fs::read(ham(kaynak)).unwrap();
    let t = String::from_utf8(b).unwrap();
    assert_eq!(t.matches(eski).count(), 1, "türetme ifadesi tek olmalı: {eski}");
    let d = gecici(&format!("{test}_girdi"));
    let y = d.join(kaynak);
    std::fs::write(&y, t.replacen(eski, yeni, 1)).unwrap();
    y
}

// ------------------------------------------------------------------ İstanbul, YP eki

#[test]
fn istanbul_21_mart_leonidas_yp_ekuri() {
    let c = calistir("ist", &[&ham("20260321-ISTANBUL-program.csv"), &ham("20260321-ISTANBUL-sonuc.csv")], &[]);
    assert_eq!(c.kod, 0, "{}", c.stderr);
    assert!(c.json().contains("\"gun_durumu\": \"TAMAM\""), "{}", c.stdout);
    assert!(!c.json().contains("EKURI_UYESI_DOGRULANAMADI"));
    let id = "20260321-ISTANBUL-K06-A06";
    assert_eq!(c.hucre("program.csv", "kayit_id", id, "at_isim_ham"), "LEONİDAS SKG SK YP", "ham ad korunmalı");
    assert_eq!(c.hucre("program.csv", "kayit_id", id, "at_adi"), "LEONİDAS");
    assert_eq!(c.hucre("program.csv", "kayit_id", id, "isim_kodlari"), "SKG SK YP");
    assert_eq!(c.hucre("program.csv", "kayit_id", id, "ekuri_grubu"), "5,6");
}

// ------------------------------------------------------------------ Antalya, ortak birincilik

#[test]
fn antalya_31_mart_ortak_birinci_ve_ayri_altili_odemeleri() {
    let c = calistir("ant", &[&ham("20260331-ANTALYA-program.csv"), &ham("20260331-ANTALYA-sonuc.csv")], &[]);
    assert_eq!(c.kod, 0, "{}", c.stderr);
    let j = c.json();
    assert!(j.contains("\"gun_durumu\": \"TAMAM\""), "{}", c.stdout);
    assert!(j.contains("\"ozet\": \"8. koşu ortak birinci: 5 MAVİŞİN KIZI + 6 ŞUKELA\""));
    // İki kazanan, ikisi de bitiş 1; üçüncü at 3. sırada kalır
    assert_eq!(c.hucre("sonuclar.csv", "kayit_id", "20260331-ANTALYA-K08-A05", "bitis_sirasi"), "1");
    assert_eq!(c.hucre("sonuclar.csv", "kayit_id", "20260331-ANTALYA-K08-A06", "bitis_sirasi"), "1");
    assert_eq!(c.hucre("sonuclar.csv", "kayit_id", "20260331-ANTALYA-K08-A06", "sonuc_liste_sirasi"), "2");
    assert_eq!(c.hucre("sonuclar.csv", "kayit_id", "20260331-ANTALYA-K08-A11", "bitis_sirasi"), "3");
    // Her altılı kombinasyonu kendi ödemesiyle; tek ödeme alanı boş
    assert!(j.contains("\"ibare\": \"2. 6'LI GANYAN(7/8/1/1/8/5) :2.093,81 TL\""));
    assert!(j.contains("\"ibare\": \"2. 6'LI GANYAN(7/8/1/1/8/6) :1.686,12 TL\""));
    assert!(j.contains("\"odeme_tl\": 2093.81") && j.contains("\"odeme_tl\": 1686.12"));
    assert!(c.stdout.contains("Bahis satırı kontrolü  : 25 kontrol, 0 uyumsuz"), "{}", c.stdout);
    assert!(c.stdout.contains("2 kazanan kombinasyon"));
    // 1. altılı (1.–6. koşu) etkilenmez
    assert!(c.stdout.contains("Altılı 20260331-ANTALYA-A1: başlangıç 1. koşu | kombinasyon 5,7/2,3/7/8/1/1 | ödeme 1.787,18 TL"));
}

#[test]
fn antalya_ayni_derece_tek_basina_ortak_birincilik_kaniti_degil() {
    // İkinci GANYAN ödemesi kaldırılırsa (dereceler hâlâ aynı, Fark 'At Başı') ortak birincilik kabul edilmez.
    let s = turet("20260331-ANTALYA-sonuc.csv", "GANYAN(6) :1,60 TL, ", "", "ant_tek_ganyan");
    let c = calistir("ant_tek_ganyan", &[&ham("20260331-ANTALYA-program.csv"), &s], &[]);
    assert_eq!(c.kod, 0, "{}", c.stderr);
    let j = c.json();
    assert!(!j.contains("ortak_birincilikler"));
    assert_eq!(c.hucre("sonuclar.csv", "kayit_id", "20260331-ANTALYA-K08-A06", "bitis_sirasi"), "2");
    // İki altılı ödemesi ortak birincilikle açıklanamadığı için gün incelemede
    assert!(j.contains("ALTILI_COKLU_ODEME_ACIKLANAMADI"), "{}", c.stdout);
    assert!(j.contains("\"gun_durumu\": \"INCELEME_GEREKLI\""));
}

#[test]
fn antalya_at_basi_ibaresi_yoksa_ortak_birincilik_dogrulanamaz() {
    let s = turet("20260331-ANTALYA-sonuc.csv", "1:33.70;1,80;At Başı", "1:33.70;1,80;Burun", "ant_burun");
    let c = calistir("ant_burun", &[&ham("20260331-ANTALYA-program.csv"), &s], &[]);
    let j = c.json();
    assert!(j.contains("ORTAK_BIRINCILIK_DOGRULANAMADI: 8. koşu"), "{}", c.stdout);
    assert!(!j.contains("\"ortak_birincilikler\""));
    assert_eq!(c.hucre("sonuclar.csv", "kayit_id", "20260331-ANTALYA-K08-A06", "bitis_sirasi"), "2");
}

// ------------------------------------------------------------------ Adana, ertelenen koşular

const NOT_15: &str = "TJK duyuru 52071 (test)";

#[test]
fn adana_15_mart_ertelenen_kosular_tek_sayilir_tarihler_ayri() {
    let c = calistir(
        "ada15",
        &[&ham("20260315-ADANA-program.csv"), &ham("20260315-ADANA-sonuc.csv")],
        &["--ek-sonuc", ham("20260316-ADANA-sonuc.csv").to_str().unwrap(), "--ertelenme-notu", NOT_15],
    );
    assert_eq!(c.kod, 0, "{}", c.stderr);
    let t = c.satirlar("kosu_tarihleri.csv");
    assert_eq!(t.len(), 1 + 8, "programdaki 8 koşu, her biri bir kez");
    for kn in 1..=8 {
        let id = format!("20260315-ADANA-K{kn:02}");
        let durum = c.hucre("kosu_tarihleri.csv", "kosu_id", &id, "tarih_durumu");
        let gt = c.hucre("kosu_tarihleri.csv", "kosu_id", &id, "gerceklesen_tarih");
        assert_eq!(c.hucre("kosu_tarihleri.csv", "kosu_id", &id, "planlanan_tarih"), "2026-03-15");
        if kn <= 2 {
            assert_eq!((durum.as_str(), gt.as_str()), ("PLANLANDIGI_GUN", "2026-03-15"), "{id}");
        } else {
            assert_eq!((durum.as_str(), gt.as_str()), ("ERTELENDI", "2026-03-16"), "{id}");
            assert_eq!(c.hucre("kosu_tarihleri.csv", "kosu_id", &id, "ertelenme_notu"), NOT_15);
        }
    }
    // İlk iki koşu 15.03 dosyasından korunur, 3.–8. koşular 16.03 dosyasından; tüm atlar tekil eşleşir
    let s = c.satirlar("sonuclar.csv");
    assert_eq!(s.len() - 1, 77);
    let b = &s[0];
    let (ie, ik) = (b.iter().position(|x| x == "eslesme_durumu").unwrap(), b.iter().position(|x| x == "kaynak_dosya").unwrap());
    let ikn = b.iter().position(|x| x == "kosu_no").unwrap();
    assert!(s.iter().skip(1).all(|r| r[ie] == "TEKIL_ESLESME"));
    assert!(s.iter().skip(1).all(|r| (r[ikn].parse::<u32>().unwrap() <= 2) == (r[ik] == "20260315-ADANA-sonuc.csv")));
    let j = c.json();
    assert!(!j.contains("ESLESMEYEN_PROGRAM_KAYDI") && !j.contains("ERTELENEN_KOSU_OZELLIK_FARKI"));
    // Dosyada olmayan iki ödeme incelemede kalır (bahis ertelemeyle iptal mi, dosyada yazmıyor)
    assert!(j.contains("GANYAN_SATIRI_BULUNAMADI: 3. koşu"));
    assert!(j.contains("ALTILI_ODEMESI_BULUNAMADI: 20260315-ADANA-A1"));
    assert!(j.contains("\"odeme_ham\": \"19.387,81 TL\""), "2. altılı ödemesi okunmalı");
}

#[test]
fn adana_farkli_tarihli_sonuc_erteleme_notu_olmadan_reddedilir() {
    let c = calistir(
        "ada15_notsuz",
        &[&ham("20260315-ADANA-program.csv"), &ham("20260315-ADANA-sonuc.csv")],
        &["--ek-sonuc", ham("20260316-ADANA-sonuc.csv").to_str().unwrap()],
    );
    assert_eq!(c.kod, 1);
    assert!(c.stderr.contains("aynı yarış gününe ait değil") && c.stderr.contains("--ertelenme-notu"), "{}", c.stderr);
}

#[test]
fn ayni_kosu_iki_sonuc_dosyasinda_ikinci_kez_sayilmaz() {
    let c = calistir(
        "ada15_cift",
        &[&ham("20260315-ADANA-program.csv"), &ham("20260315-ADANA-sonuc.csv")],
        &["--ek-sonuc", ham("20260315-ADANA-sonuc.csv").to_str().unwrap()],
    );
    assert_eq!(c.kod, 1);
    assert!(c.stderr.contains("ikinci kez sayılmaz"), "{}", c.stderr);
}

#[test]
fn baska_yaris_gunu_sonucu_reddedilir() {
    // 16.03 dosyası 32. yarış günü; 21.03 programı 34. yarış günü
    let c = calistir(
        "ada21_yanlis",
        &[&ham("20260321-ADANA-program.csv"), &ham("20260316-ADANA-sonuc.csv")],
        &["--ertelenme-notu", "test"],
    );
    assert_eq!(c.kod, 1);
    assert!(c.stderr.contains("aynı yarış gününe ait değil"), "{}", c.stderr);
}

#[test]
fn adana_21_mart_34_yaris_gunu_23_mart_sonuclari() {
    let c = calistir(
        "ada21",
        &[&ham("20260321-ADANA-program.csv"), &ham("20260323-ADANA-sonuc.csv")],
        &["--ertelenme-notu", "test"],
    );
    assert_eq!(c.kod, 0, "{}", c.stderr);
    assert!(c.json().contains("\"gun_durumu\": \"TAMAM\""), "{}", c.stdout);
    let t = c.satirlar("kosu_tarihleri.csv");
    assert_eq!(t.len(), 1 + 9);
    let id = "20260321-ADANA-K04";
    assert_eq!(c.hucre("kosu_tarihleri.csv", "kosu_id", id, "tarih_durumu"), "ERTELENDI");
    assert_eq!(c.hucre("kosu_tarihleri.csv", "kosu_id", id, "gerceklesen_tarih"), "2026-03-23");
    assert_eq!(c.hucre("kosu_tarihleri.csv", "kosu_id", id, "pist_program"), "Çim");
    assert_eq!(c.hucre("kosu_tarihleri.csv", "kosu_id", id, "pist_sonuc"), "Kum");
    assert_eq!(c.satirlar("sonuclar.csv").len() - 1, 89);
}

// ------------------------------------------------------------------ normal gün: yeni alanlar yazılmaz

#[test]
fn normal_gunde_ertelenme_ve_ortak_birincilik_alani_yok() {
    let c = calistir("ada01", &[&ham("20260301-ADANA-program.csv"), &ham("20260301-ADANA-sonuc.csv")], &[]);
    assert_eq!(c.kod, 0, "{}", c.stderr);
    let j = c.json();
    assert!(j.contains("\"gun_durumu\": \"TAMAM\""));
    assert!(!j.contains("ertelenen_kosular") && !j.contains("ortak_birincilikler") && !j.contains("kazanan_kombinasyonlar"));
    let t = c.satirlar("kosu_tarihleri.csv");
    assert_eq!(t.len(), 1 + 8);
    assert!(t.iter().skip(1).all(|r| r.contains(&"PLANLANDIGI_GUN".to_string())));
}
