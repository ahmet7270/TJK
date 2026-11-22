//! TJK günlük CSV (Program / Sonuçlar) dosyalarının ham yapısını ve alanlarını ayrıştırır.
//! Kural: dosyada olmayan değer üretilmez; çözülemeyen alan None/boş kalır.

pub const KOSMAZ_IBARE: &str = "(Koşmaz)";
/// Bu dosyalarda at adının sonunda görülen kod belirteçleri (takı olduğu varsayılıyor; anlamları dosyada yok).
/// "YP": Mart 2026 dosyalarında 20 at satırında, her seferinde başka kodlardan sonra ("SK YP", "K YP") görülüyor.
/// Listede yokken ayıklama son kelimede duruyor, ad tüm kodlarla kalıyordu ve eküri eşleşmesi bozuluyordu
/// (21.03.2026 İstanbul 6. koşu, LEONİDAS). Ham ad program.csv'de `at_isim_ham` olarak aynen korunur.
const KOD_BELIRTECLERI: [&str; 11] = ["KG", "K", "DB", "SK", "SKG", "GKR", "ÖG", "SGKR", "YP", "BB", "TGK"];

pub fn rakamlar(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// "3,95" -> 3.95 ; "29.789,50" -> 29789.5 ; "" -> None
pub fn tr_num(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    t.replace('.', "").replace(',', ".").parse().ok()
}

/// "2:08.28" -> 128.28 saniye
pub fn sure_sn(s: &str) -> Option<f64> {
    let t = s.trim();
    let (dk, kalan) = t.split_once(':')?;
    let (sn, cs) = kalan.split_once('.')?;
    if !rakamlar(dk) || sn.len() != 2 || !rakamlar(sn) || cs.len() != 2 || !rakamlar(cs) {
        return None;
    }
    let sant: u64 = (dk.parse::<u64>().ok()? * 60 + sn.parse::<u64>().ok()?) * 100 + cs.parse::<u64>().ok()?;
    Some(sant as f64 / 100.0)
}

pub fn to_int(s: &str) -> Option<i64> {
    let t = s.trim();
    if rakamlar(t) { t.parse().ok() } else { None }
}

/// "3y d  e" -> 3
pub fn yas_coz(s: &str) -> Option<i64> {
    let e = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    if e == 0 || !s[e..].starts_with('y') {
        return None;
    }
    s[..e].parse().ok()
}

pub struct Isim {
    pub ad: String,
    pub kodlar: String,
    pub kosmaz: bool,
}

/// "SERİNTEPE KG DB SK (Koşmaz)" -> ad "SERİNTEPE", kodlar "KG DB SK", kosmaz true
pub fn isim_coz(raw: &str) -> Isim {
    let kosmaz = raw.contains(KOSMAZ_IBARE);
    let temiz = raw.replace(KOSMAZ_IBARE, "");
    let mut tok: Vec<&str> = temiz.split_whitespace().collect();
    let mut kod: Vec<&str> = Vec::new();
    while tok.len() > 1 && KOD_BELIRTECLERI.contains(tok.last().unwrap()) {
        kod.insert(0, tok.pop().unwrap());
    }
    Isim { ad: tok.join(" "), kodlar: kod.join(" "), kosmaz }
}

/// "54 +0.50" -> (Some(54.0), "+0.50") ; "60,5" -> (Some(60.5), "")
pub fn kilo_coz(raw: &str) -> (Option<f64>, String) {
    let t = raw.trim();
    let son = t.find(|c: char| !(c.is_ascii_digit() || c == ',' || c == '.')).unwrap_or(t.len());
    if son == 0 {
        return (None, String::new());
    }
    let kalan = t[son..].trim_start();
    let ek = if kalan.is_empty() {
        ""
    } else {
        match kalan.strip_prefix('+') {
            Some(r) if !r.is_empty() && r.chars().all(|c| c.is_ascii_digit() || c == '.') => kalan,
            _ => return (None, String::new()),
        }
    };
    (t[..son].replace(',', ".").parse().ok(), ek.to_string())
}

/// "M.B.ÖNCEL AP" -> ("M.B.ÖNCEL", true)
pub fn jokey_coz(raw: &str) -> (String, bool) {
    let t = raw.trim();
    match t.strip_suffix(" AP") {
        Some(x) => (x.trim().to_string(), true),
        None => (t.to_string(), false),
    }
}

/// "8 - DS" -> (Some(8), "DS")
pub fn start_coz(raw: &str) -> (Option<i64>, String) {
    let t = raw.trim();
    let son = t.find(|c: char| !c.is_ascii_digit()).unwrap_or(t.len());
    if son == 0 {
        return (None, String::new());
    }
    let Ok(no) = t[..son].parse::<i64>() else { return (None, String::new()) };
    let kalan = t[son..].trim_start();
    if kalan.is_empty() {
        return (Some(no), String::new());
    }
    if let Some(r) = kalan.strip_prefix('-') {
        let ek = r.trim_start();
        if !ek.is_empty() && !ek.contains(char::is_whitespace) {
            return (Some(no), ek.to_string());
        }
    }
    (None, String::new())
}

/// "%2.72(7)" -> (Some(2.72), Some(7))
pub fn agf_coz(raw: &str) -> (Option<f64>, Option<i64>) {
    let t = raw.trim();
    let Some(r) = t.strip_prefix('%') else { return (None, None) };
    let Some((y, s)) = r.split_once('(') else { return (None, None) };
    let Some(s) = s.strip_suffix(')') else { return (None, None) };
    if y.is_empty() || !y.chars().all(|c| c.is_ascii_digit() || c == '.') || !rakamlar(s) {
        return (None, None);
    }
    match (y.parse::<f64>(), s.parse::<i64>()) {
        (Ok(a), Ok(b)) => (Some(a), Some(b)),
        _ => (None, None),
    }
}

/// AGF hücresi bir veya birden çok değer içerebilir: "%17.61(2)  %17.99(2)".
/// Boş hücre -> Some([]); çözülemeyen hücre -> None (ham değer korunur).
pub fn agf_coklu(raw: &str) -> Option<Vec<(f64, i64)>> {
    raw.split_whitespace()
        .map(|t| match agf_coz(t) {
            (Some(y), Some(s)) => Some((y, s)),
            _ => None,
        })
        .collect()
}

/// Satır başındaki ardışık "[(n)AD,(m)AD,] eküridir." ibarelerinin ayrıntılı ayrımı.
#[derive(Debug, Default, PartialEq)]
pub struct EkuriAyrim {
    /// Okunan gruplar (kaynaktaki sırayla): (program no, ad)
    pub gruplar: Vec<Vec<(i64, String)>>,
    /// Çözülemeyen eküri ibareleri, ham hâliyle ("[" ile başlayıp "eküridir." ile biten metin)
    pub cozulemeyen: Vec<String>,
    /// Addan sonra gelen fazladan "]" karakteri kabul edilen parça sayısı
    pub fazla_kapa_toleransi: usize,
}

/// Eküri ibarelerini ayırır; (ayrım, ibarelerden sonra kalan metin).
/// Her ibare "[" ile başlar ve ilk "eküridir." ile biter; gövde, "eküridir."den önceki son "]"a kadardır.
/// Gövde virgülle bölünür; her parça "(rakamlar)AD" olmalıdır. Adın sonunda TEK bir fazladan "]" varsa
/// yalnızca numara ve ad açıkça okunabiliyorsa (rakamlar boş değil, ad boş değil, adda başka köşeli parantez yok)
/// kabul edilir ve sayılır. Okunamayan ibare gruplara katılmaz, ham hâliyle `cozulemeyen`e yazılır.
pub fn ekuri_ayir_ayrintili(s: &str) -> (EkuriAyrim, &str) {
    const SON: &str = "eküridir.";
    let mut a = EkuriAyrim::default();
    let mut kalan = s;
    loop {
        let bas = kalan.trim_start();
        let Some(ic) = bas.strip_prefix('[') else { break };
        let Some(p) = ic.find(SON) else { break };
        let ibare = &bas[..1 + p + SON.len()];
        let sonra = &ic[p + SON.len()..];
        let once = ic[..p].trim_end();
        let grup = once.strip_suffix(']').and_then(|govde| {
            let mut g = Vec::new();
            let mut tol = 0;
            for parca in govde.split(',').map(str::trim).filter(|x| !x.is_empty()) {
                let r = parca.strip_prefix('(')?;
                let (no, ad) = r.split_once(')')?;
                if no.is_empty() || !no.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                let mut ad = ad.trim();
                if let Some(x) = ad.strip_suffix(']') {
                    ad = x.trim_end();
                    tol += 1;
                }
                if ad.is_empty() || ad.contains('[') || ad.contains(']') {
                    return None;
                }
                g.push((no.parse::<i64>().ok()?, ad.to_string()));
            }
            (!g.is_empty()).then_some((g, tol))
        });
        match grup {
            Some((g, tol)) => {
                a.gruplar.push(g);
                a.fazla_kapa_toleransi += tol;
            }
            None => a.cozulemeyen.push(ibare.to_string()),
        }
        kalan = sonra;
    }
    (a, kalan)
}

/// Geriye uyumlu kısa biçim: yalnızca okunan gruplar ve kalan metin.
pub fn ekuri_ayir(s: &str) -> (Vec<Vec<(i64, String)>>, &str) {
    let (a, kalan) = ekuri_ayir_ayrintili(s);
    (a.gruplar, kalan)
}

// ---------------------------------------------------------------- bahis satırı

#[derive(Clone, Debug)]
pub struct Bahis {
    pub oyun: String,
    pub kombinasyon_ham: String,
    /// Her ayak/sıra için geçerli program numaraları. Ayrıştırılamazsa boş.
    pub ayaklar: Vec<Vec<i64>>,
    /// Kazanana ödeme (ham). Devirde boş.
    pub tutar_ham: String,
    /// Kazanana ödeme. Devirde None.
    pub tutar_tl: Option<f64>,
    pub teselli_ham: String,
    /// "ODENDI" ya da "DEVIR" ("Bilen çıkmamıştır … devretmiştir")
    pub sonuc_durumu: &'static str,
    pub devir_ham: String,
    pub devir_tl: Option<f64>,
    /// Kaynaktaki ibare (oyun adından tutarın sonuna kadar), olduğu gibi
    pub ham_ibare: String,
}

/// Başta "[\d.]+,\d{2}" kalıbını okur; (tüketilen bayt, tutar metni)
fn tutar_oku(s: &str) -> Option<(usize, &str)> {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.') {
        i += 1;
    }
    if i == 0 || i + 3 > b.len() || b[i] != b',' || !b[i + 1].is_ascii_digit() || !b[i + 2].is_ascii_digit() {
        return None;
    }
    Some((i + 3, &s[..i + 3]))
}

/// Tutarı iki yazımla okur: Türkçe "1.007.862,90" ya da İngilizce "1,007,862.90".
/// Ondalık ayıracı sondaki iki basamaktan önceki işarettir. (tüketilen bayt, metin, değer)
pub fn esnek_tutar(s: &str) -> Option<(usize, &str, f64)> {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.' || b[i] == b',') {
        i += 1;
    }
    let t = &s[..i];
    if t.len() < 4 {
        return None;
    }
    let (govde, ondalik, ayirac) = (&t[..t.len() - 3], &t[t.len() - 2..], t.as_bytes()[t.len() - 3]);
    if !ondalik.bytes().all(|c| c.is_ascii_digit()) || govde.is_empty() || !govde.as_bytes()[0].is_ascii_digit() {
        return None;
    }
    let binlik = match ayirac {
        b',' => '.',
        b'.' => ',',
        _ => return None,
    };
    if govde.contains(ayirac as char) {
        return None;
    }
    let tam: String = govde.chars().filter(|c| *c != binlik).collect();
    if !tam.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some((i, t, format!("{tam}.{ondalik}").parse().ok()?))
}

fn konum(tum: &str, son_ek: &str) -> usize {
    tum.len() - son_ek.len()
}

const BILEN_YOK: &str = "Bilen çıkmamıştır";

fn bahis_dene(s: &str, bas: usize, p_ac: usize) -> Option<(Bahis, usize)> {
    let oyun = &s[bas..p_ac];
    let p_kapa = p_ac + 1 + s[p_ac + 1..].find(')')?;
    let komb = &s[p_ac + 1..p_kapa];
    let r = s[p_kapa + 1..].trim_start().strip_prefix(':')?;
    let ayaklar: Option<Vec<Vec<i64>>> = komb
        .split('/')
        .map(|p| p.split(',').map(|n| n.trim().parse::<i64>().ok()).collect::<Option<Vec<_>>>())
        .collect();
    let mut b = Bahis {
        oyun: oyun.trim_matches(|c| c == ' ' || c == ',').to_string(),
        kombinasyon_ham: komb.to_string(),
        ayaklar: ayaklar.unwrap_or_default(),
        tutar_ham: String::new(),
        tutar_tl: None,
        teselli_ham: String::new(),
        sonuc_durumu: "ODENDI",
        devir_ham: String::new(),
        devir_tl: None,
        ham_ibare: String::new(),
    };
    let son;
    if let Some(d) = r.trim_start().strip_prefix(BILEN_YOK) {
        // "Bilen çıkmamıştır, 1,007,862.90 TL devretmiştir."
        b.sonuc_durumu = "DEVIR";
        let mut kalan = d;
        let d2 = d.trim_start_matches([',', ' ']);
        if let Some((n, t, v)) = esnek_tutar(d2) {
            if let Some(r2) = d2[n..].strip_prefix(" TL") {
                b.devir_ham = format!("{t} TL");
                b.devir_tl = Some(v);
                kalan = r2;
            }
        }
        if let Some(r3) = kalan.trim_start().strip_prefix("devretmiştir") {
            kalan = r3.strip_prefix('.').unwrap_or(r3);
        }
        son = konum(s, kalan);
    } else {
        let (n, tutar) = tutar_oku(r)?;
        let r = r[n..].strip_prefix(" TL")?;
        b.tutar_ham = format!("{tutar} TL");
        b.tutar_tl = tr_num(tutar);
        let mut sn = konum(s, r);
        if let Some(r2) = r.trim_start().strip_prefix("Çıkan 1 Atla:") {
            let r3 = r2.trim_start();
            if let Some((n2, t2)) = tutar_oku(r3) {
                if let Some(r4) = r3[n2..].strip_prefix(" TL") {
                    b.teselli_ham = format!("{t2} TL");
                    sn = konum(s, r4);
                }
            }
        }
        son = sn;
    }
    b.ham_ibare = s[bas..son].trim_matches(|c: char| c == ' ' || c == ',').to_string();
    Some((b, son))
}

/// Okunamayan metin parçası: ayıraçlar dışında içerik varsa döndürür.
fn artik(t: &str) -> Option<String> {
    let x = t.trim_matches(|c: char| c.is_whitespace() || c == ',' || c == '.');
    (!x.is_empty()).then(|| x.to_string())
}

/// "GANYAN(5) :3,95 TL, İKİLİ(3/5) :6,70 TL, ..." satırını oyunlara ayırır.
/// Dönüş: (okunan bahisler, okunamayan ham parçalar).
pub fn bahis_ayristir(satir: &str) -> (Vec<Bahis>, Vec<String>) {
    let mut out = Vec::new();
    let mut okunamayan = Vec::new();
    let mut pos = 0; // okunmuş metnin sonu
    let mut ara = 0; // sonraki '(' aramasının başlangıcı
    while let Some(rel) = satir[ara..].find('(') {
        let p_ac = ara + rel;
        // oyun adı parantez içeremez: son ')' sonrasından başlar
        let mut bas = match satir[pos..p_ac].rfind(')') {
            Some(i) => pos + i + 1,
            None => pos,
        };
        // önceki okunamayan parçanın kalıntısı (':' içerir) varsa son ", " sonrasından başla
        if satir[bas..p_ac].contains(':') {
            if let Some(i) = satir[bas..p_ac].rfind(", ") {
                bas += i + 2;
            }
        }
        if bas < p_ac {
            if let Some((b, son)) = bahis_dene(satir, bas, p_ac) {
                if let Some(a) = artik(&satir[pos..bas]) {
                    okunamayan.push(a);
                }
                out.push(b);
                pos = son;
                ara = son;
                continue;
            }
        }
        ara = p_ac + 1;
    }
    if let Some(a) = artik(&satir[pos..]) {
        okunamayan.push(a);
    }
    (out, okunamayan)
}

// ---------------------------------------------------------------- gün dosyası

pub struct Kosu {
    pub kosu_no: u32,
    /// Koşu başlık satırının 1 tabanlı numarası
    pub baslik_satir: usize,
    pub saat: String,
    /// Başlıkta yazıyorsa koşunun adı (ör. "ULUSLARARASI KARACAOĞLAN KOŞUSU"), yoksa boş
    pub kosu_adi: String,
    pub kosu_cinsi: String,
    pub yas_grubu: String,
    pub kilo_sartlari: String,
    pub mesafe_m: u32,
    pub pist: String,
    pub rekor_derece: String,
    pub ikramiye: Vec<Option<f64>>,
    pub yetistirici_primi: Vec<Option<f64>>,
    pub kolonlar: Vec<String>,
    /// (1 tabanlı satır no, alanlar)
    pub atlar: Vec<(usize, Vec<String>)>,
    pub alt_satir_no: usize,
    pub alt_satir: String,
}

pub struct Gun {
    pub hipodrom: String,
    pub yaris_gunu_no: u32,
    pub tarih: String,
    pub kosular: Vec<Kosu>,
    pub ilk_satir: String,
}

/// "1. Kosu :   18.30" -> (1, "18:30", "")
/// "5. Kosu :  DÜNYA KADINLAR GÜNÜ KOŞUSU 20.00" -> (5, "20:00", "DÜNYA KADINLAR GÜNÜ KOŞUSU")
/// Saat, başlığın sonundaki "dd.dd"dir; öncesindeki metin (varsa) koşunun adıdır.
fn kosu_basligi(f0: &str) -> Option<(u32, String, String)> {
    let son = f0.find(|c: char| !c.is_ascii_digit())?;
    if son == 0 {
        return None;
    }
    let no: u32 = f0[..son].parse().ok()?;
    let kalan = f0[son..].strip_prefix(". Kosu :")?.trim();
    if kalan.len() < 5 || !kalan.is_char_boundary(kalan.len() - 5) {
        return None;
    }
    let (ad, saat) = kalan.split_at(kalan.len() - 5);
    let b = saat.as_bytes();
    let ok = b[0].is_ascii_digit() && b[1].is_ascii_digit() && b[2] == b'.' && b[3].is_ascii_digit() && b[4].is_ascii_digit();
    let ad_ok = ad.is_empty() || ad.ends_with(char::is_whitespace);
    (ok && ad_ok).then(|| (no, saat.replace('.', ":"), ad.trim().to_string()))
}

fn at_satiri_mi(s: &str) -> bool {
    let son = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    son > 0 && s[son..].starts_with(';')
}

fn yaris_gunu_no(s: &str) -> Option<u32> {
    let mut pos = 0;
    while let Some(rel) = s[pos..].find('(') {
        let r = &s[pos + rel + 1..];
        let son = r.find(|c: char| !c.is_ascii_digit()).unwrap_or(r.len());
        if son > 0 && r[son..].starts_with(". Yarış Günü)") {
            return r[..son].parse().ok();
        }
        pos += rel + 1;
    }
    None
}

fn ikramiye_degeri(x: &str) -> String {
    let b = x.as_bytes();
    let x = if b.len() >= 3 && b[0].is_ascii_digit() && b[1] == b'.' && b[2] == b')' { &x[3..] } else { x };
    x.replace(" TL", "").trim().to_string()
}

pub fn gun_ayristir(metin: &str) -> Result<Gun, String> {
    let lines: Vec<&str> = metin.lines().collect();
    let ilk = *lines.first().ok_or("dosya boş")?;
    let bas: Vec<&str> = ilk.split(';').collect();
    if bas.len() < 3 {
        return Err(format!("ilk satır beklenen biçimde değil: {ilk}"));
    }
    let hipodrom = bas[0].trim().to_string();
    let yg = yaris_gunu_no(bas[1]).ok_or_else(|| format!("yarış günü no okunamadı: {}", bas[1]))?;
    let t: Vec<&str> = bas[2].trim().split('/').collect();
    if t.len() != 3 || t[0].len() != 2 || t[1].len() != 2 || t[2].len() != 4 || !t.iter().all(|x| rakamlar(x)) {
        return Err(format!("tarih GG/AA/YYYY değil: {}", bas[2]));
    }
    let tarih = format!("{}-{}-{}", t[2], t[1], t[0]);

    let mut kosular = Vec::new();
    let mut i = 1;
    while i < lines.len() {
        let ln = lines[i];
        let Some((kosu_no, saat, kosu_adi)) = kosu_basligi(ln.split(';').next().unwrap_or("").trim()) else {
            i += 1;
            continue;
        };
        let alan: Vec<&str> = ln.split(';').map(str::trim).collect();
        let mut dolu: Vec<&str> = alan[1..].iter().copied().filter(|x| !x.is_empty()).collect();
        let rekor = dolu
            .iter()
            .find(|x| x.starts_with("Rekor Derece"))
            .map(|x| x.split_once(':').map_or("", |p| p.1).trim().to_string())
            .unwrap_or_default();
        dolu.retain(|x| !x.starts_with("Rekor Derece"));
        let mi = dolu
            .iter()
            .position(|x| x.len() > 1 && x.ends_with('m') && rakamlar(&x[..x.len() - 1]))
            .ok_or_else(|| format!("satır {}: mesafe bulunamadı", i + 1))?;
        let ik_satir = lines.get(i + 2).ok_or_else(|| format!("satır {}: ikramiye satırı yok", i + 3))?;
        let vals: Vec<String> =
            ik_satir.split(';').map(str::trim).filter(|x| !x.is_empty()).map(ikramiye_degeri).collect();
        let al = |a: usize| (a..a + 5).map(|k| vals.get(k).and_then(|v| tr_num(v))).collect::<Vec<_>>();
        let kolonlar: Vec<String> = lines
            .get(i + 3)
            .ok_or_else(|| format!("satır {}: kolon satırı yok", i + 4))?
            .split(';')
            .map(String::from)
            .collect();
        let mut j = i + 4;
        let mut atlar = Vec::new();
        while j < lines.len() && at_satiri_mi(lines[j]) {
            atlar.push((j + 1, lines[j].split(';').map(String::from).collect()));
            j += 1;
        }
        kosular.push(Kosu {
            kosu_no,
            baslik_satir: i + 1,
            saat,
            kosu_adi,
            kosu_cinsi: dolu.first().copied().unwrap_or("").to_string(),
            yas_grubu: dolu.get(1).copied().unwrap_or("").to_string(),
            kilo_sartlari: if mi > 2 { dolu[2..mi].join(" | ") } else { String::new() },
            mesafe_m: dolu[mi][..dolu[mi].len() - 1].parse().unwrap_or(0),
            pist: dolu.get(mi + 1).copied().unwrap_or("").to_string(),
            rekor_derece: rekor,
            ikramiye: al(0),
            yetistirici_primi: al(5),
            kolonlar,
            atlar,
            alt_satir_no: j + 1,
            alt_satir: lines.get(j).copied().unwrap_or("").to_string(),
        });
        i = j + 1;
    }
    Ok(Gun { hipodrom, yaris_gunu_no: yg, tarih, kosular, ilk_satir: ilk.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sayilar_ve_sureler() {
        assert_eq!(tr_num("3,95"), Some(3.95));
        assert_eq!(tr_num("29.789,50"), Some(29789.5));
        assert_eq!(tr_num(" "), None);
        assert_eq!(sure_sn("2:08.28"), Some(128.28));
        assert_eq!(sure_sn("Koşmaz"), None);
        assert_eq!(yas_coz("3y d  e"), Some(3));
    }

    /// 21.03.2026 İstanbul 6. koşu: programda "6;LEONİDAS SKG SK YP;…", eküri ibaresinde "(6)LEONİDAS".
    /// "YP" kod listesinde yokken ayıklama son kelimede durur; ad "LEONİDAS SKG SK YP" kalır ve eküri eşleşmez.
    #[test]
    fn bb_ve_tgk_kod_belirtecleri() {
        // 2026 dosyalarında bilinen kodların ardından gelen iki donanım kodu (0.15.0)
        let i = isim_coz("LADY SUPRISE DB SKG SK BB");
        assert_eq!((i.ad.as_str(), i.kodlar.as_str()), ("LADY SUPRISE", "DB SKG SK BB"));
        assert_eq!(isim_coz("LO SIENTO BB SK").ad, "LO SIENTO");
        let i = isim_coz("MERİHİN YILDIZI KG DB SK TGK");
        assert_eq!((i.ad.as_str(), i.kodlar.as_str()), ("MERİHİN YILDIZI", "KG DB SK TGK"));
        assert_eq!(isim_coz("BB").ad, "BB", "tek kelimelik ad ayıklanmaz");
    }

    #[test]
    fn yp_eki_ve_ekuri_adi() {
        let ham = "LEONİDAS SKG SK YP";
        let i = isim_coz(ham);
        assert_eq!((i.ad.as_str(), i.kodlar.as_str(), i.kosmaz), ("LEONİDAS", "SKG SK YP", false));
        let (g, _) = ekuri_ayir("[(5)LAZMAN,(6)LEONİDAS,] eküridir.2. 3'LÜ GANYAN");
        assert_eq!(g, vec![vec![(5, "LAZMAN".to_string()), (6, "LEONİDAS".to_string())]]);
        assert_eq!(g[0][1].1, i.ad, "eküri adı, ekleri ayıklanmış program adıyla aynı olmalı");
        // Mart dosyalarındaki diğer YP örnekleri (hepsi başka kodlardan sonra geliyor)
        assert_eq!(isim_coz("KAYLA MARLA K YP").ad, "KAYLA MARLA");
        assert_eq!(isim_coz("BIG RUGGED DB SK YP").kodlar, "DB SK YP");
        // Tek kelimelik ad ayıklanmaz
        assert_eq!(isim_coz("YP").ad, "YP");
    }

    #[test]
    fn alan_cozuculer() {
        let i = isim_coz("SERİNTEPE KG DB SK (Koşmaz)");
        assert_eq!((i.ad.as_str(), i.kodlar.as_str(), i.kosmaz), ("SERİNTEPE", "KG DB SK", true));
        assert_eq!(isim_coz("GEMLIK IN KIZI SKG SK").ad, "GEMLIK IN KIZI");
        assert_eq!(kilo_coz("54 +0.50"), (Some(54.0), "+0.50".to_string()));
        assert_eq!(kilo_coz("60,5"), (Some(60.5), String::new()));
        assert_eq!(jokey_coz("M.B.ÖNCEL AP"), ("M.B.ÖNCEL".to_string(), true));
        assert_eq!(start_coz("12 - DS"), (Some(12), "DS".to_string()));
        assert_eq!(agf_coz("%2.72(7)"), (Some(2.72), Some(7)));
        assert_eq!(agf_coz(""), (None, None));
    }

    #[test]
    fn bahis_satiri() {
        let s = "GANYAN(4) :3,85 TL, 1. 3'LÜ GANYAN(5/6/1,4) :212,80 TL, ÜÇLÜ BAHİS(4/9/5) :693,40 TL Çıkan 1 Atla: 93,30 TL, SIRALI 5 Lİ BAHİS(4/9/5/8/7) :11.799,56 TL, 2. ÇİFTE(6/4) :57,50 TL";
        let (b, u) = bahis_ayristir(s);
        assert!(u.is_empty(), "{u:?}");
        assert_eq!(b.len(), 5);
        assert_eq!(b[1].oyun, "1. 3'LÜ GANYAN");
        assert_eq!(b[1].ayaklar, vec![vec![5], vec![6], vec![1, 4]]);
        assert_eq!(b[2].teselli_ham, "93,30 TL");
        assert_eq!(b[3].tutar_tl, Some(11799.56));
        assert_eq!(b[4].oyun, "2. ÇİFTE");
    }

    #[test]
    fn esnek_tutar_iki_yazim() {
        assert_eq!(esnek_tutar("1,007,862.90 TL").map(|x| x.2), Some(1007862.90));
        assert_eq!(esnek_tutar("1.007.862,90 TL").map(|x| x.2), Some(1007862.90));
        assert_eq!(esnek_tutar("522.510,25").map(|x| (x.0, x.2)), Some((10, 522510.25)));
        assert_eq!(esnek_tutar("3,95").map(|x| x.2), Some(3.95));
        assert_eq!(esnek_tutar("1.2.3"), None);
    }

    /// GERÇEK KAYIT: 06.03.2026 Bursa sonuç CSV'si, 7. koşu bahis satırının son iki oyunu.
    #[test]
    fn gercek_bursa_7li_devir_ve_ayni_satirdaki_6li_odeme() {
        let s = "6'LI GANYAN(1/2/6/1/2/8) :522.510,25 TL, 5'Lİ GANYAN(2/6/1/2/8) :66.861,90 TL, 6. ÇİFTE(2/8) :192,80 TL, 7'Lİ GANYAN(1/1/2/6/1/2/8) :Bilen çıkmamıştır, 1,007,862.90 TL devretmiştir.";
        let (b, u) = bahis_ayristir(s);
        assert!(u.is_empty(), "okunamayan parça kalmamalı: {u:?}");
        assert_eq!(b.len(), 4);
        let alti = &b[0];
        assert_eq!((alti.oyun.as_str(), alti.sonuc_durumu, alti.tutar_tl), ("6'LI GANYAN", "ODENDI", Some(522510.25)));
        let yedi = &b[3];
        assert_eq!(yedi.oyun, "7'Lİ GANYAN");
        assert_eq!(yedi.kombinasyon_ham, "1/1/2/6/1/2/8");
        assert_eq!(yedi.ayaklar.len(), 7);
        assert_eq!(yedi.sonuc_durumu, "DEVIR");
        assert_eq!(yedi.tutar_tl, None);
        assert_eq!(yedi.tutar_ham, "");
        assert_eq!(yedi.devir_ham, "1,007,862.90 TL");
        assert_eq!(yedi.devir_tl, Some(1007862.90));
        assert_eq!(yedi.ham_ibare, "7'Lİ GANYAN(1/1/2/6/1/2/8) :Bilen çıkmamıştır, 1,007,862.90 TL devretmiştir.");
    }

    /// SENTETİK TEST VERİSİ — gerçek bir TJK kaydı değildir. 6'lı ganyanın devir durumunu sınar.
    #[test]
    fn sentetik_6li_devir() {
        let s = "GANYAN(4) :9,55 TL, 6'LI GANYAN(6/1,4/6/4/7/4) :Bilen çıkmamıştır, 123.456,78 TL devretmiştir., İKİLİ(2/4) :22,30 TL";
        let (b, u) = bahis_ayristir(s);
        assert!(u.is_empty(), "{u:?}");
        assert_eq!(b.len(), 3);
        assert_eq!((b[1].sonuc_durumu, b[1].tutar_tl, b[1].devir_tl), ("DEVIR", None, Some(123456.78)));
        assert_eq!(b[1].ayaklar[1], vec![1, 4]);
        assert_eq!(b[2].oyun, "İKİLİ");
    }

    #[test]
    fn okunamayan_parca_uyariya_gider_digerleri_okunur() {
        let s = "GANYAN(4) :9,55 TL, TUHAF OYUN(1/2) :belirsiz metin, İKİLİ(2/4) :22,30 TL, sondaki çöp";
        let (b, u) = bahis_ayristir(s);
        assert_eq!(b.iter().map(|x| x.oyun.as_str()).collect::<Vec<_>>(), vec!["GANYAN", "İKİLİ"]);
        assert_eq!(u, vec!["TUHAF OYUN(1/2) :belirsiz metin".to_string(), "sondaki çöp".to_string()]);
    }

    #[test]
    fn coklu_agf() {
        assert_eq!(agf_coklu("%17.61(2)  %17.99(2)"), Some(vec![(17.61, 2), (17.99, 2)]));
        assert_eq!(agf_coklu("%2.72(7)"), Some(vec![(2.72, 7)]));
        assert_eq!(agf_coklu(""), Some(vec![]));
        assert_eq!(agf_coklu("%2.72(7) bozuk"), None);
    }

    #[test]
    fn ekuri_onekini_ayirma() {
        let (g, r) = ekuri_ayir("[(4)BÜYÜK YILDIZ,(8)TURAN YILDIZI,] eküridir.GANYAN(7) :3,05 TL");
        assert_eq!(g, vec![vec![(4, "BÜYÜK YILDIZ".to_string()), (8, "TURAN YILDIZI".to_string())]]);
        assert_eq!(r, "GANYAN(7) :3,05 TL");
        assert_eq!(bahis_ayristir(r).0[0].oyun, "GANYAN");
        let (g2, r2) = ekuri_ayir("GANYAN;İKİLİ");
        assert!(g2.is_empty());
        assert_eq!(r2, "GANYAN;İKİLİ");
    }

    #[test]
    fn ardisik_ekuri_ibareleri_ve_fazladan_kapa() {
        // GERÇEK: 09.03.2026 Şanlıurfa sonuç satır 149 (başı)
        let s = "[(1)DÖRTNALBEYİ,(9)KOŞAR ZEYNEL,] eküridir.[(6)DUMRUL],(3)ÖZNİLAY],] eküridir.[(4)KAANHAKAN],(16)OĞLUMBENİM],(12)YAROVA],] eküridir.GANYAN(4) :5,45 TL, 7. ÇİFTE(1/4,12,16) :38,20 TL";
        let (a, r) = ekuri_ayir_ayrintili(s);
        let n = |g: &Vec<(i64, String)>| g.iter().map(|x| x.0).collect::<Vec<_>>();
        assert_eq!(a.gruplar.iter().map(n).collect::<Vec<_>>(), vec![vec![1, 9], vec![6, 3], vec![4, 16, 12]]);
        assert_eq!(a.gruplar[2][2].1, "YAROVA");
        assert_eq!(a.fazla_kapa_toleransi, 5);
        assert!(a.cozulemeyen.is_empty());
        assert_eq!(r, "GANYAN(4) :5,45 TL, 7. ÇİFTE(1/4,12,16) :38,20 TL");
        assert_eq!(bahis_ayristir(r).0[0].tutar_ham, "5,45 TL");
        // Okunamayan ibare: ham hâliyle saklanır, sonraki metin yine ayrılır
        let (b, r2) = ekuri_ayir_ayrintili("[(1)A,(x)B,] eküridir.[(2)C]],] eküridir.GANYAN(1) :2,00 TL");
        assert!(b.gruplar.is_empty());
        assert_eq!(b.cozulemeyen, vec!["[(1)A,(x)B,] eküridir.".to_string(), "[(2)C]],] eküridir.".to_string()]);
        assert_eq!(r2, "GANYAN(1) :2,00 TL");
    }

    #[test]
    fn kosu_basligi_ve_at_satiri() {
        assert_eq!(kosu_basligi("2. Kosu :   19.00"), Some((2, "19:00".to_string(), String::new())));
        // GERÇEK BAŞLIKLAR: 07.03.2026 İstanbul 5. koşu ve Adana 6. koşu
        assert_eq!(
            kosu_basligi("5. Kosu :  DÜNYA KADINLAR GÜNÜ KOŞUSU 20.00"),
            Some((5, "20:00".to_string(), "DÜNYA KADINLAR GÜNÜ KOŞUSU".to_string()))
        );
        assert_eq!(
            kosu_basligi("6. Kosu : ULUSLARARASI KARACAOĞLAN KOŞUSU 17.00"),
            Some((6, "17:00".to_string(), "ULUSLARARASI KARACAOĞLAN KOŞUSU".to_string()))
        );
        assert_eq!(kosu_basligi("6. Kosu : ADI17.00"), None);
        assert!(at_satiri_mi("12;HİLAL ÇİÇEĞİ"));
        assert!(!at_satiri_mi("7'Lİ PLASE;7'Lİ GANYAN"));
        assert!(!at_satiri_mi("6. ÇİFTE Bu koşudan başlar;GANYAN"));
    }
}
