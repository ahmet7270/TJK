# Son veri testi

- **Durum:** BAŞARILI
- **Test zamanı (UTC):** 2026-11-22
- **Denenen tarih / hipodrom:** 06.03.2026 / İzmir (29. Yarış Günü)
- **Araç:** `tjk-fetch` (Rust; aktarım curl ile)

## Kaynaklar

| Kaynak | URL | HTTP |
|---|---|---|
| robots.txt | https://medya-cdn.tjk.org/robots.txt | 404 (dosya yok → RFC 9309: kısıtlama yok) |
| Günlük yarış programı CSV | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-06/CSV/GunlukYarisProgrami/06.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 |
| 2026 resmî takvim PDF | https://medya-cdn.tjk.org/haberftp/2025/2026takvim161225.pdf | 200 |

## Rust doğrulama sonucu

- **CSV:** 12660 bayt, SHA-256 `45801a1bafe2fdf3bd299759844c25d59766b8d5f753cc2ebf89b0064ffbf594`
  → beklenen değerle **birebir eşleşti**. UTF-8, 107 satır.
  - İlk satır: `İzmir;(29. Yarış Günü);06/03/2026`
  - 1. koşu, 1 numaralı at: `ATİK MEHMET` (ham: `ATİK MEHMET KG DB SK GKR`; sondakiler ekipman kısaltması)
- **Takvim PDF:** 134596 bayt, SHA-256 `36a2318a2c8f13e8df78883cf2ff686ab38acd4eec1dcf5ebc2777b1713a52e6`
  - Koordinat tabanlı hizalama: hata yok (tarih–gün uyumu, yuva çakışması, yıllık toplamlar başlıkla aynı; genel toplam 758)
  - Mart 2026: **61** tarih–hipodrom çifti → `mart_2026_resmi_takvim.tsv` (20.03'teki `RMZ` = Ramazan Bayramı, hariç)
  - 06.03.2026 takvimde Bursa ve İzmir olarak yer alıyor; CSV ile tutarlı.

## Eklenen ham dosyalar

- `kontrol/ham/06.03.2026-İzmir-GunlukYarisProgrami-TR.csv` (ağdan indirilen ham dosya, değiştirilmedi)

Veri tabanı değiştirilmedi.
