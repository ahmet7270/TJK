//! TJK günlük CSV ayrıştırıcı (Rust, bağımlılıksız).
//!
//! Kullanım:
//!   tjk_ayristirici PROGRAM.csv SONUC.csv CIKTI_KLASORU
//!       [--kontrol-komb "6 / 1,4 / 6 / 4 / 7 / 4"] [--kontrol-odeme "29.789,50 TL"]
//!       [--ilgili-url URL] [--ilgili-not METIN]
//!
//! Kurallar:
//!   * Sonuç dosyasından program çıktısına hiçbir değer taşınmaz.
//!   * Dosyada olmayan değer üretilmez; boş kalır ve gerekçesi kaydedilir.
//!   * --kontrol-* seçenekleri tekrarlanabilir; k. değer günün k. altılısına uygulanır.

mod ayristir;
mod csvw;
mod json;
mod kosu_adi;
mod kodlama;
mod sha256;
mod sqlite;
mod toplu;
mod veritabani;

use ayristir::{
    agf_coklu, bahis_ayristir, ekuri_ayir, ekuri_ayir_ayrintili, gun_ayristir, isim_coz, jokey_coz, kilo_coz, start_coz, sure_sn, to_int,
    tr_num, yas_coz, Bahis, Gun, KOSMAZ_IBARE,
};
use csvw::Satir;
use json::{obj, Json};
use std::collections::{HashMap, HashSet};
use std::path::Path;

const KULLANIM: &str = "Kullanım: tjk_ayristirici PROGRAM.csv SONUC.csv CIKTI_KLASORU \
[--kontrol-komb KOMB] [--kontrol-odeme TUTAR] [--ilgili-url URL] [--ilgili-not METIN] [--sqlite VERITABANI.sqlite]";
/// Ayrıştırma mantığının sürümü (yalnızca ayrıştırma/doğrulama davranışı değişince artırılır).
/// Veri tabanındaki yaris_gunu.ayristirici_surumu bu değerdir; toplu aktarımda "aynı sürüm" kontrolü buna bakar.
pub const AYRISTIRICI_SURUMU: &str = "0.9.0";
const ALTILI_IBARE: &str = "6'LI GANYAN Bu koşudan başlar";
const ESLESME_YONTEMI: &str = "tarih+hipodrom+kosu_no+at_adi (sonuç CSV'sinde program no yok)";
const ZAMAN_UYARI_KOSMAZLI: &str = "Arşiv dosyası anlık görüntü; zaman damgası yok. Aynı dosyada 'Koşmaz' ibaresi \
bulunduğu için dosya yarış günü güncellenmiş hâli yansıtıyor olabilir. AGF/jokey/kilo değerlerinin altılı başlamadan \
önceki hâli DOĞRULANAMADI.";
const ZAMAN_UYARI_SADE: &str = "Arşiv dosyası anlık görüntü; zaman damgası yok. AGF/jokey/kilo değerlerinin altılı \
başlamadan önceki hâli DOĞRULANAMADI.";
const IKRAMIYE_ANAHTAR: [&str; 5] = ["ikramiye_1_tl", "ikramiye_2_tl", "ikramiye_3_tl", "ikramiye_4_tl", "ikramiye_5_tl"];
const PRIM_ANAHTAR: [&str; 5] = [
    "yetistirici_primi_1_tl",
    "yetistirici_primi_2_tl",
    "yetistirici_primi_3_tl",
    "yetistirici_primi_4_tl",
    "yetistirici_primi_5_tl",
];
/// Programla sonuç dosyasında tekrar eden alanlar (yalnızca tutarlılık kontrolü için; değer taşınmaz).
const KARSILASTIR: [(usize, &str); 11] = [
    (1, "at_isim_ham"),
    (2, "yas_ham"),
    (3, "baba"),
    (4, "anne"),
    (5, "kilo_ham"),
    (6, "jokey_ham"),
    (7, "sahip"),
    (8, "antrenor"),
    (9, "start_ham"),
    (10, "agf_ham"),
    (11, "h"),
];
/// Sıralı bahisler program numarası verir: (oyun, kaç sıra)
const SIRALI: [(&str, usize); 5] =
    [("GANYAN", 1), ("SIRALI İKİLİ", 2), ("ÜÇLÜ BAHİS", 3), ("TABELA BAHİS", 4), ("SIRALI 5 Lİ BAHİS", 5)];

// ---------------------------------------------------------------- veri yapıları

struct Secenekler {
    program: String,
    sonuc: String,
    cikti: String,
    kontrol_komb: Vec<String>,
    kontrol_odeme: Vec<String>,
    ilgili_url: Option<String>,
    ilgili_not: Option<String>,
    sqlite: Option<String>,
}

struct Kaynak {
    ad: String,
    sha: String,
    kodlama: String,
    gun: Gun,
}

struct ProgAt {
    kayit_id: String,
    kosu_no: u32,
    at_no: i64,
    at_adi: String,
    kosmaz: bool,
    agf_ham: String,
    /// AGF hücresindeki tüm değerler (yüzde, sıra), hücredeki sırayla
    agf_degerler: Vec<(f64, i64)>,
    /// Atın programda "eküridir" ibaresiyle bağlı olduğu grup (boşsa eküri yok) ve ibarenin satırı
    ekuri: Vec<i64>,
    ekuri_satir: usize,
    karsilastir: Vec<String>, // KARSILASTIR sırasıyla program değerleri
    satir: Satir,
}

struct SonHam {
    satir_no: usize,
    c: Vec<String>,
    ad: String,
}

struct SonucSatir {
    kayit_id: String,
    gun_id: String,
    kosu_id: String,
    tarih: String,
    hipodrom: String,
    kosu_no: u32,
    program_at_no: i64,
    at_adi: String,
    liste_sirasi: Option<i64>,
    bitis: Option<i64>,
    durum: String,
    derece_ham: String,
    derece_sn: Option<f64>,
    ganyan: Option<f64>,
    fark: String,
    eslesme_durumu: String,
    eslesme_yontemi: String,
    capraz: String,
    bahis_dogrulama: String,
    kaynak_dosya: String,
    kaynak_satir: Option<usize>,
    sha: String,
}

impl SonucSatir {
    fn satir(&self) -> Satir {
        vec![
            ("kayit_id", (&self.kayit_id).into()),
            ("gun_id", (&self.gun_id).into()),
            ("kosu_id", (&self.kosu_id).into()),
            ("tarih", (&self.tarih).into()),
            ("hipodrom", (&self.hipodrom).into()),
            ("kosu_no", self.kosu_no.into()),
            ("program_at_no", self.program_at_no.into()),
            ("at_adi", (&self.at_adi).into()),
            ("sonuc_liste_sirasi", self.liste_sirasi.into()),
            ("bitis_sirasi", self.bitis.into()),
            ("kosu_durumu", (&self.durum).into()),
            ("derece_ham", (&self.derece_ham).into()),
            ("derece_sn", self.derece_sn.into()),
            ("ganyan_oran", self.ganyan.into()),
            ("fark_ham", (&self.fark).into()),
            ("eslesme_durumu", (&self.eslesme_durumu).into()),
            ("eslesme_yontemi", (&self.eslesme_yontemi).into()),
            ("capraz_kontrol_farklari", (&self.capraz).into()),
            ("bahis_satiri_dogrulamasi", (&self.bahis_dogrulama).into()),
            ("kaynak_dosya", (&self.kaynak_dosya).into()),
            ("kaynak_satir", self.kaynak_satir.into()),
            ("kaynak_sha256", (&self.sha).into()),
            ("kaynak_url", "".into()),
        ]
    }
}

struct KosuBahis {
    kosu_no: u32,
    satir_no: usize,
    bahisler: Vec<Bahis>,
    /// Okunamayan bahis parçaları (ham metin)
    okunamayan: Vec<String>,
}

struct BahisKontrol {
    kosu_no: u32,
    oyun: String,
    bahis: Vec<i64>,
    eslesmeden: Vec<Option<i64>>,
    uyumlu: bool,
}

struct KosmazDurum {
    ayak: usize,
    kosu_no: u32,
    no: i64,
    ad: String,
    gecerli: Option<bool>,
    agf1_no: Option<i64>,
    agf1_ad: Option<String>,
    agf1_birinci: Option<bool>,
    h2_tahmin: Option<bool>,
}

/// Programdaki "... 6'LI GANYAN Bu koşudan başlar" ibaresinden çıkan altılı tanımı.
struct Tanim {
    altili_id: String,
    oyun: String,
    bas: u32,
    alt_no: usize,
    ibare: String,
}

impl Tanim {
    fn ayaklar(&self) -> Vec<u32> {
        (0..6).map(|i| self.bas + i).collect()
    }
    fn ayak_no(&self, kosu_no: u32) -> Option<usize> {
        (kosu_no >= self.bas && kosu_no < self.bas + 6).then(|| (kosu_no - self.bas + 1) as usize)
    }
}

fn altili_tanimlari(g: &Gun, gun_id: &str) -> Vec<Tanim> {
    let mut v = Vec::new();
    for k in &g.kosular {
        let (_, kalan) = ekuri_ayir(&k.alt_satir);
        for parca in kalan.split(';').map(str::trim) {
            if parca.contains(ALTILI_IBARE) {
                v.push(Tanim {
                    altili_id: format!("{gun_id}-A{}", v.len() + 1),
                    oyun: parca.replace(" Bu koşudan başlar", "").trim().to_string(),
                    bas: k.kosu_no,
                    alt_no: k.alt_satir_no,
                    ibare: parca.to_string(),
                });
            }
        }
    }
    v
}

/// Bir koşuyu içeren altılıların (tanım sırası) indeksleri.
fn kosu_altililari(t: &[Tanim], kosu_no: u32) -> Vec<usize> {
    t.iter().enumerate().filter(|(_, x)| x.ayak_no(kosu_no).is_some()).map(|(i, _)| i).collect()
}

// ---------------------------------------------------------------- yardımcılar

fn deger(it: &mut impl Iterator<Item = String>, ad: &str) -> Result<String, String> {
    it.next().ok_or_else(|| format!("{ad} bir değer bekliyor\n{KULLANIM}"))
}

fn argumanlar() -> Result<Secenekler, String> {
    let mut poz = Vec::new();
    let (mut kk, mut ko, mut iu, mut inot, mut sq) = (Vec::new(), Vec::new(), None, None, None);
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--kontrol-komb" => kk.push(deger(&mut it, &a)?),
            "--kontrol-odeme" => ko.push(deger(&mut it, &a)?),
            "--ilgili-url" => iu = Some(deger(&mut it, &a)?),
            "--ilgili-not" => inot = Some(deger(&mut it, &a)?),
            "--sqlite" => sq = Some(deger(&mut it, &a)?),
            "-h" | "--help" => return Err(KULLANIM.into()),
            s if s.starts_with("--") => return Err(format!("bilinmeyen seçenek: {s}\n{KULLANIM}")),
            _ => poz.push(a),
        }
    }
    if poz.len() != 3 {
        return Err(KULLANIM.into());
    }
    Ok(Secenekler {
        program: poz[0].clone(),
        sonuc: poz[1].clone(),
        cikti: poz[2].clone(),
        kontrol_komb: kk,
        kontrol_odeme: ko,
        ilgili_url: iu,
        ilgili_not: inot,
        sqlite: sq,
    })
}

fn yukle(yol: &str) -> Result<Kaynak, String> {
    let bytes = std::fs::read(yol).map_err(|e| format!("{yol} okunamadı: {e}"))?;
    let (metin, kodlama) = kodlama::coz(&bytes);
    let gun = gun_ayristir(&metin).map_err(|e| format!("{yol}: {e}"))?;
    let ad = Path::new(yol).file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| yol.into());
    Ok(Kaynak { ad, sha: sha256::hex(&bytes), kodlama, gun })
}

fn hipodrom_slug(h: &str) -> String {
    let mut out = String::new();
    for c in h.chars() {
        let m = match c {
            'İ' | 'ı' | 'i' => 'I',
            'Ş' | 'ş' => 'S',
            'Ğ' | 'ğ' => 'G',
            'Ü' | 'ü' => 'U',
            'Ö' | 'ö' => 'O',
            'Ç' | 'ç' => 'C',
            c if c.is_ascii_alphanumeric() => c.to_ascii_uppercase(),
            _ => '_',
        };
        if m != '_' || (!out.is_empty() && !out.ends_with('_')) {
            out.push(m);
        }
    }
    out.trim_end_matches('_').to_string()
}

/// Yükleme sırasında dosya adında Türkçe harf '_' ile değişmişse not üretir.
fn dosya_adi_notu(dosya: &str, hipodrom: &str) -> Option<String> {
    let turkce: String = hipodrom.chars().filter(|c| !c.is_ascii()).collect();
    if turkce.is_empty() || dosya.contains(hipodrom) {
        return None;
    }
    let degismis: String = hipodrom.chars().map(|c| if c.is_ascii() { c } else { '_' }).collect();
    let harfler = turkce.chars().map(|c| c.to_string()).collect::<Vec<_>>().join("', '");
    dosya
        .contains(&degismis)
        .then(|| format!("Dosya adındaki '_' orijinal adda '{harfler}' olabilir (yükleme sırasında değişmiş)."))
}

fn tr2(x: f64) -> String {
    format!("{x:.2}").replace('.', ",")
}

fn yuvarla2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0 + 0.0 // -0.0 yerine 0.0
}

fn komb_coz(s: &str) -> Option<Vec<Vec<i64>>> {
    s.split('/').map(|p| p.split(',').map(|n| n.trim().parse::<i64>().ok()).collect()).collect()
}

// ---------------------------------------------------------------- ana akış

fn main() {
    if let Err(e) = calistir() {
        eprintln!("HATA: {e}");
        std::process::exit(1);
    }
}

fn calistir() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("toplu") {
        return toplu::calistir(&args[1..]);
    }
    if args.first().map(String::as_str) == Some("goc") {
        return goc_komutu(&args[1..]);
    }
    if args.first().map(String::as_str) == Some("kosu-adi") {
        return kosu_adi::calistir(&args[1..]);
    }
    let sec = argumanlar()?;
    let p = yukle(&sec.program)?;
    let s = yukle(&sec.sonuc)?;
    gunu_isle(&sec, p, s, true, |gv| {
        if let Some(yol) = &sec.sqlite {
            let o = veritabani::aktar(yol, gv)?;
            println!(
                "SQLite                 : {} tek işlemde aktarıldı -> {yol} (koşu {}, katılım {}, sonuç {}, altılı {}, ayak {}, geçerli no {}, AGF gözlemi {})",
                gv.gun_id, o.kosu, o.katilim, o.sonuc, o.altili, o.ayak, o.gecerli, o.agf
            );
        }
        Ok(())
    })
}

/// Bir program+sonuç çiftini ayrıştırır, doğrular, CSV/JSON çıktılarını yazar ve `sonra` ile veriyi teslim eder.
fn gunu_isle<T>(
    sec: &Secenekler,
    p: Kaynak,
    s: Kaynak,
    ayrintili: bool,
    sonra: impl FnOnce(&GunVerisi) -> Result<T, String>,
) -> Result<T, String> {
    let (pg, sg) = (&p.gun, &s.gun);
    if (&pg.tarih, &pg.hipodrom, pg.yaris_gunu_no) != (&sg.tarih, &sg.hipodrom, sg.yaris_gunu_no) {
        return Err(format!(
            "Program ({} {} {}) ile sonuç ({} {} {}) aynı yarış gününe ait değil",
            pg.tarih, pg.hipodrom, pg.yaris_gunu_no, sg.tarih, sg.hipodrom, sg.yaris_gunu_no
        ));
    }
    let gun_id = format!("{}-{}", pg.tarih.replace('-', ""), hipodrom_slug(&pg.hipodrom));
    std::fs::create_dir_all(&sec.cikti).map_err(|e| format!("{} oluşturulamadı: {e}", sec.cikti))?;

    let tanimlar = altili_tanimlari(pg, &gun_id);
    let prog = program_satirlari(&p, &gun_id, &tanimlar)?;
    let (mut son, sonuc_ham, bahis_kosu, sorunlar, eslesmeyen) = eslestir(&p, &s, &prog, &gun_id)?;
    let bk = bahis_kontrol(&bahis_kosu, &mut son);
    let (ganyan_json, ganyan_inceleme) = ganyan_kontrolu(&p, &prog, &son, &bahis_kosu);

    // altılı
    let mut uyarilar = Vec::new();
    for kb in &bahis_kosu {
        for h in &kb.okunamayan {
            uyarilar.push(format!("BAHIS_PARCASI_OKUNAMADI: {} satır {} ({}. koşu): {h}", s.ad, kb.satir_no, kb.kosu_no));
        }
    }
    let mut inceleme: Vec<String> = sorunlar.clone();
    inceleme.extend(eslesmeyen.iter().cloned());
    let (ekuri_inceleme, ekuri_ozet) = ekuri_kontrolu(&p, &s, &prog);
    if ayrintili {
        println!("Eküri kontrolü         : {ekuri_ozet}");
    }
    inceleme.extend(ekuri_inceleme);
    inceleme.extend(ganyan_inceleme.iter().cloned());
    let altililar =
        altililari_kur(&sec, &p, &s, &gun_id, &tanimlar, &prog, &son, &bahis_kosu, &mut uyarilar, &mut inceleme);
    let gun_durumu = if inceleme.is_empty() { "TAMAM" } else { "INCELEME_GEREKLI" };

    // Koşmaz özeti: benzersiz katılım (program veya sonuçta Koşmaz) ve altılılar içindeki toplam kayıt
    let mut kosmaz_idler: Vec<&str> = prog.iter().filter(|r| r.kosmaz).map(|r| r.kayit_id.as_str()).collect();
    for x in son.iter().filter(|x| x.durum == "KOSMAZ") {
        if !kosmaz_idler.contains(&x.kayit_id.as_str()) {
            kosmaz_idler.push(&x.kayit_id);
        }
    }
    let kosmaz_kayitlari: Vec<Json> = kosmaz_idler
        .iter()
        .map(|id| {
            let r = prog.iter().find(|r| r.kayit_id == *id);
            let kn = r.map(|r| r.kosu_no).unwrap_or(0);
            let bag: Vec<String> = kosu_altililari(&tanimlar, kn)
                .iter()
                .map(|&i| format!("{}:{}", tanimlar[i].altili_id, tanimlar[i].ayak_no(kn).unwrap_or(0)))
                .collect();
            obj(vec![
                ("kayit_id", (*id).into()),
                ("at_adi", r.map(|r| r.at_adi.clone()).into()),
                ("kosu_no", kn.into()),
                ("altili_baglantilari", bag.clone().into()),
                ("altili_kaydi_sayisi", bag.len().into()),
            ])
        })
        .collect();
    let altili_ici: usize = kosmaz_idler
        .iter()
        .filter_map(|id| prog.iter().find(|r| r.kayit_id == *id))
        .map(|r| kosu_altililari(&tanimlar, r.kosu_no).len())
        .sum();

    let altili_json = obj(vec![
        ("gun_id", (&gun_id).into()),
        ("tarih", (&pg.tarih).into()),
        ("hipodrom", (&pg.hipodrom).into()),
        ("yaris_gunu_no", pg.yaris_gunu_no.into()),
        ("gun_durumu", gun_durumu.into()),
        ("inceleme_nedenleri", inceleme.clone().into()),
        ("ganyan_kontrolu", Json::Arr(ganyan_json.clone())),
        (
            "kosmaz_ozeti",
            obj(vec![
                ("benzersiz_kosmaz_katilim", kosmaz_idler.len().into()),
                ("altililar_icindeki_kosmaz_kaydi", altili_ici.into()),
                ("kayitlar", Json::Arr(kosmaz_kayitlari)),
            ]),
        ),
        ("altili_sayisi", altililar.len().into()),
        ("altililar", Json::Arr(altililar.clone())),
        ("uyarilar", uyarilar.clone().into()),
    ]);

    // yaz
    let cik = |ad: &str| Path::new(&sec.cikti).join(ad).to_string_lossy().into_owned();
    csvw::yaz(&cik("program.csv"), &prog.iter().map(|r| r.satir.clone()).collect::<Vec<_>>())?;
    csvw::yaz(&cik("sonuclar.csv"), &son.iter().map(SonucSatir::satir).collect::<Vec<_>>())?;
    std::fs::write(cik("altili.json"), altili_json.guzel()).map_err(|e| format!("altili.json yazılamadı: {e}"))?;

    if ayrintili {
        rapor(&p, &s, &tanimlar, &prog, &son, &sonuc_ham, &sorunlar, &eslesmeyen, &bk, &altililar, &uyarilar);
        println!("Koşmaz özeti           : {} benzersiz katılım, altılılar içinde {altili_ici} kayıt", kosmaz_idler.len());
        println!("GÜN DURUMU             : {gun_durumu}");
        for n in &inceleme {
            println!("  - {n}");
        }
    }
    {
        let gv = GunVerisi {
            p: &p,
            s: &s,
            gun_id: &gun_id,
            gun_durumu,
            inceleme: &inceleme,
            tanimlar: &tanimlar,
            prog: &prog,
            son: &son,
            bahis_kosu: &bahis_kosu,
            ganyan: &ganyan_json,
            altililar: &altililar,
        };
        sonra(&gv)
    }
}

fn program_satirlari(p: &Kaynak, gun_id: &str, tanimlar: &[Tanim]) -> Result<Vec<ProgAt>, String> {
    let g = &p.gun;
    let kosmaz_var = g.kosular.iter().any(|k| k.atlar.iter().any(|(_, c)| c.get(1).is_some_and(|x| x.contains(KOSMAZ_IBARE))));
    let zaman_uyari = if kosmaz_var { ZAMAN_UYARI_KOSMAZLI } else { ZAMAN_UYARI_SADE };
    let mut out: Vec<ProgAt> = Vec::new();
    let mut gorulen = HashSet::new();
    for k in &g.kosular {
        if k.kolonlar.len() != 16 || k.kolonlar[0] != "At No" || k.kolonlar[1] != "At İsmi" {
            return Err(format!("Program {}. koşu: beklenmeyen kolonlar {:?}", k.kosu_no, k.kolonlar));
        }
        let kosu_id = format!("{gun_id}-K{:02}", k.kosu_no);
        let (ekuriler, kalan) = ekuri_ayir(&k.alt_satir);
        let bahisler: Vec<&str> = kalan.split(';').map(str::trim).filter(|x| !x.is_empty()).collect();
        let bu_altililar = kosu_altililari(tanimlar, k.kosu_no);
        let baglantilar = bu_altililar
            .iter()
            .map(|&i| format!("{}:{}", tanimlar[i].altili_id, tanimlar[i].ayak_no(k.kosu_no).unwrap_or(0)))
            .collect::<Vec<_>>()
            .join("|");
        for (satir_no, c) in &k.atlar {
            if c.len() != 16 {
                return Err(format!("Program satır {satir_no}: {} alan var, 16 bekleniyordu", c.len()));
            }
            let at_no: i64 = c[0].parse().map_err(|_| format!("Program satır {satir_no}: at no okunamadı"))?;
            let isim = isim_coz(&c[1]);
            let (kilo, kilo_ek) = kilo_coz(&c[5]);
            let (jokey, ap) = jokey_coz(&c[6]);
            let (st, st_ek) = start_coz(&c[9]);
            let agf_degerler = agf_coklu(&c[10]);
            let agf_liste = agf_degerler.clone().unwrap_or_default();
            let (agf, agf_sira) = agf_liste.first().map_or((None, None), |x| (Some(x.0), Some(x.1)));
            let (agf2, agf2_sira) = agf_liste.get(1).map_or((None, None), |x| (Some(x.0), Some(x.1)));
            let agf_eslesme = match (agf_degerler.as_ref().map(Vec::len), bu_altililar.len()) {
                (None, _) => "COZULEMEDI_HAM_KORUNDU".to_string(),
                (Some(0), _) => String::new(),
                (Some(1), 1) => format!("TEK_ALTILI_CIKARIM:{}", tanimlar[bu_altililar[0]].altili_id),
                (Some(n), m) if n == m => format!(
                    "SIRA_DOGRULANMADI: {} değer, koşu {} altılıda ({}); hangi değerin hangi altılıya ait olduğu dosyada yazmıyor",
                    n,
                    m,
                    bu_altililar.iter().map(|&i| tanimlar[i].altili_id.as_str()).collect::<Vec<_>>().join(",")
                ),
                (Some(n), m) => format!("SAYI_UYUMSUZ: {n} değer, {m} altılı"),
            };
            // Altılıya özel AGF: yalnızca tek değer + tek altılı (eşleşme belirli). Çok değerli hücrede boş.
            let altili_ozel = |a: usize| -> Option<(f64, i64)> {
                match (agf_liste.len(), bu_altililar.as_slice()) {
                    (1, [tek]) if *tek == a => Some(agf_liste[0]),
                    _ => None,
                }
            };
            let ekuri: Vec<i64> = ekuriler
                .iter()
                .find(|g| g.iter().any(|(n, _)| *n == at_no))
                .map(|g| g.iter().map(|(n, _)| *n).collect())
                .unwrap_or_default();
            let h = to_int(&c[11]);
            let kayit_id = format!("{kosu_id}-A{at_no:02}");
            if !gorulen.insert(kayit_id.clone()) {
                return Err(format!("Tekrarlı program anahtarı {kayit_id}"));
            }
            let mut satir: Satir = vec![
                ("kayit_id", (&kayit_id).into()),
                ("gun_id", gun_id.into()),
                ("kosu_id", (&kosu_id).into()),
                ("tarih", (&g.tarih).into()),
                ("hipodrom", (&g.hipodrom).into()),
                ("yaris_gunu_no", g.yaris_gunu_no.into()),
                ("kosu_no", k.kosu_no.into()),
                ("kosu_saati", (&k.saat).into()),
                ("kosu_adi", (&k.kosu_adi).into()),
                ("kosu_cinsi", (&k.kosu_cinsi).into()),
                ("yas_grubu", (&k.yas_grubu).into()),
                ("kilo_sartlari", (&k.kilo_sartlari).into()),
                ("mesafe_m", k.mesafe_m.into()),
                ("pist", (&k.pist).into()),
                ("rekor_derece", (&k.rekor_derece).into()),
            ];
            for n in 0..5 {
                satir.push((IKRAMIYE_ANAHTAR[n], k.ikramiye[n].into()));
            }
            for n in 0..5 {
                satir.push((PRIM_ANAHTAR[n], k.yetistirici_primi[n].into()));
            }
            satir.extend([
                ("kosu_bahisleri", bahisler.join(" | ").into()),
                ("altili_baslangic_kosusu", bahisler.iter().any(|b| b.contains(ALTILI_IBARE)).into()),
                ("program_at_no", at_no.into()),
                ("at_isim_ham", (&c[1]).into()),
                ("at_adi", (&isim.ad).into()),
                ("isim_kodlari", (&isim.kodlar).into()),
                ("kosmaz_ibaresi_programda", isim.kosmaz.into()),
                ("yas_ham", (&c[2]).into()),
                ("yas", yas_coz(&c[2]).into()),
                ("baba", (&c[3]).into()),
                ("anne", (&c[4]).into()),
                ("kilo_ham", (&c[5]).into()),
                ("kilo", kilo.into()),
                ("kilo_ek", kilo_ek.into()),
                ("jokey_ham", (&c[6]).into()),
                ("jokey", jokey.into()),
                ("jokey_ap_ibaresi", ap.into()),
                ("sahip", (&c[7]).into()),
                ("antrenor", (&c[8]).into()),
                ("start_ham", (&c[9]).into()),
                ("start_no", st.into()),
                ("start_ek", st_ek.into()),
                ("agf_ham", (&c[10]).into()),
                ("agf_yuzde", agf.into()),
                ("agf_sira", agf_sira.into()),
                ("agf_deger_sayisi", agf_degerler.as_ref().map(Vec::len).into()),
                ("agf_2_yuzde", agf2.into()),
                ("agf_2_sira", agf2_sira.into()),
                ("agf_altili_eslesmesi", agf_eslesme.into()),
                ("altili_baglantilari", baglantilar.clone().into()),
                ("ekuri_grubu", ekuri.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",").into()),
                ("agf_altili1_yuzde", altili_ozel(0).map(|x| x.0).into()),
                ("agf_altili1_sira", altili_ozel(0).map(|x| x.1).into()),
                ("agf_altili2_yuzde", altili_ozel(1).map(|x| x.0).into()),
                ("agf_altili2_sira", altili_ozel(1).map(|x| x.1).into()),
                ("h", h.into()),
                ("son6_ham", (&c[12]).into()),
                ("kgs", to_int(&c[13]).into()),
                ("s20", to_int(&c[14]).into()),
                ("en_iyi_derece_ham", (&c[15]).into()),
                ("en_iyi_derece_sn", sure_sn(&c[15]).into()),
                ("agf_oncesi_dogrulandi", false.into()),
                ("jokey_oncesi_dogrulandi", false.into()),
                ("kilo_oncesi_dogrulandi", false.into()),
                ("kosmaz_zamani_dogrulandi", false.into()),
                ("zaman_uyarisi", zaman_uyari.into()),
                ("kaynak_dosya", (&p.ad).into()),
                ("kaynak_satir", (*satir_no).into()),
                ("kaynak_sha256", (&p.sha).into()),
                ("kaynak_url", "".into()),
            ]);
            let karsilastir = KARSILASTIR
                .iter()
                .map(|(idx, alan)| if *alan == "h" { h.map(|v| v.to_string()).unwrap_or_default() } else { c[*idx].clone() })
                .collect();
            out.push(ProgAt {
                kayit_id,
                kosu_no: k.kosu_no,
                at_no,
                at_adi: isim.ad,
                kosmaz: isim.kosmaz,
                agf_ham: c[10].clone(),
                agf_degerler: agf_liste,
                ekuri,
                ekuri_satir: k.alt_satir_no,
                karsilastir,
                satir,
            });
        }
    }
    Ok(out)
}

type Eslesme = (Vec<SonucSatir>, Vec<(u32, Vec<SonHam>)>, Vec<KosuBahis>, Vec<String>, Vec<String>);

fn eslestir(p: &Kaynak, s: &Kaynak, prog: &[ProgAt], gun_id: &str) -> Result<Eslesme, String> {
    let mut sonuc_ham: Vec<(u32, Vec<SonHam>)> = Vec::new();
    let mut bahis_kosu = Vec::new();
    for k in &s.gun.kosular {
        let n = k.kolonlar.len();
        if n != 15 || k.kolonlar[n - 3..] != ["Derece", "Ganyan", "Fark"] {
            return Err(format!("Sonuç {}. koşu: beklenmeyen kolonlar {:?}", k.kosu_no, k.kolonlar));
        }
        let (ekr, bahis_metni) = ekuri_ayir_ayrintili(&k.alt_satir);
        let (bahisler, mut okunamayan) = bahis_ayristir(bahis_metni);
        // Çözülemeyen eküri ibaresi ham hâliyle bahis uyarısı olarak da saklanır (kaynak satırıyla)
        okunamayan.splice(0..0, ekr.cozulemeyen);
        bahis_kosu.push(KosuBahis { kosu_no: k.kosu_no, satir_no: k.alt_satir_no, bahisler, okunamayan });
        let mut v = Vec::new();
        for (satir_no, c) in &k.atlar {
            if c.len() != 15 {
                return Err(format!("Sonuç satır {satir_no}: {} alan var, 15 bekleniyordu", c.len()));
            }
            v.push(SonHam { satir_no: *satir_no, ad: isim_coz(&c[1]).ad, c: c.clone() });
        }
        sonuc_ham.push((k.kosu_no, v));
    }

    let mut son = Vec::new();
    let mut sorunlar = Vec::new();
    let mut kullanilan: HashMap<(u32, usize), Vec<usize>> = HashMap::new(); // ham sonuç satırı -> son[] indeksleri
    let mut bekleyen: Vec<(usize, &ProgAt)> = Vec::new();
    for r in prog {
        let adaylar: Vec<&SonHam> = sonuc_ham
            .iter()
            .filter(|(kn, _)| *kn == r.kosu_no)
            .flat_map(|(_, v)| v.iter())
            .filter(|x| x.ad == r.at_adi)
            .collect();
        let mut ss = SonucSatir {
            kayit_id: r.kayit_id.clone(),
            gun_id: gun_id.into(),
            kosu_id: format!("{gun_id}-K{:02}", r.kosu_no),
            tarih: p.gun.tarih.clone(),
            hipodrom: p.gun.hipodrom.clone(),
            kosu_no: r.kosu_no,
            program_at_no: r.at_no,
            at_adi: r.at_adi.clone(),
            liste_sirasi: None,
            bitis: None,
            durum: String::new(),
            derece_ham: String::new(),
            derece_sn: None,
            ganyan: None,
            fark: String::new(),
            eslesme_durumu: String::new(),
            eslesme_yontemi: String::new(),
            capraz: String::new(),
            bahis_dogrulama: String::new(),
            kaynak_dosya: s.ad.clone(),
            kaynak_satir: None,
            sha: s.sha.clone(),
        };
        if adaylar.is_empty() {
            // 1. aşamada tam ad eşleşmesi yok: 2. aşamada (ülke eki kuralı) yeniden denenir.
            ss.durum = "SONUC_KAYDI_BULUNAMADI".into();
            ss.eslesme_durumu = "SONUC_KAYDI_BULUNAMADI".into();
            bekleyen.push((son.len(), r));
            son.push(ss);
            continue;
        }
        if adaylar.len() > 1 {
            sorunlar.push(format!(
                "PROGRAM_KAYDI_BIRDEN_FAZLA_SONUC_SATIRINA: {} {} (aday satırlar: {:?})",
                r.kayit_id,
                r.at_adi,
                adaylar.iter().map(|x| x.satir_no).collect::<Vec<_>>()
            ));
            ss.durum = "COKLU_ESLESME".into();
            ss.eslesme_durumu = "COKLU_ESLESME".into();
            son.push(ss);
            continue;
        }
        let x = adaylar[0];
        kullanilan.entry((r.kosu_no, x.satir_no)).or_default().push(son.len());
        sonucu_bagla(&mut ss, r, x, "TEKIL_ESLESME", ESLESME_YONTEMI)?;
        son.push(ss);
    }

    // 2. aşama — dar kapsamlı ülke eki kuralı. Yalnızca 1. aşamada tam ad eşleşmesi bulamayan program kayıtları ve
    // hiçbir program kaydına bağlanmamış sonuç satırları arasında, aynı koşuda:
    //   * adlar yalnızca BİR taraftaki tanınan son ülke ekiyle farklı (iki tarafta farklı ek → eşleşmez),
    //   * baba ve anne iki kaynakta da dolu ve aynı,
    //   * eşleşme iki yönde de tekil (programın tek adayı, adayın tek programı).
    // Aksi hâlde kayıt incelemeye kalır. Bağlanan kayıt ULKE_EKI_FARKI_ESLESMESI ile işaretlenir.
    let ulke_adaylari = |r: &ProgAt| -> Vec<&SonHam> {
        sonuc_ham
            .iter()
            .filter(|(kn, _)| *kn == r.kosu_no)
            .flat_map(|(_, v)| v.iter())
            .filter(|x| !kullanilan.contains_key(&(r.kosu_no, x.satir_no)))
            .filter(|x| ulke_eki_farki(&r.at_adi, &x.ad))
            .filter(|x| {
                let (pb, pa) = (r.karsilastir[2].trim(), r.karsilastir[3].trim());
                let (sb, sa) = (x.c[3].trim(), x.c[4].trim());
                !pb.is_empty() && !pa.is_empty() && pb == sb && pa == sa
            })
            .collect()
    };
    let mut baglanan = Vec::new();
    for &(i, r) in &bekleyen {
        let adaylar = ulke_adaylari(r);
        let geri_tekil = adaylar.len() == 1 && {
            let x = adaylar[0];
            bekleyen.iter().filter(|(_, r2)| r2.kosu_no == r.kosu_no && ulke_adaylari(r2).iter().any(|y| y.satir_no == x.satir_no)).count() == 1
        };
        if geri_tekil {
            baglanan.push((i, r, adaylar[0]));
        } else {
            let neden = if adaylar.len() > 1 {
                format!(" (ülke eki kuralı: {} aday {:?})", adaylar.len(), adaylar.iter().map(|x| x.satir_no).collect::<Vec<_>>())
            } else if adaylar.len() == 1 {
                format!(" (ülke eki kuralı: satır {} başka program kaydının da adayı)", adaylar[0].satir_no)
            } else {
                String::new()
            };
            sorunlar.push(format!("ESLESMEYEN_PROGRAM_KAYDI: {} {} (aday satırlar: []){neden}", r.kayit_id, r.at_adi));
        }
    }
    for (i, r, x) in baglanan {
        kullanilan.entry((r.kosu_no, x.satir_no)).or_default().push(i);
        let yontem = format!("ULKE_EKI_FARKI_ESLESMESI: program adı '{}', sonuç adı '{}'; baba+anne aynı; iki yönde tekil", r.at_adi, x.ad);
        sonucu_bagla(&mut son[i], r, x, "ULKE_EKI_FARKI_ESLESMESI", &yontem)?;
    }
    let eslesmeyen = sonuc_ham
        .iter()
        .flat_map(|(kn, v)| v.iter().map(move |x| (kn, x)))
        .filter(|(kn, x)| !kullanilan.contains_key(&(**kn, x.satir_no)))
        .map(|(kn, x)| format!("ESLESMEYEN_SONUC_SATIRI: {kn}. koşu, satır {}: {}", x.satir_no, x.c[1]))
        .collect();
    // Aynı ham sonuç satırı birden fazla program kaydına bağlandıysa her biri işaretlenir.
    let mut paylasilan: Vec<_> = kullanilan.iter().filter(|(_, v)| v.len() > 1).collect();
    paylasilan.sort();
    for ((kn, satir), idxler) in paylasilan {
        let idler: Vec<String> = idxler.iter().map(|&i| son[i].kayit_id.clone()).collect();
        for &i in idxler {
            son[i].eslesme_durumu = "SONUC_SATIRI_PAYLASILIYOR".into();
        }
        sorunlar.push(format!(
            "AYNI_SONUC_SATIRI_BIRDEN_FAZLA_PROGRAM_KAYDINA: {kn}. koşu, sonuç satır {satir} -> {}",
            idler.join(", ")
        ));
    }
    Ok((son, sonuc_ham, bahis_kosu, sorunlar, eslesmeyen))
}

fn bahis_kontrol(bahis_kosu: &[KosuBahis], son: &mut [SonucSatir]) -> Vec<BahisKontrol> {
    let mut bk = Vec::new();
    for kb in bahis_kosu {
        let sira_no: HashMap<i64, i64> = son
            .iter()
            .filter(|x| x.kosu_no == kb.kosu_no)
            .filter_map(|x| x.bitis.map(|b| (b, x.program_at_no)))
            .collect();
        for b in &kb.bahisler {
            if let Some((_, n)) = SIRALI.iter().find(|(o, _)| *o == b.oyun) {
                let beklenen: Vec<Option<i64>> = (1..=*n as i64).map(|i| sira_no.get(&i).copied()).collect();
                let gercek: Vec<i64> = b.ayaklar.iter().filter_map(|a| a.first().copied()).collect();
                let uyumlu = gercek.len() == beklenen.len() && gercek.iter().zip(&beklenen).all(|(g, e)| Some(*g) == *e);
                bk.push(BahisKontrol { kosu_no: kb.kosu_no, oyun: b.oyun.clone(), bahis: gercek, eslesmeden: beklenen, uyumlu });
            }
        }
    }
    for x in son.iter_mut() {
        x.bahis_dogrulama = bk
            .iter()
            .filter(|b| b.kosu_no == x.kosu_no && b.bahis.contains(&x.program_at_no))
            .map(|b| format!("{}:{}", b.oyun, if b.uyumlu { "UYUMLU" } else { "UYUMSUZ" }))
            .collect::<Vec<_>>()
            .join(", ");
    }
    bk
}

// ---------------------------------------------------------------- altılı

#[allow(clippy::too_many_arguments)]
fn altililari_kur(
    sec: &Secenekler,
    p: &Kaynak,
    s: &Kaynak,
    gun_id: &str,
    tanimlar: &[Tanim],
    prog: &[ProgAt],
    son: &[SonucSatir],
    bahis_kosu: &[KosuBahis],
    uyarilar: &mut Vec<String>,
    inceleme: &mut Vec<String>,
) -> Vec<Json> {
    if tanimlar.is_empty() {
        uyarilar.push(format!("Programda \"{ALTILI_IBARE}\" ibaresi bulunamadı; altılı oluşturulmadı."));
        inceleme.push("ALTILI_BULUNAMADI: programda altılı başlangıç ibaresi yok".into());
    }
    let mut out = Vec::new();
    for idx in 0..tanimlar.len() {
        match tek_altili(sec, p, s, gun_id, tanimlar, prog, son, bahis_kosu, idx, inceleme) {
            Ok(j) => out.push(j),
            Err(e) => {
                inceleme.push(format!("ALTILI_AYAGI_EKSIK: {e}"));
                uyarilar.push(e);
            }
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn tek_altili(
    sec: &Secenekler,
    p: &Kaynak,
    s: &Kaynak,
    gun_id: &str,
    tanimlar: &[Tanim],
    prog: &[ProgAt],
    son: &[SonucSatir],
    bahis_kosu: &[KosuBahis],
    idx: usize,
    inceleme: &mut Vec<String>,
) -> Result<Json, String> {
    let t = &tanimlar[idx];
    let (oyun, bas, alt_no, ibare, toplam) = (t.oyun.as_str(), t.bas, t.alt_no, t.ibare.as_str(), tanimlar.len());
    let ayak_kosulari: Vec<u32> = t.ayaklar();
    for n in &ayak_kosulari {
        if !p.gun.kosular.iter().any(|k| k.kosu_no == *n) {
            return Err(format!("{} {oyun}: {n}. koşu programda yok; altılı kurulamadı.", t.altili_id));
        }
    }
    let son_kosu = ayak_kosulari[5];
    let son_kb = bahis_kosu.iter().find(|k| k.kosu_no == son_kosu);
    let mut acik: Vec<String> = Vec::new();

    let altili_bahis: Option<&Bahis> = son_kb.and_then(|kb| {
        kb.bahisler.iter().find(|b| b.oyun == oyun).or_else(|| {
            let c: Vec<&Bahis> = kb.bahisler.iter().filter(|b| b.oyun.ends_with("6'LI GANYAN")).collect();
            (c.len() == 1 && toplam == 1).then(|| c[0])
        })
    });
    if let Some(ab) = altili_bahis {
        if ab.oyun != oyun {
            acik.push(format!("Programdaki oyun adı '{oyun}', sonuç satırında '{}' olarak eşleştirildi.", ab.oyun));
        }
        if ab.ayaklar.len() != 6 {
            acik.push(format!("Sonuç kombinasyonu {} parçalı; 6 ardışık ayak varsayımı doğrulanamadı.", ab.ayaklar.len()));
        }
    }

    let mut ayaklar_json = Vec::new();
    let mut durumlar: Vec<KosmazDurum> = Vec::new();
    let mut agf_toplamlari: Vec<Json> = Vec::new();
    let mut agf_degerleri: Vec<(f64, f64)> = Vec::new(); // (toplam, koşmaz payı)
    let mut aciklanamayan: Vec<String> = Vec::new();
    let mut eksik_ayaklar: Vec<String> = Vec::new();
    let mut coklu_agf: Vec<(usize, u32, usize, bool)> = Vec::new(); // (ayak, koşu, değer sırası, AGF1 hep aynı)

    for (i, &kn) in ayak_kosulari.iter().enumerate() {
        let ayak = i + 1;
        let birinciler: Vec<&SonucSatir> = son.iter().filter(|x| x.kosu_no == kn && x.bitis == Some(1)).collect();
        if birinciler.len() > 1 {
            acik.push(format!(
                "{ayak}. ayakta ({kn}. koşu) bitiriş sırası 1 olan {} at var; birinci olarak ilk satır alındı.",
                birinciler.len()
            ));
        }
        let birinci = birinciler.first().copied();
        let mut eksik = |neden: String| {
            let m = format!("{} {ayak}. ayak ({kn}. koşu): {neden}", t.altili_id);
            eksik_ayaklar.push(m.clone());
            inceleme.push(format!("ALTILI_AYAGI_EKSIK: {m}"));
        };
        if !s.gun.kosular.iter().any(|k| k.kosu_no == kn) {
            eksik("sonuç dosyasında koşu yok".into());
        } else if birinci.is_none() {
            eksik("sonuçta birinci at yok".into());
        }
        if altili_bahis.is_some_and(|ab| ab.ayaklar.get(i).is_none()) {
            eksik("sonuç kombinasyonunda bu ayak yok".into());
        }
        let birinci_no = birinci.map(|x| x.program_at_no);
        let gecerli: Option<Vec<i64>> = altili_bahis.and_then(|ab| ab.ayaklar.get(i).cloned());
        let kb = bahis_kosu.iter().find(|k| k.kosu_no == kn);
        let gny = kb.and_then(|k| k.bahisler.iter().find(|b| b.oyun == "GANYAN"));
        let kosu_prog: Vec<&ProgAt> = prog.iter().filter(|r| r.kosu_no == kn).collect();
        let kosmazlar: Vec<&ProgAt> = kosu_prog.iter().copied().filter(|r| r.kosmaz).collect();
        // Bu koşu birden çok altılıdaysa AGF hücresinde altılı başına bir değer bekleniyor.
        let bu_altililar = kosu_altililari(tanimlar, kn);
        let sayi_uyumlu =
            kosu_prog.iter().all(|r| r.agf_degerler.is_empty() || r.agf_degerler.len() == bu_altililar.len());
        let tek_altili = sayi_uyumlu && bu_altililar.len() == 1;
        // Altılıya özel AGF yalnızca eşleşme belirliyse (tek değer + tek altılı). Çok değerli hücrede
        // hangi değerin hangi altılıya ait olduğu doğrulanmadığı için boş bırakılır (sıra varsayımı yok).
        let agf_bu = |r: &ProgAt| -> Option<(f64, i64)> { if tek_altili { r.agf_degerler.first().copied() } else { None } };
        let max_n = kosu_prog.iter().map(|r| r.agf_degerler.len()).max().unwrap_or(0);
        let her_deger_birinci: Vec<Option<i64>> = (0..max_n)
            .map(|v| kosu_prog.iter().find(|r| r.agf_degerler.get(v).map(|x| x.1) == Some(1)).map(|r| r.at_no))
            .collect();
        let agf1_hep_ayni = !her_deger_birinci.is_empty()
            && her_deger_birinci[0].is_some()
            && her_deger_birinci.iter().all(|x| *x == her_deger_birinci[0]);
        // AGF birincisinin kimliği: tek altılıda doğrudan; çok değerlide yalnızca tüm değerlerde aynı atsa.
        let agf1 = if agf1_hep_ayni { kosu_prog.iter().copied().find(|r| Some(r.at_no) == her_deger_birinci[0]) } else { None };
        let agf_eslesme = if !sayi_uyumlu {
            "SAYI_UYUMSUZ"
        } else if bu_altililar.len() <= 1 {
            "TEK_ALTILI_CIKARIM"
        } else {
            "DOGRULANMADI_ALTILIYA_OZEL_BOS"
        };
        let agf1_dayanak = if tek_altili {
            "TEK_ALTILI_CIKARIM"
        } else if agf1.is_some() {
            "TUM_HUCRE_DEGERLERINDE_AYNI_AT"
        } else {
            "BELIRSIZ"
        };
        if bu_altililar.len() > 1 && max_n > 1 {
            coklu_agf.push((ayak, kn, 0, agf1_hep_ayni));
        }
        let diger: Vec<Json> = bu_altililar
            .iter()
            .filter(|&&x| x != idx)
            .map(|&x| {
                obj(vec![
                    ("altili_id", (&tanimlar[x].altili_id).into()),
                    ("ayak", tanimlar[x].ayak_no(kn).into()),
                ])
            })
            .collect();
        let mut ekuri_gruplari: Vec<Vec<i64>> = Vec::new();
        for r in &kosu_prog {
            if !r.ekuri.is_empty() && !ekuri_gruplari.contains(&r.ekuri) {
                ekuri_gruplari.push(r.ekuri.clone());
            }
        }
        let birinci_disi: Option<Vec<i64>> =
            gecerli.as_ref().map(|g| g.iter().copied().filter(|n| Some(*n) != birinci_no).collect());

        let mut ack = String::new();
        if let Some(bd) = birinci_disi.as_ref().filter(|v| !v.is_empty()) {
            let olgular: Vec<String> = bd
                .iter()
                .map(|n| {
                    let ekuri_es = birinci_no.and_then(|b| {
                        kosu_prog.iter().find(|r| r.at_no == *n && r.ekuri.contains(&b)).map(|r| (b, r.ekuri_satir))
                    });
                    if let Some((b, satir)) = ekuri_es {
                        return format!(
                            "{n} numara, birinci {b} numara ile programdaki 'eküridir' ibaresine göre aynı eküri grubunda ({} satır {satir})",
                            p.ad
                        );
                    }
                    let prg = kosu_prog.iter().any(|r| r.at_no == *n && r.kosmaz);
                    let snc = son.iter().any(|x| x.kosu_no == kn && x.program_at_no == *n && x.durum == "KOSMAZ");
                    if !prg && !snc {
                        aciklanamayan.push(format!(
                            "{ayak}. ayakta {n} numara birinci değil ve 'Koşmaz' ibaresi taşımıyor, ancak geçerli listede; dosyada açıklama yok."
                        ));
                    }
                    match (prg, snc) {
                        (true, true) => format!("{n} numara programda ve sonuçta 'Koşmaz'"),
                        (true, false) => format!("{n} numara programda 'Koşmaz'"),
                        (false, true) => format!("{n} numara sonuçta 'Koşmaz'"),
                        _ => format!("{n} numara için dosyada açıklayıcı bir olgu yok"),
                    }
                })
                .collect();
            let onek = if olgular.iter().any(|o| o.contains("eküri")) {
                "Eküri atların birlikte geçerli sayılma kuralı dosyada yazılı değil; ancak 'eküridir' ibaresi var. Dosyadaki olgular:"
            } else {
                "Dosyada geçerli sayılmanın kuralını açıklayan bir ibare YOK. Dosyadaki olgular:"
            };
            ack = format!("{onek} {}.", olgular.join("; "));
        }

        for r in &kosmazlar {
            let agf1_birinci = match (agf1, birinci_no) {
                (Some(a), Some(b)) => Some(a.at_no == b),
                _ => None,
            };
            durumlar.push(KosmazDurum {
                ayak,
                kosu_no: kn,
                no: r.at_no,
                ad: r.at_adi.clone(),
                gecerli: gecerli.as_ref().map(|g| g.contains(&r.at_no)),
                agf1_no: agf1.map(|a| a.at_no),
                agf1_ad: agf1.map(|a| a.at_adi.clone()),
                agf1_birinci,
                h2_tahmin: match agf1 {
                    Some(a) if a.at_no != r.at_no => agf1_birinci,
                    _ => None,
                },
            });
        }

        if max_n > 0 && sayi_uyumlu {
            // Hücredeki her değer konumu için ayrı toplam (hangi altılıya ait olduğundan bağımsız).
            let mut hucre_t = Vec::new();
            let mut hucre_kp = Vec::new();
            for v in 0..max_n {
                let t = yuvarla2(kosu_prog.iter().filter_map(|r| r.agf_degerler.get(v).map(|x| x.0)).sum());
                let kp = yuvarla2(kosmazlar.iter().filter_map(|r| r.agf_degerler.get(v).map(|x| x.0)).sum());
                agf_degerleri.push((t, kp));
                hucre_t.push(t);
                hucre_kp.push(kp);
            }
            let ozel = kosu_prog.iter().any(|r| agf_bu(r).is_some());
            agf_toplamlari.push(obj(vec![
                ("ayak", ayak.into()),
                ("kosu_no", kn.into()),
                ("agf_toplam_yuzde", ozel.then(|| hucre_t[0]).into()),
                ("kosmaz_agf_payi_yuzde", ozel.then(|| hucre_kp[0]).into()),
                ("hucre_degeri_toplamlari", hucre_t.into()),
                ("hucre_degeri_kosmaz_paylari", hucre_kp.into()),
                ("altili_eslesmesi", agf_eslesme.into()),
            ]));
        }

        ayaklar_json.push(obj(vec![
            ("ayak", ayak.into()),
            ("kosu_no", kn.into()),
            ("kosu_id", format!("{gun_id}-K{kn:02}").into()),
            ("diger_altililardaki_ayaklar", Json::Arr(diger)),
            ("ekuri_gruplari", ekuri_gruplari.into()),
            (
                "birinci",
                birinci.map_or(Json::Null, |x| {
                    obj(vec![
                        ("program_at_no", x.program_at_no.into()),
                        ("at_adi", (&x.at_adi).into()),
                        ("kayit_id", (&x.kayit_id).into()),
                        (
                            "kaynak",
                            format!("{} satır {}", s.ad, x.kaynak_satir.map(|v| v.to_string()).unwrap_or_default()).into(),
                        ),
                    ])
                }),
            ),
            ("gecerli_numaralar", gecerli.clone().into()),
            ("birinci_disi_gecerli_numaralar", birinci_disi.clone().into()),
            ("birinci_disi_aciklama", ack.into()),
            (
                "kosmaz_atlar",
                Json::Arr(
                    kosmazlar
                        .iter()
                        .map(|r| {
                            obj(vec![
                                ("program_at_no", r.at_no.into()),
                                ("at_adi", (&r.at_adi).into()),
                                ("agf_ham_programda", (&r.agf_ham).into()),
                                ("altili_gecerli_listede", gecerli.as_ref().map(|g| g.contains(&r.at_no)).into()),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "agf_birincisi_programda",
                agf1.map_or(Json::Null, |a| {
                    obj(vec![
                        ("program_at_no", a.at_no.into()),
                        ("at_adi", (&a.at_adi).into()),
                        ("agf_ham", (&a.agf_ham).into()),
                        ("agf_yuzde_bu_altili", agf_bu(a).map(|x| x.0).into()),
                        ("altili_eslesmesi", agf_eslesme.into()),
                        ("kimlik_dayanagi", agf1_dayanak.into()),
                        ("tum_degerlerde_ayni_at", agf1_hep_ayni.into()),
                        ("zaman_dogrulandi", false.into()),
                    ])
                }),
            ),
            (
                "ganyan_bahis_sonucu",
                match (gny, kb) {
                    (Some(g), Some(k)) => obj(vec![
                        ("ham", format!("GANYAN({}) :{}", g.kombinasyon_ham, g.tutar_ham).into()),
                        ("kaynak", format!("{} satır {}", s.ad, k.satir_no).into()),
                        (
                            "birinci_ile_uyumlu",
                            birinci_no.map(|b| g.ayaklar.first() == Some(&vec![b])).into(),
                        ),
                    ]),
                    _ => Json::Null,
                },
            ),
        ]));
    }

    // ---- koşmaz hipotez testi (yalnızca dosyadaki gözlemlerle)
    let h1: Option<bool> = if durumlar.is_empty() || durumlar.iter().any(|d| d.gecerli.is_none()) {
        None
    } else {
        Some(durumlar.iter().all(|d| d.gecerli == Some(true)))
    };
    let h2_tanimli: Vec<&KosmazDurum> =
        durumlar.iter().filter(|d| d.h2_tahmin.is_some() && d.gecerli.is_some()).collect();
    let h2: Option<bool> =
        if h2_tanimli.is_empty() { None } else { Some(h2_tanimli.iter().all(|d| d.h2_tahmin == d.gecerli)) };

    // ---- açık sorular
    let dahil: Vec<&KosmazDurum> = durumlar.iter().filter(|d| d.gecerli == Some(true)).collect();
    let haric: Vec<&KosmazDurum> = durumlar.iter().filter(|d| d.gecerli == Some(false)).collect();
    let mut sorular: Vec<String> = Vec::new();
    if !dahil.is_empty() {
        let mut t = dahil
            .iter()
            .map(|d| {
                format!(
                    "{}. ayakta {} numaranın ({}, 'Koşmaz') neden geçerli sayıldığını açıklayan bir ibare dosyalarda yok.",
                    d.ayak, d.no, d.ad
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        if !haric.is_empty() {
            let liste =
                haric.iter().map(|d| format!("{}. ayakta {} numara ({})", d.ayak, d.no, d.ad)).collect::<Vec<_>>().join(", ");
            t.push_str(&format!(
                " Aynı altılıda şu koşmaz atlar ise geçerli listede değil: {liste}; dolayısıyla 'koşmayan at birinciyle eşdeğer sayılır' açıklaması bu verilerle tutarlı DEĞİL."
            ));
        }
        sorular.push(t);
    }
    sorular.extend(aciklanamayan);
    if !durumlar.is_empty() {
        sorular.push(match (h1, h2) {
            (Some(true), _) => "Bu altılıdaki tüm koşmaz atlar geçerli listede yer alıyor; bunu açıklayan kural dosyada \
yazmıyor, TJK Müşterek Bahisler Yönetmeliği'nden teyit edilmeli."
                .into(),
            (_, Some(true)) => {
                let detay = h2_tanimli
                    .iter()
                    .map(|d| {
                        format!(
                            "{}. ayak: AGF birincisi {} {} {}, koşmaz {} {}",
                            d.ayak,
                            d.agf1_no.unwrap_or_default(),
                            d.agf1_ad.clone().unwrap_or_default(),
                            if d.agf1_birinci == Some(true) { "birinci geldi" } else { "kaybetti" },
                            d.no,
                            if d.gecerli == Some(true) { "geçerli" } else { "geçerli değil" }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                format!(
                    "Doğrulanmamış hipotez: koşmaz atın bahsinin o ayağın AGF birincisine aktarıldığı bir kural, bu altılıdaki \
{} koşmaz durumunun hepsiyle tutarlı ({detay}). Kural dosyada yazmıyor ve arşivdeki AGF değerlerinin zamanı \
doğrulanamadı; TJK Müşterek Bahisler Yönetmeliği'nden teyit edilmeli.",
                    h2_tanimli.len()
                )
            }
            (Some(false), Some(false)) => "Koşmaz durumları ne 'koşmayan at birinci sayılır' ne de 'AGF birincisine \
aktarılır' açıklamasıyla tutarlı; TJK kurallarından teyit edilmeli."
                .into(),
            _ => "Koşmaz atların altılıdaki durumu bu dosyalarla sınanamadı (eksik veri); TJK kurallarından teyit edilmeli."
                .into(),
        });
    }
    if agf_degerleri.iter().any(|(_, kp)| *kp > 0.0) {
        let mn = agf_degerleri.iter().map(|x| x.0).fold(f64::INFINITY, f64::min);
        let mx = agf_degerleri.iter().map(|x| x.0).fold(f64::NEG_INFINITY, f64::max);
        sorular.push(format!(
            "Programdaki AGF yüzdeleri koşmaz atlar dahil toplandığında ayak başına %{}–%{} ediyor; yani arşivdeki AGF \
anlık görüntüsü koşmaz atların payını hâlâ içeriyor. Görüntünün alındığı an bilinmiyor.",
            tr2(mn),
            tr2(mx)
        ));
    }
    if !coklu_agf.is_empty() {
        let liste = coklu_agf.iter().map(|(a, k, _, _)| format!("{a}. ayak/{k}. koşu")).collect::<Vec<_>>().join(", ");
        let ayni = coklu_agf.iter().filter(|x| x.3).count();
        let etki = if ayni == coklu_agf.len() {
            "Bu koşuların hepsinde AGF birincisi tüm hücre değerlerinde aynı at olduğu için AGF birincisinin kimliği (ve koşmaz testi) eşleşmeden bağımsız belirlenebildi."
                .to_string()
        } else {
            format!(
                "AGF birincisi {} koşunun yalnızca {ayni} tanesinde tüm değerlerde aynı at; diğerlerinde AGF birincisi BELIRSIZ bırakıldı.",
                coklu_agf.len()
            )
        };
        sorular.push(format!(
            "Başka bir altılıyla ortak koşularda ({liste}) AGF hücresinde altılı başına birer değer var; hangi değerin \
hangi altılıya ait olduğu dosyada yazmıyor ve resmî 1./2. AGF tabloları bu çalıştırmaya verilmedi; altılıya özel AGF yüzdesi BOŞ bırakıldı, ham hücre ve iki değer korunuyor. {etki}"
        ));
    }
    sorular.push(match altili_bahis {
        Some(b) if b.sonuc_durumu == "DEVIR" => format!(
            "Bu altılıda bilen çıkmamış; kaynakta devir: {}. Kazanana ödeme alanı boş bırakıldı.",
            if b.devir_ham.is_empty() { "tutar yazılmamış" } else { b.devir_ham.as_str() }
        ),
        Some(_) => "Ödemenin hangi birim bahis tutarı için olduğu CSV'de yazmıyor.".into(),
        None => format!(
            "Son ayak ({son_kosu}. koşu) bahis satırında '{oyun}' ödemesi bulunamadı; nedeni dosyada yazmıyor."
        ),
    });
    sorular.extend(acik);

    // Altılının kendisi belirsizse gün incelemeye düşer: son ayak satırında bu altılıya ait okunamayan bir parça,
    // ya da kombinasyonun 6 parçalı olmaması. (Diğer bahislerin okunamayan parçaları yalnızca uyarıdır.)
    let altili_govde = oyun.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ');
    if let Some(kb) = son_kb {
        for h in kb.okunamayan.iter().filter(|h| h.contains(altili_govde)) {
            inceleme.push(format!(
                "ALTILI_SONUCU_OKUNAMADI: {} {}. koşu bahis satırı ({} satır {}): {h}",
                t.altili_id, son_kosu, s.ad, kb.satir_no
            ));
        }
    }
    if let Some(ab) = altili_bahis.filter(|ab| ab.ayaklar.len() != 6) {
        inceleme.push(format!("ALTILI_KOMBINASYONU_6_PARCA_DEGIL: {} '{}' ({} parça)", t.altili_id, ab.kombinasyon_ham, ab.ayaklar.len()));
    }
    if altili_bahis.is_none() {
        inceleme.push(format!("ALTILI_ODEMESI_BULUNAMADI: {} son ayak ({son_kosu}. koşu) bahis satırında '{oyun}' yok", t.altili_id));
    }

    // ---- kullanıcı kontrolü
    let kk = sec.kontrol_komb.get(idx);
    let ko = sec.kontrol_odeme.get(idx);
    let kullanici = if kk.is_none() && ko.is_none() {
        Json::Null
    } else {
        let komb_esit = match (kk, altili_bahis) {
            (Some(k), Some(ab)) => komb_coz(k).map(|v| v == ab.ayaklar),
            _ => None,
        };
        let odeme_esit = match (ko, altili_bahis.and_then(|ab| ab.tutar_tl)) {
            (Some(o), Some(t)) => tr_num(o.trim().trim_end_matches("TL")).map(|v| v == t),
            _ => None,
        };
        obj(vec![
            ("kombinasyon", kk.cloned().into()),
            ("odeme", ko.cloned().into()),
            ("kombinasyon_esit", komb_esit.into()),
            ("odeme_esit", odeme_esit.into()),
            (
                "not",
                if altili_bahis.is_some() {
                    "Kontrol değerleri kullanıcı tarafından verildi; kombinasyon ve ödeme sonuç CSV'sinde zaten mevcut, ek kaynak kullanılmadı."
                } else {
                    "Kontrol değerleri kullanıcı tarafından verildi; sonuç CSV'sinde bu altılının ödemesi bulunamadı, karşılaştırma yapılamadı."
                }
                .into(),
            ),
        ])
    };

    let notu = match altili_bahis {
        Some(ab) if ab.ayaklar.len() == 6 => format!(
            " Sonuç kombinasyonunun 6 parçalı olması ve ödemenin son ayak ({son_kosu}. koşu) satırında yer alması bununla tutarlı."
        ),
        Some(ab) => format!(" UYARI: sonuç kombinasyonu {} parçalı; varsayım doğrulanamadı.", ab.ayaklar.len()),
        None => format!(" Ödeme son ayak ({son_kosu}. koşu) satırında bulunamadığı için varsayım sonuçla doğrulanamadı."),
    };

    let kaynak_notu = |k: &Kaynak| {
        let mut n = "TJK indirme URL'si dosyada yok.".to_string();
        if let Some(d) = dosya_adi_notu(&k.ad, &p.gun.hipodrom) {
            n.push(' ');
            n.push_str(&d);
        }
        n
    };
    let mut kaynaklar = vec![
        obj(vec![
            ("rol", "program".into()),
            ("dosya", (&p.ad).into()),
            ("sha256", (&p.sha).into()),
            ("kodlama", (&p.kodlama).into()),
            ("kaynak_url", Json::Null),
            ("not", kaynak_notu(p).into()),
        ]),
        obj(vec![
            ("rol", "sonuc".into()),
            ("dosya", (&s.ad).into()),
            ("sha256", (&s.sha).into()),
            ("kodlama", (&s.kodlama).into()),
            ("kaynak_url", Json::Null),
            ("not", kaynak_notu(s).into()),
        ]),
    ];
    if let Some(u) = &sec.ilgili_url {
        kaynaklar.push(obj(vec![
            ("rol", "ilgili_kaynak_bu_turda_acilmadi".into()),
            ("url", u.into()),
            ("not", sec.ilgili_not.clone().unwrap_or_else(|| "Bu ayrıştırmada kullanılmadı, açılmadı.".into()).into()),
        ]));
    }

    let durum_json: Vec<Json> = durumlar
        .iter()
        .map(|d| {
            obj(vec![
                ("ayak", d.ayak.into()),
                ("kosu_no", d.kosu_no.into()),
                ("kosmaz_at_no", d.no.into()),
                ("kosmaz_at_adi", (&d.ad).into()),
                ("altili_gecerli_listede", d.gecerli.into()),
                ("agf_birincisi_no", d.agf1_no.into()),
                ("agf_birincisi_adi", d.agf1_ad.clone().into()),
                ("agf_birincisi_birinci_geldi", d.agf1_birinci.into()),
            ])
        })
        .collect();

    Ok(obj(vec![
        ("altili_id", (&t.altili_id).into()),
        ("eksik_ayaklar", eksik_ayaklar.clone().into()),
        ("oyun_adi", oyun.into()),
        ("gun_id", gun_id.into()),
        ("tarih", (&p.gun.tarih).into()),
        ("hipodrom", (&p.gun.hipodrom).into()),
        ("yaris_gunu_no", p.gun.yaris_gunu_no.into()),
        ("baslangic_kosusu", bas.into()),
        ("baslangic_kaniti", obj(vec![("dosya", (&p.ad).into()), ("satir", alt_no.into()), ("ibare", ibare.into())])),
        ("ayak_kosulari", ayak_kosulari.clone().into()),
        (
            "ayak_kosulari_notu",
            format!("Program yalnızca başlangıç koşusunu belirtiyor; diğer 5 ayak ardışık koşular olarak alındı.{notu}").into(),
        ),
        ("ayaklar", Json::Arr(ayaklar_json)),
        ("kombinasyon_ham", altili_bahis.map(|b| b.kombinasyon_ham.clone()).into()),
        ("kombinasyon_listesi", altili_bahis.map(|b| b.ayaklar.clone()).into()),
        ("odeme_ham", altili_bahis.and_then(|b| (!b.tutar_ham.is_empty()).then(|| b.tutar_ham.clone())).into()),
        ("odeme_tl", altili_bahis.and_then(|b| b.tutar_tl).into()),
        ("sonuc_durumu", altili_bahis.map(|b| b.sonuc_durumu).into()),
        ("devir_ham", altili_bahis.and_then(|b| (!b.devir_ham.is_empty()).then(|| b.devir_ham.clone())).into()),
        ("devir_tl", altili_bahis.and_then(|b| b.devir_tl).into()),
        (
            "odeme_kaniti",
            match (altili_bahis, son_kb) {
                (Some(b), Some(k)) => obj(vec![
                    ("dosya", (&s.ad).into()),
                    ("satir", k.satir_no.into()),
                    ("ibare", (&b.ham_ibare).into()),
                ]),
                _ => Json::Null,
            },
        ),
        ("kullanici_kontrolu", kullanici),
        (
            "dosyada_olmayan_alanlar",
            obj(vec![
                ("odeme_birim_bahis_tutari", "CSV ödemenin hangi birim bahis tutarı için olduğunu belirtmiyor.".into()),
                ("bilen_kupon_sayisi", "CSV'de yok.".into()),
                ("dagitilacak_tutar_havuz", "CSV'de yok.".into()),
                ("devir", "CSV'de yok.".into()),
                (
                    "agf_zaman_damgasi",
                    "CSV'de yok; AGF değerlerinin altılı başlamadan önceki hâli doğrulanamadı.".into(),
                ),
                (
                    "tjk_kosu_id_ve_at_id",
                    "CSV'de yok; kimlikler tarih+hipodrom+koşu+program no ile üretildi.".into(),
                ),
            ]),
        ),
        (
            "kosmaz_kural_testi",
            obj(vec![
                ("durumlar", Json::Arr(durum_json)),
                ("h1_kosmaz_birinci_sayilir_tutarli", h1.into()),
                ("h2_kosmaz_agf_birincisine_aktarilir_tutarli", h2.into()),
                (
                    "not",
                    "Hipotezler yalnızca bu dosyadaki gözlemlerle sınandı; TJK kuralıyla doğrulanmadı ve AGF değerlerinin zamanı bilinmiyor."
                        .into(),
                ),
            ]),
        ),
        ("agf_ayak_toplamlari", Json::Arr(agf_toplamlari)),
        ("acik_sorular", sorular.into()),
        ("kaynaklar", Json::Arr(kaynaklar)),
    ]))
}

// ---------------------------------------------------------------- rapor (stdout)

#[allow(clippy::too_many_arguments)]
fn rapor(
    p: &Kaynak,
    s: &Kaynak,
    tanimlar: &[Tanim],
    prog: &[ProgAt],
    son: &[SonucSatir],
    sonuc_ham: &[(u32, Vec<SonHam>)],
    sorunlar: &[String],
    eslesmeyen: &[String],
    bk: &[BahisKontrol],
    altililar: &[Json],
    uyarilar: &[String],
) {
    let kb = |g: &Gun| g.kosular.iter().map(|k| format!("{}:{}", k.kosu_no, k.atlar.len())).collect::<Vec<_>>().join(" ");
    println!("== KONTROL ÖZETİ");
    println!("Program : {} | {} | {}", p.ad, p.kodlama, p.gun.ilk_satir);
    println!("          koşu {} | at satırı {} | koşu:at {}", p.gun.kosular.len(), prog.len(), kb(&p.gun));
    println!("Sonuç   : {} | {} | {}", s.ad, s.kodlama, s.gun.ilk_satir);
    println!(
        "          koşu {} | at satırı {} | koşu:at {}",
        s.gun.kosular.len(),
        sonuc_ham.iter().map(|(_, v)| v.len()).sum::<usize>(),
        kb(&s.gun)
    );
    println!("Eşleşme sorunları      : {}", if sorunlar.is_empty() { "yok".into() } else { sorunlar.join(" | ") });
    println!("Eşleşmeyen sonuç satırı: {}", if eslesmeyen.is_empty() { "yok".into() } else { eslesmeyen.join(" | ") });
    let farkli: Vec<String> =
        son.iter().filter(|x| !x.capraz.is_empty()).map(|x| format!("{} {}", x.kayit_id, x.capraz)).collect();
    println!("Çapraz alan farkı      : {}", if farkli.is_empty() { "yok".into() } else { farkli.join(" | ") });
    let mut sayim: Vec<(String, usize)> = Vec::new();
    for x in son {
        match sayim.iter_mut().find(|(d, _)| *d == x.durum) {
            Some(e) => e.1 += 1,
            None => sayim.push((x.durum.clone(), 1)),
        }
    }
    sayim.sort();
    println!("Koşu durumları         : {}", sayim.iter().map(|(d, n)| format!("{d}={n}")).collect::<Vec<_>>().join(", "));
    for d in ["KOSMAZ", "DERECESIZ"] {
        let l: Vec<String> = son.iter().filter(|x| x.durum == d).map(|x| format!("{} {}", x.kayit_id, x.at_adi)).collect();
        if !l.is_empty() {
            println!("  {d}: {}", l.join(", "));
        }
    }
    let uyumsuz: Vec<String> = bk
        .iter()
        .filter(|b| !b.uyumlu)
        .map(|b| format!("{}. koşu {} bahis={:?} eşleşme={:?}", b.kosu_no, b.oyun, b.bahis, b.eslesmeden))
        .collect();
    println!(
        "Bahis satırı kontrolü  : {} kontrol, {} uyumsuz{}",
        bk.len(),
        uyumsuz.len(),
        if uyumsuz.is_empty() { String::new() } else { format!(" -> {}", uyumsuz.join(" | ")) }
    );
    let mut derece_bozuk = Vec::new();
    for (kn, _) in sonuc_ham {
        let mut b: Vec<&SonucSatir> = son.iter().filter(|x| x.kosu_no == *kn && x.bitis.is_some()).collect();
        b.sort_by_key(|x| x.bitis);
        if !b.windows(2).all(|w| w[0].derece_sn <= w[1].derece_sn) {
            derece_bozuk.push(kn.to_string());
        }
    }
    println!(
        "Derece sırası          : {}",
        if derece_bozuk.is_empty() { "tüm koşularda artan".into() } else { format!("bozuk: {}", derece_bozuk.join(",")) }
    );
    let coklu = prog.iter().filter(|r| r.agf_degerler.len() > 1).count();
    println!("AGF hücreleri          : {} tek değer, {coklu} çok değerli, {} boş/çözülemeyen",
        prog.iter().filter(|r| r.agf_degerler.len() == 1).count(),
        prog.iter().filter(|r| r.agf_degerler.is_empty()).count());
    let mut ekr: Vec<String> = Vec::new();
    for r in prog.iter().filter(|r| !r.ekuri.is_empty()) {
        let e = format!("{}. koşu [{}]", r.kosu_no, r.ekuri.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));
        if !ekr.contains(&e) { ekr.push(e); }
    }
    println!("Eküri grupları         : {}", if ekr.is_empty() { "yok".into() } else { ekr.join(", ") });
    for t in tanimlar {
        let ortak: Vec<String> = t.ayaklar().into_iter().filter(|k| kosu_altililari(tanimlar, *k).len() > 1).map(|k| k.to_string()).collect();
        println!("Tanım {}: {} | ayak koşuları {:?} | başka altılıyla ortak koşular: {}", t.altili_id, t.oyun, t.ayaklar(),
            if ortak.is_empty() { "yok".into() } else { ortak.join(",") });
    }
    for a in altililar {
        if let Json::Obj(v) = a {
            let al = |k: &str| v.iter().find(|(x, _)| x == k).map(|(_, j)| j.hucre()).unwrap_or_default();
            println!(
                "Altılı {}: başlangıç {}. koşu | kombinasyon {} | ödeme {}",
                al("altili_id"),
                al("baslangic_kosusu"),
                al("kombinasyon_ham"),
                al("odeme_ham")
            );
        }
    }
    for u in uyarilar {
        println!("UYARI: {u}");
    }
}

// ---------------------------------------------------------------- GANYAN ↔ birinci kontrolü

/// Her koşuda GANYAN satırındaki numara(lar) ile sonuçta birinci görünen at(lar)ı karşılaştırır.
/// Açıklanabilen durumlar: tam uyum, at başı (birden çok birinci), aynı eküri grubu.
/// Kaynaktaki GANYAN numaraları değiştirilmeden saklanır.
fn ganyan_kontrolu(p: &Kaynak, prog: &[ProgAt], son: &[SonucSatir], bahis_kosu: &[KosuBahis]) -> (Vec<Json>, Vec<String>) {
    let mut out = Vec::new();
    let mut inceleme = Vec::new();
    for k in &p.gun.kosular {
        let kn = k.kosu_no;
        let mut birinciler: Vec<i64> =
            son.iter().filter(|x| x.kosu_no == kn && x.bitis == Some(1)).map(|x| x.program_at_no).collect();
        birinciler.sort();
        let kb = bahis_kosu.iter().find(|b| b.kosu_no == kn);
        let mut ganyan: Vec<i64> = kb
            .map(|b| {
                b.bahisler
                    .iter()
                    .filter(|x| x.oyun == "GANYAN")
                    .flat_map(|x| x.ayaklar.iter().flatten().copied())
                    .collect()
            })
            .unwrap_or_default();
        ganyan.sort();
        ganyan.dedup();
        let ekuri = |a: i64, b: i64| prog.iter().any(|r| r.kosu_no == kn && r.ekuri.contains(&a) && r.ekuri.contains(&b));
        let durum = if birinciler.is_empty() {
            "BIRINCI_YOK"
        } else if ganyan.is_empty() {
            "GANYAN_SATIRI_YOK"
        } else if ganyan.iter().all(|g| birinciler.contains(g)) {
            if birinciler.len() > 1 { "AT_BASI_UYUMLU" } else { "UYUMLU" }
        } else if ganyan.iter().all(|g| birinciler.iter().any(|w| w == g || ekuri(*g, *w))) {
            "EKURI_ILE_ACIKLANDI"
        } else {
            "CELISKI"
        };
        match durum {
            "CELISKI" => inceleme.push(format!(
                "GANYAN_BIRINCI_CELISKISI: {kn}. koşu GANYAN {ganyan:?}, sonuçta birinci {birinciler:?}"
            )),
            "GANYAN_SATIRI_YOK" => inceleme.push(format!("GANYAN_SATIRI_BULUNAMADI: {kn}. koşu (birinci {birinciler:?})")),
            "BIRINCI_YOK" => inceleme.push(format!("BIRINCI_BULUNAMADI: {kn}. koşu (GANYAN {ganyan:?})")),
            _ => {}
        }
        out.push(obj(vec![
            ("kosu_no", kn.into()),
            ("ganyan_numaralari", ganyan.clone().into()),
            ("birinci_program_numaralari", birinciler.clone().into()),
            ("durum", durum.into()),
            ("sonuc_bahis_satiri", kb.map(|b| b.satir_no).into()),
        ]));
    }
    (out, inceleme)
}

/// SQLite aktarımına verilen bir günün tüm ayrıştırılmış verisi.
pub struct GunVerisi<'a> {
    p: &'a Kaynak,
    s: &'a Kaynak,
    gun_id: &'a str,
    gun_durumu: &'a str,
    inceleme: &'a [String],
    tanimlar: &'a [Tanim],
    prog: &'a [ProgAt],
    son: &'a [SonucSatir],
    bahis_kosu: &'a [KosuBahis],
    ganyan: &'a [Json],
    altililar: &'a [Json],
}

/// `goc --kaynak ESKI.sqlite --hedef YENI.sqlite`: kaynağı hiç değiştirmeden kopyalar ve kopyayı güncel şemaya geçirir.
/// Tablo sayıları önce/sonra karşılaştırılır; herhangi bir tabloda satır kaybı varsa hata verir.
fn goc_komutu(args: &[String]) -> Result<(), String> {
    let mut it = args.iter();
    let (mut kaynak, mut hedef) = (None, None);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--kaynak" => kaynak = it.next().cloned(),
            "--hedef" => hedef = it.next().cloned(),
            o => return Err(format!("bilinmeyen seçenek: {o}\nKullanım: tjk_ayristirici goc --kaynak ESKI.sqlite --hedef YENI.sqlite")),
        }
    }
    let (Some(kaynak), Some(hedef)) = (kaynak, hedef) else {
        return Err("Kullanım: tjk_ayristirici goc --kaynak ESKI.sqlite --hedef YENI.sqlite".into());
    };
    let kk = std::fs::canonicalize(&kaynak).map_err(|e| format!("kaynak bulunamadı ({kaynak}): {e}"))?;
    if std::fs::canonicalize(&hedef).ok().as_ref() == Some(&kk) {
        return Err("Hedef kaynağın kendisi olamaz; göç yalnızca kopyada yapılır.".into());
    }
    let sha_once = sha256::hex(&std::fs::read(&kaynak).map_err(|e| e.to_string())?);
    let say = |yol: &str| -> Result<(i64, Vec<(String, i64)>), String> {
        let db = sqlite::Db::ac(yol, true)?;
        let v = db.tek_int("PRAGMA user_version", &[])?;
        let (_, t) = db.sorgu("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name", &[])?;
        let mut out = Vec::new();
        for r in t {
            let ad = r[0].metin();
            out.push((ad.clone(), db.tek_int(&format!("SELECT COUNT(*) FROM \"{ad}\""), &[])?));
        }
        Ok((v, out))
    };
    let (v_once, once) = say(&kaynak)?;
    if Path::new(&hedef).exists() {
        println!("Hedef zaten var, üzerine kopyalanmadı: {hedef}");
    } else {
        std::fs::copy(&kaynak, &hedef).map_err(|e| format!("kopyalanamadı: {e}"))?;
        println!("Kopyalandı: {kaynak} → {hedef}");
    }
    let (eski, yeni) = veritabani::goc(&hedef)?;
    let (_, sonra) = say(&hedef)?;
    println!("Şema sürümü: kaynak {v_once} | hedef göç öncesi {eski} → sonra {yeni}");
    println!("{:<22} {:>8} {:>8}", "tablo", "kaynak", "hedef");
    let mut kayip = Vec::new();
    for (ad, n) in &once {
        let m = sonra.iter().find(|(a, _)| a == ad).map(|x| x.1);
        println!("{ad:<22} {n:>8} {:>8}", m.map(|x| x.to_string()).unwrap_or("YOK".into()));
        if m != Some(*n) {
            kayip.push(ad.clone());
        }
    }
    for (ad, m) in sonra.iter().filter(|(a, _)| !once.iter().any(|(b, _)| b == a)) {
        println!("{ad:<22} {:>8} {m:>8}  (yeni tablo)", "-");
    }
    let db = sqlite::Db::ac(&hedef, true)?;
    let (_, ic) = db.sorgu("PRAGMA integrity_check", &[])?;
    let (_, fk) = db.sorgu("PRAGMA foreign_key_check", &[])?;
    println!("Hedef integrity_check: {} | foreign_key_check: {} satır", ic[0][0].metin(), fk.len());
    let sha_sonra = sha256::hex(&std::fs::read(&kaynak).map_err(|e| e.to_string())?);
    println!("Kaynak SHA-256 önce/sonra: {}… / {}… ({})", &sha_once[..16], &sha_sonra[..16], if sha_once == sha_sonra { "DEĞİŞMEDİ" } else { "DEĞİŞTİ!" });
    if !kayip.is_empty() || sha_once != sha_sonra || ic[0][0].metin() != "ok" || !fk.is_empty() {
        return Err(format!("Göç doğrulaması başarısız: satır sayısı farklı tablolar {kayip:?}"));
    }
    println!("Göç doğrulandı: hiçbir tabloda satır kaybı yok.");
    Ok(())
}

/// Bir program kaydını tek bir ham sonuç satırına bağlar (bitiş, derece, ganyan, fark, çapraz alan farkları).
fn sonucu_bagla(ss: &mut SonucSatir, r: &ProgAt, x: &SonHam, eslesme_durumu: &str, yontem: &str) -> Result<(), String> {
    let c = &x.c;
    let farklar: Vec<String> = KARSILASTIR
        .iter()
        .zip(&r.karsilastir)
        .filter(|((idx, _), pv)| c[*idx].trim() != pv.trim())
        .map(|((idx, alan), pv)| format!("{alan}: program='{pv}' sonuc='{}'", c[*idx]))
        .collect();
    let der = c[12].trim();
    let durum = match der {
        "Koşmaz" => "KOSMAZ".to_string(),
        "Derecesiz" => "DERECESIZ".to_string(),
        d if sure_sn(d).is_some() => "BITIRDI".to_string(),
        d => format!("BILINMEYEN:{d}"),
    };
    let sira: i64 = c[0].parse().map_err(|_| format!("Sonuç satır {}: sıra okunamadı", x.satir_no))?;
    ss.at_adi = x.ad.clone();
    ss.liste_sirasi = Some(sira);
    ss.bitis = (durum == "BITIRDI").then_some(sira);
    ss.durum = durum;
    ss.derece_ham = der.into();
    ss.derece_sn = sure_sn(der);
    ss.ganyan = tr_num(&c[13]);
    ss.fark = c[14].trim().into();
    ss.eslesme_durumu = eslesme_durumu.into();
    ss.eslesme_yontemi = yontem.into();
    ss.capraz = farklar.join(" ; ");
    ss.kaynak_satir = Some(x.satir_no);
    Ok(())
}

/// Tanınan ülke ekleri (at adının sonunda parantez içinde).
const ULKE_EKLERI: [&str; 32] = [
    "USA", "GB", "IRE", "FR", "GER", "JPN", "AUS", "CAN", "RU", "ITY", "NZ", "ARG", "BRZ", "SAF", "CHI", "UAE", "SPA", "POL",
    "HUN", "CZE", "SWE", "NOR", "DEN", "BEL", "HOL", "URU", "PER", "KSA", "QA", "SWI", "AUT", "IND",
];

/// Addaki tanınan son ülke ekini ayırır: "CURLIN DRACARYS (USA)" → ("CURLIN DRACARYS", Some("USA")).
fn ulke_eki_ayir(ad: &str) -> (&str, Option<&str>) {
    let ad = ad.trim();
    if let Some(govde) = ad.strip_suffix(')') {
        if let Some(acik) = govde.rfind(" (") {
            let ek = &govde[acik + 2..];
            if ULKE_EKLERI.contains(&ek) {
                return (govde[..acik].trim_end(), Some(ek));
            }
        }
    }
    (ad, None)
}

/// İki ad yalnızca BİR taraftaki tanınan son ülke ekiyle mi farklı? İki tarafta ek varsa (aynı ya da farklı) hayır.
fn ulke_eki_farki(program_adi: &str, sonuc_adi: &str) -> bool {
    match (ulke_eki_ayir(program_adi), ulke_eki_ayir(sonuc_adi)) {
        ((pg, Some(_)), (sg, None)) | ((pg, None), (sg, Some(_))) => !pg.is_empty() && pg == sg,
        _ => false,
    }
}

#[cfg(test)]
mod ulke_eki_testleri {
    use super::*;
    #[test]
    fn ulke_eki_farki_kurallari() {
        assert!(ulke_eki_farki("CURLIN DRACARYS (USA)", "CURLIN DRACARYS"));
        assert!(ulke_eki_farki("CURLIN DRACARYS", "CURLIN DRACARYS (USA)"));
        assert!(!ulke_eki_farki("CURLIN DRACARYS (USA)", "CURLIN DRACARYS (GB)"), "iki tarafta farklı ek");
        assert!(!ulke_eki_farki("CURLIN DRACARYS (USA)", "CURLIN DRACARYS (USA)"), "fark yok: 1. aşamanın işi");
        assert!(!ulke_eki_farki("CURLIN DRACARYS (XYZ)", "CURLIN DRACARYS"), "tanınmayan ek");
        assert!(!ulke_eki_farki("CURLIN DRACARY (USA)", "CURLIN DRACARYS"), "gövde farklı");
        assert!(!ulke_eki_farki("(USA)", ""), "boş gövde");
    }
}

/// Programdaki ve sonuçtaki eküri ibarelerini koşu koşu denetler. Dönüş: (inceleme nedenleri, özet).
/// * Her ibare ayrı grup olarak okunur (ardışık ibareler; fazladan "]" yalnızca numara ve ad açıkça okunabiliyorsa).
/// * Çözülemeyen ibare → EKURI_IBARESI_COZULEMEDI (ham metin ve kaynak satırıyla).
/// * Her (numara, ad) çifti o koşunun program katılımcısıyla doğrulanır: numara var ve ad aynı
///   (ya da yalnızca bir taraftaki tanınan ülke ekiyle farklı) olmalı → aksi hâlde EKURI_UYESI_DOGRULANAMADI.
/// * Bir numara iki grupta birden geçerse → EKURI_UYESI_BIRDEN_FAZLA_GRUPTA.
/// * Program ve sonuç grupları üye numaralarına göre, sıradan bağımsız karşılaştırılır → EKURI_GRUPLARI_CELISIYOR.
fn ekuri_kontrolu(p: &Kaynak, s: &Kaynak, prog: &[ProgAt]) -> (Vec<String>, String) {
    let mut inceleme = Vec::new();
    let (mut kosu_sayisi, mut grup_sayisi, mut tolerans) = (0, 0, 0);
    let kume = |g: &[Vec<(i64, String)>]| -> Vec<Vec<i64>> {
        let mut v: Vec<Vec<i64>> = g
            .iter()
            .map(|x| {
                let mut n: Vec<i64> = x.iter().map(|y| y.0).collect();
                n.sort();
                n
            })
            .collect();
        v.sort();
        v
    };
    let kosular: Vec<u32> = p.gun.kosular.iter().map(|k| k.kosu_no).collect();
    for kn in kosular {
        let pk = p.gun.kosular.iter().find(|k| k.kosu_no == kn);
        let sk = s.gun.kosular.iter().find(|k| k.kosu_no == kn);
        let mut kumeler = Vec::new();
        for (kaynak, k) in [(p, pk), (s, sk)] {
            let Some(k) = k else {
                kumeler.push(Vec::new());
                continue;
            };
            let (a, _) = ekuri_ayir_ayrintili(&k.alt_satir);
            for h in &a.cozulemeyen {
                inceleme.push(format!("EKURI_IBARESI_COZULEMEDI: {} satır {} ({kn}. koşu): {h}", kaynak.ad, k.alt_satir_no));
            }
            tolerans += a.fazla_kapa_toleransi;
            let mut gorulen = HashSet::new();
            for (no, ad) in a.gruplar.iter().flatten() {
                if !gorulen.insert(*no) {
                    inceleme.push(format!("EKURI_UYESI_BIRDEN_FAZLA_GRUPTA: {} satır {} ({kn}. koşu): {no} numara", kaynak.ad, k.alt_satir_no));
                }
                let dogru = prog.iter().any(|r| r.kosu_no == kn && r.at_no == *no && (r.at_adi == *ad || ulke_eki_farki(&r.at_adi, ad)));
                if !dogru {
                    let program_adi = prog.iter().find(|r| r.kosu_no == kn && r.at_no == *no).map(|r| r.at_adi.as_str()).unwrap_or("(bu numarada katılımcı yok)");
                    inceleme.push(format!(
                        "EKURI_UYESI_DOGRULANAMADI: {} satır {} ({kn}. koşu): ({no}){ad} — programda {no} numara: {program_adi}",
                        kaynak.ad, k.alt_satir_no
                    ));
                }
            }
            if !a.gruplar.is_empty() {
                grup_sayisi += a.gruplar.len();
            }
            kumeler.push(kume(&a.gruplar));
        }
        if !kumeler[0].is_empty() || !kumeler[1].is_empty() {
            kosu_sayisi += 1;
        }
        if kumeler[0] != kumeler[1] {
            inceleme.push(format!(
                "EKURI_GRUPLARI_CELISIYOR: {kn}. koşu program {:?} sonuç {:?} (üye numaraları, sıradan bağımsız)",
                kumeler[0], kumeler[1]
            ));
        }
    }
    let ozet = format!(
        "{kosu_sayisi} koşuda {grup_sayisi} grup okundu (program+sonuç); fazladan ']' toleransı {tolerans} parça; {} sorun",
        inceleme.len()
    );
    (inceleme, ozet)
}
