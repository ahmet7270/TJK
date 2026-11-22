//! Resmî takvim çiftlerinden işlenmiş günleri çıkarır ve `tjk_indir liste` çıktısını
//! kalan çiftlere (tarih VE hipodrom) daraltır. Her sayı beklenenle tutmazsa durur.

use std::collections::{BTreeMap, BTreeSet};
use std::process::exit;

fn dur(msg: String) -> ! {
    eprintln!("DUR: {msg}");
    exit(6)
}

fn tsv(path: &str) -> (String, Vec<Vec<String>>) {
    let t = std::fs::read_to_string(path).unwrap_or_else(|e| dur(format!("{path}: {e}")));
    let mut it = t.lines();
    let head = it.next().unwrap_or("").to_string();
    let rows = it.filter(|l| !l.trim().is_empty()).map(|l| l.split('\t').map(str::to_string).collect()).collect();
    (head, rows)
}

/// %XX kodlu URL parçasını çözer.
fn yuzde_coz(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// args: takvim.tsv liste.tsv haric_ilk(YYYY-AA-GG) haric_son beklenen_toplam beklenen_haric cikti.tsv
pub fn run(a: &[String]) {
    let [takvim, liste, h0, h1, bt, bh, cikti] = a else {
        eprintln!("kullanım: tjk-fetch daralt <takvim.tsv> <liste.tsv> <hariç_ilk> <hariç_son> <beklenen_çift> <beklenen_hariç> <çıktı.tsv>");
        exit(2)
    };
    let (beklenen_toplam, beklenen_haric): (usize, usize) = (bt.parse().unwrap(), bh.parse().unwrap());

    // 1) Takvim: benzersiz çiftler
    let (th, trows) = tsv(takvim);
    if th != "tarih\thipodrom" {
        dur(format!("{takvim}: beklenmeyen başlık {th:?}"));
    }
    let mut ciftler = BTreeSet::new();
    for r in &trows {
        if r.len() != 2 {
            dur(format!("{takvim}: 2 alanlı olmayan satır {r:?}"));
        }
        if !ciftler.insert((r[0].clone(), r[1].clone())) {
            dur(format!("{takvim}: yinelenen çift {r:?}"));
        }
    }
    println!("Takvim: {} satır, {} benzersiz tarih–hipodrom çifti", trows.len(), ciftler.len());
    if ciftler.len() != beklenen_toplam {
        dur(format!("benzersiz çift sayısı {} (beklenen {beklenen_toplam})", ciftler.len()));
    }
    let (haric, kalan): (BTreeSet<_>, BTreeSet<_>) =
        ciftler.into_iter().partition(|(t, _)| t.as_str() >= h0.as_str() && t.as_str() <= h1.as_str());
    println!("Daha önce işlenen ({h0} … {h1}): {} çift", haric.len());
    for (t, h) in &haric {
        println!("  - {t} {h}");
    }
    if haric.len() != beklenen_haric {
        dur(format!("hariç tutulan çift sayısı {} (beklenen {beklenen_haric})", haric.len()));
    }
    println!("Kalan: {} çift", kalan.len());

    // 2) Aday liste: tarih + hipodrom tam eşleşmesi
    let (lh, lrows) = tsv(liste);
    let cols: Vec<&str> = lh.split('\t').collect();
    let ix = |n: &str| cols.iter().position(|c| *c == n).unwrap_or_else(|| dur(format!("{liste}: '{n}' sütunu yok")));
    let (it, ih, ir, iu) = (ix("tarih"), ix("hipodrom"), ix("rol"), ix("url"));
    println!("Aday liste: {} satır (çevrimdışı üretildi; indirilmedi)", lrows.len());
    let mut secilen: BTreeMap<(String, String), BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let mut satirlar: Vec<&Vec<String>> = Vec::new();
    for r in &lrows {
        if r.len() != cols.len() {
            dur(format!("{liste}: alan sayısı {} != {}: {r:?}", r.len(), cols.len()));
        }
        let k = (r[it].clone(), r[ih].clone());
        if kalan.contains(&k) {
            // URL kendi içinde de aynı tarihi ve hipodromu taşımalı
            let u = yuzde_coz(&r[iu]);
            let g: Vec<&str> = k.0.split('-').collect();
            let nokta = format!("{}.{}.{}", g[2], g[1], g[0]);
            if !u.contains(&format!("/{}/", k.0)) || !u.contains(&format!("/{nokta}-{}-", k.1)) {
                dur(format!("URL tarih/hipodromla tutarsız: {} ({:?})", r[iu], k));
            }
            secilen.entry(k).or_default().entry(r[ir].clone()).or_default().push(r[iu].clone());
            satirlar.push(r);
        }
    }
    // Her çift: tam bir program + bir sonuç
    for k in &kalan {
        let roller = secilen.get(k).cloned().unwrap_or_default();
        let p = roller.get("program").map_or(0, Vec::len);
        let s = roller.get("sonuc").map_or(0, Vec::len);
        if p != 1 || s != 1 || roller.len() != 2 {
            dur(format!("{} {}: program={p}, sonuç={s}, roller={:?}", k.0, k.1, roller.keys().collect::<Vec<_>>()));
        }
    }
    let urller: BTreeSet<&String> = satirlar.iter().map(|r| &r[iu]).collect();
    println!(
        "Eşleşen: {} çift, {} satır, {} benzersiz URL (program {}, sonuç {})",
        secilen.len(),
        satirlar.len(),
        urller.len(),
        satirlar.iter().filter(|r| r[ir] == "program").count(),
        satirlar.iter().filter(|r| r[ir] == "sonuc").count()
    );
    if secilen.len() != kalan.len() || satirlar.len() != 2 * kalan.len() || urller.len() != satirlar.len() {
        dur("eşleşen URL sayısı çift sayısının iki katı değil ya da yinelenen URL var".into());
    }
    // Tarih, hipodrom, rol (program önce) sırasıyla yaz; biçim tjk_indir listesiyle aynı.
    satirlar.sort_by(|a, b| (&a[it], &a[ih], &a[ir] != "program").cmp(&(&b[it], &b[ih], &b[ir] != "program")));
    let mut out = lh.clone() + "\n";
    for r in &satirlar {
        out += &r.join("\t");
        out.push('\n');
    }
    std::fs::write(cikti, out).unwrap_or_else(|e| dur(format!("{cikti}: {e}")));
    println!("{cikti}: {} aday URL yazıldı", satirlar.len());
    println!("İlk çift: {} {}", satirlar[0][it], satirlar[0][ih]);
}
