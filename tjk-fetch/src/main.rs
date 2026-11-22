//! TJK arşiv erişim testi: robots.txt kontrolü, curl ile indirme,
//! SHA-256 doğrulama, CSV inceleme ve resmî takvimden Mart çiftleri.

mod takvim;

use sha2::{Digest, Sha256};
use std::process::{exit, Command};

fn usage() -> ! {
    eprintln!(
        "kullanım:\n  tjk-fetch robots <url> <yol>...\n  tjk-fetch fetch <url> <çıktı>\n  \
         tjk-fetch csv <dosya>\n  tjk-fetch takvim <pdf> <çıktı.tsv>"
    );
    exit(2)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("robots") if args.len() >= 3 => robots(&args[1], &args[2..]),
        Some("fetch") if args.len() == 3 => fetch(&args[1], &args[2]),
        Some("csv") if args.len() == 2 => csv(&args[1]),
        Some("takvim") if args.len() == 3 => takvim::run(&args[1], &args[2]),
        _ => usage(),
    }
}

/// curl ile indirir; (HTTP kodu, curl stderr) döner.
fn curl(url: &str, out: &str) -> (u16, String) {
    let o = Command::new("curl")
        .args(["-sS", "-L", "--compressed", "-o", out, "-w", "%{http_code}", url])
        .output()
        .expect("curl çalıştırılamadı");
    let code = String::from_utf8_lossy(&o.stdout).trim().parse().unwrap_or(0);
    let err = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        eprintln!("HATA: curl çıkış kodu {:?}: {}", o.status.code(), err.trim());
        exit(1);
    }
    (code, err)
}

fn stop_on_block(code: u16, body: &[u8]) {
    if matches!(code, 403 | 429 | 502 | 503 | 504) {
        eprintln!("DUR: HTTP {code}. Yanıt gövdesi:\n{}", String::from_utf8_lossy(body));
        exit(3);
    }
}

fn robots(url: &str, paths: &[String]) {
    let tmp = std::env::temp_dir().join("tjk_robots.txt");
    let tmp = tmp.to_str().unwrap();
    let (code, _) = curl(url, tmp);
    let body = std::fs::read(tmp).unwrap_or_default();
    println!("robots.txt HTTP {code}, {} bayt", body.len());
    stop_on_block(code, &body);
    if (400..500).contains(&code) {
        // RFC 9309 §2.3.1.3: 4xx "unavailable" => tarayıcı kısıtsız erişebilir.
        println!("robots.txt yok (4xx) -> RFC 9309'a göre tüm yollar serbest.");
        for p in paths {
            println!("  {p}: İZİN VERİLİYOR (kural yok)");
        }
        return;
    }
    let text = String::from_utf8_lossy(&body);
    // User-agent: * grubundaki Allow/Disallow kurallarına göre en uzun eşleşme.
    let mut in_star = false;
    let mut rules: Vec<(bool, String)> = Vec::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap().trim();
        let Some((k, v)) = line.split_once(':') else { continue };
        let (k, v) = (k.trim().to_ascii_lowercase(), v.trim().to_string());
        match k.as_str() {
            "user-agent" => in_star = v == "*",
            "allow" if in_star => rules.push((true, v)),
            "disallow" if in_star && !v.is_empty() => rules.push((false, v)),
            _ => {}
        }
    }
    println!("User-agent: * kuralları: {rules:?}");
    for p in paths {
        let best = rules.iter().filter(|(_, r)| p.starts_with(r.as_str())).max_by_key(|(a, r)| (r.len(), *a));
        match best {
            Some((false, r)) => {
                eprintln!("DUR: {p} robots.txt tarafından engelleniyor (Disallow: {r})");
                exit(4);
            }
            Some((true, r)) => println!("  {p}: İZİN VERİLİYOR (Allow: {r})"),
            None => println!("  {p}: İZİN VERİLİYOR (eşleşen kural yok)"),
        }
    }
}

fn fetch(url: &str, out: &str) {
    let (code, _) = curl(url, out);
    let body = std::fs::read(out).expect("indirilen dosya okunamadı");
    println!("URL: {url}");
    println!("HTTP durum kodu: {code}");
    stop_on_block(code, &body);
    if code != 200 {
        eprintln!("HATA: beklenmeyen HTTP {code}");
        exit(1);
    }
    println!("Bayt sayısı: {}", body.len());
    println!("SHA-256: {}", hex(&Sha256::digest(&body)));
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// UTF-8 (BOM'lu/BOM'suz) değilse Windows-1254 olarak çözer.
pub fn decode(raw: &[u8]) -> (String, &'static str) {
    let raw = raw.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(raw);
    match std::str::from_utf8(raw) {
        Ok(s) => (s.to_string(), "UTF-8"),
        Err(_) => (encoding_rs::WINDOWS_1254.decode(raw).0.into_owned(), "Windows-1254"),
    }
}

fn csv(path: &str) {
    let raw = std::fs::read(path).expect("CSV okunamadı");
    let (text, enc) = decode(&raw);
    println!("Kodlama: {enc}");
    let lines: Vec<&str> = text.lines().collect();
    println!("Satır sayısı: {}", lines.len());
    println!("İlk satır: {}", lines.first().copied().unwrap_or(""));
    // İlk "At İsmi" başlıklı tablo başlığını bul, altındaki ilk veri satırından at adını al.
    for (i, l) in lines.iter().enumerate() {
        let cols: Vec<&str> = l.split(';').map(str::trim).collect();
        if let Some(ci) = cols.iter().position(|c| *c == "At İsmi" || *c == "At Adı") {
            println!("Başlık satırı {}: {}", i + 1, l);
            if let Some(row) = lines[i + 1..].iter().find(|r| !r.trim().is_empty()) {
                let rc: Vec<&str> = row.split(';').map(str::trim).collect();
                println!("İlk koşunun ilk veri satırı: {row}");
                println!("İlk koşudaki bir at: {}", rc.get(ci).copied().unwrap_or("?"));
            }
            return;
        }
    }
    println!("UYARI: 'At İsmi' başlığı bulunamadı; ilk 15 satır:");
    for l in lines.iter().take(15) {
        println!("  {l}");
    }
}
