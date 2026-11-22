# Resmî takvim kaynakları (hedef: 22.11.2025 – 22.11.2026, dahil)

| Yıl | Kaynak URL | HTTP | Bayt | SHA-256 | Belge tarihi | Depodaki kopya |
|---|---|---|---|---|---|---|
| 2025 | https://medya-cdn.tjk.org/haberftp/2024/2025_takvim03102024.pdf | 200 | 217079 | `7731bfb8fdbca892a183d6cad486b158208054783257ff1b4e603a97a67e03a7` | 03.10.2024 (PDF içi tarih ve oluşturma zamanı) | `2025_takvim03102024.pdf` |
| 2026 | https://medya-cdn.tjk.org/haberftp/2025/2026takvim161225.pdf | 200 | 134596 | `36a2318a2c8f13e8df78883cf2ff686ab38acd4eec1dcf5ebc2777b1713a52e6` | 16.12.2025 (dosya adı) | `2026takvim161225.pdf` |

- robots.txt: `https://medya-cdn.tjk.org/robots.txt` HTTP 404 (kural yok). `https://www.tjk.org/robots.txt` HTTP 200 ve
  `User-agent: *` için `Disallow: /`: www.tjk.org'a hiçbir otomatik istek yapılmadı (resmî duyurular, yıllık program sorgu sayfası okunmadı).
- 2025 PDF'sinin adresi medya-cdn'de ad kalıbı taramasıyla bulunamadı (`takvim_url_tarama.sh`; `2025takvimGGAA25`, `2025takvimGGAA24`
  kalıpları, hepsi 404); adres bir web aramasının sonuç listesinden alındı ve dosya doğrudan medya-cdn'den indirildi.
- Bu 2025 belgesi 03.10.2024 tarihli ilk yayımdır. Sonradan yayımlanmış bir revizyon bulunamadı; takvim dışı bir gün çıkarsa
  yarış günü numarası sürekliliği denetimiyle (`tjk_denetim` §6) görünür.

## Çıkarım

```bash
tjk-fetch takvim 2025_takvim03102024.pdf takvim_2025_1122_1231.tsv 2025 2025-11-22 2025-12-31   # 80 çift
tjk-fetch takvim 2026takvim161225.pdf    takvim_2026_0101_1122.tsv 2026 2026-01-01 2026-11-22   # 680 çift
# takvim_hedef.tsv = ikisinin birleşimi: 760 benzersiz çift
```

- Her kod sayfadaki konumuna göre tarihine bağlanır; hafta günü uyumu, yuva çakışması ve hipodrom başına yıllık sayımların PDF
  başlığındaki toplamlarla eşitliği denetlenir. **Her iki yıl: hata yok, 10/10 hipodrom toplamı ve genel toplam (758) eşleşti**
  (`takvim_2025_cikti.txt`, `takvim_2026_cikti.txt`).
- Hipodrom olmayan işaretler hariç: 2026'da 20.03 `RMZ` (Ramazan Bayramı), 27.05 `KRB` (Kurban Bayramı).
- 2026 Mart çıktısı önceki `mart_2026_resmi_takvim.tsv` ile birebir aynı (61 çift).

| Ay | Çift | Not |
|---|---|---|
| 2025-11 | 18 | yalnızca 22–30 Kasım |
| 2025-12 | 62 | |
| 2026-01 | 62 | |
| 2026-02 | 56 | |
| 2026-03 | 61 | |
| 2026-04 | 62 | |
| 2026-05 | 66 | |
| 2026-06 | 64 | |
| 2026-07 | 66 | |
| 2026-08 | 67 | |
| 2026-09 | 63 | |
| 2026-10 | 67 | |
| 2026-11 | 46 | yalnızca 1–22 Kasım |
| **Toplam** | **760** | |

Planlanan gün ↔ gerçekleşen tarih ilişkisi takvimde değil, veri tabanında koşu başına tutulur (`kosu.planlanan_tarih`,
`gerceklesen_tarih`, `tarih_durumu`, `sonuc_kaynak_id`); ertelenen koşu, planlandığı günün kaydında kalır ve ikinci kez sayılmaz.
