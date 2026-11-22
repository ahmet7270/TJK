# Yıllık durum: 22.11.2025 – 22.11.2026

Üretim: `kontrol/yillik/durum_yaz.sh` (her ay için `tjk_denetim`, veri tabanı salt okunur) · 2026-11-22T18:54Z
Veri tabanı: `tjk_yillik.sqlite` · SHA-256 `ab997fc3dc56eacdf4a1f7364698a115f601896dfb4d78defc0e53e11a00bae5`

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
| 2026-07 | 66 | 66 | 0 | 0 | 0 | 0 | 0 | 562 | 5704 | 120 | 0 | ok | aynı | yok | tamamlandı |
| 2026-08 | 67 | 67 | 0 | 0 | 0 | 0 | 0 | 569 | 5799 | 124 | 0 | ok | aynı | yok | tamamlandı |
| 2026-09 | 63 | 61 | 0 | 2 | 0 | 0 | 0 | 516 | 5583 | 115 | 0 | ok | aynı | yok | eksik var |
| 2026-10 | 67 | 64 | 1 | 2 | 0 | 0 | 0 | 549 | 5779 | 120 | 0 | ok | aynı | yok | eksik var |
| 2026-11 | 46 | 46 | 0 | 0 | 0 | 0 | 0 | 380 | 3895 | 84 | 0 | ok | aynı | yok | tamamlandı |
| **Toplam** | **760** | **749** | **5** | **6** | **0** | **0** | | | | | | | | | |

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

Takvimdeki tüm çiftler TAMAMLANDI.

Altılı, birden çok kazanan kombinasyonlu: 20260724-BURSA-A1, 20260724-BURSA-A2, 20260725-IZMIR-A1 (her kombinasyon kendi ödemesine bağlı).

### 2026-08

Takvimdeki tüm çiftler TAMAMLANDI.

### 2026-09

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2026-09-28 | Bursa | EKSIK |  | program HTTP 200 ICERIK_DOGRULANDI; sonuc HTTP 200 BICIM_HATASI |
| 2026-09-29 | Kocaeli | EKSIK |  | program HTTP 200 ICERIK_DOGRULANDI; sonuc HTTP 200 BICIM_HATASI |

### 2026-10

| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |
|---|---|---|---|---|
| 2026-10-16 | Bursa | EKSIK |  | program HTTP 200 ICERIK_DOGRULANDI; sonuc HTTP 200 BICIM_HATASI |
| 2026-10-19 | Bursa | EKSIK |  | program HTTP 200 ICERIK_DOGRULANDI; sonuc HTTP 200 BICIM_HATASI |
| 2026-10-28 | Elazığ | INCELEME_GEREKLI | 45 | ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A01 DREAM CHASER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A02 CENTURIES VICTORY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A03 ERULİNA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A04 PROBABLY THE BEST (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A05 REALISTIC GIRL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A06 SÜPER ELOŞ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A07 HELLO KUTTY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A08 QUEEN YELDA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A01 VİTAL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A02 CENGEBEY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A03 DEMİR ATLAS (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A04 KILIÇBAĞI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A05 OLGUNADAM (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A06 QUATRO (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A07 İREM HATUN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A08 BALATÜRK (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A09 BİRÇETİN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A10 KING PELE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K03-A11 ÖZAYDIN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A01 JAZZMASTER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A02 SIDNEY SHOW (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A03 THE LAST UNICORN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A04 VAN KALESİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A05 AŞAN GOLD (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A06 EAGLE KHAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A07 DESERT FEVER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A08 EUROFIGHTER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A09 EL LEON (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A10 KING ANCESTOR (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A11 KATANA BOY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K04-A12 THE SNAKE PIT (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A01 ASİLBOY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A02 APOHİKO (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A03 JER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A04 LORD OF THE SEA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A05 SON ZAYN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A06 WILD FALCON (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A07 IRRITABLE GIRL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A08 MI GULIETTA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K05-A09 THE FIRST DANCE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K06-A01 BRAVO GÜR (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K06-A02 ALPERİN KIZI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K06-A03 CHEF BUĞRA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K06-A04 MELEKCAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K06-A05 WAKABAYASHI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K06-A06 KING YILDIRIM (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A01 KIRDOĞAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A02 ATAKAYA HÜKÜMDARI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A03 EMOŞUN KIZI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A04 KİNG BEDİRHAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A05 ROYAL EŞLİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A06 EDEPLİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A07 KIR PENÇE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A08 KOĞUN SESİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A09 MISRADA (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K07-A10 KALPTENSİN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A01 GREEN GÖLE (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A02 HALLAND (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A03 BLOODSHOT (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A04 GOLDEN COOGER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A05 MENKAR (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A06 KİNG İHSAN (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A07 SEHERİN YELİ (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A08 KRAL ALPER (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A09 RHAEGAL (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A10 DUMANLI PARS (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A11 ÇUKUROVA PEHLİVANI (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A12 BEYAZ SARAY (aday satırlar: []) \| ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K08-A13 ZALIMIN OĞLU (aday satırlar: []) \| EKURI_GRUPLARI_CELISIYOR: 3. koşu program [[5, 7]] sonuç [] (üye numaraları, sıradan bağımsız) \| EKURI_GRUPLARI_CELISIYOR: 7. koşu program [[3, 7]] sonuç [] (üye numaraları, sıradan bağımsız) \| BIRINCI_BULUNAMADI: 2. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 3. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 4. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 5. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 6. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 7. koşu (GANYAN []) \| BIRINCI_BULUNAMADI: 8. koşu (GANYAN []) \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A1 2. ayak (2. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A1 3. ayak (3. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A1 4. ayak (4. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A1 5. ayak (5. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A1 6. ayak (6. koşu): sonuç dosyasında koşu yok \| ALTILI_ODEMESI_BULUNAMADI: 20261028-ELAZIG-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A2 1. ayak (3. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A2 2. ayak (4. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A2 3. ayak (5. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A2 4. ayak (6. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A2 5. ayak (7. koşu): sonuç dosyasında koşu yok \| ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A2 6. ayak (8. koşu): sonuç dosyasında koşu yok \| ALTILI_ODEMESI_BULUNAMADI: 20261028-ELAZIG-A2 son ayak (8. koşu) bahis satırında '2. 6'LI GANYAN' yok |

Planlandığı gün dışında koşulan koşu: 7 (ayrıntı: `raporlar/2026-10_denetim.md` §5).

### 2026-11

Takvimdeki tüm çiftler TAMAMLANDI.

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
