//! Aylık denetim (salt okunur): resmî takvim kapsamı, kayıt sayıları, inceleme nedenleri, kaynak bağları
//! (SHA-256 → indirme kaydındaki URL ve HTTP durumu), ertelenen koşular ve çift sayım kontrolü, altılı ödemeleri,
//! yarış günü numarası sürekliliği, PRAGMA foreign_key_check / integrity_check ve tüm tabloların tam döküm özeti.
//!
//! Kullanım:
//!   tjk_denetim --db DB --ay YYYY-AA --takvim TAKVIM.tsv [--durum DURUM.tsv]... --cikti RAPOR.md [--dokum DOKUM.txt]
//!
//! * TAKVIM.tsv: `tarih<TAB>hipodrom` (başlıklı); yalnızca --ay ayındaki satırlar kullanılır.
//! * DURUM.tsv: `tjk-fetch toplu` indirme kaydı (zaman_sn, tarih, hipodrom, rol, url, http, bayt, sha256, dogrulama, ayrinti).
//!   Kaynak bağı kontrolü: bu ayın günlerinin kullandığı her kaynak_dosya SHA-256'sı bir kayıtta HTTP 200 +
//!   ICERIK_DOGRULANDI ile görünmeli. Takvim çifti veri tabanında yoksa ve hiç kaydı yoksa HENUZ_DENENMEDI sayılır.
//! * --dokum: tüm tabloların sıralı tam dökümü (ikinci aktarımla `cmp` için); özeti raporda.
//! Çıkış kodu: 0 = sert hata yok; 1 = FK/bütünlük hatası, çift sayım ya da kaynak bağı kopuk.

#[path = "../sha256.rs"]
#[allow(dead_code)]
mod sha256;
#[path = "../sqlite.rs"]
#[allow(dead_code)]
mod sqlite;

use sqlite::{Db, Deger};
use std::collections::{BTreeMap, BTreeSet, HashMap};

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

fn main() {
    if let Err(e) = calistir() {
        eprintln!("HATA: {e}");
        std::process::exit(2);
    }
}

struct Kayit {
    #[allow(dead_code)]
    url: String,
    http: String,
    dogrulama: String,
    tarih: String,
    hipodrom: String,
    rol: String,
}

fn calistir() -> Result<(), String> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut sec: HashMap<String, Vec<String>> = HashMap::new();
    let mut it = a.iter();
    while let Some(x) = it.next() {
        let k = x.strip_prefix("--").ok_or_else(|| format!("beklenmeyen argüman {x}"))?;
        sec.entry(k.into()).or_default().push(it.next().ok_or_else(|| format!("--{k} değer bekliyor"))?.clone());
    }
    let tek = |k: &str| sec.get(k).and_then(|v| v.last()).cloned().ok_or_else(|| format!("--{k} gerekli"));
    let (dbyol, ay, takvim, cikti) = (tek("db")?, tek("ay")?, tek("takvim")?, tek("cikti")?);
    let onek = ay.replace('-', "");
    let db = Db::ac(&dbyol, true)?;
    let q = |sql: &str| -> Result<Vec<Vec<Deger>>, String> { db.sorgu(sql, &[]).map(|x| x.1) };
    let qp = |sql: &str, p: &str| -> Result<Vec<Vec<Deger>>, String> { db.sorgu(sql, &[Deger::Text(p.into())]).map(|x| x.1) };
    let n = |sql: &str| -> Result<i64, String> { db.tek_int(sql, &[Deger::Text(format!("{onek}%"))]) };
    let mut sert: Vec<String> = Vec::new();
    let mut r = format!("# Aylık denetim: {ay}\n\nVeri tabanı: `{dbyol}` (salt okunur açıldı) · Takvim: `{takvim}`\n\n");

    // ---------------- takvim
    let tv = std::fs::read_to_string(&takvim).map_err(|e| format!("{takvim}: {e}"))?;
    let mut ciftler: Vec<(String, String)> = Vec::new();
    for l in tv.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let (t, h) = l.split_once('\t').ok_or_else(|| format!("{takvim}: bozuk satır {l}"))?;
        if t.starts_with(&ay) {
            ciftler.push((t.trim().into(), h.trim().into()));
        }
    }
    // ---------------- indirme kayıtları (son kayıt geçerli; SHA → kayıt)
    let mut url_son: BTreeMap<String, Kayit> = BTreeMap::new();
    for d in sec.get("durum").cloned().unwrap_or_default() {
        let t = std::fs::read_to_string(&d).map_err(|e| format!("{d}: {e}"))?;
        for l in t.lines().skip(1) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() < 10 {
                continue;
            }
            let k = Kayit { url: c[4].into(), http: c[5].into(), dogrulama: c[8].into(), tarih: c[1].into(), hipodrom: c[2].into(), rol: c[3].into() };
            url_son.insert(c[4].to_string(), k);
            if c[8] == "ICERIK_DOGRULANDI" && c[5] == "200" && c[7].len() == 64 {
                url_son.insert(format!("sha:{}", c[7]), Kayit { url: c[4].into(), http: c[5].into(), dogrulama: c[8].into(), tarih: c[1].into(), hipodrom: c[2].into(), rol: c[3].into() });
            }
        }
    }
    let sha_kaydi = |sha: &str| url_son.get(&format!("sha:{sha}"));
    let cift_kayitlari = |t: &str, h: &str| -> Vec<String> {
        let mut v: Vec<String> = url_son
            .iter()
            .filter(|(k, x)| !k.starts_with("sha:") && x.tarih == t && slug(&x.hipodrom) == slug(h))
            .map(|(_, x)| format!("{} HTTP {} {}", x.rol, x.http, x.dogrulama))
            .collect();
        v.sort();
        v
    };

    // ---------------- 1) takvim kapsamı
    let gunler = qp(
        "SELECT gun_id, tarih, hipodrom, gun_durumu, yaris_gunu_no, (SELECT group_concat(neden, ' | ') FROM (SELECT neden FROM inceleme_nedeni i WHERE i.gun_id = y.gun_id ORDER BY sira)) FROM yaris_gunu y WHERE gun_id LIKE ?1 ORDER BY tarih, gun_id",
        &format!("{onek}%"),
    )?;
    let dbde: BTreeMap<String, &Vec<Deger>> = gunler.iter().map(|g| (g[0].metin(), g)).collect();
    let mut siniflar: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    let mut tablo = String::from("| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |\n|---|---|---|---|---|\n");
    let mut takvimde: BTreeSet<String> = BTreeSet::new();
    for (t, h) in &ciftler {
        let gid = format!("{}-{}", t.replace('-', ""), slug(h));
        takvimde.insert(gid.clone());
        let (sinif, no, ayr) = match dbde.get(&gid) {
            Some(g) if g[3].metin() == "TAMAM" => ("TAMAMLANDI", g[4].metin(), String::new()),
            Some(g) => ("INCELEME_GEREKLI", g[4].metin(), g[5].metin()),
            None => {
                let k = cift_kayitlari(t, h);
                let dogru = |rol: &str| k.iter().any(|x| x.starts_with(rol) && x.ends_with("HTTP 200 ICERIK_DOGRULANDI"));
                if k.is_empty() {
                    ("HENUZ_DENENMEDI", String::new(), String::new())
                } else if dogru("program ") && dogru("sonuc ") {
                    ("AKTARILMADI", String::new(), format!("iki dosya doğrulandı, veri tabanına henüz aktarılmadı: {}", k.join("; ")))
                } else {
                    ("EKSIK", String::new(), k.join("; "))
                }
            }
        };
        siniflar.entry(sinif).or_default().push(format!("{t} {h}"));
        tablo += &format!("| {t} | {h} | {sinif} | {no} | {} |\n", ayr.replace('|', "\\|"));
    }
    let disarida: Vec<String> = gunler.iter().filter(|g| !takvimde.contains(&g[0].metin())).map(|g| g[0].metin()).collect();
    let say = |k: &str| siniflar.get(k).map_or(0, Vec::len);
    let ozet = format!(
        "takvim {} çift: TAMAMLANDI {}, INCELEME_GEREKLI {}, EKSIK {}, AKTARILMADI {}, HENUZ_DENENMEDI {}; takvim dışı DB günü {}",
        ciftler.len(),
        say("TAMAMLANDI"),
        say("INCELEME_GEREKLI"),
        say("EKSIK"),
        say("AKTARILMADI"),
        say("HENUZ_DENENMEDI"),
        disarida.len()
    );
    r += &format!("## 1. Takvim kapsamı\n\n**{ozet}**\n\n{tablo}\n");
    if !disarida.is_empty() {
        r += &format!("Takvimde olmayan ama veri tabanında olan günler: {}\n\n", disarida.join(", "));
    }

    // ---------------- 2) sayılar
    r += "## 2. Kayıt sayıları\n\n| Tablo | Bu ay | Tüm veri tabanı |\n|---|---|---|\n";
    let sayimlar: [(&str, &str, &str); 12] = [
        ("yaris_gunu", "SELECT COUNT(*) FROM yaris_gunu WHERE gun_id LIKE ?1", "SELECT COUNT(*) FROM yaris_gunu"),
        ("kosu", "SELECT COUNT(*) FROM kosu WHERE gun_id LIKE ?1", "SELECT COUNT(*) FROM kosu"),
        ("katilim", "SELECT COUNT(*) FROM katilim WHERE kosu_id LIKE ?1", "SELECT COUNT(*) FROM katilim"),
        ("sonuc", "SELECT COUNT(*) FROM sonuc WHERE katilim_id LIKE ?1", "SELECT COUNT(*) FROM sonuc"),
        ("agf_gozlem", "SELECT COUNT(*) FROM agf_gozlem WHERE katilim_id LIKE ?1", "SELECT COUNT(*) FROM agf_gozlem"),
        ("bahis_sonucu", "SELECT COUNT(*) FROM bahis_sonucu WHERE kosu_id LIKE ?1", "SELECT COUNT(*) FROM bahis_sonucu"),
        ("ganyan_kontrol", "SELECT COUNT(*) FROM ganyan_kontrol WHERE kosu_id LIKE ?1", "SELECT COUNT(*) FROM ganyan_kontrol"),
        ("altili", "SELECT COUNT(*) FROM altili WHERE gun_id LIKE ?1", "SELECT COUNT(*) FROM altili"),
        ("altili_ayak", "SELECT COUNT(*) FROM altili_ayak WHERE altili_id LIKE ?1", "SELECT COUNT(*) FROM altili_ayak"),
        ("altili_odeme", "SELECT COUNT(*) FROM altili_odeme WHERE altili_id LIKE ?1", "SELECT COUNT(*) FROM altili_odeme"),
        ("inceleme_nedeni", "SELECT COUNT(*) FROM inceleme_nedeni WHERE gun_id LIKE ?1", "SELECT COUNT(*) FROM inceleme_nedeni"),
        ("kaynak_dosya (ayın kullandığı)", "SELECT COUNT(DISTINCT k) FROM (SELECT program_kaynak_id k FROM yaris_gunu WHERE gun_id LIKE ?1 UNION SELECT sonuc_kaynak_id FROM yaris_gunu WHERE gun_id LIKE ?1 UNION SELECT sonuc_kaynak_id FROM kosu WHERE gun_id LIKE ?1 AND sonuc_kaynak_id IS NOT NULL)", "SELECT COUNT(*) FROM kaynak_dosya"),
    ];
    let mut sayi_satiri = Vec::new();
    for (ad, s_ay, s_tum) in sayimlar {
        let (x, y) = (n(s_ay)?, db.tek_int(s_tum, &[])?);
        r += &format!("| {ad} | {x} | {y} |\n");
        sayi_satiri.push(format!("{}={x}", ad.split(' ').next().unwrap()));
    }
    let at = db.tek_int("SELECT COUNT(*) FROM at", &[])?;
    r += &format!("| at (tüm) | – | {at} |\n\n");
    // katılım = sonuç (bu ay)
    let (kat, son) = (n("SELECT COUNT(*) FROM katilim WHERE kosu_id LIKE ?1")?, n("SELECT COUNT(*) FROM sonuc WHERE katilim_id LIKE ?1")?);
    if kat != son {
        r += &format!("Not: bu ay katılım {kat} ≠ sonuç {son} (sonucu olmayan katılım).\n\n");
    }

    // ---------------- 3) inceleme nedenleri
    r += "## 3. İnceleme nedenleri\n\n";
    let nd = qp("SELECT gun_id, sira, neden FROM inceleme_nedeni WHERE gun_id LIKE ?1 ORDER BY 1, 2", &format!("{onek}%"))?;
    if nd.is_empty() {
        r += "Bu ay inceleme nedeni yok.\n\n";
    }
    for x in &nd {
        r += &format!("- `{}` {}: {}\n", x[0].metin(), x[1].metin(), x[2].metin());
    }
    r += "\n";

    // ---------------- 4) kaynak bağları
    r += "## 4. Kaynak bağları\n\n";
    let kaynaklar = qp(
        "SELECT DISTINCT kd.kaynak_id, kd.sha256, kd.dosya_adi, kd.rol FROM kaynak_dosya kd WHERE kd.kaynak_id IN (SELECT program_kaynak_id FROM yaris_gunu WHERE gun_id LIKE ?1 UNION SELECT sonuc_kaynak_id FROM yaris_gunu WHERE gun_id LIKE ?1 UNION SELECT sonuc_kaynak_id FROM kosu WHERE gun_id LIKE ?1) ORDER BY 1",
        &format!("{onek}%"),
    )?;
    let mut bagli = 0;
    let mut kopuk = Vec::new();
    for k in &kaynaklar {
        match sha_kaydi(&k[1].metin()) {
            Some(x) if x.http == "200" && x.dogrulama == "ICERIK_DOGRULANDI" => bagli += 1,
            _ => kopuk.push(format!("{} ({}, {})", k[2].metin(), k[3].metin(), k[1].metin())),
        }
    }
    r += &format!(
        "- Bu ayın kullandığı {} kaynak dosyanın {}'i indirme kaydında HTTP 200 + ICERIK_DOGRULANDI ile URL'ye bağlı.\n",
        kaynaklar.len(),
        bagli
    );
    if !kopuk.is_empty() {
        r += &format!("- **URL/HTTP kaydı bulunamayan:** {}\n", kopuk.join("; "));
        sert.push(format!("{} kaynak dosyanın indirme kaydı yok", kopuk.len()));
    }
    let v4bos = n("SELECT COUNT(*) FROM kosu WHERE gun_id LIKE ?1 AND (planlanan_tarih IS NULL OR tarih_durumu IS NULL OR (tarih_durumu <> 'SONUC_YOK' AND (sonuc_kaynak_id IS NULL OR gerceklesen_tarih IS NULL)))")?;
    let son_satirsiz = n("SELECT COUNT(*) FROM sonuc WHERE katilim_id LIKE ?1 AND sonuc_kaynak_satir IS NULL AND kosu_durumu NOT IN ('KOŞMADI','KOSMADI')")?;
    r += &format!("- v4 tarih/kaynak alanı boş koşu: {v4bos}\n- Kaynak satırı olmayan sonuç (koşmayanlar hariç): {son_satirsiz}\n");
    let td = qp("SELECT COALESCE(tarih_durumu,'NULL'), COUNT(*) FROM kosu WHERE gun_id LIKE ?1 GROUP BY 1 ORDER BY 1", &format!("{onek}%"))?;
    r += &format!("- tarih_durumu: {}\n", td.iter().map(|x| format!("{}={}", x[0].metin(), x[1].metin())).collect::<Vec<_>>().join(", "));
    let yetim = db.tek_int(
        "SELECT COUNT(*) FROM kaynak_dosya WHERE kaynak_id NOT IN (SELECT program_kaynak_id FROM yaris_gunu UNION SELECT sonuc_kaynak_id FROM yaris_gunu UNION SELECT sonuc_kaynak_id FROM kosu WHERE sonuc_kaynak_id IS NOT NULL)",
        &[],
    )?;
    r += &format!("- Hiçbir kayda bağlı olmayan kaynak_dosya (tüm DB): {yetim}\n\n");

    // ---------------- 5) ertelemeler ve çift sayım
    r += "## 5. Ertelenen koşular ve çift sayım\n\n";
    let er = qp(
        "SELECT ko.kosu_id, ko.planlanan_tarih, ko.planlanan_saat, ko.gerceklesen_tarih, ko.gerceklesen_saat, kd.dosya_adi FROM kosu ko LEFT JOIN kaynak_dosya kd ON kd.kaynak_id = ko.sonuc_kaynak_id WHERE ko.gun_id LIKE ?1 AND ko.tarih_durumu <> 'PLANLANDIGI_GUN' ORDER BY 1",
        &format!("{onek}%"),
    )?;
    if er.is_empty() {
        r += "Bu ay planlandığı gün dışında koşulan ya da sonucu olmayan koşu yok.\n";
    }
    for x in &er {
        r += &format!("- `{}`: planlanan {} {} → gerçekleşen {} {} · kaynak `{}`\n", x[0].metin(), x[1].metin(), x[2].metin(), x[3].metin(), x[4].metin(), x[5].metin());
    }
    // aynı sonuç dosyası birden çok güne bağlıysa çift sayım şüphesi
    let cift = q("SELECT sonuc_kaynak_id, group_concat(DISTINCT gun_id) FROM kosu WHERE sonuc_kaynak_id IS NOT NULL GROUP BY 1 HAVING COUNT(DISTINCT gun_id) > 1")?;
    // aynı hipodrom + yarış günü no 90 gün içinde iki kez
    let cift_no = q(
        "SELECT a.gun_id, b.gun_id FROM yaris_gunu a JOIN yaris_gunu b ON a.hipodrom = b.hipodrom AND a.yaris_gunu_no = b.yaris_gunu_no AND a.gun_id < b.gun_id AND julianday(b.tarih) - julianday(a.tarih) <= 14",
    )?;
    let cift_kosu = q("SELECT COUNT(*) FROM (SELECT gun_id, kosu_no FROM kosu GROUP BY 1, 2 HAVING COUNT(*) > 1)")?;
    r += &format!(
        "\n- Birden çok güne bağlı sonuç dosyası (tüm DB): {}\n- Aynı hipodrom + yarış günü no'su 14 gün içinde iki gün (erteleme sonrası çift aktarım belirtisi; numaralar sezon başında sıfırlanır) (tüm DB): {}\n- Aynı gün + koşu no iki kez: {}\n\n",
        cift.len(),
        cift_no.len(),
        cift_kosu[0][0].metin()
    );
    if !cift.is_empty() || !cift_no.is_empty() {
        sert.push(format!(
            "çift sayım şüphesi: {:?} {:?}",
            cift.iter().map(|x| x[1].metin()).collect::<Vec<_>>(),
            cift_no.iter().map(|x| format!("{}={}", x[0].metin(), x[1].metin())).collect::<Vec<_>>()
        ));
    }

    // ---------------- 6) yarış günü no sürekliliği
    r += "## 6. Yarış günü numarası sürekliliği (hipodrom başına, önceki DB günüyle)\n\n";
    let tum = q("SELECT hipodrom, tarih, yaris_gunu_no, gun_id FROM yaris_gunu ORDER BY hipodrom, tarih")?;
    let mut onceki: HashMap<String, (String, i64)> = HashMap::new();
    let mut bosluk = Vec::new();
    for x in &tum {
        let (h, t, no) = (x[0].metin(), x[1].metin(), x[2].int().unwrap_or(-1));
        if let Some((pt, pno)) = onceki.get(&slug(&h)) {
            if t.starts_with(&ay) && no != pno + 1 && no != 1 {
                bosluk.push(format!("{h}: {pt} no {pno} → {t} no {no}"));
            }
        }
        onceki.insert(slug(&h), (t, no));
    }
    r += &if bosluk.is_empty() { "Bu ayda numara boşluğu yok (önceki gün DB'de olan hipodromlar için).\n\n".to_string() } else { format!("{}\n\n", bosluk.iter().map(|b| format!("- {b}")).collect::<Vec<_>>().join("\n")) };

    // ---------------- 7) altılı
    r += "## 7. Altılı kombinasyon ve ödemeleri\n\n";
    let alt = qp(
        "SELECT a.altili_id, a.kombinasyon_ham, a.odeme_tl, a.sonuc_durumu, (SELECT COUNT(*) FROM altili_odeme o WHERE o.altili_id = a.altili_id) FROM altili a WHERE a.gun_id LIKE ?1 ORDER BY 1",
        &format!("{onek}%"),
    )?;
    let odemeli = alt.iter().filter(|x| x[4].int().unwrap_or(0) > 0).count();
    let coklu: Vec<String> = alt.iter().filter(|x| x[4].int().unwrap_or(0) > 1).map(|x| x[0].metin()).collect();
    let devir = alt.iter().filter(|x| x[3].metin() == "DEVIR").count();
    let odemesiz: Vec<String> = alt.iter().filter(|x| x[4].int().unwrap_or(0) == 0).map(|x| x[0].metin()).collect();
    let ortak = n("SELECT COUNT(*) FROM altili_ayak_gecerli WHERE altili_id LIKE ?1 AND neden = 'ORTAK_BIRINCI'")?;
    r += &format!(
        "- Altılı {}; ödeme satırı olan {odemeli}; DEVIR {devir}; birden çok kazanan kombinasyonlu: {}; ORTAK_BIRINCI geçerli numara: {ortak}\n",
        alt.len(),
        if coklu.is_empty() { "yok".to_string() } else { coklu.join(", ") }
    );
    for id in &coklu {
        let o = qp("SELECT sira, kombinasyon_ham, odeme_ham, bahis_kosu_id, bahis_sira FROM altili_odeme WHERE altili_id = ?1 ORDER BY 1", id)?;
        for x in o {
            r += &format!("  - `{id}` #{}: {} = {} (bahis {} sıra {})\n", x[0].metin(), x[1].metin(), x[2].metin(), x[3].metin(), x[4].metin());
        }
    }
    let mut gerekcesiz = Vec::new();
    for id in &odemesiz {
        let gun = id.rsplitn(2, '-').nth(1).unwrap_or("").to_string();
        let var = db.tek_int("SELECT COUNT(*) FROM inceleme_nedeni WHERE gun_id = ?1 AND neden LIKE ?2", &[gun.clone().into(), format!("%{id}%").into()])?;
        if var == 0 {
            gerekcesiz.push(id.clone());
        }
    }
    r += &format!(
        "- Ödeme satırı olmayan altılı: {} ({}); bunlardan inceleme nedeni yazılmamış: {}\n\n",
        odemesiz.len(),
        odemesiz.join(", "),
        if gerekcesiz.is_empty() { "yok".into() } else { gerekcesiz.join(", ") }
    );
    if !gerekcesiz.is_empty() {
        sert.push(format!("gerekçesiz boş altılı ödemesi: {gerekcesiz:?}"));
    }

    // ---------------- 8) bütünlük + döküm
    let fk = q("PRAGMA foreign_key_check")?;
    let ic = q("PRAGMA integrity_check")?;
    let icm = ic.iter().map(|x| x[0].metin()).collect::<Vec<_>>().join("; ");
    if !fk.is_empty() {
        sert.push(format!("foreign_key_check {} satır", fk.len()));
    }
    if icm != "ok" {
        sert.push(format!("integrity_check: {icm}"));
    }
    let tablolar = q("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name")?;
    let mut dokum = String::new();
    let mut tsay = Vec::new();
    for t in &tablolar {
        let ad = t[0].metin();
        let (kol, _) = db.sorgu(&format!("SELECT * FROM \"{ad}\" LIMIT 0"), &[])?;
        let sira: Vec<String> = (1..=kol.len()).map(|i| i.to_string()).collect();
        let (_, sat) = db.sorgu(&format!("SELECT * FROM \"{ad}\" ORDER BY {}", sira.join(",")), &[])?;
        dokum += &format!("## {ad} ({} satır)\n", sat.len());
        for s in &sat {
            dokum += &s.iter().map(|d| format!("{d:?}")).collect::<Vec<_>>().join("\t");
            dokum.push('\n');
        }
        tsay.push(format!("{ad} {}", sat.len()));
    }
    let dsha = sha256::hex(dokum.as_bytes());
    if let Some(d) = sec.get("dokum").and_then(|v| v.last()) {
        std::fs::write(d, &dokum).map_err(|e| format!("{d}: {e}"))?;
    }
    let uv = db.tek_int("PRAGMA user_version", &[])?;
    r += &format!(
        "## 8. Bütünlük\n\n- `PRAGMA user_version`: {uv}\n- `PRAGMA foreign_key_check`: {} satır\n- `PRAGMA integrity_check`: {icm}\n- Tam döküm SHA-256 ({} tablo): `{dsha}`\n- Tablo satırları: {}\n\n",
        fk.len(),
        tablolar.len(),
        tsay.join(", ")
    );
    r += &format!("## Sonuç\n\n{}\n", if sert.is_empty() { "Sert hata yok.".to_string() } else { format!("**SERT HATA:** {}", sert.join("; ")) });
    std::fs::write(&cikti, &r).map_err(|e| format!("{cikti}: {e}"))?;
    println!("DENETIM {ay}: {ozet}");
    println!("SAYILAR {ay}: {}", sayi_satiri.join(" "));
    println!("BUTUNLUK: fk={} integrity={icm} dokum_sha256={dsha}", fk.len());
    if !sert.is_empty() {
        println!("SERT HATA: {}", sert.join("; "));
        std::process::exit(1);
    }
    Ok(())
}
