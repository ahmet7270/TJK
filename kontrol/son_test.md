# Son veri testi

- **Durum:** BAŞARILI
- **Test zamanı (UTC):** 2026-11-22
- **Denenen tarih / hipodrom:** 01.03.2026 / Adana (26. Yarış Günü)
- **Araçlar:** `tjk_ayristirici` 0.11.0 (`tjk_indir`, `tjk_ayristirici`; `cargo build --release --offline`), `tjk-fetch daralt`
- **SQLite:** kullanılmadı (`--sqlite` verilmedi, dosya oluşmadı)

## Aday liste (çevrimdışı)

- `mart_2026_resmi_takvim.tsv`: 61 satır, **61 benzersiz** tarih–hipodrom çifti
- Daha önce işlenen 06–10 Mart: **10 çift** çıkarıldı → kalan **51 çift**
- `tjk_indir liste --ay 2026-03`: 620 aday adres üretildi (indirilmedi, depoya eklenmedi)
- Tarih **ve** hipodrom eşleşmesi: 51 çift × (1 program + 1 sonuç) = **102 benzersiz URL** → `mart_2026_kalan_102.tsv`
  (adresler kalıptan üretilmiş adaydır; içerik doğrulaması yalnızca indirilen dosyalar için geçerlidir)
- Sıralamadaki ilk çift: 2026-03-01 Adana

## İndirilen dosyalar

robots.txt: `https://medya-cdn.tjk.org/robots.txt` → HTTP 404 (kural yok, indirmeye izin verildi)

| Rol | Kaynak URL | HTTP | Bayt | SHA-256 | İçerik doğrulaması |
|---|---|---|---|---|---|
| Program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-01/CSV/GunlukYarisProgrami/01.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 16347 | `a48bda6342658cecd786505694d40e619d14064610611a246c5c75ff4db7b9a8` | ICERIK_DOGRULANDI: 2026-03-01 Adana program |
| Sonuç | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-01/CSV/GunlukYarisSonuclari/01.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 16583 | `c37c3167fcae09ead0365228a8cfdd5e1459e9a2d9c791c5924af57ff9cdec77` | ICERIK_DOGRULANDI: 2026-03-01 Adana sonuc |

Her iki dosyanın ilk satırı: `Adana;(26. Yarış Günü);01/03/2026` (UTF-8 BOM'lu, CRLF, `;` ayıraç).
Rol kolon başlıklarından çıkarıldı (program: "Son 6 Yarış"; sonuç: "Derece", "Ganyan", "Fark").

## Birlikte ayrıştırma (`tjk_ayristirici PROGRAM SONUC ÇIKTI`)

- Program ve sonuç: 8 koşu, 83 at satırı; koşu başına at sayıları aynı (1:8 2:5 3:8 4:16 5:15 6:7 7:7 8:17)
- Eşleşme sorunu: yok · Eşleşmeyen sonuç satırı: yok · Çapraz alan farkı: yok
- Koşu durumları: BITIRDI=81, DERECESIZ=1 (K05 CLEAR SHOT), KOSMAZ=1 (K03 MYTHWAKE)
- Bahis satırı kontrolü: 24 kontrol, 0 uyumsuz · Derece sırası: tüm koşularda artan
- Altılı A1 (1.–6. koşu): 2/3/1,3/11/6/7, ödeme 8.315,62 TL
- Altılı A2 (3.–8. koşu): 1,3/11/6/7/5/6, ödeme 3.475,00 TL
- **GÜN DURUMU: TAMAM**, çıkış kodu 0. Hata yok.

## Eklenen ham dosyalar

- `kontrol/ham/20260301-ADANA-program.csv`
- `kontrol/ham/20260301-ADANA-sonuc.csv`
