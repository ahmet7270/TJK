# Mart 2026 SQLite aktarım provası

Tarih: 2026-11-22 · Aktarım aracı: `tjk_ayristirici` 0.12.0 (`--sqlite`, `veritabani::aktar`) · Şema: `tjk_ayristirici/src/sema.sql`
(`PRAGMA user_version = 3`), değiştirilmedi · Sorgu aracı: `tjk_sorgu` (salt okunur; ortamda `sqlite3` komut satırı aracı yok).

## Sonuç özeti

| # | Kontrol | Sonuç |
|---|---|---|
| K1 | 61 planlı çift; 60 tamam, 1 incelemede | **BAŞARILI** |
| K2a | Ertelenen koşular ikinci kez sayılmamış (15→16, 21→23 Mart Adana) | **BAŞARILI** |
| K2b | Planlanan ve gerçekleşen tarih ayrı saklanıyor | **BAŞARISIZ: şema eksik** (aşağıda) |
| K3 | 21.03 İstanbul LAZMAN + LEONİDAS eküri kaydı | **BAŞARILI** |
| K4a | 31.03 Antalya 8. koşu 5 ve 6 ortak birinci | **BAŞARILI** |
| K4b | 2. altılının iki ödemesi doğru kombinasyonlara ayrı ayrı bağlı | **BAŞARISIZ: şema eksik** (değerler var, altılı bağı yok) |
| K5 | 15.03 Adana: 8 koşu / 77 at; 3. koşu GANYAN ve 1. altılı bilinmiyor; 2. altılı 19.387,81 TL | **BAŞARILI** (kaynak izi uyarısı K2b'de) |
| K6 | İkinci aktarımda koşu/katılım/altılı ödeme kayıtları çoğalmadı, değerler aynı | **BAŞARILI** |

**7 kontrol başarılı, 2 kontrol başarısız** (ikisinin nedeni de şemanın bilgiyi kayıpsız saklayamaması).

## Ortam ve girdiler

- Ana veritabanı (`tjk_calisma_s3.sqlite` vb.) bu ortamda **yok** (dosya sistemi tarandı; yalnızca sistem dosyaları var).
  Bu yüzden depodaki şemayla boş, geçici bir Mart veritabanı oluşturuldu:
  `/tmp/claude-0/…/scratchpad/prova/mart_2026_prova.sqlite`. Depo dışında, Git'e eklenmedi.
- Ham CSV'ler yeniden indirilmedi. `git diff --quiet HEAD -- kontrol/ham` temiz ve izlenmeyen dosya 0, yani dosyalar daha önce
  SHA-256 ile doğrulanıp commit'lenen hâlleriyle aynı.
- Girdi eşlemesi (`girdiler.tsv`, 61 satır): resmî takvimdeki her çift için `kontrol/ham/YYYYAAGG-HIPODROM-program.csv` ve `-sonuc.csv`. İstisnalar:
  - 06.03 İzmir programı: `06.03.2026-İzmir-GunlukYarisProgrami-TR.csv`
  - 15.03 Adana: sonuç `20260315-ADANA-sonuc.csv` + `--ek-sonuc 20260316-ADANA-sonuc.csv` + `--ertelenme-notu "TJK duyuru 52071: … (kullanıcı beyanı)"`
  - 21.03 Adana: sonuç `20260323-ADANA-sonuc.csv` + `--ertelenme-notu "21.03.2026 Adana 34. yarış günü sonuçları 23.03.2026'da yayımlandı (kullanıcı beyanı)"`

## Komutlar

```bash
# Sürücü (aktar.sh): her satır için mevcut aracı çağırır
#   tjk_ayristirici PROGRAM SONUC CIKTI/GUN --sqlite DB [--ek-sonuc DOSYA] [--ertelenme-notu METİN]
P=/tmp/claude-0/.../scratchpad/prova
rm -f $P/mart_2026_prova.sqlite
$P/aktar.sh $P/girdiler.tsv $P/mart_2026_prova.sqlite $P/calisma1 > $P/aktarim1.txt   # 1. aktarım, çıkış 0
# kontroller: tjk_sorgu $P/mart_2026_prova.sqlite "SQL" ...
# 14 tablonun tam dökümü: tjk_sorgu DB "SELECT * FROM <tablo> ORDER BY 1,2,3"  -> dokum1/
$P/aktar.sh $P/girdiler.tsv $P/mart_2026_prova.sqlite $P/calisma2 > $P/aktarim2.txt   # 2. aktarım, çıkış 0
# aynı döküm -> dokum2/ ; cmp dokum1/<tablo>.txt dokum2/<tablo>.txt
```

Sürücü betiğinin özü:

```bash
while IFS=$'\t' read -r gun p r ek; do
  args=("$p" "$r" "$KOK/$gun" --sqlite "$DB")
  if [ -n "$ek" ]; then eks=${ek%%|*}; notu=${ek#*|}; [ -n "$eks" ] && args+=(${eks}); args+=(--ertelenme-notu "$notu"); fi
  tjk_ayristirici "${args[@]}" > "$KOK/$gun.log" 2>&1
done < girdiler.tsv
```

1. aktarım: 61/61 gün hatasız aktarıldı (60 `GÜN DURUMU=TAMAM`, 1 `INCELEME_GEREKLI`). Log toplamları: koşu 481, katılım 4776,
sonuç 4776, altılı 105, ayak 630, geçerli no 711, AGF gözlemi 6342. Bunlar veritabanındaki sayılarla aynı.

---

## K1: 61 planlı çift, 60 tamam, 1 inceleme

```sql
SELECT COUNT(*) AS gun, SUM(gun_durumu='TAMAM') AS tamam, SUM(gun_durumu='INCELEME_GEREKLI') AS inceleme,
       MIN(tarih), MAX(tarih) FROM yaris_gunu;
-- 61 | 60 | 1 | 2026-03-01 | 2026-03-31
SELECT gun_id, sira, neden FROM inceleme_nedeni ORDER BY gun_id, sira;
-- 20260315-ADANA | 1 | GANYAN_SATIRI_BULUNAMADI: 3. koşu (birinci [1])
-- 20260315-ADANA | 2 | ALTILI_ODEMESI_BULUNAMADI: 20260315-ADANA-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok
SELECT tarih || char(9) || hipodrom FROM yaris_gunu ORDER BY 1;
-- `diff` ile mart_2026_resmi_takvim.tsv (61 satır) karşılaştırıldı: fark yok
```

Not: Şemada "planlı çift" (takvim) tablosu yok. 61 planlı çift, `yaris_gunu` satırlarının resmî takvim dosyasıyla birebir aynı olmasıyla doğrulandı.

## K2: Adana ertelemeleri

```sql
SELECT g.gun_id, g.tarih, g.yaris_gunu_no, kp.dosya_adi, ks.dosya_adi FROM yaris_gunu g
  JOIN kaynak_dosya kp ON kp.kaynak_id=g.program_kaynak_id JOIN kaynak_dosya ks ON ks.kaynak_id=g.sonuc_kaynak_id
 WHERE g.gun_id IN ('20260315-ADANA','20260321-ADANA');
-- 20260315-ADANA | 2026-03-15 | 32 | 20260315-ADANA-program.csv | 20260315-ADANA-sonuc.csv
-- 20260321-ADANA | 2026-03-21 | 34 | 20260321-ADANA-program.csv | 20260323-ADANA-sonuc.csv
SELECT COUNT(*) FROM yaris_gunu WHERE gun_id IN ('20260316-ADANA','20260323-ADANA')
    OR (hipodrom='Adana' AND tarih IN ('2026-03-16','2026-03-23'));                 -- 0
SELECT hipodrom, yaris_gunu_no, COUNT(*) FROM yaris_gunu WHERE hipodrom='Adana' AND yaris_gunu_no IN (32,34) GROUP BY 1,2;
-- Adana | 32 | 1 ;  Adana | 34 | 1
SELECT ko.gun_id, COUNT(DISTINCT ko.kosu_id), COUNT(DISTINCT ko.kosu_no), COUNT(k.katilim_id), COUNT(s.katilim_id) FROM kosu ko
  LEFT JOIN katilim k ON k.kosu_id=ko.kosu_id LEFT JOIN sonuc s ON s.katilim_id=k.katilim_id
 WHERE ko.gun_id IN ('20260315-ADANA','20260321-ADANA') GROUP BY ko.gun_id;
-- 20260315-ADANA | 8 | 8 | 77 | 77 ;  20260321-ADANA | 9 | 9 | 89 | 89
```

**K2a: BAŞARILI.** 16.03 ve 23.03 için ayrı gün kaydı yok. Her yarış günü numarası bir kez var. Koşular ve katılımlar tek sayılmış.

```sql
SELECT kosu_id, kosu_no, saat, pist FROM kosu WHERE kosu_id IN ('20260315-ADANA-K03','20260321-ADANA-K04');
-- 20260315-ADANA-K03 | 3 | 19:15 | Kum     (gerçekleşen: 16.03 11:30)
-- 20260321-ADANA-K04 | 4 | 15:30 | Çim     (gerçekleşen: 23.03 12:30, pist Kum)
SELECT 'kosu', name FROM pragma_table_info('kosu') WHERE name LIKE '%tarih%' OR name LIKE '%gercek%' OR name LIKE '%plan%'
UNION ALL SELECT 'yaris_gunu', name FROM pragma_table_info('yaris_gunu') WHERE name LIKE '%tarih%'
UNION ALL SELECT 'sonuc', name FROM pragma_table_info('sonuc') WHERE name LIKE '%tarih%' OR name LIKE '%kaynak%';
-- yaris_gunu | tarih ;  sonuc | sonuc_kaynak_satir
SELECT COUNT(*) FROM kaynak_dosya WHERE dosya_adi='20260316-ADANA-sonuc.csv';      -- 0
SELECT katilim_id, sonuc_dosyasi, sonuc_kaynak_satir FROM v_kaynak_izi
 WHERE katilim_id IN ('20260315-ADANA-K01-A01','20260315-ADANA-K03-A01','20260315-ADANA-K08-A01');
-- K01-A01 | 20260315-ADANA-sonuc.csv | 6
-- K03-A01 | 20260315-ADANA-sonuc.csv | 6    <- gerçekte 20260316-ADANA-sonuc.csv satır 6
-- K08-A01 | 20260315-ADANA-sonuc.csv | 96   <- 15.03 sonuç dosyası 29 satır; satır 96 16.03 dosyasında
```

**K2b: BAŞARISIZ (şema bilgiyi kayıpsız saklayamıyor).** Eksik olan tablo ve alanlar:

- `yaris_gunu.tarih` tek tarih (program = planlanan tarih). **Koşu başına gerçekleşen tarih için alan yok.** `kosu` tablosunda
  `gerceklesen_tarih`, `gerceklesen_saat`, `pist_sonuc`, `tarih_durumu`, `ertelenme_notu` alanları ya da ayrı bir `kosu_tarihi`
  tablosu yok. Ayrıştırıcının `kosu_tarihleri.csv` çıktısındaki bu bilgiler veritabanına aktarılmıyor.
- `kosu.saat` ve `kosu.pist` programdan yazılıyor. 21.03 4. koşunun pist değişikliği (Çim → Kum) ve tüm gerçekleşen saatler kayboluyor.
- **Kaynak izi:** `yaris_gunu.sonuc_kaynak_id` gün başına tek dosya. `kaynak_dosya` ek sonuç dosyasını (16.03) içermiyor; koşu
  başına `sonuc_kaynak_id` alanı yok. Bu yüzden 15.03 Adana 3.–8. koşularının `sonuc.sonuc_kaynak_satir` ve
  `bahis_sonucu.sonuc_kaynak_satir` değerleri 16.03 dosyasının satır numaraları olduğu hâlde `v_kaynak_izi` bunları 15.03 dosyasına
  bağlıyor (yanlış iz). 21.03 Adana'da gerçekleşen tarih yalnızca kaynak dosya adında (`20260323-…`) görünüyor; bu bir tarih alanı değil.

## K3: 21.03 İstanbul eküri

```sql
SELECT k.katilim_id, k.program_at_no, k.at_isim_ham, a.at_adi, k.isim_kodlari, k.ekuri_grubu, k.program_kaynak_satir
  FROM katilim k JOIN at a ON a.at_id=k.at_id WHERE k.kosu_id='20260321-ISTANBUL-K06' AND k.program_at_no IN (5,6);
-- 20260321-ISTANBUL-K06-A05 | 5 | LAZMAN K DB        | LAZMAN   | K DB      | 5,6 | 104
-- 20260321-ISTANBUL-K06-A06 | 6 | LEONİDAS SKG SK YP | LEONİDAS | SKG SK YP | 5,6 | 105
SELECT gun_durumu, (SELECT COUNT(*) FROM inceleme_nedeni i WHERE i.gun_id=g.gun_id) FROM yaris_gunu g WHERE gun_id='20260321-ISTANBUL';
-- TAMAM | 0
```

**BAŞARILI.** Eküri grubu iki katılımda da `5,6`. Ham ad `at_isim_ham`'da korunuyor.

## K4: 31.03 Antalya 8. koşu

```sql
SELECT k.program_at_no, a.at_adi, s.sonuc_liste_sirasi, s.bitis_sirasi, s.derece_ham, s.ganyan_oran, s.fark_ham, s.sonuc_kaynak_satir
  FROM sonuc s JOIN katilim k ON k.katilim_id=s.katilim_id JOIN at a ON a.at_id=k.at_id
 WHERE k.kosu_id='20260331-ANTALYA-K08' AND s.sonuc_liste_sirasi<=3 ORDER BY s.sonuc_liste_sirasi;
-- 5  | MAVİŞİN KIZI | 1 | 1 | 1:33.70 | 1.8  | At Başı  | 105
-- 6  | ŞUKELA       | 2 | 1 | 1:33.70 | 1.6  | Baş      | 106
-- 11 | HADEHANIM    | 3 | 3 | 1:33.72 | 3.85 | 1,5  Boy | 107
SELECT * FROM ganyan_kontrol WHERE kosu_id='20260331-ANTALYA-K08';
-- 20260331-ANTALYA-K08 | 5,6 | 5,6 | AT_BASI_UYUMLU
```

**K4a: BAŞARILI.** 5 ve 6'nın bitiş sırası 1, liste sıraları 1 ve 2 korunuyor, sonraki at 3. sırada.

```sql
SELECT altili_id, oyun_adi, baslangic_kosu_no, kombinasyon_ham, odeme_ham, odeme_tl, sonuc_durumu, sonuc_kaynak_satir
  FROM altili WHERE gun_id='20260331-ANTALYA' ORDER BY 1;
-- 20260331-ANTALYA-A1 | 1. 6'LI GANYAN | 1 | 5,7/2,3/7/8/1/1 | 1.787,18 TL | 1787.18 | ODENDI | 85
-- 20260331-ANTALYA-A2 | 2. 6'LI GANYAN | 3 | NULL            | NULL        | NULL    | NULL   | NULL
SELECT sira, oyun, kombinasyon_ham, tutar_ham, tutar_tl, sonuc_durumu, sonuc_kaynak_satir FROM bahis_sonucu
 WHERE kosu_id='20260331-ANTALYA-K08' AND oyun IN ('GANYAN','2. 6''LI GANYAN') ORDER BY sira;
-- 1 | GANYAN         | 5           | 1,80 TL     | 1.8     | ODENDI | 122
-- 2 | GANYAN         | 6           | 1,60 TL     | 1.6     | ODENDI | 122
-- 4 | 2. 6'LI GANYAN | 7/8/1/1/8/5 | 2.093,81 TL | 2093.81 | ODENDI | 122
-- 5 | 2. 6'LI GANYAN | 7/8/1/1/8/6 | 1.686,12 TL | 1686.12 | ODENDI | 122
SELECT a.altili_id, ay.kosu_id, b.kombinasyon_ham, b.tutar_ham, b.tutar_tl FROM altili a
  JOIN altili_ayak ay ON ay.altili_id=a.altili_id AND ay.ayak_no=6
  JOIN bahis_sonucu b ON b.kosu_id=ay.kosu_id AND b.oyun=a.oyun_adi WHERE a.gun_id='20260331-ANTALYA' ORDER BY 1, b.sira;
-- A1 | K06 | 5,7/2,3/7/8/1/1 | 1.787,18 TL | 1787.18
-- A2 | K08 | 7/8/1/1/8/5     | 2.093,81 TL | 2093.81
-- A2 | K08 | 7/8/1/1/8/6     | 1.686,12 TL | 1686.12
SELECT altili_id, ayak_no, kosu_no, program_at_no, at_isim_ham, neden FROM v_altili_gecerli
 WHERE altili_id='20260331-ANTALYA-A2' AND ayak_no=6;
-- A2 | 6 | 8 | 5 | MAVİŞİN KIZI KG K DB | BIRINCI
-- A2 | 6 | 8 | 6 | ŞUKELA KG K DB       | ACIKLANMADI
```

**K4b: BAŞARISIZ (şema bilgiyi kayıpsız saklayamıyor).** İki ödeme değeri ve kombinasyonları `bahis_sonucu`'nda doğru ve ayrı
satırlarda var. Altılıya bağları ise yalnızca `altili.oyun_adi` + 6. ayağın `kosu_id`'si üzerinden dolaylı bir eşleştirmeyle
kurulabiliyor. Eksik olanlar:

- `altili` tablosunda tek `kombinasyon_ham` / `odeme_ham` / `odeme_tl` / `sonuc_durumu` / `sonuc_kaynak_satir` alanı var. Birden çok
  kazanan kombinasyon için **`altili_id`'ye bağlı bir ödeme tablosu (ör. `altili_odeme(altili_id, sira, kombinasyon_ham, odeme_ham, odeme_tl, sonuc_kaynak_satir)`) yok**.
  Bu yüzden A2 satırında bu alanlar NULL; ayrıştırıcının `kazanan_kombinasyonlar` listesi aktarılmıyor.
- `altili_ayak_gecerli.neden` CHECK kısıtı yalnızca `BIRINCI`, `KOSMAZ`, `EKURI`, `ACIKLANMADI` değerlerine izin veriyor. Ortak birinci
  ŞUKELA (6) **`ACIKLANMADI`** olarak yazılıyor; bu yanıltıcı. `ORTAK_BIRINCI` değeri yok.
- Ortak birincilik kanıtları (`altili.json` → `ortak_birincilikler`) için tablo yok. Bitiş sırası, fark ve ganyan oranı `sonuc`'ta,
  GANYAN ödemeleri `bahis_sonucu`'nda olduğu için K4a doğrulanabiliyor.

## K5: 15.03 Adana

```sql
SELECT altili_id, oyun_adi, baslangic_kosu_no, kombinasyon_ham, odeme_ham, odeme_tl, sonuc_durumu, devir_tl, sonuc_kaynak_satir
  FROM altili WHERE gun_id='20260315-ADANA' ORDER BY 1;
-- 20260315-ADANA-A1 | 1. 6'LI GANYAN | 1 | NULL                  | NULL         | NULL     | NULL   | NULL | NULL
-- 20260315-ADANA-A2 | 2. 6'LI GANYAN | 3 | 1,12/3,4/1,2,3/5/5/12 | 19.387,81 TL | 19387.81 | ODENDI | NULL | 97
SELECT kosu_id, sira, oyun, kombinasyon_ham, tutar_ham, tutar_tl, sonuc_durumu FROM bahis_sonucu WHERE kosu_id='20260315-ADANA-K03' ORDER BY sira;
-- K03 | 1 | 7'Lİ PLASE  | … | 7.334,10 TL   | 7334.1   | ODENDI
-- K03 | 2 | 7'Lİ GANYAN | … | 105.255,50 TL | 105255.5 | ODENDI
SELECT COUNT(*) FROM bahis_sonucu WHERE kosu_id='20260315-ADANA-K03' AND oyun='GANYAN';                         -- 0
SELECT * FROM ganyan_kontrol WHERE kosu_id='20260315-ADANA-K03';                -- K03 | (boş) | 1 | GANYAN_SATIRI_YOK
SELECT COUNT(*) FROM bahis_sonucu WHERE kosu_id LIKE '20260315-ADANA-%' AND oyun='1. 6''LI GANYAN';              -- 0
SELECT COUNT(*) FROM bahis_sonucu WHERE kosu_id LIKE '20260315-ADANA-%'
   AND (tutar_tl = 0 OR sonuc_durumu <> 'ODENDI' OR ham_ibare LIKE '%iade%' OR ham_ibare LIKE '%İADE%');         -- 0
SELECT COUNT(*) FROM altili WHERE gun_id='20260315-ADANA' AND (odeme_tl = 0 OR sonuc_durumu IS NOT NULL AND odeme_tl IS NULL); -- 0
SELECT ko.kosu_no, COUNT(k.katilim_id), SUM(s.eslesme_durumu='TEKIL_ESLESME'), SUM(s.bitis_sirasi=1) FROM kosu ko
  JOIN katilim k ON k.kosu_id=ko.kosu_id JOIN sonuc s ON s.katilim_id=k.katilim_id
 WHERE ko.gun_id='20260315-ADANA' GROUP BY ko.kosu_no ORDER BY 1;
-- 1|8|8|1  2|8|8|1  3|12|12|1  4|12|12|1  5|10|10|1  6|9|9|1  7|6|6|1  8|12|12|1   (toplam 77 katılım)
```

**BAŞARILI.**
- 8 koşu ve 77 katılımın hepsi tekil eşleşmiş durumda.
- 1. altılının ödemesi ve sonuç durumu NULL, yani bilinmiyor. 3. koşu için GANYAN satırı yok; GANYAN kontrolü `GANYAN_SATIRI_YOK`.
  İki neden `inceleme_nedeni`'nde kayıtlı.
- Hiçbir yere sıfır ya da iade yazılmadı. 2. altılı 19.387,81 TL.

Not: Şemada açık bir "bilinmiyor" durum alanı yok. Bilinmezlik NULL ve `inceleme_nedeni` ile ifade ediliyor. 3.–8. koşuların
kaynak izi yanlış (K2b).

## K6: İkinci aktarım (tekrar çalıştırılabilirlik)

Aynı `aktar.sh` aynı girdilerle ikinci kez çalıştırıldı (çıkış 0; 60 TAMAM + 1 INCELEME_GEREKLI; 61 özet satırı birinciyle aynı).
14 tablonun her biri `SELECT * FROM <tablo> ORDER BY 1,2,3` ile önce ve sonra tam döküldü ve `cmp` ile karşılaştırıldı.

| Tablo | 1. aktarım | 2. aktarım | İçerik |
|---|---|---|---|
| kaynak_dosya | 122 | 122 | aynı |
| yaris_gunu | 61 | 61 | aynı |
| inceleme_nedeni | 2 | 2 | aynı |
| kosu | 481 | 481 | aynı |
| at | 2768 | 2768 | aynı |
| katilim | 4776 | 4776 | aynı |
| agf_gozlem | 6342 | 6342 | aynı |
| sonuc | 4776 | 4776 | aynı |
| bahis_sonucu | 3828 | 3828 | aynı |
| bahis_uyarisi | 20 | 20 | aynı |
| ganyan_kontrol | 481 | 481 | aynı |
| altili | 105 | 105 | aynı |
| altili_ayak | 630 | 630 | aynı |
| altili_ayak_gecerli | 711 | 711 | aynı |

**BAŞARILI.** Koşu, katılım ve altılı/ödeme kayıtları çoğalmadı. Kimlik değerleri (`at_id`, `kaynak_id`) dahil tüm değerler aynı.
Aktarım günü önce silip tek işlemde yeniden yazıyor (`veritabani.rs`).

## Uyuşmazlıklar ve şema eksikleri (özet)

1. **Koşu başına planlanan/gerçekleşen tarih, saat ve pist** için alan ya da tablo yok (`kosu_tarihleri.csv` aktarılmıyor). K2b başarısız.
2. **Ek sonuç dosyası ve koşu başına sonuç kaynağı** saklanmıyor (`yaris_gunu.sonuc_kaynak_id` tek). 15.03 Adana 3.–8. koşularının
   kaynak izi `v_kaynak_izi`'de yanlış dosyaya bağlı.
3. **Birden çok kazanan altılı kombinasyonu** için `altili_id`'ye bağlı ödeme tablosu yok. A2'nin ödeme alanları NULL; ödemeler
   yalnızca `bahis_sonucu`'nda. K4b başarısız.
4. **`altili_ayak_gecerli.neden`** ortak birinciyi temsil edemiyor (ŞUKELA `ACIKLANMADI`).
5. Şemada **planlı çift (takvim) tablosu** yok. 61 çift, takvim dosyasıyla karşılaştırılarak doğrulandı.

Şema ve aktarım aracı bu provada değiştirilmedi. Yukarıdaki eksikler giderilene kadar ana veritabanına aktarım, bu bilgileri kaybeder.
