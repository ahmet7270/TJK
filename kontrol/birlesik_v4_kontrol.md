# Gerçek v3 yedeği + Mart 2026 → birleşik v4 veritabanı

Tarih: 2026-11-22 · Araç: `tjk_ayristirici` 0.13.0 · Sorgu: `tjk_sorgu` · Şema 4

## Sonuç: tüm kontroller BAŞARILI

| Kontrol | Beklenen | Bulunan | Sonuç |
|---|---|---|---|
| Yedek SHA-256 | `af5d47df…dffa6a` | `af5d47df82b365ad0f056e30c32615918ff63e5c23c9f167edda03efdadffa6a` | ✅ |
| Yedek değişmedi (işlem sonunda) | aynı SHA | aynı SHA (dosya salt okunur yapıldı) | ✅ |
| Benzersiz tarih–hipodrom çifti | 63 | 63 (`COUNT(*)` = `COUNT(DISTINCT tarih‖hipodrom)` = 63) | ✅ |
| Koşu | 497 | 497 | ✅ |
| Katılım | 4.901 | 4901 | ✅ |
| Sonuç | 4.901 | 4901 | ✅ |
| Altılı | 109 | 109 | ✅ |
| 6–10 Mart çift sayılmadı | 0 tekrar | 0 (`GROUP BY gun_id HAVING COUNT(*)>1` boş) | ✅ |
| 28.12.2025 Adana + 30.06.2026 Ankara korunuyor | 16 koşu, 125 katılım, 4 altılı | 16 / 125 (125 sonuç) / 4; 13/13 karşılaştırma sorgusu v3 ile aynı | ✅ |
| Bu iki günün yeni alanları boş | hepsi NULL | 16/16 koşuda 9 yeni alan NULL; ödeme satırı 0; ORTAK_BIRINCI 0 | ✅ |
| Mart durumları | 60 TAMAM, yalnızca 15.03 Adana incelemede | 60 TAMAM, 1 INCELEME_GEREKLI = `20260315-ADANA` | ✅ |
| Ertelenen koşuların gerçek kaynakları | 15.03 K3–8 → 16.03; 21.03 K1–9 → 23.03 | aynı (15 koşu); katılım/bahis kaynak izleri de aynı | ✅ |
| Antalya A2 iki ödeme | …/5 = 2.093,81; …/6 = 1.686,12 | aynı, kendi bahis satırlarına (K08 sıra 4, 5) bağlı; 5 ve 6 `ORTAK_BIRINCI` | ✅ |
| İkinci Mart aktarımı | kayıt ve değerler aynı | 16 tablonun tam dökümü aynı; Mart dışı 2 gün 13/13 aynı | ✅ |
| `PRAGMA foreign_key_check` | 0 satır | 0 satır | ✅ |
| `PRAGMA integrity_check` | ok | ok | ✅ |

**Birleşik dosya:** `tjk_calisma_v4_birlesik.sqlite`, 4.689.920 bayt, SHA-256
`a4dce9ff4946cd74bb4ef23983eff4ff39acb33a3bd0aa40e94cce69da036a66`. Paylaşım paketi: `tjk_calisma_v4_birlesik.zip`, SHA-256
`c223e76e0dfbc163e51271d9fba4af6a95aa3d59e22dbdd4c52ebb472e938c9d`; içinde veritabanı, `.sha256` ve kısa kontrol özeti var.
Paket açılıp `sha256sum -c` ile doğrulandı. SQLite dosyası ve zip Git'e eklenmedi.

## Girdi: v3 yedeği

`tjk_calisma_s3_yedek.zip` → `tjk_calisma_s3_yedek.sqlite` (1.048.576 bayt), `.sha256` ve `yedek_kontrol.txt`.
`user_version` 3, `integrity_check` ok, `foreign_key_check` 0 satır. İçerik: 12 gün (hepsi TAMAM, ayrıştırıcı 0.9.0), 94 koşu,
944 katılım, 944 sonuç, 21 altılı, 24 kaynak dosya:

- Mart dışı: `20251228-ADANA` (8 koşu), `20260630-ANKARA` (8 koşu)
- 6–10 Mart: `20260306-BURSA`, `-IZMIR`, `20260307-ADANA`, `-ISTANBUL`, `20260308-ADANA`, `-IZMIR`, `20260309-BURSA`,
  `-SANLIURFA`, `20260310-ADANA`, `-ANTALYA`. Bunlar Mart'ın 61 çiftiyle örtüşüyor.

Beklenen birleşim: 12 + 61 − 10 = 63 çift; koşu 16 + 481 = 497; katılım 125 + 4776 = 4901; altılı 4 + 105 = 109.

## Adımlar

```bash
sha256sum tjk_calisma_s3_yedek.sqlite        # af5d47df…dffa6a = beklenen
chmod a-w tjk_calisma_s3_yedek.sqlite         # yedek korunur
# Mart dışı 2 günün v3 görüntüsü (13 sorgu) -> once/
tjk_ayristirici goc --kaynak tjk_calisma_s3_yedek.sqlite --hedef tjk_calisma_v4_birlesik.sqlite
#   "Kaynak SHA-256 önce/sonra: af5d47df… / af5d47df… (DEĞİŞMEDİ)", "hiçbir tabloda satır kaybı yok",
#   hedef integrity_check ok, foreign_key_check 0
aktar.sh kontrol/mart_sqlite_girdiler.tsv tjk_calisma_v4_birlesik.sqlite aktar1   # 61/61, çıkış 0, 60 TAMAM
aktar.sh kontrol/mart_sqlite_girdiler.tsv tjk_calisma_v4_birlesik.sqlite aktar2   # ikinci kez, çıkış 0
```

`aktar.sh` her satır için mevcut aracı çağırıyor (`kontrol/mart_sqlite_prova.md` ile aynı sürücü):
`tjk_ayristirici PROGRAM SONUC ÇIKTI --sqlite DB [--ek-sonuc …] [--ertelenme-notu …]`. Ham CSV'ler ve girdi dosyası commit'li
hâlleriyle aynı (`git diff --quiet HEAD -- kontrol/ham kontrol/mart_sqlite_girdiler.tsv`). Aktarım her günü tek işlemde silip
yeniden yazdığı için örtüşen 6–10 Mart günleri yerinde güncellendi, ikinci bir kayıt oluşmadı.

## SQL kontrolleri ve sonuçları

```sql
SELECT COUNT(*), COUNT(DISTINCT tarih||hipodrom), (SELECT COUNT(*) FROM kosu), (SELECT COUNT(*) FROM katilim),
       (SELECT COUNT(*) FROM sonuc), (SELECT COUNT(*) FROM altili) FROM yaris_gunu;
-- 63 | 63 | 497 | 4901 | 4901 | 109
SELECT CASE WHEN tarih LIKE '2026-03-%' THEN 'Mart' ELSE 'diğer' END, COUNT(*), SUM(gun_durumu='TAMAM'), SUM(gun_durumu<>'TAMAM'),
       group_concat(CASE WHEN gun_durumu<>'TAMAM' THEN gun_id END) FROM yaris_gunu GROUP BY 1;
-- Mart | 61 | 60 | 1 | 20260315-ADANA ;  diğer | 2 | 2 | 0 | NULL
SELECT ayristirici_surumu, COUNT(*) FROM yaris_gunu GROUP BY 1;          -- 0.13.0 | 61 ;  0.9.0 | 2
SELECT gun_id, COUNT(*) FROM yaris_gunu WHERE gun_id BETWEEN '20260306' AND '20260311' GROUP BY 1 HAVING COUNT(*)>1;  -- 0 satır
```

### Mart dışı iki gün (28.12.2025 Adana, 30.06.2026 Ankara)

Aynı 13 sorgu dört anda çalıştırıldı ve çıktılar `cmp` ile karşılaştırıldı: v3 yedeği, `goc` sonrası, 1. Mart aktarımı sonrası,
2. Mart aktarımı sonrası. Sorgular: gün + kaynak SHA'ları + sürüm, inceleme nedenleri, koşu (v3 sütunları), katılım + at anahtarı,
sonuç, AGF, bahis, bahis uyarısı, GANYAN kontrolü, altılı, altılı ayak, geçerli numaralar ve sayılar.
**Her üç karşılaştırmada 13/13 aynı.** Sayılar: 16 koşu, 125 katılım, 125 sonuç, 4 altılı.

```sql
SELECT gun_id, COUNT(*), SUM(planlanan_tarih IS NULL AND planlanan_saat IS NULL AND planlanan_pist IS NULL AND gerceklesen_tarih IS NULL
       AND gerceklesen_saat IS NULL AND gerceklesen_pist IS NULL AND tarih_durumu IS NULL AND ertelenme_notu IS NULL AND sonuc_kaynak_id IS NULL)
  FROM kosu WHERE gun_id IN ('20251228-ADANA','20260630-ANKARA') GROUP BY 1;
-- 20251228-ADANA | 8 | 8 ;  20260630-ANKARA | 8 | 8
SELECT COUNT(*) FROM altili_odeme o JOIN altili a USING(altili_id) WHERE a.gun_id IN (…);              -- 0
SELECT COUNT(*) FROM altili_ayak_gecerli ag JOIN altili a USING(altili_id) WHERE a.gun_id IN (…) AND neden='ORTAK_BIRINCI';  -- 0
```

Kaynağı olmayan yeni alanlar boş kaldı; bu günlerin ham dosyaları depoda olmadığı için tahminle doldurulmadı.

### Ertelemeler ve kaynaklar

```sql
SELECT ko.kosu_id, ko.planlanan_tarih, ko.gerceklesen_tarih, ko.gerceklesen_saat, ko.tarih_durumu, kd.dosya_adi
  FROM kosu ko JOIN kaynak_dosya kd ON kd.kaynak_id=ko.sonuc_kaynak_id WHERE ko.tarih_durumu='ERTELENDI' ORDER BY 1;
-- 20260315-ADANA-K03…K08: 2026-03-15 → 2026-03-16 (11:30…14:00), 20260316-ADANA-sonuc.csv
-- 20260321-ADANA-K01…K09: 2026-03-21 → 2026-03-23 (11:00…14:45), 20260323-ADANA-sonuc.csv     (15 satır)
SELECT ko.gun_id, v.sonuc_dosyasi, COUNT(*) FROM v_kaynak_izi v JOIN katilim k USING(katilim_id) JOIN kosu ko USING(kosu_id)
 WHERE ko.gun_id IN ('20260315-ADANA','20260321-ADANA') GROUP BY 1,2;
-- 20260315-ADANA | 20260315-ADANA-sonuc.csv | 16 ;  20260315-ADANA | 20260316-ADANA-sonuc.csv | 61 ;  20260321-ADANA | 20260323-ADANA-sonuc.csv | 89
-- v_bahis_kaynak_izi: 15.03 K01–K02 → 15.03 dosyası; K03–K08 → 16.03 dosyası; 21.03 K01–K09 → 23.03 dosyası
```

### Antalya 31.03 A2 ve 15.03 Adana ödemeleri

```sql
SELECT o.altili_id, oa.program_at_no, o.kombinasyon_ham, o.odeme_ham, o.odeme_tl, b.ham_ibare FROM altili_odeme o
  JOIN altili_odeme_ayak oa ON oa.altili_id=o.altili_id AND oa.odeme_sira=o.sira AND oa.ayak_no=6
  JOIN bahis_sonucu b ON b.kosu_id=o.bahis_kosu_id AND b.sira=o.bahis_sira WHERE o.altili_id='20260331-ANTALYA-A2' ORDER BY 2;
-- A2 | 5 | 7/8/1/1/8/5 | 2.093,81 TL | 2093.81 | 2. 6'LI GANYAN(7/8/1/1/8/5) :2.093,81 TL
-- A2 | 6 | 7/8/1/1/8/6 | 1.686,12 TL | 1686.12 | 2. 6'LI GANYAN(7/8/1/1/8/6) :1.686,12 TL
SELECT program_at_no, neden FROM v_altili_gecerli WHERE altili_id='20260331-ANTALYA-A2' AND ayak_no=6;
-- 5 | ORTAK_BIRINCI ;  6 | ORTAK_BIRINCI
SELECT altili_id, odeme_tl, sonuc_durumu, (SELECT COUNT(*) FROM altili_odeme o WHERE o.altili_id=a.altili_id) FROM altili a
 WHERE gun_id='20260315-ADANA';
-- A1 | NULL | NULL | 0   (bilinmiyor) ;  A2 | 19387.81 | ODENDI | 1
```

### İkinci aktarım ve bütünlük

16 tablonun `SELECT * … ORDER BY 1,2,3` dökümü iki aktarım arasında **aynı**. Sayılar: kaynak_dosya 127, yaris_gunu 63,
inceleme_nedeni 2, kosu 497, at 2810, katilim 4901, agf_gozlem 6532, sonuc 4901, bahis_sonucu 3926, bahis_uyarisi 20,
ganyan_kontrol 497, altili 109, altili_ayak 654, altili_ayak_gecerli 740, altili_odeme 105, altili_odeme_ayak 716.
Aktarım özet satırları da aynı.

```sql
PRAGMA user_version;       -- 4
PRAGMA foreign_key_check;  -- 0 satır
PRAGMA integrity_check;    -- ok
```

`kaynak_dosya` 127 = Mart 123 (61 program + 61 sonuç + 16.03 ek sonuç) + Mart dışı 2 günün 4 dosyası. Yedekteki 6–10 Mart kaynak
satırları aynı SHA-256'lı dosyalar olduğu için yeniden kullanıldı; sahipsiz kaynak kalmadı.

## Yedekteki 6–10 Mart kayıtlarıyla karşılaştırma (bilgi)

Yedek (0.9.0) ile birleşik (0.13.0) arasında 6–10 Mart için:

- **Aynı:** program ve sonuç SHA-256'ları; gün başına koşu ve katılım sayıları; tüm `sonuc` alanları (liste/bitiş sırası, durum,
  derece, ganyan, fark, kaynak satırı); altılı kombinasyon ve ödemeleri; tüm bahis satırları.
- **Fark:** yalnızca `YP` ekinin ayıklanması (0.12.0 düzeltmesi), 3 katılımda:
  - `20260306-BURSA-K06-A04`: `ALTUNÇ DB YP` → at_adi `ALTUNÇ`, kodlar `DB YP`
  - `20260307-ADANA-K05-A07`: `LAMAZİ GOGO DB SKG SK YP` → at_adi `LAMAZİ GOGO`, kodlar `DB SKG SK YP`
  - `20260307-ISTANBUL-K07-A01`: `ALPHA JUNIOR DB YP` → at_adi `ALPHA JUNIOR`, kodlar `DB YP`

  Ham ad `at_isim_ham` aynen korunuyor.
