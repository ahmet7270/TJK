# Açık 12 çift: belge araştırması (2026-11-22)

Kaynaklar yalnızca `medya-cdn.tjk.org` (robots.txt HTTP 404, kural yok). `www.tjk.org` robots.txt tüm botlara `Disallow: /` dediği için
kullanılmadı: resmî duyurular, komiser raporu sayfaları ve yıllık program sorgusu **okunamadı**. 403/429/5xx alınmadı.

## Yöntem

- `pdf_yokla.sh`: 11 çift için CSV/PDF/PDFOzetSonuc × program/sonuç × TR/EN adreslerini yoklar → `alternatif_belgeler.tsv`.
- **Uyarı:** CDN, olmayan PDF için de HTTP 200 ve içinde "Hata kodu : 404" yazan 31546 baytlık sayfa döndürüyor. Bu yüzden her PDF
  `pdftotext` ile okunup sınıflandı: **içerikli**, **404 yer tutucusu**, **içeriksiz şablon** (başlık var, 0 kelime).
- İndirilen PDF'ler `pdf/` (11 çift) ve `pdf_ek/` (12.01 ve 16.03 Adana, 02.04/04.04 İzmir) altında; URL, HTTP, bayt, SHA-256 `pdf_durum.tsv` içinde (başlıksız; sütunlar: tarih, hipodrom, klasör, HTTP, bayt, SHA-256[, URL]).
- Komiser raporu: CDN'de 8 olası klasör/ad kalıbı 404 (15.03.2026 Adana ile denendi); bulunamadı. İptal/erteleme duyuruları da erişilebilir değil.
- Hiçbir ödeme, iade ya da iptal kararı çıkarılmadı. Aşağıdaki "kanıt" sütunu yalnızca belgede doğrudan yazanı içerir.

## Çiftler

| # | Çift | Belgelerde doğrudan görülen | Ayrım | Veri tabanı |
|---|---|---|---|---|
| 1 | 2026-01-11 Adana | 11.01 sonuç PDF (3 koşu, 3 GANYAN) ve özet PDF yalnızca 1–3. koşu. 12.01 sonuç PDF/özet: 4.–8. koşu (özet K4–K8 netice ve tutarlar). İkisinde de `6'LI GANYAN`, "iade/iptal" ibaresi yok | **Erteleme** (4.–8. koşu 12.01'de koşulmuş); altılı ödemesi hiçbir belgede yok | Değişmedi: K4–K8 `ERTELENDI`, A1/A2 ödemesi boş, gün incelemede |
| 2 | 2026-02-28 İstanbul | Sonuç PDF (3 sayfa) ve özet PDF 4. koşu için netice 6-8-5-3 ve tutarlar (ganyan 2,35, ikili 4,30, sıralı ikili 9,70, çifte 6,20) gösteriyor; kombinasyon **numaraları** hiçbirinde yok. CSV'deki `GANYAN() :2,35 …` satırı boş numaralı. Ganyan 2,35 zaten birinci atın (6, LUCERYS) sonuç satırındaki ganyan oranıyla aynı | **Yayımlanmış dosyada biçim bozukluğu**; resmî belgelerde de numaralar yok, yeni kanıt yok | Değişmedi (`GANYAN_SATIRI_BULUNAMADI` açık); numara uydurulmadı |
| 3 | 2026-03-15 Adana | 15.03 PDF/özet: 1.–2. koşu. 16.03 PDF/özet: 3.–8. koşu; bahis satırlarında yalnızca `2. 6'LI GANYAN (1,12/3,4/1,2,3/5/5/12): 19.387,81 TL`. 3. koşu için ayrı GANYAN bahis satırı yok (özetteki 1,85 atın sonuç satırındaki ganyan oranı); `1. 6'LI GANYAN` hiçbir belgede yok; iade/iptal ibaresi yok | Erteleme (zaten kayıtlı); iki açık kayıt için yeni kanıt yok | Değişmedi: iki açık ödeme korundu |
| 4 | 2026-04-03 İzmir | CSV, sonuç PDF ve özet yalnızca 9. koşu (özet: `1 - 2- 3- 4`, tutarsız/boş). 1.–8. koşu hiçbir belgede yok. 04.04 İzmir sonuç PDF/özet 404; 02.04 İzmir sonuç PDF'i var (40. yarış günü), 03.04 = 41., 05.04 = 42. yarış günü: numara sürekli, kayıp gün yok | Ne iptal ne erteleme kanıtı var; **eksik yayımlanmış dosya**. Başka tarihte koşulduğuna dair belge yok | Değişmedi (program var, sonuç yok, gün incelemede) |
| 5 | 2026-10-28 Elazığ | CSV, sonuç PDF ve özet aynı: yalnızca 1. koşu (netice 2-5-3-1, ganyan 3,35, ikili 7,70, sıralı 16,00); program 8 koşu | Yarım yayımlanmış dosya (1/8). İptal/erteleme kanıtı yok | Değişmedi |
| 6 | 2025-12-31 Şanlıurfa | Sonuç CSV 0 bayt (HTTP 200, iki kez). Program/sonuç/özet PDF: başlıklı ama **0 kelime** (1,2 KB şablon) | Yayımlanmamış/boş üretilmiş sonuç; iptal kanıtı değil (program CSV'si dolu) | Aktarılmadı (sonuçsuz gün üretilmez) |
| 7 | 2026-09-28 Bursa | Aynı: CSV 0 bayt; program/sonuç/özet PDF içeriksiz şablon (program PDF'i de boş, program CSV'si 17259 bayt) | Boş üretilmiş belge; iptal kanıtı değil | Aktarılmadı |
| 8 | 2026-09-29 Kocaeli | Aynı (program CSV 20465 bayt, PDF'ler içeriksiz) | Aynı | Aktarılmadı |
| 9 | 2026-10-16 Bursa | Sonuç CSV 0 bayt; sonuç ve özet PDF = 404 yer tutucusu; program PDF 303 KB içerikli | Sonuç hiçbir biçimde yayımlanmamış | Aktarılmadı |
| 10 | 2026-10-19 Bursa | Sonuç CSV 0 bayt; sonuç PDF 404 yer tutucusu; özet PDF **başlık ve sütun adları var, satır yok** (17100 bayt); program PDF içerikli | Boş sonuç tablosu yayımlanmış (koşuların yapılıp yapılmadığı yazmıyor) | Aktarılmadı |
| 11 | 2026-04-18 Ankara | Program CSV ve sonuç CSV 404. **Program PDF içerikli** (3 sayfa, 9 koşu, 436 KB); sonuç ve özet PDF = 404 yer tutucusu | Program PDF olarak yayımlanmış, sonuç yok | Aktarılmadı: mevcut ayrıştırıcı yalnızca CSV okur |

## Karmaşık değişiklik gerektiren noktalar (dururum; karar sizde)

1. **PDF ayrıştırma:** 18.04 Ankara (program PDF'i var, CSV yok) ve 28.02 İstanbul 4. koşu (özet PDF'te tutarlar var) ancak PDF okuyan yeni bir
   ayrıştırıcıyla işlenebilir. PDF yerleşimi çok sütunlu; bu mevcut CSV ayrıştırıcısının kapsamı dışında ve yeni kod gerektirir.
2. **Sonuçsuz gün:** `yaris_gunu.sonuc_kaynak_id` NOT NULL ve gün programla sonucu birlikte ister. Programı doğrulanmış ama sonucu yayımlanmamış
   günler (31.12.2025 Şanlıurfa, 28.09 Bursa, 29.09 Kocaeli, 16.10 Bursa, 19.10 Bursa, 18.04 Ankara) bugünkü şemada saklanamaz;
   `SONUC_YOK` koşu durumu yalnızca günün başka sonuç dosyası varsa kullanılabiliyor. Çözüm şema/aktarım değişikliği (ör. sonuçsuz gün durumu) olur.
3. **Kısmi sonuç:** 03.04 İzmir ve 28.10 Elazığ zaten bu yolla (program tam, sonuç kısmi) aktarılmış durumda ve `INCELEME_GEREKLI`.

## Sonuç

Hiçbir çiftin iptal edildiği ya da başka tarihte koşulduğu (11.01 Adana hariç; o zaten kayıtlı) doğrudan kanıtlanamadı. Bu 11 çift için
verilerde değişiklik yapılmadı; kaynak durumu yukarıdaki ayrımla kaydedildi. 22.11 Adana ayrı izleniyor (aşağıda güncellenecek).
