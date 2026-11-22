//! Regresyon karşılaştırması: iki çıktı klasörü ağacındaki (gün başına bir alt klasör) program.csv ve
//! sonuclar.csv dosyalarını alan düzeyinde karşılaştırır. Her farkı (gün, dosya, kayıt, sütun, eski, yeni)
//! TSV olarak yazar ve sütun başına özet verir. altili.json için yalnızca aynı/farklı bilgisi verilir.

use std::collections::BTreeMap;
use std::process::exit;

/// Tırnaklı alanları destekleyen basit CSV okuyucu (csvw.rs çıktısı: ',' ayıraç, CRLF, "" kaçışı).
fn csv_oku(yol: &str) -> Vec<Vec<String>> {
    let t = std::fs::read_to_string(yol).unwrap_or_else(|e| {
        eprintln!("{yol}: {e}");
        exit(2)
    });
    let mut satirlar = Vec::new();
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
                satirlar.push(std::mem::take(&mut satir));
            }
            (c, _) => alan.push(c),
        }
    }
    if !alan.is_empty() || !satir.is_empty() {
        satir.push(alan);
        satirlar.push(satir);
    }
    satirlar
}

/// args: eski_kok yeni_kok cikti.tsv
pub fn run(a: &[String]) {
    let [eski, yeni, cikti] = a else {
        eprintln!("kullanım: tjk-fetch karsilastir <eski_kök> <yeni_kök> <farklar.tsv>");
        exit(2)
    };
    let mut gunler: Vec<String> = std::fs::read_dir(eski)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    gunler.sort();
    let mut out = String::from("gun\tdosya\tkayit\tsutun\teski\tyeni\n");
    let mut ozet: BTreeMap<(String, String), (usize, Vec<String>)> = BTreeMap::new();
    let (mut ayni_gun, mut json_farkli) = (0, Vec::new());
    for g in &gunler {
        let mut gun_farki = false;
        for d in ["program.csv", "sonuclar.csv"] {
            let (e, y) = (csv_oku(&format!("{eski}/{g}/{d}")), csv_oku(&format!("{yeni}/{g}/{d}")));
            if e.first() != y.first() {
                println!("{g} {d}: BAŞLIK FARKLI");
                gun_farki = true;
                continue;
            }
            if e.len() != y.len() {
                println!("{g} {d}: SATIR SAYISI {} -> {}", e.len(), y.len());
                gun_farki = true;
            }
            let bas = &e[0];
            for (re, ry) in e.iter().zip(&y).skip(1) {
                for (i, col) in bas.iter().enumerate() {
                    let (ve, vy) = (re.get(i).map_or("", String::as_str), ry.get(i).map_or("", String::as_str));
                    if ve != vy {
                        gun_farki = true;
                        out.push_str(&format!("{g}\t{d}\t{}\t{col}\t{ve}\t{vy}\n", re[0]));
                        let o = ozet.entry((d.to_string(), col.clone())).or_default();
                        o.0 += 1;
                        if !o.1.contains(g) {
                            o.1.push(g.clone());
                        }
                    }
                }
            }
        }
        let je = std::fs::read(format!("{eski}/{g}/altili.json")).unwrap_or_default();
        let jy = std::fs::read(format!("{yeni}/{g}/altili.json")).unwrap_or_default();
        if je != jy {
            json_farkli.push(g.clone());
            gun_farki = true;
        }
        if !gun_farki {
            ayni_gun += 1;
        }
    }
    std::fs::write(cikti, out).unwrap();
    println!("Gün: {}, program.csv + sonuclar.csv + altili.json tamamen aynı: {ayni_gun}", gunler.len());
    println!("Sütun bazında farklar:");
    for ((d, c), (n, g)) in &ozet {
        println!("  {d} / {c}: {n} hücre, günler {g:?}");
    }
    println!("altili.json farklı: {json_farkli:?}");
}
