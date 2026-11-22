//! Doğrulanmış program–sonuç çiftlerinden SQLite aktarım girdisi üretir (`aktar.sh` biçimi):
//! `gun_id<TAB>program<TAB>sonuc<TAB>ek` (dosya adları --ham klasörüne göre; `ek` = "--ek-sonuc DOSYA|not" ya da "|not").
//!
//! Bir çift ancak iki dosyası da diskte varsa ve SHA-256'ları indirme kaydındaki son ICERIK_DOGRULANDI satırıyla
//! (ya da listedeki "elde olan ham dosya" SHA'sıyla) eşleşiyorsa yazılır. `--ozel` dosyasındaki satırlar
//! (aynı biçim) üretilen satırın yerine geçer ya da eksik çifti tamamlar (ertelenen koşular, başka tarihli sonuç).
//!
//! Kullanım: tjk-fetch girdiler --liste L.tsv --durum D.tsv --ham KLASOR --cikti G.tsv [--ozel O.tsv]

use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::process::exit;

fn dur(m: String) -> ! {
    eprintln!("DUR: {m}");
    exit(6)
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

pub fn run(a: &[String]) {
    let mut m: HashMap<String, String> = HashMap::new();
    let mut it = a.iter();
    while let Some(x) = it.next() {
        let k = x.strip_prefix("--").unwrap_or_else(|| dur(format!("beklenmeyen argüman {x}")));
        m.insert(k.into(), it.next().unwrap_or_else(|| dur(format!("--{k} değer bekliyor"))).clone());
    }
    let g = |k: &str| m.get(k).cloned().unwrap_or_else(|| dur(format!("--{k} gerekli")));
    let (liste, durum, ham, cikti) = (g("liste"), g("durum"), g("ham"), g("cikti"));

    // son kayıt: url -> (dogrulama, sha)
    let mut son: HashMap<String, (String, String, String)> = HashMap::new();
    if let Ok(t) = std::fs::read_to_string(&durum) {
        for l in t.lines().skip(1) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() >= 10 {
                son.insert(c[4].into(), (c[8].into(), c[7].into(), c[5].into()));
            }
        }
    }
    let t = std::fs::read_to_string(&liste).unwrap_or_else(|e| dur(format!("{liste}: {e}")));
    let mut lines = t.lines();
    let head: Vec<&str> = lines.next().unwrap_or("").split('\t').collect();
    let ix = |n: &str| head.iter().position(|c| *c == n).unwrap_or_else(|| dur(format!("{liste}: '{n}' yok")));
    let (it_, ih, ir, iu, iy, id, ish, ik) =
        (ix("tarih"), ix("hipodrom"), ix("rol"), ix("url"), ix("yerel_dosya"), ix("url_durumu"), ix("sha256"), ix("kanit"));
    // (tarih, hipodrom) -> rol -> (dosya, durum açıklaması)
    let mut ciftler: BTreeMap<(String, String), BTreeMap<String, Result<String, String>>> = BTreeMap::new();
    for l in lines.filter(|l| !l.trim().is_empty()) {
        let c: Vec<&str> = l.split('\t').collect();
        let (dosya, beklenen_sha) = if c[id] == "ICERIK_DOGRULANDI" {
            match c[ik].strip_prefix("elde olan ham dosya: ") {
                Some(ad) => (ad.to_string(), Some(c[ish].to_string())),
                None => (c[iy].to_string(), None),
            }
        } else {
            (c[iy].to_string(), None)
        };
        let sonuc = match (beklenen_sha, son.get(c[iu])) {
            (Some(sha), _) => Ok((dosya, sha)),
            (None, Some((d, sha, _))) if d == "ICERIK_DOGRULANDI" => Ok((dosya, sha.clone())),
            (None, Some((d, _, http))) => Err(format!("{d} (HTTP {http})")),
            (None, None) => Err("indirme kaydı yok".into()),
        };
        let sonuc = sonuc.and_then(|(dosya, sha)| {
            let b = std::fs::read(Path::new(&ham).join(&dosya)).map_err(|e| format!("{dosya}: {e}"))?;
            let h: String = Sha256::digest(&b).iter().map(|x| format!("{x:02x}")).collect();
            if h == sha {
                Ok(dosya)
            } else {
                Err(format!("{dosya}: SHA-256 kayıtla uyuşmuyor"))
            }
        });
        ciftler.entry((c[it_].into(), c[ih].into())).or_default().insert(c[ir].into(), sonuc);
    }
    let mut ozel: BTreeMap<String, String> = BTreeMap::new();
    if let Some(o) = m.get("ozel") {
        if let Ok(t) = std::fs::read_to_string(o) {
            for l in t.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
                let gid = l.split('\t').next().unwrap().to_string();
                if l.split('\t').count() != 4 {
                    dur(format!("{o}: 4 alanlı olmayan satır: {l}"));
                }
                ozel.insert(gid, l.to_string());
            }
        }
    }
    let mut out = String::new();
    let (mut yazilan, mut atlanan, mut ozel_kul) = (0, 0, 0);
    for ((t, h), roller) in &ciftler {
        let gid = format!("{}-{}", t.replace('-', ""), slug(h));
        if let Some(l) = ozel.remove(&gid) {
            out += &format!("{l}\n");
            ozel_kul += 1;
            yazilan += 1;
            println!("  {gid}: özel girdi kullanıldı");
            continue;
        }
        match (roller.get("program"), roller.get("sonuc")) {
            (Some(Ok(p)), Some(Ok(s))) => {
                out += &format!("{gid}\t{p}\t{s}\t\n");
                yazilan += 1;
            }
            (p, s) => {
                atlanan += 1;
                let f = |x: Option<&Result<String, String>>| match x {
                    Some(Ok(d)) => format!("doğrulandı ({d})"),
                    Some(Err(e)) => e.clone(),
                    None => "listede yok".into(),
                };
                println!("  {gid}: AKTARILMAYACAK — program: {}; sonuç: {}", f(p), f(s));
            }
        }
    }
    for (gid, l) in &ozel {
        dur(format!("özel girdi listede olmayan güne ait: {gid} ({l})"));
    }
    std::fs::write(&cikti, out).unwrap_or_else(|e| dur(format!("{cikti}: {e}")));
    println!("girdiler: {} çift, {yazilan} yazıldı ({ozel_kul} özel), {atlanan} aktarılmayacak -> {cikti}", ciftler.len());
}
