//! Dosya kodlaması: UTF-8 (BOM'lu/BOM'suz) ve geçersiz UTF-8 için Windows-1254 (Türkçe) geri dönüşü.

const CP1254_80_9F: [char; 32] = [
    '€', '\u{FFFD}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{FFFD}', '\u{FFFD}', '\u{FFFD}',
    '\u{FFFD}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\u{FFFD}', '\u{FFFD}', 'Ÿ',
];

pub fn cp1254(b: &[u8]) -> String {
    b.iter()
        .map(|&x| match x {
            0x80..=0x9F => CP1254_80_9F[(x - 0x80) as usize],
            0xD0 => 'Ğ',
            0xDD => 'İ',
            0xDE => 'Ş',
            0xF0 => 'ğ',
            0xFD => 'ı',
            0xFE => 'ş',
            _ => x as char, // 0x00–0x7F ASCII, kalan 0xA0–0xFF Latin-1 ile aynı
        })
        .collect()
}

/// (metin, "UTF-8 (BOM'lu), CRLF, ';' ayıraç" gibi açıklama)
pub fn coz(b: &[u8]) -> (String, String) {
    let bom = b.starts_with(&[0xEF, 0xBB, 0xBF]);
    let govde = if bom { &b[3..] } else { b };
    let (metin, ad) = match std::str::from_utf8(govde) {
        Ok(s) => (s.to_string(), if bom { "UTF-8 (BOM'lu)" } else { "UTF-8" }),
        Err(_) => (cp1254(govde), "Windows-1254"),
    };
    let satir_sonu = if metin.contains("\r\n") { "CRLF" } else { "LF" };
    (metin.clone(), format!("{ad}, {satir_sonu}, ';' ayıraç"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_1254_turkce() {
        assert_eq!(cp1254(&[0xDD, b'z', b'm', b'i', b'r']), "İzmir");
        assert_eq!(cp1254(&[0xDE, 0xFD, 0xF0, 0xFE, 0xD0]), "ŞığşĞ");
        assert_eq!(cp1254(&[0xC7, 0xD6, 0xDC, 0xE7, 0xF6, 0xFC]), "ÇÖÜçöü");
    }

    #[test]
    fn utf8_bom_ve_satir_sonu() {
        let (m, a) = coz("\u{FEFF}İzmir;x\r\ny".as_bytes());
        assert_eq!(m, "İzmir;x\r\ny");
        assert_eq!(a, "UTF-8 (BOM'lu), CRLF, ';' ayıraç");
        let (_, a2) = coz(&[0xDD, b'z', b'\n']);
        assert_eq!(a2, "Windows-1254, LF, ';' ayıraç");
    }
}
