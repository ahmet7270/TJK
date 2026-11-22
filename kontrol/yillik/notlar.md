## Kaynak hataları ve kararlar

| Tarih / gün | Konu | Kanıt | Karar |
|---|---|---|---|
| 2026-01-11 Adana (5. yarış günü) | 4.–8. koşular 11.01 sonuç dosyasında yok | 12.01 Adana program/sonuç dosyaları (takvimde 12.01 Adana yok) başlıkları `Adana;(5. Yarış Günü);12/01/2026`, yalnızca 4.–8. koşular; saatler 20:00–22:00 → 17:15–18:45, cins/yaş/kilo/mesafe aynı | `--ek-sonuc 20260112-ADANA-sonuc.csv` + erteleme notu (`listeler/2026-01_ozel.tsv`). 5 koşu ERTELENDI, tek kez sayıldı. |
| 2026-01-11 Adana | 1. ve 2. altılının ödemesi yok | 12.01 dosyasında `6'LI GANYAN` ödemesi yok; 8. koşuda 5'li/4'lü ganyan ödenmiş. İade/iptal ibaresi yok | Ödemeler boş (NULL, ödeme satırı yok), `ALTILI_ODEMESI_BULUNAMADI` ×2 ile incelemede (15.03 Adana ile aynı durum). |
| 2026-02-28 İstanbul | 4. koşu bahis satırında numaralar boş | `GANYAN() :2,35 TL, İKİLİ() :4,30 TL, SIRALI İKİLİ() :9,70 TL, 3. ÇİFTE() :6,20 TL` | Kaynak hatası. Birinci sonuç listesinden (6 LUCERYS) biliniyor; ödeme numarası uydurulmadı. `GANYAN_SATIRI_BULUNAMADI` ile incelemede. |
| 2026-02-02 Bursa K3, 2026-02-16 Bursa K6 | İki GANYAN ödemesi aynı tutar | Kazanan ile eküri eşi (`[(1)…,(7)…] eküridir.` / `[(3)…,(4)…]`) aynı GANYAN tutarıyla; dereceler farklı, Fark `Burun` / `Yarım Boy` | Ölü yarış değil, eküri ödemesi. Ayrıştırıcı 0.14.0: yalnızca ölü yarış kanıtı tutmadığında ve tüm numaralar aynı eküride, tutarlar eşit, liste 1. atı bunlardan biriyse incelemeye düşürmez (`tests/yillik_istisnalar.rs`, olumsuz testlerle). GANYAN kontrolü `EKURI_ILE_ACIKLANDI`. |
| 2026-02-06 İzmir A1, 2026-02-24 Adana A1/A2 | Ortak birincilik, birden çok altılı kombinasyonu | Ayrıştırıcının 5 kanıtı (ayrı GANYAN'lar, ilk N sıra, oran = ödeme, `At Başı`, aynı derece) | Her kazanan kombinasyon kendi ödemesine ve bahis satırına bağlı (`altili_odeme`), geçerli numaralar `ORTAK_BIRINCI`. |
| 2026-04-03 İzmir (41. yarış günü) | Sonuç dosyasında sonuç yok | Sonuç CSV'si (2133 bayt) yalnızca 9. koşunun başlığını ve 15 atlık start listesini derece/ganyan/fark olmadan içeriyor; 1.–8. koşular yok. 04.04 İzmir program/sonuç adresleri HTTP 404; 05.04 İzmir = 42. yarış günü | Program verisi aktarıldı; 1.–8. koşu `SONUC_YOK`, 9. koşu sonuçları `BILINMEYEN`, altılı ödemeleri boş; gün incelemede (`ESLESMEYEN_PROGRAM_KAYDI` ×68 vb.). Koşuların yapılıp yapılmadığı kaynaklarda yazmıyor. |
| 2026-04-18 Ankara | Program ve sonuç adresleri HTTP 404 | 16.04 Ankara = 1., 23.04 Ankara = 3. yarış günü; 2. gün 17, 19, 20, 21, 22.04 adreslerinde de yok (12 istek, hepsi 404; `listeler/2026-04_ek_durum.tsv`) | EKSIK. Tahminle doldurulmadı. 2. yarış gününün tarihi/dosyası bulunamadı. |
| 2025-12-31 Şanlıurfa | Sonuç dosyası 0 bayt | Sonuç adresi iki kez HTTP 200 ve 0 bayt döndü (indirme + elle yeniden deneme); program dosyası doğrulandı (14957 bayt) | EKSIK: program tek başına aktarılmadı (sonuçsuz gün üretilmez). Sonuç yayımlanırsa `ay_isle.sh indir/aktar 2025-12` ile tamamlanır. |
| 2026-09-28 Bursa, 2026-09-29 Kocaeli | Sonuç dosyası 0 bayt | Sonuç adresleri iki kez HTTP 200 ve 0 bayt; programlar doğrulandı | EKSIK: sonuçsuz gün aktarılmadı. Sonuç yayımlanırsa `ay_isle.sh indir/aktar 2026-09` ile tamamlanır. |
| 2026-10-16 Bursa, 2026-10-19 Bursa | Sonuç dosyası 0 bayt | Adresler yeniden denemede de HTTP 200 / 0 bayt; programlar doğrulandı | EKSIK: sonuçsuz gün aktarılmadı. |
| 2026-10-28 Elazığ (45. yarış günü) | Sonuç dosyası yarım | 1592 bayt: yalnızca 1. koşunun başlığı ve ikramiyeler, atlar/sonuç yok; yeniden denemede aynı | Program aktarıldı, sonuçlar boş; gün incelemede (`ESLESMEYEN_PROGRAM_KAYDI` ×69 vb.). |
| 2026-11-22 Adana | Sonuç dosyası yarım (hedef aralığın son günü) | 9426 bayt, yalnızca bir kısım koşu sonucu; yeniden denemede aynı | Program aktarıldı, sonuçsuz koşular boş; gün incelemede. Sonuç tamamlanırsa `ay_isle.sh indir/aktar 2026-11` ile yenilenir. |
| 2026-03-15 Adana | İki açık ödeme | Önceki inceleme (`kontrol/mart_istisnalar.md` §5) | Korundu: `GANYAN_SATIRI_BULUNAMADI: 3. koşu`, `ALTILI_ODEMESI_BULUNAMADI: …-A1` (A1 ödemesi NULL). |

### Erişim

- `medya-cdn.tjk.org/robots.txt`: HTTP 404 (kural yok). Bu çalışmada hiçbir medya-cdn isteği 403/429/5xx almadı (ayrıntı: `listeler/AY_durum.tsv`).
- `www.tjk.org/robots.txt`: HTTP 200, `User-agent: *` → `Disallow: /`. Bu siteye otomatik istek yapılmadı; resmî erteleme duyuruları
  okunmadı. Ertelemeler dosya içeriğinden (aynı yarış günü numarası, aynı koşu tanımları) kuruldu, not metninde belirtildi.

### Araç değişiklikleri

- `tjk-fetch takvim PDF ÇIKTI YIL İLK SON`: herhangi bir yılın takviminden tarih aralığı (Mart çıktısı değişmedi).
- `tjk-fetch girdiler`: doğrulanmış çiftlerden aktarım girdisi (SHA-256 indirme kaydıyla eşleşmeli).
- `tjk_denetim`: aylık denetim (salt okunur). `tjk_ayristirici` 0.14.0: eküri GANYAN ödemesi.
- 0.14.0 regresyonu: Ocak + Mart (123 gün) yeniden aktarıldı; 16 tablonun tam dökümünde **yalnızca** `ayristirici_surumu` değişti.
- Kaynak URL/HTTP: v4 şeması değiştirilmedi. `kaynak_dosya.sha256` → `listeler/AY_durum.tsv` (URL, HTTP, bayt, SHA-256, zaman);
  denetim §4 her ayın kullandığı her kaynak dosyanın bu kayıtta HTTP 200 + ICERIK_DOGRULANDI ile bulunduğunu doğrular.

### Ayrıştırıcı 0.15.0 (BB/TGK kodları, eküri ibaresinde kod)

- Bulgu: 14.06.2026 İstanbul'da `ZİNCİR KOPARAN KG K BB` adındaki `BB` kod belirteci tanınmıyordu (eküri üyesi doğrulanamadı);
  58 dosya satırında `BB`, 2'sinde `TGK` var. `KOD_BELIRTECLERI` listesine `BB`, `TGK` eklendi; eküri ibaresindeki ad da kodlardan arındırılarak karşılaştırılıyor.
- Testler: birim `bb_ve_tgk_kod_belirtecleri`, uçtan uca `istanbul_14_haziran_*`, `istanbul_10_haziran_*` (tümü geçti).
- Regresyon (Kasım 2025 + Ocak–Mayıs 2026, 324 gün yeniden aktarıldı; 16 tablonun tam dökümü): `ayristirici_surumu` dışında yalnızca
  ham adında BB/TGK olan 23 katılımın `at_id`/`isim_kodlari` alanı değişti (`at` tablosunda 13 yinelenen at kaydı birleşti, 4 yeni). Ham ad korundu.
  Fark dosyası: `raporlar/regresyon_0.15.0_farklar.txt`. Yeniden aktarım sırasında kesilen Mayıs işlemi yeniden çalıştırıldı (journal geri alındı, FK/integrity temiz).
