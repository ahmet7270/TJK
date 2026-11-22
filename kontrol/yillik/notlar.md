## Kaynak hataları ve kararlar

| Tarih / gün | Konu | Kanıt | Karar |
|---|---|---|---|
| 2026-01-11 Adana (5. yarış günü) | 4.–8. koşular 11.01 sonuç dosyasında yok | 12.01 Adana program/sonuç dosyaları (takvimde 12.01 Adana yok) başlıkları `Adana;(5. Yarış Günü);12/01/2026`, yalnızca 4.–8. koşular; saatler 20:00–22:00 → 17:15–18:45, cins/yaş/kilo/mesafe aynı | `--ek-sonuc 20260112-ADANA-sonuc.csv` + erteleme notu (`listeler/2026-01_ozel.tsv`). 5 koşu ERTELENDI, tek kez sayıldı. |
| 2026-01-11 Adana | 1. ve 2. altılının ödemesi yok | 12.01 dosyasında `6'LI GANYAN` ödemesi yok; 8. koşuda 5'li/4'lü ganyan ödenmiş. İade/iptal ibaresi yok | Ödemeler boş (NULL, ödeme satırı yok), `ALTILI_ODEMESI_BULUNAMADI` ×2 ile incelemede (15.03 Adana ile aynı durum). |
| 2026-02-28 İstanbul | 4. koşu bahis satırında numaralar boş | `GANYAN() :2,35 TL, İKİLİ() :4,30 TL, SIRALI İKİLİ() :9,70 TL, 3. ÇİFTE() :6,20 TL` | Kaynak hatası. Birinci sonuç listesinden (6 LUCERYS) biliniyor; ödeme numarası uydurulmadı. `GANYAN_SATIRI_BULUNAMADI` ile incelemede. |
| 2026-02-02 Bursa K3, 2026-02-16 Bursa K6 | İki GANYAN ödemesi aynı tutar | Kazanan ile eküri eşi (`[(1)…,(7)…] eküridir.` / `[(3)…,(4)…]`) aynı GANYAN tutarıyla; dereceler farklı, Fark `Burun` / `Yarım Boy` | Ölü yarış değil, eküri ödemesi. Ayrıştırıcı 0.14.0: yalnızca ölü yarış kanıtı tutmadığında ve tüm numaralar aynı eküride, tutarlar eşit, liste 1. atı bunlardan biriyse incelemeye düşürmez (`tests/yillik_istisnalar.rs`, olumsuz testlerle). GANYAN kontrolü `EKURI_ILE_ACIKLANDI`. |
| 2026-02-06 İzmir A1, 2026-02-24 Adana A1/A2 | Ortak birincilik, birden çok altılı kombinasyonu | Ayrıştırıcının 5 kanıtı (ayrı GANYAN'lar, ilk N sıra, oran = ödeme, `At Başı`, aynı derece) | Her kazanan kombinasyon kendi ödemesine ve bahis satırına bağlı (`altili_odeme`), geçerli numaralar `ORTAK_BIRINCI`. |
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
