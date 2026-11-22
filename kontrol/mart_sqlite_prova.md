# Mart 2026 SQLite aktarım provası (şema 4)

Tarih: 2026-11-22 · Aktarım: `tjk_ayristirici` 0.13.0 (`--sqlite`, `veritabani::aktar`) · Şema: `tjk_ayristirici/src/sema.sql`
**sürüm 4** · Sorgu: `tjk_sorgu` (salt okunur). Ana veritabanı bu ortamda yok ve dokunulmadı. Tüm veritabanları depo dışında, geçici
(`scratchpad/prova/…`); hiçbiri Git'e eklenmedi. Ham CSV'ler yeniden indirilmedi (`git diff --quiet HEAD -- kontrol/ham` temiz).

## Sonuç özeti

| # | Kontrol | Şema 3 provası | **Şema 4 (bu prova)** |
|---|---|---|---|
| K1a | 61 gün; 60 TAMAM, 1 INCELEME_GEREKLI | başarılı | **BAŞARILI** |
| K1b | `yaris_gunu` (tarih, hipodrom) = resmî takvimdeki 61 çift | başarılı | **BAŞARILI** |
| K1c | İnceleme nedenleri yalnızca 15.03 Adana'da, 2 adet | başarılı | **BAŞARILI** |
| K2a | 16.03/23.03 için ayrı gün yok; koşular ikinci kez sayılmadı (8 koşu/77, 9 koşu/89) | başarılı | **BAŞARILI** |
| K2b | Koşu başına planlanan ve gerçekleşen tarih/saat/pist ayrı | **başarısız** | **BAŞARILI** |
| K2c | Koşunun gerçek sonuç dosyası: 15.03 K1–2 → 15.03, K3–8 → 16.03; 21.03 → 23.03 | **başarısız** | **BAŞARILI** |
| K2d | `v_kaynak_izi` ve `v_bahis_kaynak_izi` satırları gerçek dosyadaki doğru satırı gösteriyor (tüm Mart) | **başarısız** | **BAŞARILI** (4776/4776 sonuç, 3828/3828 bahis) |
| K3 | 21.03 İstanbul LAZMAN (5) + LEONİDAS (6) eküri `5,6`, ham ad korunuyor | başarılı | **BAŞARILI** |
| K4a | 31.03 Antalya 8. koşu 5 ve 6 bitiş sırası 1, GANYAN `AT_BASI_UYUMLU` | başarılı | **BAŞARILI** |
| K4b | A2: 5 ile biten 2.093,81 TL, 6 ile biten 1.686,12 TL; altılıya doğrudan bağlı | **başarısız** | **BAŞARILI** |
| K4c | Geçerlilik nedeni: 5 ve 6 `ORTAK_BIRINCI` | **başarısız** | **BAŞARILI** |
| K4d | Tek ödemeli altılı sorguları korunuyor (`altili` alanları aynen) | — | **BAŞARILI** (103/103) |
| K5a | 15.03 Adana 8 koşu / 77 katılım | başarılı | **BAŞARILI** |
| K5b | 3. koşu GANYAN ve 1. altılı ödemesi bilinmiyor (satır yok / NULL); sıfır ya da iade yok | başarılı | **BAŞARILI** |
| K5c | 2. altılı 19.387,81 TL, 16.03 dosyası satır 97'ye bağlı | kısmen (kaynak yanlış) | **BAŞARILI** |
| K6a | İkinci aktarımda 16 tablonun tam dökümü aynı (kimlikler dahil) | başarılı (14 tablo) | **BAŞARILI** |
| K6b | Koşu, katılım, altılı ödeme kayıtları çoğalmadı | başarılı | **BAŞARILI** |
| G1 | Boş veritabanında v4 kurulumu | — | **BAŞARILI** |
| G2 | Dolu v3 prova kopyasından v4'e geçiş (satır kaybı yok, yeni alanlar NULL, tahmin yok) | — | **BAŞARILI** |
| G3 | Geçiş + yeniden aktarım = boş kurulum (11 anlamsal sorgu) | — | **BAŞARILI** |
| B1 | `PRAGMA foreign_key_check` boş, `integrity_check` ok, ek yetim kayıt kontrolleri 0 | — | **BAŞARILI** |

**21/21 kontrol başarılı.** Şema 3 provasındaki iki başarısız kontrol (K2b, K4b) ve bunlara bağlı kaynak izi hatası giderildi.
Şema 3 sonucundaki "7 başarılı, 2 başarısız" sayımı yerine bu tablo geçerlidir.

## Yapılan değişiklikler

**Şema 4** (`sema.sql`, `PRAGMA user_version = 4`; göç `veritabani.rs` → `GOC_3_4`):

- `kosu` tablosuna eklenen alanlar: `planlanan_tarih`, `planlanan_saat`, `planlanan_pist`, `gerceklesen_tarih`, `gerceklesen_saat`,
  `gerceklesen_pist`, `tarih_durumu` (PLANLANDIGI_GUN / ERTELENDI / SONUC_YOK), `ertelenme_notu`,
  `sonuc_kaynak_id → kaynak_dosya`. Eski `saat` ve `pist` alanları planlanan değerler olarak aynen kalıyor.
- `kaynak_dosya`: ek sonuç dosyaları (ör. `20260316-ADANA-sonuc.csv`) artık kayıtlı. Sahipsiz kaynak temizliği koşu kaynaklarını koruyor.
- `v_kaynak_izi`: sonuç dosyası `COALESCE(kosu.sonuc_kaynak_id, yaris_gunu.sonuc_kaynak_id)`. Yeni görünümler `v_bahis_kaynak_izi` ve `v_altili_odeme`.
- `altili_odeme(altili_id, sira, kombinasyon_ham, odeme_ham, odeme_tl, sonuc_durumu, devir_tl, bahis_kosu_id, bahis_sira)`:
  her kazanan kombinasyon ayrı satır. `(bahis_kosu_id, bahis_sira)` → `bahis_sonucu(kosu_id, sira)` yabancı anahtarı doğrudan bağ kuruyor.
  Ödemesi dosyada olmayan altılının satırı yok (bilinmiyor).
- `altili_odeme_ayak(altili_id, odeme_sira, ayak_no, program_at_no)`: kombinasyonun ayak numaraları. `altili_ayak`'a yabancı anahtarla bağlı.
- `altili_ayak_gecerli.neden` kısıtına `ORTAK_BIRINCI` eklendi (tablo yeniden kuruldu, satırlar aynen kopyalandı).
- Tek ödemeli altılıda `altili.kombinasyon_ham/odeme_ham/odeme_tl/sonuc_durumu` eskisi gibi dolu. Çok ödemelide NULL kalıyor, ödemeler `altili_odeme`'de.

**Aktarım** (`veritabani.rs`, `main.rs`): koşu tarih satırları (`kosu_tarihleri.csv` ile aynı) ve ek dosyaların kodlaması `GunVerisi`
ile aktarıma geçiyor. Altılı ödemesi son ayak bahis satırında `ham_ibare` + kombinasyonla bulunuyor; bulunamazsa aktarım
hata verip geri alınıyor. Ayrıştırıcı sürümü 0.13.0. `program.csv`, `sonuclar.csv` ve `altili.json` çıktıları 50 günde 0.12.0 ile bayt bayt aynı.

**Testler** (`cargo test --release --offline`: 21 birim + 15 paylaşılan + 10 + 3 uçtan uca, hepsi geçti):

- `veritabani::tests::v3_ten_v4_e_goc_kayitlari_korur_tahmin_etmez`: v3 şemasıyla (`tests/veri/sema_v3.sql`) örnek kayıtlar yazılıyor,
  `goc` 3→4 çalışıyor. Satırlar korunuyor, yeni alanlar NULL, görünümler çalışıyor, `ORTAK_BIRINCI` kabul edilip bilinmeyen neden
  reddediliyor. FK ve bütünlük temiz, ikinci `goc` 4→4 hiçbir şey değiştirmiyor.
- `veritabani::tests::bos_kurulum_ve_goc_ayni_semayi_uretir`: boş kurulumla geçişin tablo, görünüm, dizin ve sütun kümeleri aynı.
- `tests/sqlite_aktarim.rs`: Antalya A2'nin iki ödemesi ayaklarıyla, `ORTAK_BIRINCI`, A1'in tek ödemesi; 15.03 Adana'nın koşu
  kaynakları, iki bilinmeyen ödeme, A2'nin 16.03 kaynağı; iki kez aktarımda 9 tablonun aynı kalması; FK ve bütünlük.

## Komutlar

Girdi eşlemesi `kontrol/mart_sqlite_girdiler.tsv` dosyasında (61 satır; 06.03 İzmir programının özgün adı, 15.03 Adana
`--ek-sonuc 20260316-ADANA-sonuc.csv` + erteleme notu, 21.03 Adana sonucu `20260323-ADANA-sonuc.csv` + erteleme notu).
Sürücü `aktar.sh`, her satır için mevcut aracı çağırıyor:

```bash
while IFS=$'\t' read -r gun p r ek; do
  args=("$p" "$r" "$KOK/$gun" --sqlite "$DB")
  if [ -n "$ek" ]; then eks=${ek%%|*}; notu=${ek#*|}; [ -n "$eks" ] && args+=(${eks}); args+=(--ertelenme-notu "$notu"); fi
  tjk_ayristirici "${args[@]}" > "$KOK/$gun.log" 2>&1
done < mart_sqlite_girdiler.tsv
```

```bash
# A) boş v4 veritabanı
aktar.sh girdiler.tsv v4/bos_v4.sqlite v4/calisma1      # 61/61, çıkış 0, 60 TAMAM
# 16 tablo tam döküm: tjk_sorgu DB "SELECT * FROM <tablo> ORDER BY 1,2,3" -> dokum1/
aktar.sh girdiler.tsv v4/bos_v4.sqlite v4/calisma2      # çıkış 0; döküm -> dokum2/; cmp
# B) dolu v3 prova kopyasından geçiş (kaynak dosya değişmez)
tjk_ayristirici goc --kaynak mart_2026_prova.sqlite --hedef v4/goc_v4.sqlite
aktar.sh girdiler.tsv v4/goc_v4.sqlite v4/goc_calisma1  # yeniden aktarım
aktar.sh girdiler.tsv v4/goc_v4.sqlite v4/goc_calisma2  # ikinci kez
```

---

## A) Boş v4 veritabanı: K1–K6

### K1

```sql
SELECT COUNT(*), SUM(gun_durumu='TAMAM'), SUM(gun_durumu='INCELEME_GEREKLI') FROM yaris_gunu;   -- 61 | 60 | 1
SELECT gun_id, sira, neden FROM inceleme_nedeni ORDER BY 1,2;
-- 20260315-ADANA | 1 | GANYAN_SATIRI_BULUNAMADI: 3. koşu (birinci [1])
-- 20260315-ADANA | 2 | ALTILI_ODEMESI_BULUNAMADI: 20260315-ADANA-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok
SELECT COUNT(*) FROM yaris_gunu g WHERE NOT EXISTS (SELECT 1 FROM kosu k WHERE k.gun_id=g.gun_id);          -- 0
SELECT tarih || char(9) || hipodrom FROM yaris_gunu ORDER BY 1;   -- diff mart_2026_resmi_takvim.tsv: boş (61 = 61)
```

### K2

```sql
SELECT COUNT(*) FROM yaris_gunu WHERE gun_id IN ('20260316-ADANA','20260323-ADANA')
    OR (hipodrom='Adana' AND tarih IN ('2026-03-16','2026-03-23'));                                  -- 0
-- gün başına: 20260315-ADANA 8 koşu, 8 farklı koşu no, 77 katılım, 77 sonuç; 20260321-ADANA 9 / 9 / 89 / 89
SELECT ko.kosu_id, ko.planlanan_tarih, ko.planlanan_saat, ko.planlanan_pist, ko.gerceklesen_tarih, ko.gerceklesen_saat,
       ko.gerceklesen_pist, ko.tarih_durumu, kd.dosya_adi
  FROM kosu ko LEFT JOIN kaynak_dosya kd ON kd.kaynak_id=ko.sonuc_kaynak_id
 WHERE ko.gun_id IN ('20260315-ADANA','20260321-ADANA') ORDER BY 1;
```

| kosu_id | planlanan | gerçekleşen | durum | sonuç kaynağı |
|---|---|---|---|---|
| 20260315-ADANA-K01 | 2026-03-15 17:45 Kum | 2026-03-15 17:45 Kum | PLANLANDIGI_GUN | 20260315-ADANA-sonuc.csv |
| 20260315-ADANA-K02 | 2026-03-15 18:30 Kum | 2026-03-15 18:30 Kum | PLANLANDIGI_GUN | 20260315-ADANA-sonuc.csv |
| 20260315-ADANA-K03 | 2026-03-15 19:15 Kum | 2026-03-16 11:30 Kum | ERTELENDI | 20260316-ADANA-sonuc.csv |
| 20260315-ADANA-K04 | 2026-03-15 19:45 Kum | 2026-03-16 12:00 Kum | ERTELENDI | 20260316-ADANA-sonuc.csv |
| 20260315-ADANA-K05 | 2026-03-15 20:15 Kum | 2026-03-16 12:30 Kum | ERTELENDI | 20260316-ADANA-sonuc.csv |
| 20260315-ADANA-K06 | 2026-03-15 20:45 Kum | 2026-03-16 13:00 Kum | ERTELENDI | 20260316-ADANA-sonuc.csv |
| 20260315-ADANA-K07 | 2026-03-15 21:15 Kum | 2026-03-16 13:30 Kum | ERTELENDI | 20260316-ADANA-sonuc.csv |
| 20260315-ADANA-K08 | 2026-03-15 21:45 Kum | 2026-03-16 14:00 Kum | ERTELENDI | 20260316-ADANA-sonuc.csv |
| 20260321-ADANA-K01 | 2026-03-21 14:00 Kum | 2026-03-23 11:00 Kum | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K02 | 2026-03-21 14:30 Kum | 2026-03-23 11:30 Kum | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K03 | 2026-03-21 15:00 Kum | 2026-03-23 12:00 Kum | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K04 | 2026-03-21 15:30 **Çim** | 2026-03-23 12:30 **Kum** | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K05 | 2026-03-21 16:00 Kum | 2026-03-23 13:00 Kum | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K06 | 2026-03-21 16:30 Çim | 2026-03-23 13:30 Çim | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K07 | 2026-03-21 17:00 Kum | 2026-03-23 14:00 Kum | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K08 | 2026-03-21 17:30 Kum | 2026-03-23 14:30 Kum | ERTELENDI | 20260323-ADANA-sonuc.csv |
| 20260321-ADANA-K09 | 2026-03-21 18:00 Kum | 2026-03-23 14:45 Kum | ERTELENDI | 20260323-ADANA-sonuc.csv |

```sql
SELECT tarih_durumu, COUNT(*), SUM(planlanan_tarih<>gerceklesen_tarih), SUM(sonuc_kaynak_id IS NULL),
       SUM(planlanan_tarih IS NULL OR gerceklesen_tarih IS NULL) FROM kosu GROUP BY 1;
-- ERTELENDI | 15 | 15 | 0 | 0 ;  PLANLANDIGI_GUN | 466 | 0 | 0 | 0
SELECT COUNT(*) FROM kosu WHERE tarih_durumu='ERTELENDI' AND ertelenme_notu IS NULL;                     -- 0 (2 farklı not)
SELECT ko.kosu_no, v.sonuc_dosyasi, COUNT(*), MIN(v.sonuc_kaynak_satir), MAX(v.sonuc_kaynak_satir) FROM v_kaynak_izi v
  JOIN katilim k ON k.katilim_id=v.katilim_id JOIN kosu ko ON ko.kosu_id=k.kosu_id WHERE ko.gun_id='20260315-ADANA' GROUP BY 1,2;
-- 1 | 20260315-ADANA-sonuc.csv | 8 | 6-13      2 | 20260315-ADANA-sonuc.csv | 8 | 20-27
-- 3 | 20260316-ADANA-sonuc.csv | 12 | 6-17     4 | 20260316-ADANA-sonuc.csv | 12 | 24-35
-- 5 | 20260316-ADANA-sonuc.csv | 10 | 42-51    6 | 20260316-ADANA-sonuc.csv | 9 | 58-66
-- 7 | 20260316-ADANA-sonuc.csv | 6 | 73-78     8 | 20260316-ADANA-sonuc.csv | 12 | 85-96
SELECT v.katilim_id, k.at_isim_ham, v.sonuc_dosyasi, v.sonuc_kaynak_satir FROM v_kaynak_izi v JOIN katilim k USING (katilim_id) WHERE …;
-- 20260315-ADANA-K02-A01 | FATİH ŞAH         | 20260315-ADANA-sonuc.csv | 24
-- 20260315-ADANA-K03-A01 | ATAKLI KIZ KG K   | 20260316-ADANA-sonuc.csv | 6
-- 20260315-ADANA-K08-A01 | ŞERDİL K (Koşmaz) | 20260316-ADANA-sonuc.csv | 96   (şema 3'te: 15.03 dosyası, satır 96 — yanlıştı)
-- 20260321-ADANA-K04-A01 | DARK VEYRON KG K  | 20260323-ADANA-sonuc.csv | 57
SELECT kosu_id, sonuc_dosyasi, COUNT(*), MIN(sonuc_kaynak_satir) FROM v_bahis_kaynak_izi WHERE kosu_id LIKE '20260315-ADANA-%' GROUP BY 1,2;
-- K01 15.03 (3 bahis, satır 14)  K02 15.03 (7, 28)  K03 16.03 (2, 18)  K04 16.03 (11, 36)
-- K05 16.03 (10, 52)  K06 16.03 (7, 67)  K07 16.03 (3, 79)  K08 16.03 (16, 97)
SELECT COUNT(*) FROM kosu ko JOIN yaris_gunu g USING (gun_id) WHERE ko.sonuc_kaynak_id <> g.sonuc_kaynak_id;   -- 6 (15.03 K3–K8)
```

**K2d, tüm Mart için ham dosyaya karşı:** `v_kaynak_izi`'nin her satırı için (`sonuc_dosyasi`, `sonuc_kaynak_satir`) gösterilen
ham dosya satırı okundu ve at adıyla karşılaştırıldı: **4770 tam eşleşme + 6 ülke eki eşleşmesi** (`eslesme_durumu =
ULKE_EKI_FARKI_ESLESMESI`: MUSHARRAF (USA) ×3, CURLIN DRACARYS (USA) ×2, WHITE STARLIGHT (USA); satırlarda ad eksiz yazıyor).
Uyuşmaz satır yok. `v_bahis_kaynak_izi`'nin 3828 satırında gösterilen dosya satırı `OYUN(KOMBİNASYON)` ibaresini içeriyor;
**3828/3828**.

### K3

```sql
SELECT k.katilim_id, k.program_at_no, k.at_isim_ham, a.at_adi, k.isim_kodlari, k.ekuri_grubu FROM katilim k JOIN at a USING (at_id)
 WHERE k.kosu_id='20260321-ISTANBUL-K06' AND k.program_at_no IN (5,6);
-- 20260321-ISTANBUL-K06-A05 | 5 | LAZMAN K DB        | LAZMAN   | K DB      | 5,6
-- 20260321-ISTANBUL-K06-A06 | 6 | LEONİDAS SKG SK YP | LEONİDAS | SKG SK YP | 5,6
SELECT gun_durumu FROM yaris_gunu WHERE gun_id='20260321-ISTANBUL';                                       -- TAMAM
```

### K4

```sql
-- sonuç: 5 MAVİŞİN KIZI liste 1 bitiş 1 1:33.70 1.8 'At Başı'; 6 ŞUKELA liste 2 bitiş 1 1:33.70 1.6 'Baş';
--        11 HADEHANIM liste 3 bitiş 3
SELECT * FROM ganyan_kontrol WHERE kosu_id='20260331-ANTALYA-K08';         -- 5,6 | 5,6 | AT_BASI_UYUMLU
SELECT o.altili_id, o.sira, oa.program_at_no AS ayak6_no, o.kombinasyon_ham, o.odeme_ham, o.odeme_tl, o.sonuc_durumu,
       o.bahis_kosu_id, o.bahis_sira, b.ham_ibare
  FROM altili_odeme o
  JOIN altili_odeme_ayak oa ON oa.altili_id=o.altili_id AND oa.odeme_sira=o.sira AND oa.ayak_no=6
  JOIN bahis_sonucu b ON b.kosu_id=o.bahis_kosu_id AND b.sira=o.bahis_sira
 WHERE o.altili_id='20260331-ANTALYA-A2' ORDER BY o.sira;
-- A2 | 1 | 5 | 7/8/1/1/8/5 | 2.093,81 TL | 2093.81 | ODENDI | 20260331-ANTALYA-K08 | 4 | 2. 6'LI GANYAN(7/8/1/1/8/5) :2.093,81 TL
-- A2 | 2 | 6 | 7/8/1/1/8/6 | 1.686,12 TL | 1686.12 | ODENDI | 20260331-ANTALYA-K08 | 5 | 2. 6'LI GANYAN(7/8/1/1/8/6) :1.686,12 TL
-- altili_odeme_ayak: iki ödemede 1.–5. ayak 7/8/1/1/8 aynı, 6. ayak 5 ve 6
SELECT altili_id, sira, sonuc_dosyasi, sonuc_kaynak_satir FROM v_altili_odeme WHERE altili_id LIKE '20260331-ANTALYA-%';
-- A1 | 1 | 20260331-ANTALYA-sonuc.csv | 85 ;  A2 | 1 | … | 122 ;  A2 | 2 | … | 122
SELECT program_at_no, neden FROM v_altili_gecerli WHERE altili_id='20260331-ANTALYA-A2' AND ayak_no=6;
-- 5 | ORTAK_BIRINCI ;  6 | ORTAK_BIRINCI                       (şema 3'te: 5 BIRINCI, 6 ACIKLANMADI)
SELECT neden, COUNT(*) FROM altili_ayak_gecerli GROUP BY 1;
-- ACIKLANMADI 4 | BIRINCI 623 | EKURI 28 | KOSMAZ 54 | ORTAK_BIRINCI 2
SELECT altili_id, kombinasyon_ham, odeme_ham, odeme_tl, sonuc_durumu FROM altili WHERE gun_id='20260331-ANTALYA';
-- A1 | 5,7/2,3/7/8/1/1 | 1.787,18 TL | 1787.18 | ODENDI ;  A2 | NULL | NULL | NULL | NULL (çok ödemeli)
-- K4d: tek ödemeli altılılarda altili alanları = altili_odeme satırı
SELECT COUNT(*), SUM(a.odeme_tl IS o.odeme_tl AND a.kombinasyon_ham=o.kombinasyon_ham AND a.odeme_ham IS o.odeme_ham
       AND a.sonuc_durumu=o.sonuc_durumu) FROM altili a JOIN altili_odeme o USING (altili_id)
 WHERE (SELECT COUNT(*) FROM altili_odeme x WHERE x.altili_id=a.altili_id)=1;                  -- 103 | 103
-- altılı 105; ödemeli 104; ödeme satırı 105; ödemesiz: 20260315-ADANA-A1; çok ödemeli: 20260331-ANTALYA-A2; DEVIR 0
```

Şema 3 provasıyla karşılaştırma: `altili`, `bahis_sonucu`, `sonuc` ve `katilim` tablolarının tam dökümü **aynı**; mevcut tek ödemeli
altılı sorguları aynı sonucu veriyor. `altili_ayak_gecerli`'de yalnızca iki satır değişti:
`20260331-ANTALYA-A2|6|5` BIRINCI → ORTAK_BIRINCI ve `…|6|6` ACIKLANMADI → ORTAK_BIRINCI.

### K5

```sql
SELECT altili_id, oyun_adi, kombinasyon_ham, odeme_ham, odeme_tl, sonuc_durumu, devir_tl FROM altili WHERE gun_id='20260315-ADANA';
-- A1 | 1. 6'LI GANYAN | NULL | NULL | NULL | NULL | NULL
-- A2 | 2. 6'LI GANYAN | 1,12/3,4/1,2,3/5/5/12 | 19.387,81 TL | 19387.81 | ODENDI | NULL
SELECT COUNT(*) FROM altili_odeme WHERE altili_id='20260315-ADANA-A1';                                      -- 0
SELECT * FROM v_altili_odeme WHERE altili_id LIKE '20260315-ADANA-%';
-- A2 | 1 | 1,12/3,4/1,2,3/5/5/12 | 19.387,81 TL | 19387.81 | ODENDI | 20260316-ADANA-sonuc.csv | satır 97
SELECT oyun, tutar_ham, sonuc_dosyasi, sonuc_kaynak_satir FROM v_bahis_kaynak_izi WHERE kosu_id='20260315-ADANA-K03';
-- 7'Lİ PLASE 7.334,10 TL | 7'Lİ GANYAN 105.255,50 TL — ikisi de 20260316-ADANA-sonuc.csv satır 18
SELECT COUNT(*) FROM bahis_sonucu WHERE kosu_id='20260315-ADANA-K03' AND oyun='GANYAN';                     -- 0
SELECT * FROM ganyan_kontrol WHERE kosu_id='20260315-ADANA-K03';                    -- (boş) | 1 | GANYAN_SATIRI_YOK
SELECT COUNT(*) FROM bahis_sonucu WHERE kosu_id LIKE '20260315-ADANA-%'
   AND (tutar_tl=0 OR sonuc_durumu<>'ODENDI' OR ham_ibare LIKE '%iade%' OR ham_ibare LIKE '%İADE%');        -- 0
SELECT COUNT(*) FROM altili_odeme WHERE odeme_tl=0;                                                          -- 0
-- 15.03 Adana: 8 koşu, 77 katılım
```

### K6: ikinci aktarım

| Tablo | Satır (1. = 2.) | Tam döküm |
|---|---|---|
| kaynak_dosya | 123 | aynı |
| yaris_gunu | 61 | aynı |
| inceleme_nedeni | 2 | aynı |
| kosu | 481 | aynı |
| at | 2768 | aynı |
| katilim | 4776 | aynı |
| agf_gozlem | 6342 | aynı |
| sonuc | 4776 | aynı |
| bahis_sonucu | 3828 | aynı |
| bahis_uyarisi | 20 | aynı |
| ganyan_kontrol | 481 | aynı |
| altili | 105 | aynı |
| altili_ayak | 630 | aynı |
| altili_ayak_gecerli | 711 | aynı |
| altili_odeme | 105 | aynı |
| altili_odeme_ayak | 716 | aynı |

Aktarım özet satırları da iki çalıştırmada aynı (log toplamı: altılı ödeme 105). `kaynak_dosya` 123 = 61 program + 61 sonuç +
16.03 ek sonuç dosyası.

### Bütünlük

```sql
PRAGMA user_version;        -- 4
PRAGMA foreign_keys;        -- 1
PRAGMA foreign_key_check;   -- 0 satır
PRAGMA integrity_check;     -- ok
SELECT COUNT(*) FROM kosu WHERE sonuc_kaynak_id NOT IN (SELECT kaynak_id FROM kaynak_dosya);                         -- 0
SELECT COUNT(*) FROM altili_odeme o WHERE NOT EXISTS (SELECT 1 FROM bahis_sonucu b WHERE b.kosu_id=o.bahis_kosu_id AND b.sira=o.bahis_sira); -- 0
SELECT COUNT(*) FROM altili_odeme_ayak oa WHERE NOT EXISTS (SELECT 1 FROM altili_ayak a WHERE a.altili_id=oa.altili_id AND a.ayak_no=oa.ayak_no); -- 0
SELECT COUNT(*) FROM altili_odeme o JOIN altili_ayak a ON a.altili_id=o.altili_id AND a.ayak_no=6 WHERE a.kosu_id<>o.bahis_kosu_id;          -- 0
SELECT COUNT(*) FROM kaynak_dosya WHERE kaynak_id NOT IN (SELECT program_kaynak_id FROM yaris_gunu UNION SELECT sonuc_kaynak_id FROM yaris_gunu
       UNION SELECT sonuc_kaynak_id FROM kosu);                                                                     -- 0
```

## B) Dolu v3 prova kopyasından geçiş

- Kaynak: önceki provanın v3 veritabanı (`mart_2026_prova.sqlite`, `user_version` 3). `tjk_ayristirici goc` kopya üzerinde çalıştı;
  kaynağın SHA-256'sı önce ve sonra `498743718b5d2de8…`, değişmedi. Araç çıktısı: `integrity_check: ok | foreign_key_check: 0 satır`,
  "hiçbir tabloda satır kaybı yok".
- **Geçiş sonrası, yeniden aktarımdan önce:** 14 tablonun satır sayısı aynı (kaynak_dosya 122, kosu 481, katilim 4776, altili 105,
  altili_ayak_gecerli 711, …). `kosu`, `altili`, `sonuc`, `bahis_sonucu`, `katilim` eski sütunları ve `altili_ayak_gecerli`
  satırları birebir aynı. 481 koşunun 481'inde yeni alanların hepsi NULL. `altili_odeme` 0, `ORTAK_BIRINCI` 0; eski kayıtta bilgi
  tahmin edilmedi. `v_kaynak_izi` bu aşamada 15.03 K08-A01 için hâlâ günün dosyasını gösteriyor (`20260315-ADANA-sonuc.csv`, 96);
  doğru bilgi yalnızca yeniden aktarımla geliyor. `user_version` 4, FK boş, bütünlük ok.
- **Ham dosyalardan yeniden aktarım** (61/61, çıkış 0, 60 TAMAM): sonuç, boş kurulumla 11 anlamsal sorguda **11/11 aynı**.
  Sorgular kimlik değil dosya adı ve doğal anahtarlarla karşılaştırıyor: gün + kaynak dosyalar, `kaynak_dosya`, koşu tarih ve kaynak
  alanları, `v_kaynak_izi`, `v_bahis_kaynak_izi`, `v_altili_odeme`, `altili_odeme_ayak`, `altili`, `altili_ayak_gecerli`,
  katılım + at anahtarı + sonuç satırı, `inceleme_nedeni`.
- İkinci yeniden aktarım: 11/11 aynı. Sayılar: kosu 481, katilim 4776, altili 105, altili_odeme 105, altili_odeme_ayak 716,
  bahis_sonucu 3828, kaynak_dosya 123. FK boş, bütünlük ok.

## Kalan notlar

- Şemada hâlâ "planlı çift" (takvim) tablosu yok. 61 çift, takvim dosyasıyla karşılaştırılarak doğrulanıyor (K1b).
- 15.03 Adana'daki iki ödeme bilinmiyor olarak açık ve gün `INCELEME_GEREKLI`. Doğrulanmış 8 koşu ve 77 katılım aktarıldı.
- Ana veritabanına dokunulmadı. Ona uygulanacaksa önce kopyada `tjk_ayristirici goc`, ardından Mart günlerinin ham dosyalardan
  yeniden aktarılması gerekir (geçiş eski kayıtların yeni alanlarını boş bırakır).
