//! CSV yazıcı: ',' ayıraç, gerektiğinde çift tırnak, CRLF satır sonu, UTF-8.

use crate::json::Json;
use std::fs;

pub type Satir = Vec<(&'static str, Json)>;

pub fn hucre(s: &str) -> String {
    if s.contains([',', '"', '\r', '\n']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn yaz(yol: &str, satirlar: &[Satir]) -> Result<(), String> {
    let ilk = satirlar.first().ok_or_else(|| format!("{yol}: yazılacak satır yok"))?;
    let basliklar: Vec<&str> = ilk.iter().map(|(k, _)| *k).collect();
    let mut out = String::new();
    out.push_str(&basliklar.iter().map(|k| hucre(k)).collect::<Vec<_>>().join(","));
    out.push_str("\r\n");
    for (n, s) in satirlar.iter().enumerate() {
        let anahtarlar: Vec<&str> = s.iter().map(|(k, _)| *k).collect();
        if anahtarlar != basliklar {
            return Err(format!("{yol}: {n}. satırın sütunları başlıkla uyuşmuyor"));
        }
        out.push_str(&s.iter().map(|(_, v)| hucre(&v.hucre())).collect::<Vec<_>>().join(","));
        out.push_str("\r\n");
    }
    fs::write(yol, out).map_err(|e| format!("{yol} yazılamadı: {e}"))
}

#[cfg(test)]
mod tests {
    use super::hucre;

    #[test]
    fn tirnaklama() {
        assert_eq!(hucre("GANYAN:UYUMLU, ÜÇLÜ BAHİS:UYUMLU"), "\"GANYAN:UYUMLU, ÜÇLÜ BAHİS:UYUMLU\"");
        assert_eq!(hucre("a\"b"), "\"a\"\"b\"");
        assert_eq!(hucre("6'LI GANYAN Bu koşudan başlar | GANYAN"), "6'LI GANYAN Bu koşudan başlar | GANYAN");
    }
}
