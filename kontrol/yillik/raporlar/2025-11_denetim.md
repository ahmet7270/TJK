# Aylık denetim: 2025-11

Veri tabanı: `/home/user/TJK/calisma/tjk_yillik.sqlite` (salt okunur açıldı) · Takvim: `/home/user/TJK/kontrol/yillik/takvim/takvim_hedef.tsv`

## 1. Takvim kapsamı

**takvim 18 çift: TAMAMLANDI 18, INCELEME_GEREKLI 0, EKSIK 0, AKTARILMADI 0, HENUZ_DENENMEDI 0; takvim dışı DB günü 0**

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2025-11-22 | Adana | TAMAMLANDI | 2 |  |
| 2025-11-22 | İstanbul | TAMAMLANDI | 2 |  |
| 2025-11-23 | İzmir | TAMAMLANDI | 3 |  |
| 2025-11-23 | Adana | TAMAMLANDI | 3 |  |
| 2025-11-24 | Bursa | TAMAMLANDI | 3 |  |
| 2025-11-24 | Şanlıurfa | TAMAMLANDI | 2 |  |
| 2025-11-25 | Adana | TAMAMLANDI | 4 |  |
| 2025-11-25 | Antalya | TAMAMLANDI | 3 |  |
| 2025-11-26 | İstanbul | TAMAMLANDI | 3 |  |
| 2025-11-26 | Elazığ | TAMAMLANDI | 50 |  |
| 2025-11-27 | İzmir | TAMAMLANDI | 4 |  |
| 2025-11-27 | Antalya | TAMAMLANDI | 4 |  |
| 2025-11-28 | Antalya | TAMAMLANDI | 5 |  |
| 2025-11-28 | İzmir | TAMAMLANDI | 5 |  |
| 2025-11-29 | Adana | TAMAMLANDI | 5 |  |
| 2025-11-29 | İstanbul | TAMAMLANDI | 4 |  |
| 2025-11-30 | İzmir | TAMAMLANDI | 6 |  |
| 2025-11-30 | Adana | TAMAMLANDI | 6 |  |

## 2. Kayıt sayıları

| Tablo | Bu ay | Tüm veri tabanı |
|---|---|---|
| yaris_gunu | 18 | 326 |
| kosu | 149 | 2615 |
| katilim | 1489 | 26484 |
| sonuc | 1489 | 26484 |
| agf_gozlem | 1896 | 34339 |
| bahis_sonucu | 1244 | 21120 |
| ganyan_kontrol | 149 | 2615 |
| altili | 31 | 573 |
| altili_ayak | 186 | 3438 |
| altili_odeme | 33 | 570 |
| inceleme_nedeni | 0 | 97 |
| kaynak_dosya (ayın kullandığı) | 36 | 654 |
| at (tüm) | – | 5504 |

## 3. İnceleme nedenleri

Bu ay inceleme nedeni yok.


## 4. Kaynak bağları

- Bu ayın kullandığı 36 kaynak dosyanın 36'i indirme kaydında HTTP 200 + ICERIK_DOGRULANDI ile URL'ye bağlı.
- v4 tarih/kaynak alanı boş koşu: 0
- Kaynak satırı olmayan sonuç (koşmayanlar hariç): 0
- tarih_durumu: PLANLANDIGI_GUN=149
- Hiçbir kayda bağlı olmayan kaynak_dosya (tüm DB): 0

## 5. Ertelenen koşular ve çift sayım

Bu ay planlandığı gün dışında koşulan ya da sonucu olmayan koşu yok.

- Birden çok güne bağlı sonuç dosyası (tüm DB): 0
- Aynı hipodrom + yarış günü no'su 14 gün içinde iki gün (erteleme sonrası çift aktarım belirtisi; numaralar sezon başında sıfırlanır) (tüm DB): 0
- Aynı gün + koşu no iki kez: 0

## 6. Yarış günü numarası sürekliliği (hipodrom başına, önceki DB günüyle)

Bu ayda numara boşluğu yok (önceki gün DB'de olan hipodromlar için).

## 7. Altılı kombinasyon ve ödemeleri

- Altılı 31; ödeme satırı olan 31; DEVIR 1; birden çok kazanan kombinasyonlu: 20251127-ANTALYA-A1, 20251127-ANTALYA-A2; ORTAK_BIRINCI geçerli numara: 4
  - `20251127-ANTALYA-A1` #1: 3/1,3,5,6,8/1/1/7/2 = 1.490,20 TL (bahis 20251127-ANTALYA-K06 sıra 3)
  - `20251127-ANTALYA-A1` #2: 3/1,3,5,6,8/1/1/7/3 = 2.098,20 TL (bahis 20251127-ANTALYA-K06 sıra 4)
  - `20251127-ANTALYA-A2` #1: 1/1/7/2/1/2 = 5.116,55 TL (bahis 20251127-ANTALYA-K08 sıra 3)
  - `20251127-ANTALYA-A2` #2: 1/1/7/3/1/2 = 7.217,45 TL (bahis 20251127-ANTALYA-K08 sıra 4)
- Ödeme satırı olmayan altılı: 0 (); bunlardan inceleme nedeni yazılmamış: yok

## 8. Bütünlük

- `PRAGMA user_version`: 4
- `PRAGMA foreign_key_check`: 0 satır
- `PRAGMA integrity_check`: ok
- Tam döküm SHA-256 (16 tablo): `e8c049037005b6e490aa4fe6f917a0f6aa152733dafb2c18121322f345c5916a`
- Tablo satırları: agf_gozlem 34339, altili 573, altili_ayak 3438, altili_ayak_gecerli 3975, altili_odeme 570, altili_odeme_ayak 3980, at 5504, bahis_sonucu 21120, bahis_uyarisi 107, ganyan_kontrol 2615, inceleme_nedeni 97, katilim 26484, kaynak_dosya 654, kosu 2615, sonuc 26484, yaris_gunu 326

## Sonuç

Sert hata yok.

## İkinci aktarım

Aynı girdilerle ikinci aktarımdan sonra tüm tabloların sıralı tam dökümü birinciyle bayt bayt aynı (`cmp`).
