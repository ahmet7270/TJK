# Son veri testi

- **Durum:** TAMAMLANDI (liste sonuna kadar işlendi)
- **Çalışma:** Mart 2026 toplu indirme, `/home/user/TJK/kontrol/yillik/listeler/2026-06_liste.tsv`
- **Araçlar:** `tjk-fetch toplu` (curl ile HTTPS; içerik doğrulaması `tjk_ayristirici/src/ayristir.rs` ile), birlikte ayrıştırma `tjk_ayristirici`
- **SQLite:** kullanılmadı
- **Ön kontrol:** 64 çift, 128 satır, 128 benzersiz URL; her çiftte bir program ve bir sonuç
- **robots.txt:** HTTP 404: robots.txt yok, kural tanımlı değil (RFC 9309: izinli)

## Özet (bu çalışmadaki 64 çift)

| Durum | Çift |
|---|---|
| Tamamlandı | 63 |
| İnceleme gerekli | 1 |
| Eksik | 0 |
| Henüz denenmedi | 0 |
| **Toplam** | **64** |

Önceki çalışmadan SHA-256'sı eşleşerek yeniden kullanılan dosya: 0

## Çiftler

| # | Tarih | Hipodrom | Durum | Gerekçe |
|---|---|---|---|---|
| 1 | 2026-06-01 | Bursa | TAMAMLANDI |  |
| 2 | 2026-06-01 | Şanlıurfa | TAMAMLANDI |  |
| 3 | 2026-06-02 | Ankara | TAMAMLANDI |  |
| 4 | 2026-06-02 | Kocaeli | TAMAMLANDI |  |
| 5 | 2026-06-03 | Elazığ | TAMAMLANDI |  |
| 6 | 2026-06-03 | İstanbul | TAMAMLANDI |  |
| 7 | 2026-06-04 | Ankara | TAMAMLANDI |  |
| 8 | 2026-06-04 | Kocaeli | TAMAMLANDI |  |
| 9 | 2026-06-05 | Bursa | TAMAMLANDI |  |
| 10 | 2026-06-05 | İstanbul | TAMAMLANDI |  |
| 11 | 2026-06-06 | Ankara | TAMAMLANDI |  |
| 12 | 2026-06-06 | Diyarbakır | TAMAMLANDI |  |
| 13 | 2026-06-06 | İzmir | TAMAMLANDI |  |
| 14 | 2026-06-07 | İstanbul | TAMAMLANDI |  |
| 15 | 2026-06-07 | İzmir | TAMAMLANDI |  |
| 16 | 2026-06-08 | Bursa | TAMAMLANDI |  |
| 17 | 2026-06-08 | Şanlıurfa | TAMAMLANDI |  |
| 18 | 2026-06-09 | Ankara | TAMAMLANDI |  |
| 19 | 2026-06-09 | Kocaeli | TAMAMLANDI |  |
| 20 | 2026-06-10 | Elazığ | TAMAMLANDI |  |
| 21 | 2026-06-10 | İstanbul | TAMAMLANDI |  |
| 22 | 2026-06-11 | Ankara | TAMAMLANDI |  |
| 23 | 2026-06-11 | Kocaeli | TAMAMLANDI |  |
| 24 | 2026-06-12 | Bursa | TAMAMLANDI |  |
| 25 | 2026-06-12 | İstanbul | TAMAMLANDI |  |
| 26 | 2026-06-13 | Ankara | TAMAMLANDI |  |
| 27 | 2026-06-13 | Diyarbakır | TAMAMLANDI |  |
| 28 | 2026-06-13 | İzmir | TAMAMLANDI |  |
| 29 | 2026-06-14 | İstanbul | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: EKURI_UYESI_DOGRULANAMADI ×4 (ilk: EKURI_UYESI_DOGRULANAMADI: 20260614-ISTANBUL-program.csv satır 105 (6. koşu): (8)ZİNCİR KOPARAN — programda 8 numara: ZİNCİR KOPARAN KG K BB) |
| 30 | 2026-06-14 | İzmir | TAMAMLANDI |  |
| 31 | 2026-06-15 | Bursa | TAMAMLANDI |  |
| 32 | 2026-06-15 | Şanlıurfa | TAMAMLANDI |  |
| 33 | 2026-06-16 | Ankara | TAMAMLANDI |  |
| 34 | 2026-06-16 | Kocaeli | TAMAMLANDI |  |
| 35 | 2026-06-17 | Elazığ | TAMAMLANDI |  |
| 36 | 2026-06-17 | İstanbul | TAMAMLANDI |  |
| 37 | 2026-06-18 | Ankara | TAMAMLANDI |  |
| 38 | 2026-06-18 | Kocaeli | TAMAMLANDI |  |
| 39 | 2026-06-19 | Bursa | TAMAMLANDI |  |
| 40 | 2026-06-19 | İstanbul | TAMAMLANDI |  |
| 41 | 2026-06-20 | Ankara | TAMAMLANDI |  |
| 42 | 2026-06-20 | Diyarbakır | TAMAMLANDI |  |
| 43 | 2026-06-20 | İzmir | TAMAMLANDI |  |
| 44 | 2026-06-21 | İstanbul | TAMAMLANDI |  |
| 45 | 2026-06-21 | İzmir | TAMAMLANDI |  |
| 46 | 2026-06-22 | Bursa | TAMAMLANDI |  |
| 47 | 2026-06-22 | Elazığ | TAMAMLANDI |  |
| 48 | 2026-06-23 | Ankara | TAMAMLANDI |  |
| 49 | 2026-06-23 | Kocaeli | TAMAMLANDI |  |
| 50 | 2026-06-24 | Elazığ | TAMAMLANDI |  |
| 51 | 2026-06-24 | İstanbul | TAMAMLANDI |  |
| 52 | 2026-06-25 | Bursa | TAMAMLANDI |  |
| 53 | 2026-06-25 | İstanbul | TAMAMLANDI |  |
| 54 | 2026-06-26 | Ankara | TAMAMLANDI |  |
| 55 | 2026-06-26 | Kocaeli | TAMAMLANDI |  |
| 56 | 2026-06-27 | Ankara | TAMAMLANDI |  |
| 57 | 2026-06-27 | Diyarbakır | TAMAMLANDI |  |
| 58 | 2026-06-27 | İzmir | TAMAMLANDI |  |
| 59 | 2026-06-28 | İstanbul | TAMAMLANDI |  |
| 60 | 2026-06-28 | İzmir | TAMAMLANDI |  |
| 61 | 2026-06-29 | Bursa | TAMAMLANDI |  |
| 62 | 2026-06-29 | Elazığ | TAMAMLANDI |  |
| 63 | 2026-06-30 | Ankara | TAMAMLANDI |  |
| 64 | 2026-06-30 | Kocaeli | TAMAMLANDI |  |

## Dosyalar

| Tarih | Hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | Doğrulama |
|---|---|---|---|---|---|---|---|
| 2026-06-01 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-01/CSV/GunlukYarisProgrami/01.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 19567 | `cc17b13278a30c1862fea41a890b1859cb28b442d7ac48a5be01f0cfb3673af7` | ICERIK_DOGRULANDI |
| 2026-06-01 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-01/CSV/GunlukYarisSonuclari/01.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 20174 | `c0c8460c8e154efc827aa0a1d95edb68c9e3bc4dc3fa78f84b2e85180071cb1f` | ICERIK_DOGRULANDI |
| 2026-06-01 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-01/CSV/GunlukYarisProgrami/01.06.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 17694 | `75f079049058d94610e33b040c2a448f164394420c4401408593548aa4c32960` | ICERIK_DOGRULANDI |
| 2026-06-01 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-01/CSV/GunlukYarisSonuclari/01.06.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 18658 | `88e141036cff838b24fa5a6f4b9755cd40b3a904dabfe38b664d27c3442ad866` | ICERIK_DOGRULANDI |
| 2026-06-02 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-02/CSV/GunlukYarisProgrami/02.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 13013 | `89b2db9281724c5dc72eaaaa09f907ba308deb269bb9b2222a0e86a219d0ca78` | ICERIK_DOGRULANDI |
| 2026-06-02 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-02/CSV/GunlukYarisSonuclari/02.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 13796 | `2e795b446cbdbfb344eab575dc007dd81e4eb4608f361c86e9db9f3088b27c49` | ICERIK_DOGRULANDI |
| 2026-06-02 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-02/CSV/GunlukYarisProgrami/02.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 17567 | `8fbb62a24f200aabebea0e7f1e8b5836b5f2c4769a1c87e3f3e6a129ad2f8005` | ICERIK_DOGRULANDI |
| 2026-06-02 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-02/CSV/GunlukYarisSonuclari/02.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 18237 | `bdcab70065448d8ec5dbd3297b3e638ebb647b02e85e73fec2c8d02cc19ace04` | ICERIK_DOGRULANDI |
| 2026-06-03 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-03/CSV/GunlukYarisProgrami/03.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 14393 | `1f7dfccde414eb8db7251765bb294eacbfa043f69c41b13aa55ffa645514ee86` | ICERIK_DOGRULANDI |
| 2026-06-03 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-03/CSV/GunlukYarisSonuclari/03.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 15077 | `5d62c8847d6828ea2b7018edc66cf3bb3cfb26ae0f6059e805a4892c7db2810b` | ICERIK_DOGRULANDI |
| 2026-06-03 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-03/CSV/GunlukYarisProgrami/03.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 14186 | `4bbaf0fa16e48535f20d19d059f366a6af809a1ab5ebff22314579ec7cc26c2b` | ICERIK_DOGRULANDI |
| 2026-06-03 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-03/CSV/GunlukYarisSonuclari/03.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 14880 | `6ad73447b3b9217c81dca5ac6a532e160d115957abc2f7ba63d605ca3fb9e307` | ICERIK_DOGRULANDI |
| 2026-06-04 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-04/CSV/GunlukYarisProgrami/04.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 16521 | `8589d40df5f4b128910989bd0c809fb2f960988a7fb1fa56626cca7a0df581ac` | ICERIK_DOGRULANDI |
| 2026-06-04 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-04/CSV/GunlukYarisSonuclari/04.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 17246 | `3d0cd342d9153ee7dafbe11cb2e93faff8dda328031bbdb029e26c593d685936` | ICERIK_DOGRULANDI |
| 2026-06-04 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-04/CSV/GunlukYarisProgrami/04.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 19151 | `1cad7c10282ddc7b1405be1b6bb286553cf8ac986b68c306405800f7c0452ea4` | ICERIK_DOGRULANDI |
| 2026-06-04 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-04/CSV/GunlukYarisSonuclari/04.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 19818 | `b28db13aa006601c652a2227e1e35f8b061e5be64921a72b64c348670e77216b` | ICERIK_DOGRULANDI |
| 2026-06-05 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-05/CSV/GunlukYarisProgrami/05.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 24131 | `fbfae91b938fc576eceb881d6e7b02afc5f8d95d5e0895eddf7f5a02d775d135` | ICERIK_DOGRULANDI |
| 2026-06-05 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-05/CSV/GunlukYarisSonuclari/05.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 25008 | `4096265851aca12a0b4b24038d001c7427215c65aeeb5afaf07ba1d5a3399358` | ICERIK_DOGRULANDI |
| 2026-06-05 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-05/CSV/GunlukYarisProgrami/05.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 15357 | `bf17a07745f90e3a9f7e5efb6d9fee93b8af1db286fa6ec2ded638e22e592c74` | ICERIK_DOGRULANDI |
| 2026-06-05 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-05/CSV/GunlukYarisSonuclari/05.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 15930 | `e12568bdd6e27b2af4573ad62a183afa93331cbf628a7e579448ab6d9ef19233` | ICERIK_DOGRULANDI |
| 2026-06-06 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-06/CSV/GunlukYarisProgrami/06.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 16845 | `4e0014579b384b6ff1e97bf3282f17013eec8ed1914fa838f0d5e0e934af065e` | ICERIK_DOGRULANDI |
| 2026-06-06 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-06/CSV/GunlukYarisSonuclari/06.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 17616 | `1643c158a6485859bd51c9bfc96de6fc3724e570bc85451cbe491794a413eb16` | ICERIK_DOGRULANDI |
| 2026-06-06 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-06/CSV/GunlukYarisProgrami/06.06.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 13645 | `48720075a36a7f99fed47e9ec14ceba8e9e7b929547c0d481e1a23d5778d3dba` | ICERIK_DOGRULANDI |
| 2026-06-06 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-06/CSV/GunlukYarisSonuclari/06.06.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 14121 | `eaaf68c43552b2b7b4470591b4129a7187a40beb1fd48f06f3c303962f24ecbe` | ICERIK_DOGRULANDI |
| 2026-06-06 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-06/CSV/GunlukYarisProgrami/06.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 14782 | `55c987cb4ff5dab1199066b0ed596dc89ac57de502fb4092f0b2c823e89d19eb` | ICERIK_DOGRULANDI |
| 2026-06-06 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-06/CSV/GunlukYarisSonuclari/06.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 15456 | `dc13d476d4e5e46259ff0b8c0c291bfdb71c28fda1ea7ae3517de73a2bafd394` | ICERIK_DOGRULANDI |
| 2026-06-07 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-07/CSV/GunlukYarisProgrami/07.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 20694 | `785a71749a8ca661a94a65c6bb7e4ff4f189b5c7b147d3ae0750e4b20f998577` | ICERIK_DOGRULANDI |
| 2026-06-07 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-07/CSV/GunlukYarisSonuclari/07.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 21259 | `6a8f9ecc9282e56c578b8127b0eaecba71dc489eb48381251daeb62175b3997a` | ICERIK_DOGRULANDI |
| 2026-06-07 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-07/CSV/GunlukYarisProgrami/07.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 17307 | `a806a3a8d5cc6fa9d53358eb6bd32814f05e459d81220eb51e615d7f20dc9256` | ICERIK_DOGRULANDI |
| 2026-06-07 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-07/CSV/GunlukYarisSonuclari/07.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 17888 | `c74439e431f1f7ef49b5e769906dcfe63f2243a32ceee45a553d705c38e96322` | ICERIK_DOGRULANDI |
| 2026-06-08 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-08/CSV/GunlukYarisProgrami/08.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 19134 | `9b2fc48e756dc43c9d721bd57b66e42de7a753f6c053049c0abb88e83fa9899b` | ICERIK_DOGRULANDI |
| 2026-06-08 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-08/CSV/GunlukYarisSonuclari/08.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 19887 | `1b38e15e5676f0ee89fb16c1c25c99a59dafc6fa20c9cb6f62a98e13adc02850` | ICERIK_DOGRULANDI |
| 2026-06-08 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-08/CSV/GunlukYarisProgrami/08.06.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 19624 | `1634f3b5e08238e83f40380ebf01c0038f9855864cb954787e208b60373e6835` | ICERIK_DOGRULANDI |
| 2026-06-08 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-08/CSV/GunlukYarisSonuclari/08.06.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 20290 | `1b7571798af15fd2c003685cf5c7f4e658fbbd9c1d151ff78edfd85a67381e2c` | ICERIK_DOGRULANDI |
| 2026-06-09 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-09/CSV/GunlukYarisProgrami/09.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 16150 | `bbaa62f631da0431c7d85df116f59c6ba4013851b3f67fd5bbd9cb0faf60db53` | ICERIK_DOGRULANDI |
| 2026-06-09 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-09/CSV/GunlukYarisSonuclari/09.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 16658 | `ac2cc36468af54a52dd9e783a8a377be3ab265cc19668c85ee33ebe51729a7e7` | ICERIK_DOGRULANDI |
| 2026-06-09 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-09/CSV/GunlukYarisProgrami/09.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 14551 | `d5f65da409adf67a5f2ee45860a0fe7064c57457cfeea0a90aeebc5b68ca55f0` | ICERIK_DOGRULANDI |
| 2026-06-09 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-09/CSV/GunlukYarisSonuclari/09.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 15201 | `663bf97fe2237faf78add25ffdd2edc65dcb33f3961cab7286441dee821cee32` | ICERIK_DOGRULANDI |
| 2026-06-10 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-10/CSV/GunlukYarisProgrami/10.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 18061 | `6b4a1ef473a851f3270199e334d359d571a647ca9ae6b7bcf0b923c96332565b` | ICERIK_DOGRULANDI |
| 2026-06-10 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-10/CSV/GunlukYarisSonuclari/10.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 18752 | `15b2be352beb5227f6ddf56ecc4fce9713e34604aa7f56216fe673b007848a2d` | ICERIK_DOGRULANDI |
| 2026-06-10 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-10/CSV/GunlukYarisProgrami/10.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16688 | `04b2fd3b093af0031f4428c0ae7e888d6cf71665787b551f32ec99325799da2a` | ICERIK_DOGRULANDI |
| 2026-06-10 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-10/CSV/GunlukYarisSonuclari/10.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17176 | `0d72c41cdbefbeb23bc3d2949713a1b4b3c2695aa8d4954ce5dd9dbb0487ff59` | ICERIK_DOGRULANDI |
| 2026-06-11 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-11/CSV/GunlukYarisProgrami/11.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 17278 | `74f255a6a5d843ca79cb8d63686ab4ef1ce3e7420653492c10febb31fa2f26eb` | ICERIK_DOGRULANDI |
| 2026-06-11 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-11/CSV/GunlukYarisSonuclari/11.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 18027 | `a984aeaccfa53c586ed974573d9971f71f86a641fe17fbcdad89cef313c2b29d` | ICERIK_DOGRULANDI |
| 2026-06-11 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-11/CSV/GunlukYarisProgrami/11.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 18593 | `04d1fe30ed9402b367e93a418b0fed923c0ab70d02edf5122f84aa3307517528` | ICERIK_DOGRULANDI |
| 2026-06-11 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-11/CSV/GunlukYarisSonuclari/11.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 19391 | `53c46ef498b799125181b42f1c7d5f3c0c91fa350edb78f4ce658f419c382a6e` | ICERIK_DOGRULANDI |
| 2026-06-12 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-12/CSV/GunlukYarisProgrami/12.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 17796 | `c32617e238302ffcfac646ba37da283db747f6ce64b712e88ae9d4077b95e244` | ICERIK_DOGRULANDI |
| 2026-06-12 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-12/CSV/GunlukYarisSonuclari/12.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 18236 | `22a1b8622d2ddcd88ffc15b03ed85162c64fbf157309ddef9b49e8cf091d1197` | ICERIK_DOGRULANDI |
| 2026-06-12 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-12/CSV/GunlukYarisProgrami/12.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 14139 | `b0a1a6e30899c15848a92634e2714c43aaf020688f147f1fde2211b68a7bf34e` | ICERIK_DOGRULANDI |
| 2026-06-12 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-12/CSV/GunlukYarisSonuclari/12.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 14845 | `d85c19c7c32a2a34c1af41741c546e6e8f3e63d18313845602e5a79a85f55bc2` | ICERIK_DOGRULANDI |
| 2026-06-13 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-13/CSV/GunlukYarisProgrami/13.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 17332 | `63f994b0d8b7a424ff15e93635f3aef8b2949ea0476e3d3b9835f5d4557bbc64` | ICERIK_DOGRULANDI |
| 2026-06-13 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-13/CSV/GunlukYarisSonuclari/13.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 17649 | `326adec6f32662b755c697920142621e143ee9d6c2209f7fc06f2d29968e2620` | ICERIK_DOGRULANDI |
| 2026-06-13 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-13/CSV/GunlukYarisProgrami/13.06.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 12680 | `ef04fa06f3b73b25b11aa55e5eed08b57a1733d5f7bad41fc4a699418de88300` | ICERIK_DOGRULANDI |
| 2026-06-13 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-13/CSV/GunlukYarisSonuclari/13.06.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 12893 | `551bbddb62bbfdcb4d9c795c8a9449110b85df9bee5d7ed84bc08961b08a7f7f` | ICERIK_DOGRULANDI |
| 2026-06-13 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-13/CSV/GunlukYarisProgrami/13.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 15463 | `0117af7433f81cdc1454eef767d5b6e5f064ebffe6ef0a580f10cc67fd3a8258` | ICERIK_DOGRULANDI |
| 2026-06-13 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-13/CSV/GunlukYarisSonuclari/13.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16147 | `4fb37360e71f1c3634b3f2d8d59014186b4a27f2979ba2b8a86eca011adf4dc7` | ICERIK_DOGRULANDI |
| 2026-06-14 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-14/CSV/GunlukYarisProgrami/14.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 18769 | `6d4b272d0331bc264922fa40f1d2492fb10833beafe93e69a01b4fe0fbc517dc` | ICERIK_DOGRULANDI |
| 2026-06-14 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-14/CSV/GunlukYarisSonuclari/14.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 19661 | `a1a74ae28f9f6174f347e56745cc732deea18aebdbeca365fc964cd0fd58f8bc` | ICERIK_DOGRULANDI |
| 2026-06-14 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-14/CSV/GunlukYarisProgrami/14.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16726 | `98caf3d838ae89ef7177f3fb48d6b0398448bb46349f45049f154728f306d350` | ICERIK_DOGRULANDI |
| 2026-06-14 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-14/CSV/GunlukYarisSonuclari/14.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 17290 | `ad0f76a4f081af063bd020b3249cc681fc27e6a334825369b4679306ebc50281` | ICERIK_DOGRULANDI |
| 2026-06-15 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-15/CSV/GunlukYarisProgrami/15.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 19318 | `50d32bca42d644bd19cc97cccade0f6e7cff59dec2a55b2f38bfb2430aff62e6` | ICERIK_DOGRULANDI |
| 2026-06-15 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-15/CSV/GunlukYarisSonuclari/15.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 20767 | `c0a6c6a384ae86c689ec41f1ac17175a76871943127710053304df9ff644f030` | ICERIK_DOGRULANDI |
| 2026-06-15 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-15/CSV/GunlukYarisProgrami/15.06.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 21217 | `18c5b629d5aa6207b6d8f3f6b9c13e8a99a778c828c81b48f80ea6f18500866f` | ICERIK_DOGRULANDI |
| 2026-06-15 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-15/CSV/GunlukYarisSonuclari/15.06.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 22139 | `4bdbd62fe554c8ba83950afccde2fb1de92d9bda183dc09199af33502ce6ac30` | ICERIK_DOGRULANDI |
| 2026-06-16 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-16/CSV/GunlukYarisProgrami/16.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 16195 | `33873225793f3ab5d6cf4ede639ac39f8252482dce829f0e640dcaeb40685e6b` | ICERIK_DOGRULANDI |
| 2026-06-16 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-16/CSV/GunlukYarisSonuclari/16.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 16848 | `3384b059a64385b67725c46ab4143c5c613f26a8dcf163d6d0e619e4ac3ca73a` | ICERIK_DOGRULANDI |
| 2026-06-16 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-16/CSV/GunlukYarisProgrami/16.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 12748 | `575caa258e78b5600076be7e4bc01654acd47e15c707c378aa91bcb3dedd2fe3` | ICERIK_DOGRULANDI |
| 2026-06-16 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-16/CSV/GunlukYarisSonuclari/16.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 13373 | `1f5b8fb5706c712914418326c0b39c1bf53807afb389ead9f67b09ca47c6a514` | ICERIK_DOGRULANDI |
| 2026-06-17 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-17/CSV/GunlukYarisProgrami/17.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 19031 | `a48ef5a70cd128db5660816e58db8cba038d78c8c1cea8fca873cffd24dd7005` | ICERIK_DOGRULANDI |
| 2026-06-17 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-17/CSV/GunlukYarisSonuclari/17.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 19969 | `e0a7d0e7cbd57d150c070c39ec145dfd7f4982ce478638a0ef118caf92a0b15a` | ICERIK_DOGRULANDI |
| 2026-06-17 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-17/CSV/GunlukYarisProgrami/17.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 13537 | `59e67ec56600620d04344ee8d499dbe077fe84732eeda9d336a0d42b53481107` | ICERIK_DOGRULANDI |
| 2026-06-17 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-17/CSV/GunlukYarisSonuclari/17.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 13952 | `1b8b1d5bea8672bdc98c43549796fe06bb2c46f1f0acf86e211beaf706fef9d9` | ICERIK_DOGRULANDI |
| 2026-06-18 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-18/CSV/GunlukYarisProgrami/18.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 13599 | `53484530ecd0017cbeb5dd257eee88be1896a0f5928eed28601d542ace9d70a3` | ICERIK_DOGRULANDI |
| 2026-06-18 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-18/CSV/GunlukYarisSonuclari/18.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 13893 | `59c77750fb16375797057198b8221c4c83959795cafef4854d756dd79b73bf3d` | ICERIK_DOGRULANDI |
| 2026-06-18 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-18/CSV/GunlukYarisProgrami/18.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 16812 | `a533664f4eb9929806694b294b5071ba4510576ae5071c1224c8de9b2c94076c` | ICERIK_DOGRULANDI |
| 2026-06-18 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-18/CSV/GunlukYarisSonuclari/18.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 17342 | `04b19096152e430c6844c3f971126d060edcccc33c57180f085dec4453cd2631` | ICERIK_DOGRULANDI |
| 2026-06-19 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-19/CSV/GunlukYarisProgrami/19.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 18617 | `567f38aa27a78a7ac13343c26fbe5a301165c4bb397e0e928fb1cf215da7d48e` | ICERIK_DOGRULANDI |
| 2026-06-19 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-19/CSV/GunlukYarisSonuclari/19.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 19173 | `797137c64b1171486a0630e0b4a3876d8936e8af236b28714e521415e19a93ac` | ICERIK_DOGRULANDI |
| 2026-06-19 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-19/CSV/GunlukYarisProgrami/19.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17054 | `576574996fa40e0311c5ee7f72f5b9089b7963bbe72cbee6a6989cad531f4633` | ICERIK_DOGRULANDI |
| 2026-06-19 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-19/CSV/GunlukYarisSonuclari/19.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17727 | `e3428091a0ad72f20f061c206aa050d43c910dbb1d994a67389fa4f892ea1ba8` | ICERIK_DOGRULANDI |
| 2026-06-20 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-20/CSV/GunlukYarisProgrami/20.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 18191 | `8da25baf3bbf93135ee8af8ed061be976e24038228b9c93a1ba5cd8fa3759641` | ICERIK_DOGRULANDI |
| 2026-06-20 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-20/CSV/GunlukYarisSonuclari/20.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 19321 | `1e18b2146924036eb5ed61a8beabb4af3233a2fa87ff5e896da45fb3a09d2abf` | ICERIK_DOGRULANDI |
| 2026-06-20 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-20/CSV/GunlukYarisProgrami/20.06.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 13207 | `cd36abc14ba9af9d47b4927081372edcdc7d88e7d87a938c94aec0f67b83e690` | ICERIK_DOGRULANDI |
| 2026-06-20 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-20/CSV/GunlukYarisSonuclari/20.06.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 13779 | `52d368a5f4cdebaea4fc9661aa83f76ecdf728f7c66aed965f9ca52af55ddc7e` | ICERIK_DOGRULANDI |
| 2026-06-20 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-20/CSV/GunlukYarisProgrami/20.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16305 | `e4129f4b34c5db5b4b427ac9d6324fc12d79632257a6e3bf2d44c2e509510f0d` | ICERIK_DOGRULANDI |
| 2026-06-20 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-20/CSV/GunlukYarisSonuclari/20.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16764 | `316892531048262ad704cddb75ab51bd3f5aa8af54b180c9b4046a2e276304eb` | ICERIK_DOGRULANDI |
| 2026-06-21 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-21/CSV/GunlukYarisProgrami/21.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 18701 | `71912850fef915fccc394fcd031b470a8ae7a2f213e540aef487595c73ea8c93` | ICERIK_DOGRULANDI |
| 2026-06-21 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-21/CSV/GunlukYarisSonuclari/21.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 19811 | `978f1616f459ddd1098e245eae51a61ce5aed9444dba64a34619f08748db53dc` | ICERIK_DOGRULANDI |
| 2026-06-21 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-21/CSV/GunlukYarisProgrami/21.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 15900 | `0dba778ec12f29457cb0849a12218a3f4c65d1b666f09a636cb305ee94c33fad` | ICERIK_DOGRULANDI |
| 2026-06-21 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-21/CSV/GunlukYarisSonuclari/21.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16487 | `36204c38c5aef76cc919b27fb49676acc910e9ac89dbe2f754d6518b00f84d56` | ICERIK_DOGRULANDI |
| 2026-06-22 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-22/CSV/GunlukYarisProgrami/22.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 26109 | `00ee1aec5343b6d150a5ba965fe144dbf3247318e461fe83cc3767d019dc1810` | ICERIK_DOGRULANDI |
| 2026-06-22 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-22/CSV/GunlukYarisSonuclari/22.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 26887 | `4cfa124e92f05a12298a4cf2012c34b4ebfff2784cb49d9a94b8e407f9350da3` | ICERIK_DOGRULANDI |
| 2026-06-22 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-22/CSV/GunlukYarisProgrami/22.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 17692 | `363111917a762fab58cb0373f07ca1ae95bd86b8e7009eb81b7e0217fc53516b` | ICERIK_DOGRULANDI |
| 2026-06-22 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-22/CSV/GunlukYarisSonuclari/22.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 18661 | `a577473b7558544bd30e00bfe82e6257070844066893c1953f23cbf71e59343b` | ICERIK_DOGRULANDI |
| 2026-06-23 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-23/CSV/GunlukYarisProgrami/23.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 12399 | `849fef977c26a182c5d89c0c57e38b115892afde5af3cb5edbe0202c35e62f75` | ICERIK_DOGRULANDI |
| 2026-06-23 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-23/CSV/GunlukYarisSonuclari/23.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 12620 | `b753dd7a964c62c652c6b3ebb0362ca683589475027c5aa637cd67234ad78ac5` | ICERIK_DOGRULANDI |
| 2026-06-23 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-23/CSV/GunlukYarisProgrami/23.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 12255 | `210a98bef9cfa81dfad04ac11dfbf4bdf920386499ebb645d0e8557e12e92937` | ICERIK_DOGRULANDI |
| 2026-06-23 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-23/CSV/GunlukYarisSonuclari/23.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 12808 | `aa5d67d68d217591e32e9421c26aacea6cd41b90b3d89f0bc6deb0ef7937dffd` | ICERIK_DOGRULANDI |
| 2026-06-24 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-24/CSV/GunlukYarisProgrami/24.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 18471 | `3233d41ac09f14778fbd30d545aee13cf54664e0306066be430ea7e866ea27fd` | ICERIK_DOGRULANDI |
| 2026-06-24 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-24/CSV/GunlukYarisSonuclari/24.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 19280 | `43a79e1fac612a6cc7425c70ee24451863f6bfa14aca742fe11a4f4dff9d2039` | ICERIK_DOGRULANDI |
| 2026-06-24 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-24/CSV/GunlukYarisProgrami/24.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17156 | `8b9d812722d0b94e2d4e683c6c0320e0d3e54745548c46a9a38150d37a7394d8` | ICERIK_DOGRULANDI |
| 2026-06-24 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-24/CSV/GunlukYarisSonuclari/24.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17452 | `929b301f5e374f900d6553020bc9102fb8d5a3f157a2595fcff365e3bdad1b51` | ICERIK_DOGRULANDI |
| 2026-06-25 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-25/CSV/GunlukYarisProgrami/25.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 18890 | `2a846fa4e8fff599e91bfacd2e83e7cf9f2822135103a4a1f361fb879699d584` | ICERIK_DOGRULANDI |
| 2026-06-25 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-25/CSV/GunlukYarisSonuclari/25.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 19748 | `3fd08124c4082af3bd4d2a571d97f1eeca290f3b395fd35c9b2f1688220be718` | ICERIK_DOGRULANDI |
| 2026-06-25 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-25/CSV/GunlukYarisProgrami/25.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17947 | `cf83b9c0c39e15e7577be8228e5185ec4b771021409bd8a3979b3048beafe133` | ICERIK_DOGRULANDI |
| 2026-06-25 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-25/CSV/GunlukYarisSonuclari/25.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 18508 | `98d64b087617a8d2967578e314e3ce456b8707f4ec009c64b11474413272825d` | ICERIK_DOGRULANDI |
| 2026-06-26 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-26/CSV/GunlukYarisProgrami/26.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 15478 | `699ef3f0ce116be1ce09e8904e20ec8bb7208a9563f299b85012a37bca37cf90` | ICERIK_DOGRULANDI |
| 2026-06-26 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-26/CSV/GunlukYarisSonuclari/26.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 15864 | `e164eb306ab381da944611e8158593f843afeb98aead53d693d6afc10c4cf36c` | ICERIK_DOGRULANDI |
| 2026-06-26 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-26/CSV/GunlukYarisProgrami/26.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 15351 | `8afff26a702a08726a905c6a12f9d7a293b45ea75176d052e8b67b5973c3801d` | ICERIK_DOGRULANDI |
| 2026-06-26 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-26/CSV/GunlukYarisSonuclari/26.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 15642 | `63363ebd64a620b0bcbe5f6a4e0d7c2485be5e79e27038ac9f827ddf29e2f0e9` | ICERIK_DOGRULANDI |
| 2026-06-27 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-27/CSV/GunlukYarisProgrami/27.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 13718 | `ff9fdb97a6413e55f60bd3c4ebea822557c51a5477ed49d3cd4ccde566f5617e` | ICERIK_DOGRULANDI |
| 2026-06-27 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-27/CSV/GunlukYarisSonuclari/27.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 13960 | `1c420fbdaa7ff0f2a5b47ab292e29b63c59b2f60b4764671421a86b5dbc96f46` | ICERIK_DOGRULANDI |
| 2026-06-27 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-27/CSV/GunlukYarisProgrami/27.06.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 12230 | `99a962d3568baa0c55a5e924ff094615a83700924f53c21b15541af1d7ab99ce` | ICERIK_DOGRULANDI |
| 2026-06-27 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-27/CSV/GunlukYarisSonuclari/27.06.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 12493 | `b1d4c57e4db5922f129ad692b337870e44aaa6c39ce3252700d56caa066c8bd5` | ICERIK_DOGRULANDI |
| 2026-06-27 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-27/CSV/GunlukYarisProgrami/27.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 12899 | `62905be75a2b9b035ed88f62c1a7ebf05ebf931ca03c2d9f791fd7f1d2cd94f6` | ICERIK_DOGRULANDI |
| 2026-06-27 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-27/CSV/GunlukYarisSonuclari/27.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 13206 | `5671a877e2ed13ce962e3835455105601562ba9b2d5cad9de57f3e04462a8132` | ICERIK_DOGRULANDI |
| 2026-06-28 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-28/CSV/GunlukYarisProgrami/28.06.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 23390 | `676732bb58e0d48d849b886d3a1e5e194f77e6ff4ab6a6f2cae1b298b078696d` | ICERIK_DOGRULANDI |
| 2026-06-28 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-28/CSV/GunlukYarisSonuclari/28.06.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 23721 | `6c0972240c11ba7da0cab4c8e1c85b2d9b42c9c864d1969c3f0dd491ee320091` | ICERIK_DOGRULANDI |
| 2026-06-28 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-28/CSV/GunlukYarisProgrami/28.06.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 12652 | `3051d186757c53ac97affcf6cf40325313a4b2b4f1ccd56ad4f7b512a7e02e7b` | ICERIK_DOGRULANDI |
| 2026-06-28 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-28/CSV/GunlukYarisSonuclari/28.06.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 12950 | `231679f901a180bb4e6aa8127e3e8e5766c1edf80be1bfe52c4f628a7925e165` | ICERIK_DOGRULANDI |
| 2026-06-29 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-29/CSV/GunlukYarisProgrami/29.06.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 18490 | `800b497c2c99c355b254dd7a447b685d5a5b4900dd504d8778b08a9e0809477a` | ICERIK_DOGRULANDI |
| 2026-06-29 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-29/CSV/GunlukYarisSonuclari/29.06.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 19265 | `f991b7b745d0012aac8a93517fc5f02f8f94300adf107f71ab7883a5d614b692` | ICERIK_DOGRULANDI |
| 2026-06-29 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-29/CSV/GunlukYarisProgrami/29.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 15178 | `f3c6a52245fdcf875e47bfd78d1da7e4f0d8018e5768d65439e73f99e5470986` | ICERIK_DOGRULANDI |
| 2026-06-29 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-29/CSV/GunlukYarisSonuclari/29.06.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 15852 | `86160038e0588f1e0f01291cf3c41b97e67579450628dd8af7c961c60d5e20e0` | ICERIK_DOGRULANDI |
| 2026-06-30 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-30/CSV/GunlukYarisProgrami/30.06.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 12251 | `ad226739d5f349e89157cd96a69835801e358a8c1962324c549119b92c9b4287` | ICERIK_DOGRULANDI |
| 2026-06-30 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-30/CSV/GunlukYarisSonuclari/30.06.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 12329 | `f36a3e179b4b407289233f562d780073aa02285ea271b5fded7b870d66094fe3` | ICERIK_DOGRULANDI |
| 2026-06-30 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-30/CSV/GunlukYarisProgrami/30.06.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 14475 | `a28ef59c7ab6e15cf2392aa04bfb0277ca1f9cf64b410cd61641328a38b5519f` | ICERIK_DOGRULANDI |
| 2026-06-30 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-06-30/CSV/GunlukYarisSonuclari/30.06.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 15069 | `ff14704448197ecb769c49f5b6f92369b2dad8c66996773e24bba2a1f680903e` | ICERIK_DOGRULANDI |

Doğrulanan ham CSV'ler `/home/user/TJK/kontrol/ham/2026-06` altında (`yerel_dosya` adlarıyla). Ayrıntılı kayıt: `/home/user/TJK/kontrol/yillik/listeler/2026-06_durum.tsv`.
