# Mart 2026: inceleme bekleyen dört yarış günü

İnceleme tarihi: 2026-11-22. Bu adımda ham CSV'ler, ayrıştırıcı ve SQLite değiştirilmedi.

## Resmî kaynak erişimi: DURDURULDU

CLAUDE.md kuralı gereği ilk HTTP 403'te ağ istekleri durduruldu. Resmî sonuç sayfaları ve
iptal/erteleme duyuruları **kontrol edilemedi**. Aşağıdaki kanıtların hepsi, önceki adımda indirilip
içerikten doğrulanan resmî TJK CSV'lerinden (`kontrol/ham/`) geliyor.

| İstek | Sonuç (tam mesaj) |
|---|---|
| `https://www.tjk.org/robots.txt` | `curl: (56) CONNECT tunnel failed, response 403`: bu bulut ortamının ağ politikası `www.tjk.org` adresine izin vermiyor. İstek TJK'ya ulaşmadı. |
| `https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-15/` | `HTTP 403`, gövde başlığı `403 - Forbidden: Access is denied.` (sunucu dizin listelemeyi reddetti) |

Bu yüzden 15 ve 21 Mart Adana için resmî sonuç ya da iptal/erteleme duyurusu doğrulanamadı.
21 Mart İstanbul ve 31 Mart Antalya da yalnızca ham CSV ile karşılaştırıldı; resmî sonuç sayfasıyla karşılaştırma yapılamadı.

## Özet

| Gün | Ham CSV'deki bulgu | Sonuç | Durum |
|---|---|---|---|
| 15.03.2026 Adana | Sonuç dosyasında yalnızca 1. ve 2. koşu var | Koşuların yapılıp yapılmadığı resmî kaynaktan doğrulanamadı | **İNCELEME GEREKLİ** |
| 21.03.2026 Adana | Sonuç adresi HTTP 200, 0 bayt (iki deneme) | Sonuç verisi yok; nedeni doğrulanamadı | **İNCELEME GEREKLİ** |
| 21.03.2026 İstanbul | Eküri kaydı (5) LAZMAN + (6) LEONİDAS; ham veri kendi içinde tutarlı | Uyarı, ayrıştırıcının bilmediği `YP` ekipman kısaltmasından geliyor | **İNCELEME GEREKLİ** (ayrıştırıcı düzeltmesi ve resmî teyit bekliyor) |
| 31.03.2026 Antalya | 8. koşuda iki birinci (5 ve 6), aynı derece, iki GANYAN ödemesi | Ham veri dead heat (ölü yarış) gösteriyor; ayrıştırıcı tek birinci bekliyor | **İNCELEME GEREKLİ** (resmî teyit bekliyor) |

---

## 1. 15.03.2026 Adana (Gün durumu: INCELEME_GEREKLI)

- **Kaynak URL'ler:**
  - Program: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-15/CSV/GunlukYarisProgrami/15.03.2026-Adana-GunlukYarisProgrami-TR.csv (HTTP 200, 15803 bayt, SHA-256 `78b0403a6190a0e3ab52844cc10ecb9afa214c33e7160a3a788483f691f0e008`)
  - Sonuç: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-15/CSV/GunlukYarisSonuclari/15.03.2026-Adana-GunlukYarisSonuclari-TR.csv (HTTP 200, 3178 bayt, SHA-256 `038eb81a9d6d5dbd62a8d44a2a431e8a2841962db45570c677c111db6e10427e`)
- **İlgili koşular:** programda 8 koşu var (17.45–21.45); sonuç dosyasında yalnızca 1. ve 2. koşu var. Eksikler: 3. koşu (19.15) ile 8. koşu (21.45) arası.
- **Kanıt satırları:**
  - Program, satır 30: `3. Kosu :   19.15;Handikap 15/DHÖW/Dişi /H1; 4 ve Yukarı Araplar; kg; 1900m; Kum;...`
  - Program, satır 109: `8. Kosu :   21.45;Handikap 16 /H1; 4 ve Yukarı İngilizler; kg; 1400m; Kum;...`
  - Sonuç dosyasında koşu başlıkları yalnızca satır 2 (`1. Kosu :   17.45;...`) ve satır 16 (`2. Kosu :   18.30;...`).
  - Sonuç dosyasının son veri satırı (2. koşu bahisleri): `[(6)KARA AYHAN,(5)BEŞİR ÇAVUŞ,] eküridir.GANYAN(3) :1,55 TL, ... 1. ÇİFTE(1/3) :2,85 TL`. Dosya bu satırdan sonra `CRLF CRLF` ile düzgün bitiyor; satır ortasında kesilmiş bir aktarım izi yok.
- **Sonuç:** Dosya biçim olarak eksiksiz görünüyor ama yalnızca iki koşu içeriyor. Bunun iki açıklaması olabilir: koşular 2. koşudan sonra iptal edildi ya da ertelendi, veya TJK yalnızca kısmi bir sonuç dosyası yayımladı. Resmî sonuç sayfası ve duyurular kontrol edilemediği için hangisinin doğru olduğu **kesinleşmedi**.
- **Durum:** İNCELEME GEREKLİ.

## 2. 21.03.2026 Adana (Gün durumu: INCELEME_GEREKLI)

- **Kaynak URL'ler:**
  - Program: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisProgrami/21.03.2026-Adana-GunlukYarisProgrami-TR.csv (HTTP 200, 17685 bayt, SHA-256 `de660aa8547daa74529d915bd42b14d58461b88652b31f3cd947d62eef35f26d`; içerik doğrulandı: `Adana;(34. Yarış Günü);21/03/2026`, 9 koşu, ilk koşu 14.00)
  - Sonuç: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisSonuclari/21.03.2026-Adana-GunlukYarisSonuclari-TR.csv (HTTP 200, **0 bayt**; iki ayrı çalıştırmada aynı sonuç, `kontrol/mart_indirme_durumu.tsv`)
- **İlgili koşular:** günün tamamı (programda 9 koşu).
- **Kanıt:** `kontrol/mart_indirme_durumu.tsv` içinde iki satır: `2026-03-21 Adana sonuc 200 0 (sha yok) BICIM_HATASI dosya boş`.
- **Sonuç:** Sunucu dosya adresini tanıyor (404 değil, 200) ama içerik boş. Yarışların iptal mi edildiğini, ertelendi mi, yoksa yalnızca CSV'nin mi eksik yayımlandığını söylemek için resmî sonuç ya da duyuru gerekiyor; bu kontrol yapılamadı. **Kesinleşmedi.**
  Not: 20.03.2026 resmî takvimde Ramazan Bayramı (`RMZ`) olarak işaretli; 21.03 bayramın 2. günü. Bu yalnızca bağlam bilgisi, kanıt değil.
- **Durum:** İNCELEME GEREKLİ.

## 3. 21.03.2026 İstanbul, 6. koşu: LEONİDAS eküri kaydı

- **Kaynak URL'ler:**
  - Program: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisProgrami/21.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv
  - Sonuç: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisSonuclari/21.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv
  - (HTTP, bayt ve SHA-256 değerleri: `kontrol/son_test.md`)
- **İlgili koşu:** 6. koşu, 20.30, ŞARTLI 5/DHÖW, 4 ve Yukarı Araplar, 1400m sentetik.
- **Kanıt satırları:**
  - Program, satır 104: `5;LAZMAN K DB;5y a  a;BAŞSEKLAVİ;SİM SİM;56;O.YILDIZ;METE ÇELİK;B.ÇAPKIN;...`
  - Program, satır 105: `6;LEONİDAS SKG SK YP;8y d  a;ŞAMİLBEY;BUZDENİZİ;52 +0.60;Ç.TAŞCI AP;METE ÇELİK;B.ÇAPKIN;...`
  - Program, satır 113: `[(5)LAZMAN,(6)LEONİDAS,] eküridir.2. 3'LÜ GANYAN;6. ÇİFTE Bu koşudan başlar;...`
  - Sonuç, satır 107: `8;LEONİDAS SKG SK YP;...;METE ÇELİK;B.ÇAPKIN;...;1:33.43;17,95;` (sonuç dosyasında "At No" sütunu liste sırası, program numarası değil)
  - Sonuç, satır 113: `[(5)LAZMAN,(6)LEONİDAS,] eküridir.GANYAN(11) :9,90 TL, ...`
  - Ayrıştırıcı uyarısı: `EKURI_UYESI_DOGRULANAMADI: ... satır 113 (6. koşu): (6)LEONİDAS — programda 6 numara: LEONİDAS SKG SK YP`
- **Değerlendirme:**
  - Ham veri kendi içinde tutarlı. Program numarası 6 gerçekten LEONİDAS. LAZMAN (5) ile aynı sahip (METE ÇELİK) ve aynı antrenörde (B.ÇAPKIN), yani eküri kaydı mantıklı. Eküri satırı program ve sonuç dosyasında aynı.
  - Uyarının nedeni ayrıştırıcıda. `tjk_ayristirici/src/ayristir.rs` satır 6'daki ekipman listesi (`KOD_BELIRTECLERI` = KG, K, DB, SK, SKG, GKR, ÖG, SGKR) `YP` kısaltmasını içermiyor. Bu yüzden at adından `YP` ayıklanamıyor ve "LEONİDAS" ile "LEONİDAS SKG SK YP" eşleşmiyor. Ayrıştırıcı bu adımda değiştirilmedi.
- **Sonuç:** Kaynak verisinde bir hata görünmüyor; sorun ayrıştırıcının `YP` ekini tanımaması. Resmî sonuç sayfasıyla karşılaştırma yapılamadı. Kayıt, ayrıştırıcı düzeltilip yeniden ayrıştırılana ve resmî teyit gelene kadar **incelemede kalıyor**.
- **Durum:** İNCELEME GEREKLİ.

## 4. 31.03.2026 Antalya, 8. koşu: bitiriş sırası ve GANYAN

- **Kaynak URL'ler:**
  - Program: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-31/CSV/GunlukYarisProgrami/31.03.2026-Antalya-GunlukYarisProgrami-TR.csv
  - Sonuç: https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-31/CSV/GunlukYarisSonuclari/31.03.2026-Antalya-GunlukYarisSonuclari-TR.csv
  - (HTTP, bayt ve SHA-256 değerleri: `kontrol/son_test.md`)
- **İlgili koşu:** 8. koşu, 21.30, Handikap 15/DHÖW/Dişi /H1, 4 ve Yukarı Araplar, 1400m sentetik, 17 at.
- **Kanıt satırları:**
  - Program: `5;MAVİŞİN KIZI KG K DB;...` ve `6;ŞUKELA KG K DB;...`
  - Sonuç, 1. sıra: `1;MAVİŞİN KIZI KG K DB;6y k k;...;U.TEMUR;...;1:33.70;1,80;At Başı`
  - Sonuç, 2. sıra: `2;ŞUKELA KG K DB;4y a k;...;A.ÇELİK;...;1:33.70;1,60;Baş`
  - Sonuç, 3. sıra: `3;HADEHANIM K GKR;...;1:33.72;3,85;1,5  Boy`
  - Bahis satırı: `GANYAN(5) :1,80 TL, GANYAN(6) :1,60 TL, ... 2. 6'LI GANYAN(7/8/1/1/8/5) :2.093,81 TL, 2. 6'LI GANYAN(7/8/1/1/8/6) :1.686,12 TL, 5'Lİ GANYAN(8/1/1/8/5) :596,85 TL, 5'Lİ GANYAN(8/1/1/8/6) :528,75 TL, 4'LÜ GANYAN(1/1/8/5) :179,81 TL, 4'LÜ GANYAN(1/1/8/6) :139,56 TL, ...`
  - Ayrıştırıcı uyarısı: `GANYAN_BIRINCI_CELISKISI: 8. koşu GANYAN [5, 6], sonuçta birinci [5]`
- **Değerlendirme:** Program 5 (MAVİŞİN KIZI) ve program 6 (ŞUKELA) aynı derecede (1:33.70) bitirmiş. Her birine ayrı GANYAN ödemesi var. Satırdaki ganyan değerleri (1,80 ve 1,60) bahis satırıyla birebir aynı. Bu koşuyla biten tüm zincirleme bahisler (6'lı, 5'li, 4'lü) hem 5 hem 6 ile ayrı ayrı ödenmiş. Bu tablo birincilikte dead heat (ölü yarış) ile tutarlı. Ayrıştırıcı ise tek bir birinci beklediği için çelişki bildiriyor.
- **Sonuç:** Ham CSV'ye göre 8. koşuda iki birinci var (5 ve 6, dead heat). Resmî sonuç sayfasıyla karşılaştırma yapılamadığı için **kesinleşmedi**.
- **Durum:** İNCELEME GEREKLİ.

---

## Sonraki adım için gerekenler

- Resmî sonuç sayfaları ve duyurular `www.tjk.org` üzerinde. Bu adres bu ortamın ağ izin listesine eklenirse dört günün resmî teyidi yapılabilir.
- 21 Mart İstanbul için ayrıştırıcının ekipman listesine `YP` eklenmeli. 31 Mart Antalya için dead heat desteği gerekiyor. İkisi de bu adımda istenmediği için yapılmadı.
