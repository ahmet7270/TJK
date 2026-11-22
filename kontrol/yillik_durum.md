# Yıllık durum: 22.11.2025 – 22.11.2026

Üretim: `kontrol/yillik/durum_yaz.sh` (her ay için `tjk_denetim`, veri tabanı salt okunur) · 2026-11-22T17:18Z
Veri tabanı: `tjk_yillik.sqlite` · SHA-256 `93162689bf92d00f5b11d85db626cf7189dd352285affdb39ec0d8492f6bec0a`

Takvim: resmî 2025 ve 2026 PDF'leri (`kontrol/yillik/takvim/KAYNAKLAR.md`), 760 çift. Sınıflar: TAMAMLANDI (gün TAMAM),
INCELEME_GEREKLI (gün aktarıldı, gerekçeli açık kayıt var), EKSIK (denendi, dosya alınamadı/doğrulanamadı),
AKTARILMADI (iki dosya doğrulandı, aktarım bekliyor), HENUZ_DENENMEDI (indirme kaydı yok).

| Ay | Takvim çifti | Tamam | İnceleme | Eksik | İndirildi, aktarılmadı | Denenmedi | Takvim dışı | Koşu | Katılım | Altılı | FK | integrity | 2. aktarım | Sert hata | Durum |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2025-11 | 18 | 18 | 0 | 0 | 0 | 0 | 0 | 149 | 1489 | 31 | 0 | ok | aynı | yok | tamamlandı |
| 2025-12 | 62 | 61 | 0 | 1 | 0 | 0 | 0 | 490 | 4849 | 103 | 0 | ok | aynı | yok | eksik var |
| 2026-01 | 62 | 61 | 1 | 0 | 0 | 0 | 0 | 469 | 4610 | 97 | 0 | ok | aynı | yok | inceleme var |
| 2026-02 | 56 | 55 | 1 | 0 | 0 | 0 | 0 | 431 | 4203 | 96 | 0 | ok | aynı | yok | inceleme var |
| 2026-03 | 61 | 60 | 1 | 0 | 0 | 0 | 0 | 481 | 4776 | 105 | 0 | ok | aynı | yok | inceleme var |
| 2026-04 | 62 | 60 | 1 | 1 | 0 | 0 | 0 | 507 | 5229 | 114 | 0 | ok | aynı | yok | eksik var |
| 2026-05 | 66 | 66 | 0 | 0 | 0 | 0 | 0 | 562 | 6052 | 126 | 0 | ok | aynı | yok | tamamlandı |
| 2026-06 | 64 | 64 | 0 | 0 | 0 | 0 | 0 | 533 | 5630 | 121 | 0 | ok | aynı | yok | tamamlandı |
| 2026-07 | 66 | 0 | 0 | 1 | 64 | 1 | 0 | 0 | 0 | 0 | 0 | ok | — | yok | denenmedi |
| 2026-08 | 67 | 0 | 0 | 0 | 0 | 67 | 0 | 0 | 0 | 0 | 0 | ok | — | yok | henüz başlanmadı |
| 2026-09 | 63 | 0 | 0 | 0 | 0 | 63 | 0 | 0 | 0 | 0 | 0 | ok | — | yok | henüz başlanmadı |
| 2026-10 | 67 | 0 | 0 | 0 | 0 | 67 | 0 | 0 | 0 | 0 | 0 | ok | — | yok | henüz başlanmadı |
| 2026-11 | 46 | 0 | 0 | 0 | 0 | 46 | 0 | 0 | 0 | 0 | 0 | ok | — | yok | henüz başlanmadı |
| **Toplam** | **760** | **445** | **4** | **3** | **64** | **244** | | | | | | | | | |

## Tamamlanmamış çiftler (ay bazında)

### 2025-11

Takvimdeki tüm çiftler TAMAMLANDI.

Altılı, birden çok kazanan kombinasyonlu: 20251127-ANTALYA-A1, 20251127-ANTALYA-A2 (her kombinasyon kendi ödemesine bağlı).

### 2025-12

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2025-12-31 | Şanlıurfa | EKSIK |  | program HTTP 200 ICERIK_DOGRULANDI; sonuc HTTP 200 BICIM_HATASI |

### 2026-01

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2026-01-11 | Adana | INCELEME_GEREKLI | 5 | ALTILI_ODEMESI_BULUNAMADI: 20260111-ADANA-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok \| ALTILI_ODEMESI_BULUNAMADI: 20260111-ADANA-A2 son ayak (8. koşu) bahis satırında '2. 6'LI GANYAN' yok |

Planlandığı gün dışında koşulan koşu: 5 (ayrıntı: `raporlar/2026-01_denetim.md` §5).

### 2026-02

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2026-02-28 | İstanbul | INCELEME_GEREKLI | 17 | GANYAN_SATIRI_BULUNAMADI: 4. koşu (birinci [6]) |

Altılı, birden çok kazanan kombinasyonlu: 20260206-IZMIR-A1, 20260224-ADANA-A1, 20260224-ADANA-A2 (her kombinasyon kendi ödemesine bağlı).

### 2026-03

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2026-03-15 | Adana | INCELEME_GEREKLI | 32 | GANYAN_SATIRI_BULUNAMADI: 3. koşu (birinci [1]) \| ALTILI_ODEMESI_BULUNAMADI: 20260315-ADANA-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok |

Planlandığı gün dışında koşulan koşu: 15 (ayrıntı: `raporlar/2026-03_denetim.md` §5).

Altılı, birden çok kazanan kombinasyonlu: 20260331-ANTALYA-A2 (her kombinasyon kendi ödemesine bağlı).

### 2026-04

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2026-04-03 | İzmir | INCELEME_GEREKLI | 41 | ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A01 BIG RUGGED (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A02 SPARK (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A03 ATLASIN LEYDİSİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A04 AYLEMA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A05 BOLUN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A06 GÜLFEM HATUN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A07 HARD TIGER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A08 HOLLABACK GIRL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A09 KIZIL ANKA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A10 SEBELLA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K01-A11 VAHŞİ YILDIZ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K02-A01 PHANTOM STRIKE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K02-A02 ERİNÖZ ALP (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K02-A03 AVENGING (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K02-A04 MR DEMİRAYAK (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K02-A05 ROOCKET OF RUNNER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K02-A06 BURNING HEART (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A01 OLAYIN ASLI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A02 CASHOUT (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A03 MAGGIE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A04 ALFA ORENDA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A05 AMORE GARDEN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A06 DARK RİSHA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A07 KAİMEİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A08 LADY ONAT (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A09 QUEEN TUANA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K03-A10 RISE REASON (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A01 OBRADA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A02 MUĞLALI ADNAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A03 THE ADMIRAL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A04 BORN FOR GLORY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A05 THE BIG BANG (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A06 BEULIEU (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A07 THUNDER JEST (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A08 TIZ A ROCKET (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A09 DILO (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A10 LA BOMBONERA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A11 LATİF AĞA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K04-A12 MİKEA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A01 ÇİFTÇİ STELLA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A02 DAUGHTER OF GODDES (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A03 HALEP MEKTUBU (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A04 MAKE IT RAIN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A05 MAMMA LUNA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A06 POP ART (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A07 ROYAL CANDY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A08 ROYAL CIPHER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A09 SENSE OF GRACE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K05-A10 STRONG WIND (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A01 BER KEMAL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A02 CANMETE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A03 İTAATSİZ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A04 KARİZMATİK (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A05 RÜZGAR BARIŞ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A06 SEÇERHAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A07 TABİP (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K06-A08 TUNÇ SİPERİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K07-A01 BRAVE ORPHAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K07-A02 ÇEPNİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K07-A03 GLORIOUS LIFE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K07-A04 MOĞOL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K07-A05 ÖNDERBEYİM (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K08-A01 JOLENE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K08-A02 PEHLİVAN YILDIZI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K08-A03 PRENSES SYLVESTER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K08-A04 AEGEAN QUEEN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K08-A05 EL RIO (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20260403-IZMIR-K08-A06 MAKUL (aday satırlar: []) \| EKURI_GRUPLARI_CELISIYOR: 4. koşu program [[9, 12]] sonuç [] (üye numaraları, sıradan bağımsız) \| BIRINCI_BULUNAMADI: 1. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 2. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 3. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 4. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 5. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 6. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 7. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 8. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 9. koşu (GANYAN []) \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A1 1. ayak (1. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A1 2. ayak (2. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A1 3. ayak (3. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A1 4. ayak (4. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A1 5. ayak (5. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A1 6. ayak (6. koşu): sonuç dosyasında koşu yok \| ALTILI_ODEMESI_BULUNAMADI: 20260403-IZMIR-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A2 1. ayak (4. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A2 2. ayak (5. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A2 3. ayak (6. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A2 4. ayak (7. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A2 5. ayak (8. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20260403-IZMIR-A2 6. ayak (9. koşu): sonuçta birinci at yok \| ALTILI_ODEMESI_BULUNAMADI: 20260403-IZMIR-A2 son ayak (9. koşu) bahis satırında '2. 6'LI GANYAN' yok |
| 2026-04-18 | Ankara | EKSIK |  | program HTTP 404 URL_YOK; sonuc HTTP 404 URL_YOK |

Planlandığı gün dışında koşulan koşu: 8 (ayrıntı: `raporlar/2026-04_denetim.md` §5).

### 2026-05

Takvimdeki tüm çiftler TAMAMLANDI.

### 2026-06

Takvimdeki tüm çiftler TAMAMLANDI.

Altılı, birden çok kazanan kombinasyonlu: 20260621-ISTANBUL-A2 (her kombinasyon kendi ödemesine bağlı).

### 2026-07

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2026-07-31 | Bursa | EKSIK |  | program HTTP 200 ICERIK_DOGRULANDI |

Henüz denenmeyen: 2026-07-31 İstanbul

### 2026-08

Henüz denenmeyen: 2026-08-01 İzmir, 2026-08-01 Ankara, 2026-08-01 Diyarbakır, 2026-08-02 İstanbul, 2026-08-02 İzmir, 2026-08-03 Bursa, 2026-08-03 Elazığ, 2026-08-04 Ankara, 2026-08-04 Kocaeli, 2026-08-05 İstanbul, 2026-08-05 Elazığ, 2026-08-06 Ankara, 2026-08-06 Kocaeli, 2026-08-07 Bursa, 2026-08-07 İstanbul, 2026-08-08 İzmir, 2026-08-08 Ankara, 2026-08-08 Diyarbakır, 2026-08-09 İstanbul, 2026-08-09 İzmir, 2026-08-10 Bursa, 2026-08-10 Elazığ, 2026-08-11 Ankara, 2026-08-11 Kocaeli, 2026-08-12 İstanbul, 2026-08-12 Elazığ, 2026-08-13 Ankara, 2026-08-13 Kocaeli, 2026-08-14 Bursa, 2026-08-14 İstanbul, 2026-08-15 İzmir, 2026-08-15 Ankara, 2026-08-15 Diyarbakır, 2026-08-16 İstanbul, 2026-08-16 İzmir, 2026-08-17 Bursa, 2026-08-17 Elazığ, 2026-08-18 Ankara, 2026-08-18 Kocaeli, 2026-08-19 İstanbul, 2026-08-19 Elazığ, 2026-08-20 Ankara, 2026-08-20 Kocaeli, 2026-08-21 Bursa, 2026-08-21 İstanbul, 2026-08-22 İzmir, 2026-08-22 Ankara, 2026-08-22 Diyarbakır, 2026-08-23 İstanbul, 2026-08-23 İzmir, 2026-08-24 Bursa, 2026-08-24 Elazığ, 2026-08-25 Ankara, 2026-08-25 Kocaeli, 2026-08-26 İstanbul, 2026-08-26 Elazığ, 2026-08-27 Ankara, 2026-08-27 Kocaeli, 2026-08-28 Bursa, 2026-08-28 İstanbul, 2026-08-29 Ankara, 2026-08-29 İzmir, 2026-08-29 Diyarbakır, 2026-08-30 İstanbul, 2026-08-30 Adana, 2026-08-31 Bursa, 2026-08-31 Elazığ

### 2026-09

Henüz denenmeyen: 2026-09-01 Ankara, 2026-09-01 Kocaeli, 2026-09-02 İstanbul, 2026-09-02 Elazığ, 2026-09-03 Ankara, 2026-09-03 İzmir, 2026-09-04 Bursa, 2026-09-04 İzmir, 2026-09-05 Ankara, 2026-09-05 İstanbul, 2026-09-06 İstanbul, 2026-09-06 Adana, 2026-09-07 Bursa, 2026-09-07 Elazığ, 2026-09-08 Ankara, 2026-09-08 Kocaeli, 2026-09-09 İstanbul, 2026-09-09 Elazığ, 2026-09-10 Ankara, 2026-09-10 İzmir, 2026-09-11 Bursa, 2026-09-11 İstanbul, 2026-09-12 İzmir, 2026-09-12 Ankara, 2026-09-12 Diyarbakır, 2026-09-13 İstanbul, 2026-09-13 Adana, 2026-09-14 Bursa, 2026-09-14 Şanlıurfa, 2026-09-15 Ankara, 2026-09-15 Kocaeli, 2026-09-16 İstanbul, 2026-09-16 Elazığ, 2026-09-17 Ankara, 2026-09-17 İzmir, 2026-09-18 Bursa, 2026-09-18 İstanbul, 2026-09-19 İzmir, 2026-09-19 Ankara, 2026-09-19 Diyarbakır, 2026-09-20 İstanbul, 2026-09-20 Adana, 2026-09-21 Bursa, 2026-09-21 Şanlıurfa, 2026-09-22 Ankara, 2026-09-22 Kocaeli, 2026-09-23 İstanbul, 2026-09-23 Elazığ, 2026-09-24 Ankara, 2026-09-24 İzmir, 2026-09-25 Bursa, 2026-09-25 İstanbul, 2026-09-26 İzmir, 2026-09-26 Ankara, 2026-09-26 Diyarbakır, 2026-09-27 İstanbul, 2026-09-27 Adana, 2026-09-28 Bursa, 2026-09-28 Şanlıurfa, 2026-09-29 Adana, 2026-09-29 Kocaeli, 2026-09-30 İstanbul, 2026-09-30 Elazığ

### 2026-10

Henüz denenmeyen: 2026-10-01 Ankara, 2026-10-01 İzmir, 2026-10-02 Bursa, 2026-10-02 İstanbul, 2026-10-03 Ankara, 2026-10-03 İzmir, 2026-10-03 Diyarbakır, 2026-10-04 İstanbul, 2026-10-04 Adana, 2026-10-05 Bursa, 2026-10-05 Şanlıurfa, 2026-10-06 Adana, 2026-10-06 Kocaeli, 2026-10-07 İstanbul, 2026-10-07 Elazığ, 2026-10-08 Ankara, 2026-10-08 İzmir, 2026-10-09 Bursa, 2026-10-09 İstanbul, 2026-10-10 İzmir, 2026-10-10 Ankara, 2026-10-10 Diyarbakır, 2026-10-11 İstanbul, 2026-10-11 Adana, 2026-10-12 Bursa, 2026-10-12 Şanlıurfa, 2026-10-13 Adana, 2026-10-13 Kocaeli, 2026-10-14 İstanbul, 2026-10-14 Elazığ, 2026-10-15 Ankara, 2026-10-15 İzmir, 2026-10-16 Bursa, 2026-10-16 İstanbul, 2026-10-17 İzmir, 2026-10-17 Ankara, 2026-10-17 Diyarbakır, 2026-10-18 İstanbul, 2026-10-18 Adana, 2026-10-19 Bursa, 2026-10-19 Şanlıurfa, 2026-10-20 Adana, 2026-10-20 Kocaeli, 2026-10-21 İstanbul, 2026-10-21 Elazığ, 2026-10-22 Ankara, 2026-10-22 İzmir, 2026-10-23 Bursa, 2026-10-23 İstanbul, 2026-10-24 İzmir, 2026-10-24 Ankara, 2026-10-24 Diyarbakır, 2026-10-25 İstanbul, 2026-10-25 Adana, 2026-10-26 Bursa, 2026-10-26 Şanlıurfa, 2026-10-27 Adana, 2026-10-27 Kocaeli, 2026-10-28 İstanbul, 2026-10-28 Elazığ, 2026-10-29 Ankara, 2026-10-29 İzmir, 2026-10-30 Bursa, 2026-10-30 İstanbul, 2026-10-31 Ankara, 2026-10-31 Diyarbakır, 2026-10-31 İzmir

### 2026-11

Henüz denenmeyen: 2026-11-01 İstanbul, 2026-11-01 Adana, 2026-11-02 Bursa, 2026-11-02 Şanlıurfa, 2026-11-03 Adana, 2026-11-03 Kocaeli, 2026-11-04 İstanbul, 2026-11-04 Elazığ, 2026-11-05 Ankara, 2026-11-05 İzmir, 2026-11-06 Bursa, 2026-11-06 İstanbul, 2026-11-07 İzmir, 2026-11-07 Ankara, 2026-11-07 Diyarbakır, 2026-11-08 İstanbul, 2026-11-08 Adana, 2026-11-09 Bursa, 2026-11-09 Şanlıurfa, 2026-11-10 Adana, 2026-11-10 Kocaeli, 2026-11-11 İstanbul, 2026-11-11 Elazığ, 2026-11-12 Ankara, 2026-11-12 İzmir, 2026-11-13 Bursa, 2026-11-13 İstanbul, 2026-11-14 İzmir, 2026-11-14 Ankara, 2026-11-14 Diyarbakır, 2026-11-15 İstanbul, 2026-11-15 Adana, 2026-11-16 Bursa, 2026-11-16 Şanlıurfa, 2026-11-17 Adana, 2026-11-17 Antalya, 2026-11-18 İstanbul, 2026-11-18 Elazığ, 2026-11-19 İzmir, 2026-11-19 Antalya, 2026-11-20 Antalya, 2026-11-20 İzmir, 2026-11-21 Adana, 2026-11-21 İstanbul, 2026-11-22 İzmir, 2026-11-22 Adana

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
