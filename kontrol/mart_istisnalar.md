# Mart 2026: dört istisna günü (çözüm)

Tarih: 2026-11-22 · Ayrıştırıcı: `tjk_ayristirici` 0.12.0 · SQLite kullanılmadı · Mevcut ham CSV'ler değiştirilmedi
(yalnızca doğrulanan iki yeni sonuç dosyası eklendi).

## Özet

| Gün | Önceki durum | Yeni durum | Dayanak |
|---|---|---|---|
| 15.03.2026 Adana | İNCELEME (3.–8. koşu yok) | **İNCELEME GEREKLİ** (dar kapsam) | 3.–8. koşular 16.03 sonuç dosyasında bulundu ve eşleşti. Dosyada olmayan iki ödeme kaldı. |
| 21.03.2026 Adana | İNCELEME (sonuç 0 bayt) | **TAMAMLANDI** | 34. yarış gününün sonuçları 23.03 dosyasında; 9 koşu, 89 at eşleşti |
| 21.03.2026 İstanbul | İNCELEME (LEONİDAS eküri) | **TAMAMLANDI** | `YP` eki ayrıştırıcıya eklendi; eküri doğrulandı |
| 31.03.2026 Antalya | İNCELEME (GANYAN çelişkisi) | **TAMAMLANDI** | 8. koşuda 5 ve 6 ortak birinci; iki 2. altılı ödemesi ayrı korundu |

Resmî duyurular (ör. TJK duyuru 52071) `www.tjk.org` bu ortamın ağ politikasınca engellendiği için açılamadı.
Ertelemeye ilişkin bilgiler kullanıcı beyanı olarak `--ertelenme-notu` ile kaydedildi; kapanış kararları
`medya-cdn.tjk.org`'dan indirilen resmî CSV'lerin içeriğine dayanıyor.

---

## 1. Adana: ertelenen koşular

### İndirilen dosyalar

`tjk_indir liste --ay 2026-03 --gunler 16,23` ile üretilen adaylardan yalnızca Adana sonuç satırları; `tjk_indir indir` ile
(robots.txt: HTTP 404, kural yok).

| Dosya | Kaynak URL | HTTP | Bayt | SHA-256 | İçerik |
|---|---|---|---|---|---|
| `kontrol/ham/20260316-ADANA-sonuc.csv` | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-16/CSV/GunlukYarisSonuclari/16.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 12192 | `70763d1a1258cc66801d786c043fe2eef9b5e8713f3d12666188a0a4a6343dd3` | `Adana;(32. Yarış Günü);16/03/2026`, sonuç, koşu 3–8 |
| `kontrol/ham/20260323-ADANA-sonuc.csv` | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-23/CSV/GunlukYarisSonuclari/23.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 17779 | `2db0876bcaa528479cb7b75d0abfa08ee0103b8a95180bcac1b0f821bf2d34cb` | `Adana;(34. Yarış Günü);23/03/2026`, sonuç, koşu 1–9 |

### Eşleştirme kuralı (ayrıştırıcıda)

- `tjk_ayristirici PROGRAM SONUC ÇIKTI [--ek-sonuc DOSYA]... [--ertelenme-notu METİN]`
- Her sonuç dosyasının hipodromu ve **yarış günü numarası** programla aynı olmalı. Aksi hâlde hata.
- Programdan farklı tarihli sonuç yalnızca `--ertelenme-notu` ile kabul edilir. Sonuç tarihi programdan önce olamaz.
- Farklı tarihte koşulan her koşuda **koşu cinsi, yaş grubu, kilo şartları, koşu adı ve mesafe** programla karşılaştırılır.
  Saat ve pist değişebilir; bunlar kaydedilir. **Atlar** tarih + hipodrom + koşu no + at adıyla tek tek eşleştirilir.
- Bir koşu iki sonuç dosyasında birden varsa hata verilir: ertelenen koşu ikinci kez sayılmaz.
- `kosu_tarihleri.csv` (her gün için yeni çıktı): `planlanan_tarih`, `planlanan_saat`, `gerceklesen_tarih`, `gerceklesen_saat`,
  `pist_program`, `pist_sonuc`, `tarih_durumu` (PLANLANDIGI_GUN / ERTELENDI / SONUC_YOK), kaynak dosya ve sha256, erteleme notu.
  Gerçekleşen tarih, koşunun bulunduğu sonuç dosyasının ilk satırındaki tarihtir.

### 15.03.2026 Adana (32. yarış günü): İNCELEME GEREKLİ

- Komut: `tjk_ayristirici 20260315-ADANA-program.csv 20260315-ADANA-sonuc.csv … --ek-sonuc 20260316-ADANA-sonuc.csv --ertelenme-notu "TJK duyuru 52071 …"`
- 1.–2. koşu: 15.03 dosyasından korundu (planlanan = gerçekleşen = 2026-03-15).
- 3.–8. koşu: 16.03 dosyasından alındı (planlanan 2026-03-15, gerçekleşen 2026-03-16). Saatler 19:15–21:45'ten 11:30–14:00'e değişmiş;
  koşu cinsi, yaş grubu, kilo, mesafe, pist ve koşu adı (6. koşu KEMAL ÖZEKİCİ KOŞUSU) programla aynı.
- 8 koşu, 77 at satırının hepsi tekil eşleşti. Eşleşmeyen sonuç satırı yok. Eküri grupları (5 koşu, 10 grup) tutarlı.
  Bahis satırı kontrolü 21/21 uyumlu. 2. altılı (3.–8. koşu): `1,12/3,4/1,2,3/5/5/12`, ödeme 19.387,81 TL.
- **Açık kalan iki kayıt (dosyada yok, kapatılmadı):**
  - `GANYAN_SATIRI_BULUNAMADI: 3. koşu (birinci [1])`. 16.03 dosyasında 3. koşunun bahis satırında yalnızca
    `7'Lİ PLASE(…) :7.334,10 TL, 7'Lİ GANYAN(…) :105.255,50 TL` var; GANYAN, İKİLİ vb. yok. Birinci 1 numara; bu,
    4. koşu satırındaki `3. ÇİFTE(1/4) :7,30 TL` ile tutarlı.
  - `ALTILI_ODEMESI_BULUNAMADI: 20260315-ADANA-A1`. 1. altılı 15.03'te 1. koşuda başladı; 3.–6. ayakları 16.03'e kaldı.
    6. koşunun satırında (`GANYAN(5) :1,35 TL, …, 5. ÇİFTE(3/5) :6,65 TL`) `1. 6'LI GANYAN` ödemesi yok.
  - Bu bahislerin erteleme nedeniyle iptal mi edildiği, yoksa ödemenin başka bir yerde mi yayımlandığı CSV'lerde yazmıyor.
    Duyuru metni açılamadı. Bu yüzden gün **incelemede kalıyor**. Koşu sonuçlarının kendisi tam.

### 21.03.2026 Adana (34. yarış günü): TAMAMLANDI

- 21.03 sonuç adresi iki denemede HTTP 200 ile 0 bayt döndü (önceki kayıt). Sonuçlar 23.03 adresinde:
  `tjk_ayristirici 20260321-ADANA-program.csv 20260323-ADANA-sonuc.csv … --ertelenme-notu "…"`.
- 9 koşunun hepsi planlanan 2026-03-21, gerçekleşen 2026-03-23 (saatler 14:00–18:00'den 11:00–14:45'e). Koşu cinsi, yaş grubu,
  kilo, mesafe ve koşu adı (6. koşu TENDÜREK KOŞUSU, G 3) programla aynı.
- **Pist değişikliği:** 4. koşu (ŞARTLI 4, 3 Yaşlı İngilizler, 1300 m) programda **Çim**, sonuçta **Kum**.
  `kosu_tarihleri.csv`'de `pist_program=Çim`, `pist_sonuc=Kum` olarak kayıtlı. Koşunun kimliği yarış günü no, koşu no, cins,
  yaş grubu, mesafe ve 11 atın tamamıyla doğrulandığı için bu bir eşleşme sorunu sayılmadı.
- 89 at satırının hepsi tekil eşleşti. Eküri (4 koşu, 8 grup) tutarlı. Bahis satırı kontrolü 25/25 uyumlu.
  Altılı A1 `5/1/2/5/8/6` 11.991,81 TL; A2 `5/8/6/1/3,7/1` 40.488,62 TL. **Gün durumu: TAMAM.**
- Not: Kullanıcı "sonuçlar 23 Mart'ta yayımlandı" dedi. Dosyadaki tarih (23/03/2026) ve değişen koşu saatleri koşuların
  23.03'te yapıldığını gösteriyor; gerçekleşen tarih bu içerikten kaydedildi.

## 2. 21.03.2026 İstanbul, 6. koşu: YP eki

- **Neden bozuluyordu:** `isim_coz` at adının sonundan yalnızca `KOD_BELIRTECLERI` listesindeki kodları ayıklıyor.
  `LEONİDAS SKG SK YP` adında son kelime `YP` listede olmadığı için döngü hemen duruyordu. Ad `LEONİDAS SKG SK YP` olarak
  kalıyor (kodlar bile ayıklanmıyordu) ve eküri ibaresindeki `(6)LEONİDAS` ile eşleşmiyordu.
- **Test:** `ayristir::tests::yp_eki_ve_ekuri_adi` düzeltmeden önce başarısız oldu
  (`left: ("LEONİDAS SKG SK YP", "", false)`, `right: ("LEONİDAS", "SKG SK YP", false)`), düzeltmeden sonra geçti.
- **Düzeltme:** `tjk_ayristirici/src/ayristir.rs` içinde `KOD_BELIRTECLERI` listesine `YP` eklendi. Mart dosyalarında `YP` 20 at
  satırında geçiyor ve her seferinde başka kodlardan sonra geliyor (`SK YP`, `K YP`).
- **Ham ad korunuyor:** `program.csv` → `at_isim_ham = "LEONİDAS SKG SK YP"`, `at_adi = "LEONİDAS"`, `isim_kodlari = "SKG SK YP"`,
  `ekuri_grubu = "5,6"`.
- Sonuç: eküri sorunu kalmadı, **gün durumu: TAMAM**.

## 3. 31.03.2026 Antalya, 8. koşu: ortak birincilik

- **Kural (ayrıştırıcıda):** ortak birincilik yalnızca şu kanıtların hepsi varsa kabul edilir:
  1. bahis satırında birden çok ayrı, tek numaralı ve tutarı okunan GANYAN ödemesi,
  2. bu numaralar sonuç listesinin ilk N sırasındaki bitiren atlar,
  3. her atın sonuç satırındaki ganyan oranı kendi GANYAN ödemesine eşit,
  4. aralarındaki Fark hücresi `At Başı`,
  5. dereceleri aynı. Bu yalnızca destekleyici bir kanıt; **aynı derece tek başına ortak birincilik sayılmaz**.
- **Kanıt satırları** (`20260331-ANTALYA-sonuc.csv`):
  - satır 105: `1;MAVİŞİN KIZI KG K DB;…;1:33.70;1,80;At Başı` (program 5)
  - satır 106: `2;ŞUKELA KG K DB;…;1:33.70;1,60;Baş` (program 6)
  - satır 122: `GANYAN(5) :1,80 TL, GANYAN(6) :1,60 TL, … 2. 6'LI GANYAN(7/8/1/1/8/5) :2.093,81 TL, 2. 6'LI GANYAN(7/8/1/1/8/6) :1.686,12 TL, …`
- **Sonuç:**
  - 5 MAVİŞİN KIZI ve 6 ŞUKELA ikisi de bitiş sırası 1 (liste sıraları 1 ve 2 korunuyor). HADEHANIM bitiş sırası 3.
    `altili.json` → `ortak_birincilikler` kanıtlarıyla birlikte.
  - 2. altılı (3.–8. koşu): tek ödeme alanları bilerek boş. `kazanan_kombinasyonlar` içinde iki ayrı kayıt var:
    `7/8/1/1/8/5 → 2.093,81 TL` ve `7/8/1/1/8/6 → 1.686,12 TL`. 6. ayakta `ortak_birinciler` [5, 6], geçerli numaralar [5, 6].
  - Bahis satırı kontrolü 25/25 uyumlu (önce 4 uyumsuz: GANYAN(6), SIRALI İKİLİ(6/5), SIRALI 5 Lİ(6/5/…), TABELA(6/5/…)).
  - 1. altılı (1.–6. koşu) değişmedi: `5,7/2,3/7/8/1/1`, 1.787,18 TL. **Gün durumu: TAMAM.**
- Olumsuz testler: ikinci GANYAN ödemesi kaldırılırsa (dereceler hâlâ aynı) ortak birincilik kabul edilmiyor, ŞUKELA 2. kalıyor
  ve çift altılı ödemesi `ALTILI_COKLU_ODEME_ACIKLANAMADI` ile incelemeye düşüyor. `At Başı` yerine `Burun` yazılırsa
  `ORTAK_BIRINCILIK_DOGRULANAMADI` ile incelemeye düşüyor.
- Diğer günlerdeki `At Başı` ölü yarışları (13.03 İzmir 4.–5., 22.03 İzmir 4.–5., 25.03 İstanbul 2.–3., 30.03 Şanlıurfa 3.–4.
  sıra) birincilikte değil. Ayrıştırıcı onlara dokunmuyor; o günler zaten TAMAM.

## 4. Regresyon

- `cargo test --release --offline`: 19 birim testi (paylaşılan modüllerin 15 testi `tjk_indir` ikilisinde de) ve 10 uçtan uca test (`tjk_ayristirici/tests/mart_istisnalar.rs`) geçti; başarısız test yok.
- Eski sürüm (0.11.0, değiştirilmemiş kaynak) ile yeni sürüm aynı 50 günde çalıştırıldı ve `tjk-fetch karsilastir` ile
  alan düzeyinde karşılaştırıldı (`kontrol/regresyon_0.12.0_farklar.tsv`, 36 hücre):
  - **41 gün** `program.csv`, `sonuclar.csv` ve `altili.json` dosyalarında bayt bayt aynı.
  - **8 gün** (`YP`'li atlar): yalnızca 10 at satırında `at_adi` ve `isim_kodlari` değişti (MAHSA, HUMAY KIZIM, PHANTOM STRIKE,
    BIG RUGGED, KAYLA MARLA, LEONİDAS). Üç günün `altili.json` dosyasında aynı adların metni değişti. 21.03 İstanbul
    INCELEME → TAMAM oldu.
  - **31.03 Antalya:** ŞUKELA `bitis_sirasi` 2 → 1, 5 hücrede bahis doğrulaması UYUMSUZ → UYUMLU, `altili.json`'da ortak
    birincilik ve ayrı ödemeler.
  - Beklenmedik değişiklik yok. Daha önce TAMAM olan 46 günün hepsi hâlâ TAMAM.
- Normal günlerde yeni alanlar (`ertelenen_kosular`, `ortak_birincilikler`, `kazanan_kombinasyonlar`) yazılmıyor. Her güne yeni
  `kosu_tarihleri.csv` dosyası ekleniyor; normal günde tüm koşular PLANLANDIGI_GUN.
