//! Resmî yarış takvimi PDF'sinden (tek sayfa, 12 aylık ızgara) tarih–hipodrom
//! çiftlerini kelime koordinatlarıyla çıkarır. Metin `pdftotext -bbox` ile alınır.
//!
//! Izgara: her ay bloğunda satırlar haftanın günleri (Pzt..Paz), sütunlar
//! haftalardır. Gün numarası satırın hemen üstünde, hipodrom kodları (her gün
//! için en fazla iki: sol/sağ yuva) hemen altındadır. Her kod, aynı gün
//! satırındaki yatay merkezi en yakın tarihe atanır.

use std::collections::BTreeMap;
use std::process::{exit, Command};

const MONTHS: [&str; 12] = [
    "Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran", "Temmuz", "Ağustos", "Eylül", "Ekim",
    "Kasım", "Aralık",
];
const DAYS: [&str; 7] = ["Pzt", "Sal", "Çar", "Per", "Cum", "Cmt", "Paz"];
/// Takvimdeki kodlar -> Türkiye hipodromları.
const TRACKS: [(&str, &str, &str); 10] = [
    ("ADA", "ADANA", "Adana"),
    ("ANK", "ANKARA", "Ankara"),
    ("ANT", "ANTALYA", "Antalya"),
    ("BUR", "BURSA", "Bursa"),
    ("DYB", "DİYARBAKIR", "Diyarbakır"),
    ("ELZ", "ELAZIĞ", "Elazığ"),
    ("İST", "İSTANBUL", "İstanbul"),
    ("İZM", "İZMİR", "İzmir"),
    ("KOC", "KOCAELİ", "Kocaeli"),
    ("URF", "ŞANLIURFA", "Şanlıurfa"),
];
/// Hipodrom olmayan, bilinen işaretler (ızgarada yarış yerine yazılmış).
const NON_TRACK: [(&str, &str); 2] = [("RMZ", "Ramazan Bayramı"), ("KRB", "Kurban Bayramı")];

#[derive(Clone, Debug)]
struct W {
    x0: f64,
    y0: f64,
    x1: f64,
    t: String,
}
impl W {
    fn cx(&self) -> f64 {
        (self.x0 + self.x1) / 2.0
    }
}

fn words(pdf: &str) -> Vec<W> {
    let o = Command::new("pdftotext").args(["-bbox", pdf, "-"]).output().expect("pdftotext yok");
    if !o.status.success() {
        eprintln!("pdftotext hatası: {}", String::from_utf8_lossy(&o.stderr));
        exit(1);
    }
    let html = String::from_utf8_lossy(&o.stdout);
    let attr = |s: &str, k: &str| -> f64 {
        let i = s.find(&format!("{k}=\"")).unwrap() + k.len() + 2;
        s[i..].split('"').next().unwrap().parse().unwrap()
    };
    html.lines()
        .filter_map(|l| {
            let l = l.trim();
            let rest = l.strip_prefix("<word ")?;
            let (a, t) = rest.split_once('>')?;
            let t = t.strip_suffix("</word>")?;
            let t = t.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">");
            Some(W { x0: attr(a, "xMin"), y0: attr(a, "yMin"), x1: attr(a, "xMax"), t })
        })
        .collect()
}

fn is_code(t: &str) -> bool {
    t.chars().count() == 3 && t.chars().all(|c| c.is_alphabetic() && c.is_uppercase())
}

/// 0 = Pazartesi (Sakamoto).
fn weekday(y: i32, m: u32, d: u32) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { y - 1 } else { y };
    let w = (y + y / 4 - y / 100 + y / 400 + t[m as usize - 1] + d as i32) % 7; // 0=Pazar
    ((w + 6) % 7) as usize
}

fn days_in(y: i32, m: u32) -> u32 {
    let artik = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    match m {
        2 if artik => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Eski kullanım: 2026 takvimi, yalnızca Mart çıktısı.
pub fn run(pdf: &str, out: &str) {
    run_aralik(pdf, out, 2026, "2026-03-01", "2026-03-31")
}

/// `yil` takviminden [bas, bit] (YYYY-AA-GG, dahil) aralığındaki Türkiye çiftlerini yazar.
pub fn run_aralik(pdf: &str, out: &str, year: i32, bas: &str, bit: &str) {
    let ws = words(pdf);
    let mut errors: Vec<String> = Vec::new();

    // Ay başlıkları ve blok sınırları.
    let titles: Vec<(u32, &W)> = MONTHS
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let w = ws.iter().find(|w| w.t == *m).unwrap_or_else(|| panic!("ay başlığı yok: {m}"));
            (i as u32 + 1, w)
        })
        .collect();
    let mut col_x: Vec<f64> = ws.iter().filter(|w| w.t == "Pzt").map(|w| w.x0).collect();
    col_x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    col_x.dedup_by(|a, b| (*a - *b).abs() < 5.0);
    let mut row_y: Vec<f64> = titles.iter().map(|(_, w)| w.y0).collect();
    row_y.sort_by(|a, b| a.partial_cmp(b).unwrap());
    row_y.dedup_by(|a, b| (*a - *b).abs() < 5.0);
    println!("Izgara: {} sütun x={col_x:.1?}, {} satır y={row_y:.1?}", col_x.len(), row_y.len());

    // (ay, gün) -> [(x ofseti, kod)]
    let mut cal: BTreeMap<(u32, u32), Vec<(f64, String, i64)>> = BTreeMap::new();
    for (m, tw) in &titles {
        let ci = col_x.iter().rposition(|&x| x <= tw.cx()).unwrap();
        let ri = row_y.iter().position(|&y| (y - tw.y0).abs() < 5.0).unwrap();
        let (bx0, bx1) = (col_x[ci] - 1.0, col_x.get(ci + 1).copied().unwrap_or(f64::MAX) - 1.0);
        let (by0, by1) = (row_y[ri], row_y.get(ri + 1).copied().unwrap_or(f64::MAX));
        let blk: Vec<&W> =
            ws.iter().filter(|w| w.x0 >= bx0 && w.x0 < bx1 && w.y0 > by0 + 1.0 && w.y0 < by1 - 1.0).collect();
        let mut seen_days = 0;
        for (di, dn) in DAYS.iter().enumerate() {
            let Some(lab) = blk.iter().find(|w| w.t == *dn) else {
                errors.push(format!("{}: {dn} etiketi yok", MONTHS[*m as usize - 1]));
                continue;
            };
            let dates: Vec<(u32, f64)> = blk
                .iter()
                .filter(|w| w.y0 > lab.y0 - 12.0 && w.y0 < lab.y0 && w.x0 > lab.x1)
                .filter_map(|w| w.t.parse::<u32>().ok().map(|d| (d, w.cx())))
                .collect();
            let codes: Vec<&&W> =
                blk.iter().filter(|w| w.y0 > lab.y0 && w.y0 < lab.y0 + 14.0 && is_code(&w.t)).collect();
            for &(d, _) in &dates {
                seen_days += 1;
                if d < 1 || d > days_in(year, *m) {
                    errors.push(format!("{m}/{d}: geçersiz gün"));
                } else if weekday(year, *m, d) != di {
                    errors.push(format!("{d:02}.{m:02}: '{dn}' satırında ama gerçek gün {}", DAYS[weekday(year, *m, d)]));
                }
                cal.entry((*m, d)).or_default();
            }
            for c in codes {
                let best = dates
                    .iter()
                    .map(|&(d, x)| (d, c.cx() - x))
                    .min_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap());
                match best {
                    Some((d, off)) if off.abs() <= 16.0 => {
                        if !(5.0..=15.0).contains(&off.abs()) {
                            errors.push(format!("{d:02}.{m:02} {}: yuva ofseti şüpheli ({off:.1})", c.t));
                        }
                        cal.entry((*m, d)).or_default().push((off, c.t.clone(), (c.y0 - lab.y0).round() as i64));
                    }
                    other => errors.push(format!(
                        "{} {dn}: '{}' (x={:.1}) hiçbir tarihe hizalanamadı ({other:?})",
                        MONTHS[*m as usize - 1],
                        c.t,
                        c.cx()
                    )),
                }
            }
        }
        if seen_days != days_in(year, *m) {
            errors.push(format!("{}: {} gün bulundu, {} olmalı", MONTHS[*m as usize - 1], seen_days, days_in(year, *m)));
        }
    }
    // Her hücre satırında (dikey konum) en fazla bir sol, bir sağ yuva.
    let mut stacked = 0;
    for ((m, d), v) in &cal {
        let mut slots: Vec<(bool, i64)> = v.iter().map(|(o, _, y)| (*o < 0.0, *y)).collect();
        let n = slots.len();
        slots.sort();
        slots.dedup();
        if slots.len() != n {
            errors.push(format!("{d:02}.{m:02}: yuva çakışması {v:?}"));
        }
        if v.iter().any(|(_, _, y)| *y != v[0].2) {
            stacked += 1;
        }
    }
    println!("Alt alta iki kod satırı olan gün sayısı (tüm yıl): {stacked}");

    // Yıllık sayımları PDF başlığındaki "2026 TOPLAM" değerleriyle karşılaştır.
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for v in cal.values() {
        for (_, c, _) in v {
            *counts.entry(c.clone()).or_default() += 1;
        }
    }
    let header_y = row_y[0] - 5.0;
    let head: Vec<&W> = ws.iter().filter(|w| w.y0 < header_y).collect();
    let mut grand_hdr = None;
    let mut sum_counts = 0;
    println!("\nYıllık doğrulama (ızgara sayımı vs PDF başlığı):");
    for (code, upper, _) in TRACKS {
        let n = counts.get(code).copied().unwrap_or(0);
        sum_counts += n;
        let lab = head.iter().find(|w| w.t == upper);
        let total = lab.and_then(|lab| {
            let top = head
                .iter()
                .filter(|w| w.t == "TOPLAM" && w.y0 > lab.y0 && w.y0 < lab.y0 + 70.0 && (w.x0 - lab.x0).abs() < 40.0)
                .min_by(|a, b| a.y0.partial_cmp(&b.y0).unwrap())?;
            head.iter()
                .filter(|w| (w.y0 - top.y0).abs() < 2.0 && w.x0 > top.x1 && w.x0 < top.x1 + 90.0)
                .filter_map(|w| w.t.parse::<usize>().ok().map(|n| (w.x0, n)))
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .map(|(_, n)| n)
        });
        let ok = total == Some(n);
        println!("  {code} {upper:<11} ızgara={n:>3} başlık={:>4} {}", total.map_or("?".into(), |t| t.to_string()), if ok { "OK" } else { "UYUŞMAZ" });
        if !ok {
            errors.push(format!("{code}: ızgara {n} != başlık {total:?}"));
        }
    }
    if let Some(i) = head.iter().position(|w| w.t == "TOPLAM:") {
        grand_hdr = head[i + 1..].iter().find_map(|w| w.t.parse::<usize>().ok());
    }
    println!("  GENEL TOPLAM ızgara={sum_counts} başlık={grand_hdr:?}");
    if grand_hdr != Some(sum_counts) {
        errors.push(format!("genel toplam {sum_counts} != {grand_hdr:?}"));
    }
    for (code, n) in &counts {
        if !TRACKS.iter().any(|t| t.0 == code) {
            let what = NON_TRACK.iter().find(|t| t.0 == code).map(|t| t.1);
            match what {
                Some(w) => println!("  {code}: {n} adet, hipodrom değil ({w}) -> hariç"),
                None => errors.push(format!("bilinmeyen kod {code} ({n} adet)")),
            }
        }
    }

    // Aralık çıktısı (sıra: tarih, sonra sol yuva -> sağ yuva).
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    println!("\n{bas} – {bit} (gün: sol yuva | sağ yuva):");
    for (&(m, d), v0) in &cal {
        let iso = format!("{year}-{m:02}-{d:02}");
        if iso.as_str() < bas || iso.as_str() > bit {
            continue;
        }
        let mut v = v0.clone();
        v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let cs: Vec<&str> = v.iter().map(|(_, c, _)| c.as_str()).collect();
        if !cs.is_empty() {
            println!("  {d:02}.{m:02} {} {}", DAYS[weekday(year, m, d)], cs.join(" | "));
        }
        for (_, c, _) in &v {
            match TRACKS.iter().find(|t| t.0 == c) {
                Some(t) => rows.push(format!("{iso}\t{}", t.2)),
                None => skipped.push(format!("{d:02}.{m:02} {c}")),
            }
        }
    }
    if !errors.is_empty() {
        eprintln!("\nHİZALAMA/DOĞRULAMA HATALARI ({}):", errors.len());
        for e in &errors {
            eprintln!("  - {e}");
        }
        eprintln!("TSV yazılmadı.");
        exit(5);
    }
    let mut tsv = String::from("tarih\thipodrom\n");
    for r in &rows {
        tsv.push_str(r);
        tsv.push('\n');
    }
    std::fs::write(out, tsv).expect("TSV yazılamadı");
    println!("\nHizalama doğrulaması: hata yok.");
    println!("Hariç tutulan (hipodrom değil): {skipped:?}");
    println!("{out}: {} tarih–hipodrom çifti yazıldı.", rows.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn weekday_known() {
        assert_eq!(weekday(2026, 3, 6), 4); // Cuma
        assert_eq!(weekday(2026, 1, 1), 3); // Perşembe
        assert_eq!(weekday(2026, 3, 1), 6); // Pazar
    }
}
