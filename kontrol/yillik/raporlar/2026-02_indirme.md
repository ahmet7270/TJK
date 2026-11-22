# Son veri testi

- **Durum:** TAMAMLANDI (liste sonuna kadar işlendi)
- **Çalışma:** Mart 2026 toplu indirme, `/home/user/TJK/kontrol/yillik/listeler/2026-02_liste.tsv`
- **Araçlar:** `tjk-fetch toplu` (curl ile HTTPS; içerik doğrulaması `tjk_ayristirici/src/ayristir.rs` ile), birlikte ayrıştırma `tjk_ayristirici`
- **SQLite:** kullanılmadı
- **Ön kontrol:** 56 çift, 112 satır, 112 benzersiz URL; her çiftte bir program ve bir sonuç
- **robots.txt:** HTTP 404: robots.txt yok, kural tanımlı değil (RFC 9309: izinli)

## Özet (bu çalışmadaki 56 çift)

| Durum | Çift |
|---|---|
| Tamamlandı | 53 |
| İnceleme gerekli | 3 |
| Eksik | 0 |
| Henüz denenmedi | 0 |
| **Toplam** | **56** |

Önceki çalışmadan SHA-256'sı eşleşerek yeniden kullanılan dosya: 0

## Çiftler

| # | Tarih | Hipodrom | Durum | Gerekçe |
|---|---|---|---|---|
| 1 | 2026-02-01 | Adana | TAMAMLANDI |  |
| 2 | 2026-02-01 | İzmir | TAMAMLANDI |  |
| 3 | 2026-02-02 | Bursa | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: ORTAK_BIRINCILIK_DOGRULANAMADI ×1 (ilk: ORTAK_BIRINCILIK_DOGRULANAMADI: 3. koşu GANYAN [1, 7]: ilk 2 sıradaki program numaraları [2, 7], GANYAN ödemeleri [1, 7]; ortak birinciler arasındaki Fark hücreleri ["Burun"]; 'At Başı' bekleniyordu; dereceler farklı ["2:12.51", "2:12.52"]) |
| 4 | 2026-02-02 | Şanlıurfa | TAMAMLANDI |  |
| 5 | 2026-02-03 | Adana | TAMAMLANDI |  |
| 6 | 2026-02-03 | Antalya | TAMAMLANDI |  |
| 7 | 2026-02-04 | İstanbul | TAMAMLANDI |  |
| 8 | 2026-02-04 | Şanlıurfa | TAMAMLANDI |  |
| 9 | 2026-02-05 | Antalya | TAMAMLANDI |  |
| 10 | 2026-02-05 | İzmir | TAMAMLANDI |  |
| 11 | 2026-02-06 | Bursa | TAMAMLANDI |  |
| 12 | 2026-02-06 | İzmir | TAMAMLANDI |  |
| 13 | 2026-02-07 | Adana | TAMAMLANDI |  |
| 14 | 2026-02-07 | İstanbul | TAMAMLANDI |  |
| 15 | 2026-02-08 | Adana | TAMAMLANDI |  |
| 16 | 2026-02-08 | İzmir | TAMAMLANDI |  |
| 17 | 2026-02-09 | Bursa | TAMAMLANDI |  |
| 18 | 2026-02-09 | Şanlıurfa | TAMAMLANDI |  |
| 19 | 2026-02-10 | Adana | TAMAMLANDI |  |
| 20 | 2026-02-10 | Antalya | TAMAMLANDI |  |
| 21 | 2026-02-11 | İstanbul | TAMAMLANDI |  |
| 22 | 2026-02-11 | Şanlıurfa | TAMAMLANDI |  |
| 23 | 2026-02-12 | Antalya | TAMAMLANDI |  |
| 24 | 2026-02-12 | İzmir | TAMAMLANDI |  |
| 25 | 2026-02-13 | Antalya | TAMAMLANDI |  |
| 26 | 2026-02-13 | İzmir | TAMAMLANDI |  |
| 27 | 2026-02-14 | Adana | TAMAMLANDI |  |
| 28 | 2026-02-14 | İstanbul | TAMAMLANDI |  |
| 29 | 2026-02-15 | Adana | TAMAMLANDI |  |
| 30 | 2026-02-15 | İzmir | TAMAMLANDI |  |
| 31 | 2026-02-16 | Bursa | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: ORTAK_BIRINCILIK_DOGRULANAMADI ×1 (ilk: ORTAK_BIRINCILIK_DOGRULANAMADI: 6. koşu GANYAN [3, 4]: ilk 2 sıradaki program numaraları [2, 3], GANYAN ödemeleri [3, 4]; ortak birinciler arasındaki Fark hücreleri ["Yarım Boy"]; 'At Başı' bekleniyordu; dereceler farklı ["1:38.33", "1:38.41"]) |
| 32 | 2026-02-16 | Şanlıurfa | TAMAMLANDI |  |
| 33 | 2026-02-17 | Adana | TAMAMLANDI |  |
| 34 | 2026-02-17 | Antalya | TAMAMLANDI |  |
| 35 | 2026-02-18 | İstanbul | TAMAMLANDI |  |
| 36 | 2026-02-18 | Şanlıurfa | TAMAMLANDI |  |
| 37 | 2026-02-19 | Antalya | TAMAMLANDI |  |
| 38 | 2026-02-19 | İzmir | TAMAMLANDI |  |
| 39 | 2026-02-20 | Bursa | TAMAMLANDI |  |
| 40 | 2026-02-20 | İzmir | TAMAMLANDI |  |
| 41 | 2026-02-21 | Adana | TAMAMLANDI |  |
| 42 | 2026-02-21 | İstanbul | TAMAMLANDI |  |
| 43 | 2026-02-22 | Adana | TAMAMLANDI |  |
| 44 | 2026-02-22 | İzmir | TAMAMLANDI |  |
| 45 | 2026-02-23 | Bursa | TAMAMLANDI |  |
| 46 | 2026-02-23 | Şanlıurfa | TAMAMLANDI |  |
| 47 | 2026-02-24 | Adana | TAMAMLANDI |  |
| 48 | 2026-02-24 | Antalya | TAMAMLANDI |  |
| 49 | 2026-02-25 | İstanbul | TAMAMLANDI |  |
| 50 | 2026-02-25 | Şanlıurfa | TAMAMLANDI |  |
| 51 | 2026-02-26 | Antalya | TAMAMLANDI |  |
| 52 | 2026-02-26 | İzmir | TAMAMLANDI |  |
| 53 | 2026-02-27 | Antalya | TAMAMLANDI |  |
| 54 | 2026-02-27 | İzmir | TAMAMLANDI |  |
| 55 | 2026-02-28 | Adana | TAMAMLANDI |  |
| 56 | 2026-02-28 | İstanbul | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: GANYAN_SATIRI_BULUNAMADI ×1 (ilk: GANYAN_SATIRI_BULUNAMADI: 4. koşu (birinci [6])) |

## Dosyalar

| Tarih | Hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | Doğrulama |
|---|---|---|---|---|---|---|---|
| 2026-02-01 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-01/CSV/GunlukYarisProgrami/01.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13762 | `ee12244ede53912ae7e8727f2b99526fd08cb60ed929d8a5d95f608343b2fee0` | ICERIK_DOGRULANDI |
| 2026-02-01 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-01/CSV/GunlukYarisSonuclari/01.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 13961 | `5f4bacb0ed5920ee9d55839d19fedaebd106fcbd91e191e29b3c91dc60622355` | ICERIK_DOGRULANDI |
| 2026-02-01 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-01/CSV/GunlukYarisProgrami/01.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 12832 | `a25ffe2f1165ef0feaa072bfaff1aaaf50b045d4484d466fc1b8db3754f49830` | ICERIK_DOGRULANDI |
| 2026-02-01 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-01/CSV/GunlukYarisSonuclari/01.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 13178 | `2fea3ce6b718c1181b83f9f15fb773860e72ab9ce09552e9c8733a412c2a779a` | ICERIK_DOGRULANDI |
| 2026-02-02 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-02/CSV/GunlukYarisProgrami/02.02.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 16574 | `c71439a20d3b2e59ed13e778533e66c07cefae85728cb57202e92ea59639ffef` | ICERIK_DOGRULANDI |
| 2026-02-02 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-02/CSV/GunlukYarisSonuclari/02.02.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 17085 | `f2b0bda5e1e3725a937562386fbbf2db88bae6d3dc30190b4b404e921fcfcc02` | ICERIK_DOGRULANDI |
| 2026-02-02 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-02/CSV/GunlukYarisProgrami/02.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 15379 | `9ae0f640d9dcd5eccdbe2ffd76573ce5573d82cbc0ec592937c9867930cc3b7a` | ICERIK_DOGRULANDI |
| 2026-02-02 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-02/CSV/GunlukYarisSonuclari/02.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 15981 | `cfa6f38e0384e05b6f362d7f8f0ad6b55c78ce5f7efa9e91a5cd0eb26a51256b` | ICERIK_DOGRULANDI |
| 2026-02-03 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-03/CSV/GunlukYarisProgrami/03.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13612 | `9d10c9b666b8a4d6b936d60b59dfd72a9b0ccee7029d271590957e77fa6e1f96` | ICERIK_DOGRULANDI |
| 2026-02-03 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-03/CSV/GunlukYarisSonuclari/03.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 13914 | `0f62c7d929a58d39754725d4ce147af4f829d03aac1ec8a19717a20de4d980d7` | ICERIK_DOGRULANDI |
| 2026-02-03 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-03/CSV/GunlukYarisProgrami/03.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 15463 | `f175fbaca70e3c09ee3f44d687a48ed5eb30ac30ff5d729112b96d0a2e6ab6f9` | ICERIK_DOGRULANDI |
| 2026-02-03 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-03/CSV/GunlukYarisSonuclari/03.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15870 | `a8bfdc841967b5b6dd9627168ddb648358ff2f8ef638fdcaf09da7c82db365fc` | ICERIK_DOGRULANDI |
| 2026-02-04 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-04/CSV/GunlukYarisProgrami/04.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16879 | `5aa770d6b8b5724132f92e8159af02c04509fa6978810ab84f1e8764da329e9c` | ICERIK_DOGRULANDI |
| 2026-02-04 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-04/CSV/GunlukYarisSonuclari/04.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17441 | `1721858a097a51ccf25669abdd4000c34dc2374ebbefa3692ae92867712a35ae` | ICERIK_DOGRULANDI |
| 2026-02-04 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-04/CSV/GunlukYarisProgrami/04.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 14866 | `03fe0db02ac3c0f4b01bd6c8f9156f0c0528e4b6521162ffaea43d54ea10a8cd` | ICERIK_DOGRULANDI |
| 2026-02-04 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-04/CSV/GunlukYarisSonuclari/04.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 15316 | `af66e803586cf9b0c3aa775dc62c2df7e51f10f08b82d71c0a84c26adab5bd4c` | ICERIK_DOGRULANDI |
| 2026-02-05 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-05/CSV/GunlukYarisProgrami/05.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 14401 | `bb129385b25c78fbb145ab2faa4a86b864fbae944ec7eb58296df994b880fb61` | ICERIK_DOGRULANDI |
| 2026-02-05 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-05/CSV/GunlukYarisSonuclari/05.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 14855 | `a03c6862fa52a0fb3770e322aecaab50d3bd9e092e69d1ffd43937e7db3b853c` | ICERIK_DOGRULANDI |
| 2026-02-05 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-05/CSV/GunlukYarisProgrami/05.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 8537 | `2a5fea7a8481184ae8a53352ad8de45f22cf42fcf58e443909df397ffc8a56d6` | ICERIK_DOGRULANDI |
| 2026-02-05 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-05/CSV/GunlukYarisSonuclari/05.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 8492 | `1975626f2c35eb522c644c57579c3bfdd3741ea95b134db8ffc36b3a80234ce6` | ICERIK_DOGRULANDI |
| 2026-02-06 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-06/CSV/GunlukYarisProgrami/06.02.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 9883 | `1aa6a59d5bca47b6f175951c85384b086034ffde665a3b6039cee64993cd268a` | ICERIK_DOGRULANDI |
| 2026-02-06 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-06/CSV/GunlukYarisSonuclari/06.02.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 10065 | `020871563c8736443db604b384188845d6ccbd2711d96369b45e98357d4878d1` | ICERIK_DOGRULANDI |
| 2026-02-06 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-06/CSV/GunlukYarisProgrami/06.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 10973 | `a8ae7e264e3f1f943150ae56d2d5826380a82cd8ba84d20bcc2230f6d0a700d6` | ICERIK_DOGRULANDI |
| 2026-02-06 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-06/CSV/GunlukYarisSonuclari/06.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 11261 | `e5e2646f089d1a797de1018aacad07bb65ccc9d905311a19e2b1280594733756` | ICERIK_DOGRULANDI |
| 2026-02-07 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-07/CSV/GunlukYarisProgrami/07.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 14903 | `a5c359d1995c79ab430384f5df402da6b2079e5fad54aac268d318e072922c23` | ICERIK_DOGRULANDI |
| 2026-02-07 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-07/CSV/GunlukYarisSonuclari/07.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15209 | `e3aff0916d428774e7c13cc2a221b09300737e42b6960bafd290fd002eed6c55` | ICERIK_DOGRULANDI |
| 2026-02-07 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-07/CSV/GunlukYarisProgrami/07.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16912 | `f9685383855797ec0f794816d2dd930c481b6cc901bba1df3c1cf998e58ea19d` | ICERIK_DOGRULANDI |
| 2026-02-07 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-07/CSV/GunlukYarisSonuclari/07.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17428 | `04fec94478fab3b94d420b66e377120a2b09486a444118d23197015c74b76032` | ICERIK_DOGRULANDI |
| 2026-02-08 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-08/CSV/GunlukYarisProgrami/08.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13723 | `45b857cbb3065f10cf45778344510e95ccb7e86f811c5f07aed40a065e6f9f5f` | ICERIK_DOGRULANDI |
| 2026-02-08 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-08/CSV/GunlukYarisSonuclari/08.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 13794 | `bac02a3cb5ed7084996b1650ea16b32e19ac76baf18121981719ca8fc24f79c7` | ICERIK_DOGRULANDI |
| 2026-02-08 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-08/CSV/GunlukYarisProgrami/08.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 13672 | `11a6bf587c949293fad2d1db5c8d54f9b499b7692303c88a1f6ea04b7b41552c` | ICERIK_DOGRULANDI |
| 2026-02-08 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-08/CSV/GunlukYarisSonuclari/08.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 13828 | `0b8f1b5ce919f7ca4a3120485f14c4011235010eb2c22d1843dc7d9850248dcf` | ICERIK_DOGRULANDI |
| 2026-02-09 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-09/CSV/GunlukYarisProgrami/09.02.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 17424 | `362f58363723d5fc17c9242b2075409c873c7f5150f3a47b05fcc19f6577b5a6` | ICERIK_DOGRULANDI |
| 2026-02-09 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-09/CSV/GunlukYarisSonuclari/09.02.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 18233 | `afd19a2ec1b0e135eb30b96677205b4b85f1ddb909d5900a0b1851ccd086dafa` | ICERIK_DOGRULANDI |
| 2026-02-09 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-09/CSV/GunlukYarisProgrami/09.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 17857 | `0c031fa5d9fb3acb42e5c313e040c8b1fa520dae49fd80300195b9563079a60f` | ICERIK_DOGRULANDI |
| 2026-02-09 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-09/CSV/GunlukYarisSonuclari/09.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 18420 | `79f4b7e697de9ea9ec43b07851d24ad7d1ba62668b82f4299de3a9f164af9c9d` | ICERIK_DOGRULANDI |
| 2026-02-10 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-10/CSV/GunlukYarisProgrami/10.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 16317 | `5b2d4b3ddb2be9734fccf54d7ca6a7b001b008f4befd270657cd18b3c0595f49` | ICERIK_DOGRULANDI |
| 2026-02-10 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-10/CSV/GunlukYarisSonuclari/10.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 16639 | `68fff809647ea56665078363007def1ec14a2dce117d0ad3138e6730d9c6f5fe` | ICERIK_DOGRULANDI |
| 2026-02-10 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-10/CSV/GunlukYarisProgrami/10.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 10766 | `a20d90f2defbac5750168d5e93f476854900d1871951c83da91c33ac075197ac` | ICERIK_DOGRULANDI |
| 2026-02-10 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-10/CSV/GunlukYarisSonuclari/10.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 11027 | `9080b609779f57349db6bd16cbffd3cad79d5d922b307b7c7d088e2e5af88aee` | ICERIK_DOGRULANDI |
| 2026-02-11 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-11/CSV/GunlukYarisProgrami/11.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 13862 | `f5afaa7311393966a8307661f37702bc36f6e90da024f3f679722d1f6147e0d5` | ICERIK_DOGRULANDI |
| 2026-02-11 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-11/CSV/GunlukYarisSonuclari/11.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 14109 | `cdfd4fe1b9be9c2b68fe6c87c08d0d6444f4230abac6456b37c3b9083544844f` | ICERIK_DOGRULANDI |
| 2026-02-11 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-11/CSV/GunlukYarisProgrami/11.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 15756 | `b58617756327fa554e75ba037c468fbe2af0d77e10a1e91bd8d2b592538c17aa` | ICERIK_DOGRULANDI |
| 2026-02-11 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-11/CSV/GunlukYarisSonuclari/11.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 16306 | `757df2769ff6e8176673e5f9346c44c03630c66d7f108344ae03f0839f77ce45` | ICERIK_DOGRULANDI |
| 2026-02-12 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-12/CSV/GunlukYarisProgrami/12.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 16823 | `d713ab2775c71ce706899f31189fdb02f74b581479b16c12825035f38ca49ff6` | ICERIK_DOGRULANDI |
| 2026-02-12 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-12/CSV/GunlukYarisSonuclari/12.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 17310 | `56df2b7457afc5185f67749001d1b58e6ee72e56b453d85cb4a86e73887d9d4e` | ICERIK_DOGRULANDI |
| 2026-02-12 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-12/CSV/GunlukYarisProgrami/12.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 9531 | `f6e39941a1626464b0bcfdb60901681e52a52954929f5f5a151e660a213c9720` | ICERIK_DOGRULANDI |
| 2026-02-12 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-12/CSV/GunlukYarisSonuclari/12.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 9571 | `ac0fb08b9bb6e5a2dd6222beeabad77ea275a16383bfc442b1a2d61676ea77fc` | ICERIK_DOGRULANDI |
| 2026-02-13 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-13/CSV/GunlukYarisProgrami/13.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 16284 | `c273b14d760c5cb23fd5cdfe290eedc0c70ebe16185f8dd18052a5b4b7294ce1` | ICERIK_DOGRULANDI |
| 2026-02-13 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-13/CSV/GunlukYarisSonuclari/13.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 17117 | `3282e7b39317ec9cb4a3c4fa45035467a04ba33cd9cb2107267d9140b8016554` | ICERIK_DOGRULANDI |
| 2026-02-13 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-13/CSV/GunlukYarisProgrami/13.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 9055 | `5e29a9d93596e8d90d4bedeeb10489d202d2556041269f4691112af79b5b1694` | ICERIK_DOGRULANDI |
| 2026-02-13 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-13/CSV/GunlukYarisSonuclari/13.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 8947 | `0a1063dc83939bd1fdfd32fe83ebaeb036a3b78244322a22779ffd19146b1476` | ICERIK_DOGRULANDI |
| 2026-02-14 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-14/CSV/GunlukYarisProgrami/14.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15041 | `c5631f99b52cd57b148fc3ca5cb0ab1da8b820c77eac32618dcbe3905db50e92` | ICERIK_DOGRULANDI |
| 2026-02-14 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-14/CSV/GunlukYarisSonuclari/14.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15341 | `3589b5255297341aa408f789c0ec630fbfae5ac6222956de86659ae19847976a` | ICERIK_DOGRULANDI |
| 2026-02-14 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-14/CSV/GunlukYarisProgrami/14.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16327 | `54c77ef2de9447db6ea955f95f265d6e67c4fc1f8b65a1abc5bb76ac64bf9394` | ICERIK_DOGRULANDI |
| 2026-02-14 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-14/CSV/GunlukYarisSonuclari/14.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17144 | `0dce51733a6f6cd74e5fefe1d51236ed6f1b913f8af21d58beefe1644a967595` | ICERIK_DOGRULANDI |
| 2026-02-15 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-15/CSV/GunlukYarisProgrami/15.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13698 | `0844c472331486b9773c139623934e44bafade4c24cd30c0e7d3b599142b5e1f` | ICERIK_DOGRULANDI |
| 2026-02-15 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-15/CSV/GunlukYarisSonuclari/15.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14021 | `31dbb90817cdefcf2693ba13386f36c95045fcfe76bbab9d6136896fe2e410d6` | ICERIK_DOGRULANDI |
| 2026-02-15 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-15/CSV/GunlukYarisProgrami/15.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 15440 | `524e1a54276b264cdda1fb3ec0d5b58014840b5f1a8725765098140ba971fe51` | ICERIK_DOGRULANDI |
| 2026-02-15 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-15/CSV/GunlukYarisSonuclari/15.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16117 | `5b48b90ef506887b450f3f6804d87b89a7eefb29e841e824532074a07c46dc19` | ICERIK_DOGRULANDI |
| 2026-02-16 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-16/CSV/GunlukYarisProgrami/16.02.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 12367 | `826a4be797706bc154eb53af45c3f0372e725756561eb26e47ea05c4322601de` | ICERIK_DOGRULANDI |
| 2026-02-16 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-16/CSV/GunlukYarisSonuclari/16.02.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 12760 | `421f69f18f7318072222bdd0434322338cba5083f383376f8a29ffc2b13dc76c` | ICERIK_DOGRULANDI |
| 2026-02-16 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-16/CSV/GunlukYarisProgrami/16.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 14829 | `40936b5bc6b9a41816277ac0811936308eb80c699077a42950e6ee94189a94e8` | ICERIK_DOGRULANDI |
| 2026-02-16 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-16/CSV/GunlukYarisSonuclari/16.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 15291 | `6f05e4026854b837296d5c9f75e40dabca5c56064ba8394c43c1ccb283d322ef` | ICERIK_DOGRULANDI |
| 2026-02-17 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-17/CSV/GunlukYarisProgrami/17.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13344 | `2cd0971493e5c6798cc84091fb268e1bb852d05a9ec707e1ffb36ca2f8d13042` | ICERIK_DOGRULANDI |
| 2026-02-17 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-17/CSV/GunlukYarisSonuclari/17.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 13424 | `3f55ee4d83d34e3b6066a6bc6b69f7f67fa4325ccf82ab910306f3b2bf43825e` | ICERIK_DOGRULANDI |
| 2026-02-17 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-17/CSV/GunlukYarisProgrami/17.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 13617 | `258368b346240feeab57b35e65d2da6e746d5fa96c239d8ec071ccf56576f87a` | ICERIK_DOGRULANDI |
| 2026-02-17 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-17/CSV/GunlukYarisSonuclari/17.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 14042 | `8d9cfee6ada89c15c85be16477c969c25862bf52a0cad8148453d252e3c25faa` | ICERIK_DOGRULANDI |
| 2026-02-18 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-18/CSV/GunlukYarisProgrami/18.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17026 | `e1441a9a97bf59452db52a97fd4779b5bfb3675c5d8418a889d691c082748604` | ICERIK_DOGRULANDI |
| 2026-02-18 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-18/CSV/GunlukYarisSonuclari/18.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17209 | `0b0c7342ce0412c63b49b3bec59b13036ca677bf36552719bc8628b803e36e55` | ICERIK_DOGRULANDI |
| 2026-02-18 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-18/CSV/GunlukYarisProgrami/18.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 17193 | `208d0c14649e9e2387d3b67fa62c3d3136d98964d324312957f16ac08c8e9a23` | ICERIK_DOGRULANDI |
| 2026-02-18 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-18/CSV/GunlukYarisSonuclari/18.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 17607 | `bdcae7efcb0efbe5c3032084707b4c70cf4c0d8f1fbb2464bc2ae4d046fa1eb4` | ICERIK_DOGRULANDI |
| 2026-02-19 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-19/CSV/GunlukYarisProgrami/19.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 10769 | `8d716953686ec2c48d84850240185894da1b8bb42764861b8902ab958ac27554` | ICERIK_DOGRULANDI |
| 2026-02-19 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-19/CSV/GunlukYarisSonuclari/19.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 11162 | `30647eafb00f344771c795c48a9a44428b70b321a0869eb739b7c752eb3f9add` | ICERIK_DOGRULANDI |
| 2026-02-19 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-19/CSV/GunlukYarisProgrami/19.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 15662 | `81558f5c2f42d27be626552b269f66f4d91aef6dabbb468e732da803f96d90f4` | ICERIK_DOGRULANDI |
| 2026-02-19 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-19/CSV/GunlukYarisSonuclari/19.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16499 | `4f8366cd613f34fe5d78367ed02cca1013153e39518f0e55e88bb71ea164b949` | ICERIK_DOGRULANDI |
| 2026-02-20 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-20/CSV/GunlukYarisProgrami/20.02.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 15011 | `0482f28bc20b07af4ffa34db4e83659cfb3088dd4efda1fb6cf05e54e4d338a5` | ICERIK_DOGRULANDI |
| 2026-02-20 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-20/CSV/GunlukYarisSonuclari/20.02.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 15775 | `d30d14ffcccd19f27155ad030bc8d6874b859b32e98ff46868747eeca0218455` | ICERIK_DOGRULANDI |
| 2026-02-20 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-20/CSV/GunlukYarisProgrami/20.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 13641 | `7941a086d2dd030464bf148d05d2b53c2b810e3bf68df49a4550f360692ae982` | ICERIK_DOGRULANDI |
| 2026-02-20 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-20/CSV/GunlukYarisSonuclari/20.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 13765 | `946a14300ab6a8b645191f14b8b44c9e8295213c0a6030249b2874f028a3c3b8` | ICERIK_DOGRULANDI |
| 2026-02-21 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-21/CSV/GunlukYarisProgrami/21.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 11744 | `576665f0183d0b7f58e3561b4247cd7f78a38ff5c7c172577c0cac751e4e8a48` | ICERIK_DOGRULANDI |
| 2026-02-21 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-21/CSV/GunlukYarisSonuclari/21.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 11927 | `f9ab2acdb333daf9e51ddec09fa4b52595661ff0fdcea187ca8a0055b36a54da` | ICERIK_DOGRULANDI |
| 2026-02-21 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-21/CSV/GunlukYarisProgrami/21.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 19159 | `8d32f58adfbb4296209b793dddf10d0d79e118bfb7657525c817713371fba610` | ICERIK_DOGRULANDI |
| 2026-02-21 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-21/CSV/GunlukYarisSonuclari/21.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 19916 | `4fe989ca8d53258511d84806006f3dea6a77dde2cf3178e46f8b53c018139459` | ICERIK_DOGRULANDI |
| 2026-02-22 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-22/CSV/GunlukYarisProgrami/22.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 14072 | `d19e040e0b7959a1eafb02c27719551056b53d5db827d9ce71ce5038998713fb` | ICERIK_DOGRULANDI |
| 2026-02-22 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-22/CSV/GunlukYarisSonuclari/22.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14524 | `62435d7a651c2987b946d17742354eb94a6d6bbb2b30b071d962775d8a1a28a2` | ICERIK_DOGRULANDI |
| 2026-02-22 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-22/CSV/GunlukYarisProgrami/22.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 14130 | `ca0c2d13bee720d58de47ce89c9838a42d92a01b41cecabc89c470be83c0b6ee` | ICERIK_DOGRULANDI |
| 2026-02-22 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-22/CSV/GunlukYarisSonuclari/22.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 14346 | `af530f1be27fea467bd53a9dc8265b73095ff2968c808b28f4abed325779fd0f` | ICERIK_DOGRULANDI |
| 2026-02-23 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-23/CSV/GunlukYarisProgrami/23.02.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 15060 | `a9423d2241668616ade890bc947773d1bb72c4534265c8c30f9dab2a355d8212` | ICERIK_DOGRULANDI |
| 2026-02-23 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-23/CSV/GunlukYarisSonuclari/23.02.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 15513 | `a79a9b8f5dbceff64c66c3ce8df443703118a1dd926d4f8e61da8bd2cad7c9e4` | ICERIK_DOGRULANDI |
| 2026-02-23 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-23/CSV/GunlukYarisProgrami/23.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 20048 | `00150ce0b01c695cd24b955cae3fee87e91dcfd668bf086650dd12607db3dc1a` | ICERIK_DOGRULANDI |
| 2026-02-23 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-23/CSV/GunlukYarisSonuclari/23.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 20616 | `1ce381e2d2a65d9f83153bfdbf1f8dfd4575b821a5674060c137f82840e9d949` | ICERIK_DOGRULANDI |
| 2026-02-24 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-24/CSV/GunlukYarisProgrami/24.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 16317 | `3ef053fb5d96019712db5b0667e49a84da5aa9537184acb804fae9b5f7d1a798` | ICERIK_DOGRULANDI |
| 2026-02-24 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-24/CSV/GunlukYarisSonuclari/24.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 17095 | `8dcf78b27782caa120d73ff306af70f28367a51a4276d02cb8451c82ffa3a1cb` | ICERIK_DOGRULANDI |
| 2026-02-24 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-24/CSV/GunlukYarisProgrami/24.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 15082 | `34502aefc60f964bc3833ec4559ea41b57bbcb323178c1484c89e625a2754a34` | ICERIK_DOGRULANDI |
| 2026-02-24 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-24/CSV/GunlukYarisSonuclari/24.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15614 | `26f143b79bd8f67d3793544a3327c6b0c49efabb7ccbaa6c3e39866c8643626d` | ICERIK_DOGRULANDI |
| 2026-02-25 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-25/CSV/GunlukYarisProgrami/25.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 15239 | `98ea90fe30165a6e772f8b8103cbe0094240b666dfe0fecff8165a4a2552d235` | ICERIK_DOGRULANDI |
| 2026-02-25 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-25/CSV/GunlukYarisSonuclari/25.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 15705 | `81da9854888a1430738179ffe86eb299c16f411a86318b578faf45dc5a87f5da` | ICERIK_DOGRULANDI |
| 2026-02-25 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-25/CSV/GunlukYarisProgrami/25.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 18706 | `4170f348446bdf90e2a27974c491ee706a75631af36b7f9993ead107a43b1f2a` | ICERIK_DOGRULANDI |
| 2026-02-25 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-25/CSV/GunlukYarisSonuclari/25.02.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 18995 | `baeb2d904020c55667e6166b104f6cd5d24e98143a04665fd8743c6252c74737` | ICERIK_DOGRULANDI |
| 2026-02-26 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-26/CSV/GunlukYarisProgrami/26.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 15363 | `98bc54579bc3c640c22c4b5030066298efcff536441f3b2dc0c7e191e3e60338` | ICERIK_DOGRULANDI |
| 2026-02-26 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-26/CSV/GunlukYarisSonuclari/26.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15725 | `aea12b0b6ff88e705e6553f662b6b55e521c92030228f87e6824e08f29fe0a18` | ICERIK_DOGRULANDI |
| 2026-02-26 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-26/CSV/GunlukYarisProgrami/26.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 13964 | `0c40481b808c177b3da4cf325e3adc0f8ef0cff9cbbc43b7db66c5db96cd5355` | ICERIK_DOGRULANDI |
| 2026-02-26 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-26/CSV/GunlukYarisSonuclari/26.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 14236 | `c7acdce0ea1f29a0d2bfd25fd4b91bbb8a440cbbe705e6f8d09dacfd29f07c44` | ICERIK_DOGRULANDI |
| 2026-02-27 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-27/CSV/GunlukYarisProgrami/27.02.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 16484 | `17efbb8e70f6b76ae4f98294c3318814ecb7aed4682d0870e35b708bd22c0fa2` | ICERIK_DOGRULANDI |
| 2026-02-27 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-27/CSV/GunlukYarisSonuclari/27.02.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 16843 | `20d0ce706679f2b9d50161a44eaf5d4c176a5c08161693313a875b0622ae3fac` | ICERIK_DOGRULANDI |
| 2026-02-27 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-27/CSV/GunlukYarisProgrami/27.02.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 10418 | `6d66947ba559c0ff8ed1cc2577d1f0f0d9f44eb47b3a37eaea85bb5d50c796a8` | ICERIK_DOGRULANDI |
| 2026-02-27 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-27/CSV/GunlukYarisSonuclari/27.02.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 10522 | `290269b8f81e7f533f4f824cb43edfa34a51197c8f463718d4e0f850d236a456` | ICERIK_DOGRULANDI |
| 2026-02-28 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-28/CSV/GunlukYarisProgrami/28.02.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15251 | `d0e5963b8c92f3d24906e232e38d49eb8433ef08c3997b83d353cef23243acad` | ICERIK_DOGRULANDI |
| 2026-02-28 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-28/CSV/GunlukYarisSonuclari/28.02.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15647 | `00f3930985787e30b4fbb7e0318fd517f3d32dec28144ad04ba9a0219a980f78` | ICERIK_DOGRULANDI |
| 2026-02-28 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-28/CSV/GunlukYarisProgrami/28.02.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 14696 | `b869eb56272c58ef38392d7de7b8cb0d2fba9e0ef8765f2cd47c5f61f0d8131b` | ICERIK_DOGRULANDI |
| 2026-02-28 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-02-28/CSV/GunlukYarisSonuclari/28.02.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 15257 | `7fcd23ae0893e58df4c9908f53cc5b5194d32964c62775fdf20de4dffe5ed64e` | ICERIK_DOGRULANDI |

Doğrulanan ham CSV'ler `/home/user/TJK/kontrol/ham/2026-02` altında (`yerel_dosya` adlarıyla). Ayrıntılı kayıt: `/home/user/TJK/kontrol/yillik/listeler/2026-02_durum.tsv`.
