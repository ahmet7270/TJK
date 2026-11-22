# Son veri testi

- **Durum:** BAŞARILI (6–10 Mart: 10/10 çift tamamlandı) · 15.03 Adana'daki iki kayıt açık
- **Tarih:** 2026-11-22
- **Araçlar:** `tjk_indir liste --mevcut` (aday + depodaki ham dosyaların içerik/SHA doğrulaması), `tjk-fetch daralt`, `tjk-fetch toplu`
  (curl; robots.txt HTTP 404, kural yok), `tjk_ayristirici` 0.12.0
- **SQLite:** kullanılmadı · **Mevcut ham CSV'ler:** değiştirilmedi
- **Çalışma listesi:** `mart_2026_0610_20.tsv` (resmî takvimdeki 6–10 Mart'ın 10 çifti, 20 benzersiz URL)

## 6–10 Mart çiftleri

| Tarih | Hipodrom | Durum | Koşu / at satırı | Bahis kontrolü |
|---|---|---|---|---|
| 2026-03-06 | Bursa | TAMAMLANDI | 7 koşu / 73 at | 21 kontrol, 0 uyumsuz |
| 2026-03-06 | İzmir | TAMAMLANDI | 7 koşu / 64 at | 21 kontrol, 0 uyumsuz |
| 2026-03-07 | Adana | TAMAMLANDI | 8 koşu / 87 at | 23 kontrol, 0 uyumsuz |
| 2026-03-07 | İstanbul | TAMAMLANDI | 8 koşu / 96 at | 25 kontrol, 0 uyumsuz |
| 2026-03-08 | Adana | TAMAMLANDI | 9 koşu / 100 at | 27 kontrol, 0 uyumsuz |
| 2026-03-08 | İzmir | TAMAMLANDI | 8 koşu / 73 at | 23 kontrol, 0 uyumsuz |
| 2026-03-09 | Bursa | TAMAMLANDI | 8 koşu / 89 at | 23 kontrol, 0 uyumsuz |
| 2026-03-09 | Şanlıurfa | TAMAMLANDI | 8 koşu / 101 at | 24 kontrol, 0 uyumsuz |
| 2026-03-10 | Adana | TAMAMLANDI | 8 koşu / 79 at | 23 kontrol, 0 uyumsuz |
| 2026-03-10 | Antalya | TAMAMLANDI | 7 koşu / 57 at | 20 kontrol, 0 uyumsuz |

06.03 İzmir programı indirilmedi: depodaki `kontrol/ham/06.03.2026-İzmir-GunlukYarisProgrami-TR.csv` (12660 bayt,
SHA-256 `45801a1b…bf594`, ilk testteki değerle aynı) içerikten doğrulanıp kullanıldı. Kalan 19 dosya indirildi.

**Önceki çıktılarla karşılaştırma:** 6–10 Mart çiftleri daha önce bu depo dışında işlenmişti; o çıktılar (ör. ZIP'ten bilerek
çıkarılan `tests/veri/izmir/beklenen_v0_2`) depoda ve bu ortamda yok, bu yüzden karşılaştırma yapılamadı. Karşılaştırılabilen tek
öğe 06.03 İzmir programı: bayt sayısı ve SHA-256 ilk testteki değerle aynı; ilk satır `İzmir;(29. Yarış Günü);06/03/2026`.

## Dosyalar (HTTP "-" = depodaki ham dosya kullanıldı)

| Tarih | Hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | Doğrulama |
|---|---|---|---|---|---|---|---|
| 2026-03-06 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-06/CSV/GunlukYarisProgrami/06.03.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 13790 | `5b09836cc4b1c9cbee6560d72be77460139717fc3bb691febae67771d2a51db6` | ICERIK_DOGRULANDI |
| 2026-03-06 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-06/CSV/GunlukYarisSonuclari/06.03.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 14173 | `81d0bffb0d6fde6d5b593175dc5220bcde789617c902b0f76d6474824486f5fb` | ICERIK_DOGRULANDI |
| 2026-03-06 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-06/CSV/GunlukYarisProgrami/06.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | - | 12660 | `45801a1bafe2fdf3bd299759844c25d59766b8d5f753cc2ebf89b0064ffbf594` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-06 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-06/CSV/GunlukYarisSonuclari/06.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 12942 | `4f2154b961bd76579866f823c787db7a68db2a74b3b93f330177709c3615af46` | ICERIK_DOGRULANDI |
| 2026-03-07 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-07/CSV/GunlukYarisProgrami/07.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 16731 | `1fa9a343ac499556f4a3ee26c1925229ffa35b8971b475da72deda263f524ad1` | ICERIK_DOGRULANDI |
| 2026-03-07 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-07/CSV/GunlukYarisSonuclari/07.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 17143 | `656b0752847b8073f1553eaca6b61ba1ebbb9aac578c3b8d4c2f6f675de12795` | ICERIK_DOGRULANDI |
| 2026-03-07 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-07/CSV/GunlukYarisProgrami/07.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17606 | `833d99774ad6e996493062e4f564ebd63de04c8aa5ba956cff22408ab52b8852` | ICERIK_DOGRULANDI |
| 2026-03-07 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-07/CSV/GunlukYarisSonuclari/07.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 18295 | `e687917ff2191f2a7dac0a900231c1accfc5fdf5c30d06f86e4721043282014d` | ICERIK_DOGRULANDI |
| 2026-03-08 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-08/CSV/GunlukYarisProgrami/08.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 18921 | `6873ea160754021e7faed3f56bb729ec09c99d7c29974b0f0369e74a1e8e2d38` | ICERIK_DOGRULANDI |
| 2026-03-08 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-08/CSV/GunlukYarisSonuclari/08.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 19470 | `3cb6926b851376911792a376c041bbf7ad125068b319c9ae183e5ff79693eb39` | ICERIK_DOGRULANDI |
| 2026-03-08 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-08/CSV/GunlukYarisProgrami/08.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 14672 | `11b11e64af9f726713dc9cfa9687a5ee6498b83b5547417ce99f07dd3d42a661` | ICERIK_DOGRULANDI |
| 2026-03-08 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-08/CSV/GunlukYarisSonuclari/08.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 14978 | `5c1b0c717e6cc3199182b32332f585406db435a9547b1e0dfbefa548cccf15d4` | ICERIK_DOGRULANDI |
| 2026-03-09 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-09/CSV/GunlukYarisProgrami/09.03.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 17027 | `cce8e04cfb2d4c9c6c85b5495bf505873747e461700275cd465f91a0c0d76893` | ICERIK_DOGRULANDI |
| 2026-03-09 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-09/CSV/GunlukYarisSonuclari/09.03.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 17371 | `261328257a1e17f1680e2d709ee3f89c7aad19803626c84539dfe375309d1ecb` | ICERIK_DOGRULANDI |
| 2026-03-09 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-09/CSV/GunlukYarisProgrami/09.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 19195 | `1566e1c05c5e93d5741ead600bf24bea1eb214f0b48749014cbc1a7006dcfc72` | ICERIK_DOGRULANDI |
| 2026-03-09 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-09/CSV/GunlukYarisSonuclari/09.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 19780 | `faef351bd26bc49a6e74dfe1d6ccc7f236e40210a18a7173ce5cf34e8a2395f0` | ICERIK_DOGRULANDI |
| 2026-03-10 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-10/CSV/GunlukYarisProgrami/10.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15305 | `15ce4b071cbe8537b966984f8e69469c78c5ddc8d16da49bd2958f969a4b1c99` | ICERIK_DOGRULANDI |
| 2026-03-10 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-10/CSV/GunlukYarisSonuclari/10.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15720 | `41f8780a03e4511f013ca81415fd0e8c77e0252aa06015c9087b7f7cbbf1054e` | ICERIK_DOGRULANDI |
| 2026-03-10 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-10/CSV/GunlukYarisProgrami/10.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 11302 | `2b32f31afd0a35eddba822a09705a793f58c4a71066a698cf00ea7abdb4d11e7` | ICERIK_DOGRULANDI |
| 2026-03-10 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-10/CSV/GunlukYarisSonuclari/10.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 11552 | `b2a7f3950092110f692d35deeafdb1d2a7c25cb420e873a866cbe1c1f134155a` | ICERIK_DOGRULANDI |

Doğrulanan ham CSV'ler `kontrol/ham` altında (`yerel_dosya` adlarıyla). Ayrıntılı kayıt: `kontrol/mart_indirme_durumu.tsv`.

## 15.03.2026 Adana: 3. koşu GANYAN ve 1. altılı ödemesi (araştırma)

- `medya-cdn.tjk.org`'dan 16.03 Adana programı indirildi ve doğrulandı: `kontrol/ham/20260316-ADANA-program.csv`
  (https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-16/CSV/GunlukYarisProgrami/16.03.2026-Adana-GunlukYarisProgrami-TR.csv,
  HTTP 200, 12063 bayt, SHA-256 `655800eeed82567d53d9c689784e8d68b2ecfcfd2b56ddf7bbcebb9f6aafdb2b`, `Adana;(32. Yarış Günü);16/03/2026`).
  - 3. koşunun bahis listesi: `7'Lİ PLASE;7'Lİ GANYAN;2. 6'LI GANYAN Bu koşudan başlar;3. ÇİFTE Bu koşudan başlar;GANYAN;İKİLİ;SIRALI İKİLİ;SABİT GANYAN;…`.
    GANYAN 16.03'te 3. koşu için programda var; 16.03 sonuç dosyasının 3. koşu satırında ise yalnızca 7'li ödemeleri var.
  - `1. 6'LI GANYAN` 16.03 programında hiç geçmiyor (1.–2. ayakları 15.03'te koşulmuştu).
  - 15.03 ve 16.03 program/sonuç dosyalarının hiçbirinde "iade", "iptal", "geçersiz" ya da "ertel" ibaresi yok.
- `www.tjk.org` (resmî sonuç sayfaları ve duyurular) bu ortamın izin listesine kullanıcı onayıyla eklendi. Ancak siteye yapılan ilk
  istek (robots.txt kontrolü) oturumun otomatik izin denetimi tarafından reddedildi (`Unauthorized Access`). Atlatılmaya çalışılmadı,
  bu yüzden resmî sonuç sayfası ve bahis duyurusu okunamadı.
- **Sonuç:** Tutar ya da iade kararı için doğrudan kanıt bulunamadı. İki kayıt **açık**: `GANYAN_SATIRI_BULUNAMADI: 3. koşu` ve
  `ALTILI_ODEMESI_BULUNAMADI: 20260315-ADANA-A1`. Doğrulanmış sekiz koşu (1–2: 15.03, 3–8: 16.03; 77/77 at) korunuyor.

## Mart 2026 toplamı (resmî takvim: 61 planlı tarih–hipodrom çifti)

| Durum | Çift |
|---|---|
| Tamamlandı | **60** |
| İnceleme gerekli | **1** (15.03 Adana: 2 açık ödeme kaydı) |
| Eksik | 0 |
| **Toplam** | **61** |

| Tarih | Hipodrom | Durum | Not |
|---|---|---|---|
| 2026-03-01 | İzmir | TAMAMLANDI |  |
| 2026-03-01 | Adana | TAMAMLANDI |  |
| 2026-03-02 | Bursa | TAMAMLANDI |  |
| 2026-03-02 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-03 | Adana | TAMAMLANDI |  |
| 2026-03-03 | Antalya | TAMAMLANDI |  |
| 2026-03-04 | İstanbul | TAMAMLANDI |  |
| 2026-03-04 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-05 | İzmir | TAMAMLANDI |  |
| 2026-03-05 | Antalya | TAMAMLANDI |  |
| 2026-03-06 | Bursa | TAMAMLANDI | bu testte |
| 2026-03-06 | İzmir | TAMAMLANDI | bu testte |
| 2026-03-07 | Adana | TAMAMLANDI | bu testte |
| 2026-03-07 | İstanbul | TAMAMLANDI | bu testte |
| 2026-03-08 | İzmir | TAMAMLANDI | bu testte |
| 2026-03-08 | Adana | TAMAMLANDI | bu testte |
| 2026-03-09 | Bursa | TAMAMLANDI | bu testte |
| 2026-03-09 | Şanlıurfa | TAMAMLANDI | bu testte |
| 2026-03-10 | Adana | TAMAMLANDI | bu testte |
| 2026-03-10 | Antalya | TAMAMLANDI | bu testte |
| 2026-03-11 | İstanbul | TAMAMLANDI |  |
| 2026-03-11 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-12 | İzmir | TAMAMLANDI |  |
| 2026-03-12 | Antalya | TAMAMLANDI |  |
| 2026-03-13 | Antalya | TAMAMLANDI |  |
| 2026-03-13 | İzmir | TAMAMLANDI |  |
| 2026-03-14 | Adana | TAMAMLANDI |  |
| 2026-03-14 | İstanbul | TAMAMLANDI |  |
| 2026-03-15 | İzmir | TAMAMLANDI |  |
| 2026-03-15 | Adana | İNCELEME GEREKLİ | 8 koşu doğrulandı (3–8: 16.03); 3. koşu GANYAN ve 1. altılı ödemesi açık |
| 2026-03-16 | Bursa | TAMAMLANDI |  |
| 2026-03-16 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-17 | Adana | TAMAMLANDI |  |
| 2026-03-17 | Antalya | TAMAMLANDI |  |
| 2026-03-18 | İstanbul | TAMAMLANDI |  |
| 2026-03-18 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-19 | İzmir | TAMAMLANDI |  |
| 2026-03-19 | Antalya | TAMAMLANDI |  |
| 2026-03-20 | İzmir | TAMAMLANDI |  |
| 2026-03-21 | Adana | TAMAMLANDI | Sonuçlar 23.03 dosyasında (ertelendi); 4. koşu pist Çim→Kum |
| 2026-03-21 | İstanbul | TAMAMLANDI | YP düzeltmesi |
| 2026-03-22 | İzmir | TAMAMLANDI |  |
| 2026-03-22 | Adana | TAMAMLANDI |  |
| 2026-03-23 | Bursa | TAMAMLANDI |  |
| 2026-03-23 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-24 | Adana | TAMAMLANDI |  |
| 2026-03-24 | Antalya | TAMAMLANDI |  |
| 2026-03-25 | İstanbul | TAMAMLANDI |  |
| 2026-03-25 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-26 | İzmir | TAMAMLANDI |  |
| 2026-03-26 | Antalya | TAMAMLANDI |  |
| 2026-03-27 | Antalya | TAMAMLANDI |  |
| 2026-03-27 | İzmir | TAMAMLANDI |  |
| 2026-03-28 | Adana | TAMAMLANDI |  |
| 2026-03-28 | İstanbul | TAMAMLANDI |  |
| 2026-03-29 | İzmir | TAMAMLANDI |  |
| 2026-03-29 | Adana | TAMAMLANDI |  |
| 2026-03-30 | Bursa | TAMAMLANDI |  |
| 2026-03-30 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-31 | Adana | TAMAMLANDI |  |
| 2026-03-31 | Antalya | TAMAMLANDI | 8. koşu ortak birinci 5+6 |
