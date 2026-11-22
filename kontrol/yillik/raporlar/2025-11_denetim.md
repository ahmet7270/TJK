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
| yaris_gunu | 18 | 260 |
| kosu | 149 | 2053 |
| katilim | 1489 | 20432 |
| sonuc | 1489 | 20432 |
| agf_gozlem | 1896 | 26235 |
| bahis_sonucu | 1244 | 16361 |
| ganyan_kontrol | 149 | 2053 |
| altili | 31 | 447 |
| altili_ayak | 186 | 2682 |
| altili_odeme | 33 | 444 |
| inceleme_nedeni | 0 | 97 |
| kaynak_dosya (ayın kullandığı) | 36 | 522 |
| at (tüm) | – | 4933 |

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
- Tam döküm SHA-256 (16 tablo): `774526ea8907e99b05052394ac9df049d6244d8219b81048caa53ed048009cd9`
- Tablo satırları: agf_gozlem 26235, altili 447, altili_ayak 2682, altili_ayak_gecerli 3068, altili_odeme 444, altili_odeme_ayak 3073, at 4933, bahis_sonucu 16361, bahis_uyarisi 75, ganyan_kontrol 2053, inceleme_nedeni 97, katilim 20432, kaynak_dosya 522, kosu 2053, sonuc 20432, yaris_gunu 260

## Sonuç

Sert hata yok.
