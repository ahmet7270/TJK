# Son veri testi

- **Durum:** TAMAMLANDI (liste sonuna kadar işlendi)
- **Çalışma:** Mart 2026 toplu indirme, `/home/user/TJK/kontrol/yillik/listeler/2026-01_liste.tsv`
- **Araçlar:** `tjk-fetch toplu` (curl ile HTTPS; içerik doğrulaması `tjk_ayristirici/src/ayristir.rs` ile), birlikte ayrıştırma `tjk_ayristirici`
- **SQLite:** kullanılmadı
- **Ön kontrol:** 62 çift, 124 satır, 124 benzersiz URL; her çiftte bir program ve bir sonuç
- **robots.txt:** HTTP 404: robots.txt yok, kural tanımlı değil (RFC 9309: izinli)

## Özet (bu çalışmadaki 62 çift)

| Durum | Çift |
|---|---|
| Tamamlandı | 61 |
| İnceleme gerekli | 1 |
| Eksik | 0 |
| Henüz denenmedi | 0 |
| **Toplam** | **62** |

Önceki çalışmadan SHA-256'sı eşleşerek yeniden kullanılan dosya: 0

## Çiftler

| # | Tarih | Hipodrom | Durum | Gerekçe |
|---|---|---|---|---|
| 1 | 2026-01-01 | Antalya | TAMAMLANDI |  |
| 2 | 2026-01-01 | İzmir | TAMAMLANDI |  |
| 3 | 2026-01-02 | Antalya | TAMAMLANDI |  |
| 4 | 2026-01-02 | İzmir | TAMAMLANDI |  |
| 5 | 2026-01-03 | Adana | TAMAMLANDI |  |
| 6 | 2026-01-03 | İstanbul | TAMAMLANDI |  |
| 7 | 2026-01-04 | Adana | TAMAMLANDI |  |
| 8 | 2026-01-04 | İzmir | TAMAMLANDI |  |
| 9 | 2026-01-05 | Bursa | TAMAMLANDI |  |
| 10 | 2026-01-05 | Şanlıurfa | TAMAMLANDI |  |
| 11 | 2026-01-06 | Adana | TAMAMLANDI |  |
| 12 | 2026-01-06 | Antalya | TAMAMLANDI |  |
| 13 | 2026-01-07 | İstanbul | TAMAMLANDI |  |
| 14 | 2026-01-07 | Şanlıurfa | TAMAMLANDI |  |
| 15 | 2026-01-08 | Antalya | TAMAMLANDI |  |
| 16 | 2026-01-08 | İzmir | TAMAMLANDI |  |
| 17 | 2026-01-09 | Bursa | TAMAMLANDI |  |
| 18 | 2026-01-09 | İzmir | TAMAMLANDI |  |
| 19 | 2026-01-10 | Adana | TAMAMLANDI |  |
| 20 | 2026-01-10 | İstanbul | TAMAMLANDI |  |
| 21 | 2026-01-11 | Adana | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: ESLESMEYEN_PROGRAM_KAYDI ×49 (ilk: ESLESMEYEN_PROGRAM_KAYDI: 20260111-ADANA-K04-A01 ALİMENDER (aday satırlar: [])); EKURI_GRUPLARI_CELISIYOR ×1 (ilk: EKURI_GRUPLARI_CELISIYOR: 8. koşu program [[9, 15], [14, 18]] sonuç [] (üye numaraları, sıradan bağımsız)); BIRINCI_BULUNAMADI ×5 (ilk: BIRINCI_BULUNAMADI: 4. koşu (GANYAN [])); ALTILI_AYAGI_EKSIK ×8 (ilk: ALTILI_AYAGI_EKSIK: 20260111-ADANA-A1 4. ayak (4. koşu): sonuç dosyasında koşu yok); ALTILI_ODEMESI_BULUNAMADI ×2 (ilk: ALTILI_ODEMESI_BULUNAMADI: 20260111-ADANA-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok) |
| 22 | 2026-01-11 | İzmir | TAMAMLANDI |  |
| 23 | 2026-01-12 | Bursa | TAMAMLANDI |  |
| 24 | 2026-01-12 | Şanlıurfa | TAMAMLANDI |  |
| 25 | 2026-01-13 | Adana | TAMAMLANDI |  |
| 26 | 2026-01-13 | Antalya | TAMAMLANDI |  |
| 27 | 2026-01-14 | İstanbul | TAMAMLANDI |  |
| 28 | 2026-01-14 | Şanlıurfa | TAMAMLANDI |  |
| 29 | 2026-01-15 | Antalya | TAMAMLANDI |  |
| 30 | 2026-01-15 | İzmir | TAMAMLANDI |  |
| 31 | 2026-01-16 | Antalya | TAMAMLANDI |  |
| 32 | 2026-01-16 | İzmir | TAMAMLANDI |  |
| 33 | 2026-01-17 | Adana | TAMAMLANDI |  |
| 34 | 2026-01-17 | İstanbul | TAMAMLANDI |  |
| 35 | 2026-01-18 | Adana | TAMAMLANDI |  |
| 36 | 2026-01-18 | İzmir | TAMAMLANDI |  |
| 37 | 2026-01-19 | Bursa | TAMAMLANDI |  |
| 38 | 2026-01-19 | Şanlıurfa | TAMAMLANDI |  |
| 39 | 2026-01-20 | Adana | TAMAMLANDI |  |
| 40 | 2026-01-20 | Antalya | TAMAMLANDI |  |
| 41 | 2026-01-21 | İstanbul | TAMAMLANDI |  |
| 42 | 2026-01-21 | Şanlıurfa | TAMAMLANDI |  |
| 43 | 2026-01-22 | Antalya | TAMAMLANDI |  |
| 44 | 2026-01-22 | İzmir | TAMAMLANDI |  |
| 45 | 2026-01-23 | Bursa | TAMAMLANDI |  |
| 46 | 2026-01-23 | İzmir | TAMAMLANDI |  |
| 47 | 2026-01-24 | Adana | TAMAMLANDI |  |
| 48 | 2026-01-24 | İstanbul | TAMAMLANDI |  |
| 49 | 2026-01-25 | Adana | TAMAMLANDI |  |
| 50 | 2026-01-25 | İzmir | TAMAMLANDI |  |
| 51 | 2026-01-26 | Bursa | TAMAMLANDI |  |
| 52 | 2026-01-26 | Şanlıurfa | TAMAMLANDI |  |
| 53 | 2026-01-27 | Adana | TAMAMLANDI |  |
| 54 | 2026-01-27 | Antalya | TAMAMLANDI |  |
| 55 | 2026-01-28 | İstanbul | TAMAMLANDI |  |
| 56 | 2026-01-28 | Şanlıurfa | TAMAMLANDI |  |
| 57 | 2026-01-29 | Antalya | TAMAMLANDI |  |
| 58 | 2026-01-29 | İzmir | TAMAMLANDI |  |
| 59 | 2026-01-30 | Antalya | TAMAMLANDI |  |
| 60 | 2026-01-30 | İzmir | TAMAMLANDI |  |
| 61 | 2026-01-31 | Adana | TAMAMLANDI |  |
| 62 | 2026-01-31 | İstanbul | TAMAMLANDI |  |

## Dosyalar

| Tarih | Hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | Doğrulama |
|---|---|---|---|---|---|---|---|
| 2026-01-01 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-01/CSV/GunlukYarisProgrami/01.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 15095 | `71b81ef20ec3ba33f27d421f1810b58c8a07084bb01f7323ad551aa214a97d36` | ICERIK_DOGRULANDI |
| 2026-01-01 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-01/CSV/GunlukYarisSonuclari/01.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15510 | `726dc45182a520b8c18d2eb126416e13374264c3d30376a1a740327121bd7fbe` | ICERIK_DOGRULANDI |
| 2026-01-01 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-01/CSV/GunlukYarisProgrami/01.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 9231 | `a7bf30d4e54f49c093ba98440b01fe6f5edc6029aa82121f5cddf516883b8c6c` | ICERIK_DOGRULANDI |
| 2026-01-01 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-01/CSV/GunlukYarisSonuclari/01.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 9374 | `1965dce974f3aa4510e7b265dad434ed14426c1e027b7f5bf5fc402040a571f3` | ICERIK_DOGRULANDI |
| 2026-01-02 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-02/CSV/GunlukYarisProgrami/02.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 13146 | `71e76f3c6d04bc7a613cb3e96707289d6d67e246b75891e3c6a8f62657016560` | ICERIK_DOGRULANDI |
| 2026-01-02 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-02/CSV/GunlukYarisSonuclari/02.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 13643 | `c7f22a82148fdb490d48eb50d52193431f9b4484c98e540b565df0d3bc755675` | ICERIK_DOGRULANDI |
| 2026-01-02 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-02/CSV/GunlukYarisProgrami/02.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 10352 | `b9148e8dc3ec6f466a3b55a0b5cfd638ae14a515ce1f3cd611944ab5852bad84` | ICERIK_DOGRULANDI |
| 2026-01-02 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-02/CSV/GunlukYarisSonuclari/02.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 10561 | `7aa7bba5943566980242db2fdc512cbf1e156d5977a7f476f4a81669e949317c` | ICERIK_DOGRULANDI |
| 2026-01-03 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-03/CSV/GunlukYarisProgrami/03.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 12585 | `7147c232973c3479bdff25568adaca6a6ccd69a65208303c111d71445c916062` | ICERIK_DOGRULANDI |
| 2026-01-03 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-03/CSV/GunlukYarisSonuclari/03.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 12689 | `6b7dad6aac1839424a07cae4d680bce392382fde0f31456fa2955a0f14c16f32` | ICERIK_DOGRULANDI |
| 2026-01-03 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-03/CSV/GunlukYarisProgrami/03.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17428 | `df61cec1612dec80ed808dd9defcc0db56f9cdcac75c421e3230f2ba76c61a39` | ICERIK_DOGRULANDI |
| 2026-01-03 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-03/CSV/GunlukYarisSonuclari/03.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17976 | `64f6158795c08641562f1ae797c7721143707ed228c462db17fe29cc040b8b74` | ICERIK_DOGRULANDI |
| 2026-01-04 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-04/CSV/GunlukYarisProgrami/04.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13845 | `e94853a4309792773a9c87135d9400697ce7d7f694b22a794da583ab25b22e4c` | ICERIK_DOGRULANDI |
| 2026-01-04 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-04/CSV/GunlukYarisSonuclari/04.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14060 | `75554e344dbb65f51e2b57e4bf11045cc20dc477b2f26f68c30f4e09fe0a8dd4` | ICERIK_DOGRULANDI |
| 2026-01-04 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-04/CSV/GunlukYarisProgrami/04.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 12761 | `c52328ec74433023e3617c1ad06bb395b9e801f0f02e9402f1e0961733338bd7` | ICERIK_DOGRULANDI |
| 2026-01-04 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-04/CSV/GunlukYarisSonuclari/04.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 12997 | `1f94d4de671f882b95e40ccdf81c61ddd8c71f4d16ea48fe190c40cec05209a0` | ICERIK_DOGRULANDI |
| 2026-01-05 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-05/CSV/GunlukYarisProgrami/05.01.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 10707 | `48eb56d36933cdf89bec16773d6aab4dc108f2d15f7afcd0f66df81dc1954901` | ICERIK_DOGRULANDI |
| 2026-01-05 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-05/CSV/GunlukYarisSonuclari/05.01.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 10829 | `ab945055b3107f72256cdc569fc703e82cdc6d260c3aac0a92901ea698fe83eb` | ICERIK_DOGRULANDI |
| 2026-01-05 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-05/CSV/GunlukYarisProgrami/05.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 15101 | `56b1a9603670cfebb5d78c61767028ef0bf7f294966ab9b1a8b640d242b7c0c5` | ICERIK_DOGRULANDI |
| 2026-01-05 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-05/CSV/GunlukYarisSonuclari/05.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 15296 | `4499c3736ea5d5a35636ae6d4f10b5344bddcdbea1ebcce5d4b67908469515e6` | ICERIK_DOGRULANDI |
| 2026-01-06 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-06/CSV/GunlukYarisProgrami/06.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 14408 | `6963ee18df4a19a3ae4182d56f143517aad4d5dfa12af9607395f7c6fe77e237` | ICERIK_DOGRULANDI |
| 2026-01-06 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-06/CSV/GunlukYarisSonuclari/06.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14717 | `a24a660aa8bdc59d40c63f241a2fde03548dc4ee1eb211352c0570b58555394e` | ICERIK_DOGRULANDI |
| 2026-01-06 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-06/CSV/GunlukYarisProgrami/06.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 13100 | `35596695bc62fff8be711b1891d7d1473f436f4dd119285429e0657cd60960cb` | ICERIK_DOGRULANDI |
| 2026-01-06 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-06/CSV/GunlukYarisSonuclari/06.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 13513 | `d13eac15f5cce999f2e1b0a03c279b99b10f57ffe8808c677d388a70d9873b61` | ICERIK_DOGRULANDI |
| 2026-01-07 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-07/CSV/GunlukYarisProgrami/07.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16097 | `6c968edd58c1ec47e5c5ccc72688dbf0be2c9e6148e2d28c3f10496357373c47` | ICERIK_DOGRULANDI |
| 2026-01-07 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-07/CSV/GunlukYarisSonuclari/07.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 16970 | `a864313018b354241ea636f2d3ba3b770906beb426eef8a7c44e52f1c3386165` | ICERIK_DOGRULANDI |
| 2026-01-07 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-07/CSV/GunlukYarisProgrami/07.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 12252 | `03617b46a542d9f38ef7867fcacb062c35d840003dc32f68bc47268a11353952` | ICERIK_DOGRULANDI |
| 2026-01-07 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-07/CSV/GunlukYarisSonuclari/07.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 12443 | `55fc7e84aaacf36a8453bfad88770b8254cb3fecc695b1e63228a7eb8a98034c` | ICERIK_DOGRULANDI |
| 2026-01-08 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-08/CSV/GunlukYarisProgrami/08.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 18271 | `2a0da6cada8c2ab8550d3b6c72b4747a2632cbdcbc8085e535fca58ffc0af750` | ICERIK_DOGRULANDI |
| 2026-01-08 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-08/CSV/GunlukYarisSonuclari/08.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 18778 | `dab863aabe1601262b56d1b6821e29f2fd8ae35a236b2c3b25d2b53a02d6c96b` | ICERIK_DOGRULANDI |
| 2026-01-08 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-08/CSV/GunlukYarisProgrami/08.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 9789 | `f8a5cd35e6dedb2afe66f9d5cdfd6a74a208e927c9b1caac32bb8cca35c23f1b` | ICERIK_DOGRULANDI |
| 2026-01-08 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-08/CSV/GunlukYarisSonuclari/08.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 9998 | `3c4e3aa4cd19c2e07687e36026fe5eceef8b9a130739adf6308bd75b0850df79` | ICERIK_DOGRULANDI |
| 2026-01-09 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-09/CSV/GunlukYarisProgrami/09.01.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 14268 | `ade4633ba332c8c57519e94002cbbe7b5f535573123dbb5c1dfdefb69b1ffb7c` | ICERIK_DOGRULANDI |
| 2026-01-09 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-09/CSV/GunlukYarisSonuclari/09.01.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 14778 | `8e9c9ba6c6b3237ae790292486e1c952530e9e826c9d2a8deb8d8aec26a9c59c` | ICERIK_DOGRULANDI |
| 2026-01-09 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-09/CSV/GunlukYarisProgrami/09.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 11410 | `6cb9e5209f852bdb8b3c0de8c231e0ac6b1ed6f31aad93318737e2774489685a` | ICERIK_DOGRULANDI |
| 2026-01-09 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-09/CSV/GunlukYarisSonuclari/09.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 11837 | `2eab38860cd1eb504ead732a40e4e7fbf58fa122f4df6c4707cc977f6f40b388` | ICERIK_DOGRULANDI |
| 2026-01-10 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-10/CSV/GunlukYarisProgrami/10.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13283 | `473c0269163dc7d8e9b379c26d4c08529466065ad7562504114e120226674a23` | ICERIK_DOGRULANDI |
| 2026-01-10 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-10/CSV/GunlukYarisSonuclari/10.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 13673 | `5f232df1ad3a94cb74624c3e033a15dcdd1d7dabfaa936d7ce8a8e71c5d04734` | ICERIK_DOGRULANDI |
| 2026-01-10 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-10/CSV/GunlukYarisProgrami/10.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17375 | `f273947e4390a4876ee0b3f8191fef1befeee73118bfb726a3af51bb173a8a4f` | ICERIK_DOGRULANDI |
| 2026-01-10 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-10/CSV/GunlukYarisSonuclari/10.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17835 | `cee9a0aad9cb3f610aeb39880d01c52ce5a07c7e7fd0979f15c553cf9d9cb599` | ICERIK_DOGRULANDI |
| 2026-01-11 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-11/CSV/GunlukYarisProgrami/11.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15282 | `d814ab5da36544aebfb10eaff23081117d16078f6bbfb3f3b1e33961e7a35b33` | ICERIK_DOGRULANDI |
| 2026-01-11 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-11/CSV/GunlukYarisSonuclari/11.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 5349 | `9c9f870d5d69b5d25f1b0c169377702a5a00558b7fb5dc2754fa8275a0a9f58b` | ICERIK_DOGRULANDI |
| 2026-01-11 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-11/CSV/GunlukYarisProgrami/11.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16098 | `905272d654ee7a7d2e93d81931b5c97f24a6635eb377626d027b38a70273eb1f` | ICERIK_DOGRULANDI |
| 2026-01-11 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-11/CSV/GunlukYarisSonuclari/11.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16421 | `e361e5b50910c1713ae5e6fc91fcf9650ef41978a00fc24235f319e3172ad571` | ICERIK_DOGRULANDI |
| 2026-01-12 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-12/CSV/GunlukYarisProgrami/12.01.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 13026 | `a264a3ed87b14ca18df026765f9481a9d37c3c78ba373b729eec29e6c5c53457` | ICERIK_DOGRULANDI |
| 2026-01-12 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-12/CSV/GunlukYarisSonuclari/12.01.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 13438 | `9502229d41dd1d91d6d8f1bb7d3049b2bcd84b7b912bc1c4a8468158cda40ead` | ICERIK_DOGRULANDI |
| 2026-01-12 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-12/CSV/GunlukYarisProgrami/12.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 15676 | `159a9f3f9f3afe1aa56e056732ba185c25fbc31086e7707bd78f7ec47e20953d` | ICERIK_DOGRULANDI |
| 2026-01-12 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-12/CSV/GunlukYarisSonuclari/12.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 16231 | `778ec610415382ed4f9cef4c2fb3b683a6106d0c0255735e8758038e1a433da5` | ICERIK_DOGRULANDI |
| 2026-01-13 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-13/CSV/GunlukYarisProgrami/13.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 12201 | `e9ecf82f97dc63fdca1b86f1776f8f4b68ecf4ff69c7cb3af487de1ff64e1478` | ICERIK_DOGRULANDI |
| 2026-01-13 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-13/CSV/GunlukYarisSonuclari/13.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 12374 | `ec22e5f858709ab04a1b63f8f5bc8e19e6e8235ff105bafbabfc551528fda7b3` | ICERIK_DOGRULANDI |
| 2026-01-13 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-13/CSV/GunlukYarisProgrami/13.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 12556 | `f9f2d1842ee08791839d5fd3bf8810b3346b59856f88c863276e802bcdb50e49` | ICERIK_DOGRULANDI |
| 2026-01-13 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-13/CSV/GunlukYarisSonuclari/13.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 12913 | `e4b4206f66484fd35658d33ad3dead7eabee9c4230fc6734889b7addd90bc921` | ICERIK_DOGRULANDI |
| 2026-01-14 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-14/CSV/GunlukYarisProgrami/14.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 11834 | `c3f6aa7a358be2fefe341f503f50576dd81ab3fbc5a5f4642d9c99000846367a` | ICERIK_DOGRULANDI |
| 2026-01-14 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-14/CSV/GunlukYarisSonuclari/14.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 12265 | `27c44ab1365a5ce5777d2385dd35db9da2143fd2723bda72f8f19ac0035c4566` | ICERIK_DOGRULANDI |
| 2026-01-14 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-14/CSV/GunlukYarisProgrami/14.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 19031 | `3f66a32145dedba7ee8026c834494bfc02e76a3a3407277334408135191bd050` | ICERIK_DOGRULANDI |
| 2026-01-14 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-14/CSV/GunlukYarisSonuclari/14.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 19601 | `a60ed4ccd4ea5d1bc86f800dc3308693bb2d4dc73af9cacc885d39aa90e2dd9a` | ICERIK_DOGRULANDI |
| 2026-01-15 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-15/CSV/GunlukYarisProgrami/15.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 18595 | `b6acef8f125762936a171909f17893b18f5644769d3bb6b3b8cd27f95c4ccf2a` | ICERIK_DOGRULANDI |
| 2026-01-15 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-15/CSV/GunlukYarisSonuclari/15.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 19399 | `ded735c4c8937a7cca4d8bb96482c7a6cf5906fbbd4d24a4fc627830b9e97a9d` | ICERIK_DOGRULANDI |
| 2026-01-15 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-15/CSV/GunlukYarisProgrami/15.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 10474 | `24340fcb99ddedd7006b3916980e56206cdc866e5e092cec760dca1a8862e99d` | ICERIK_DOGRULANDI |
| 2026-01-15 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-15/CSV/GunlukYarisSonuclari/15.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 10608 | `2133bd024a9c7b460f9eec83ff0beac4f4f54b017a409f087010ed720875f8fd` | ICERIK_DOGRULANDI |
| 2026-01-16 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-16/CSV/GunlukYarisProgrami/16.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 18745 | `a52cf369fc109b0c203ac5218fe232b35062bd3b32237b76c48e855794a4d305` | ICERIK_DOGRULANDI |
| 2026-01-16 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-16/CSV/GunlukYarisSonuclari/16.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 19212 | `7ff9a2d7fe6143dd0809ddcb83fb746478ac4090f9fdbfc5b1c5ab2c51457faa` | ICERIK_DOGRULANDI |
| 2026-01-16 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-16/CSV/GunlukYarisProgrami/16.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 11093 | `905369b330abcfe3062f9d04a8c80d8b15a22a4780e0de0ae40b5ecca0174ba0` | ICERIK_DOGRULANDI |
| 2026-01-16 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-16/CSV/GunlukYarisSonuclari/16.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 11244 | `67d2c280f11130dc2e18b66d458adb7a0ed27348d8683c98b76e4559e721b4f8` | ICERIK_DOGRULANDI |
| 2026-01-17 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-17/CSV/GunlukYarisProgrami/17.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 13878 | `bda4957abfb7c86fef289cdd35f19df786a60c780db0a0a4b7230d0098a98daa` | ICERIK_DOGRULANDI |
| 2026-01-17 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-17/CSV/GunlukYarisSonuclari/17.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14220 | `0328248e0026e8139784b11075f5e9619c397276252788cfb0db1a2862092c07` | ICERIK_DOGRULANDI |
| 2026-01-17 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-17/CSV/GunlukYarisProgrami/17.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 18178 | `5fcba223f61cc2252b5c8fbd47236d05174e5b39d02d338a9a58ceafd6f87db2` | ICERIK_DOGRULANDI |
| 2026-01-17 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-17/CSV/GunlukYarisSonuclari/17.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 18601 | `874bd66336e5a1606b0189d5515004e65bee6c4effb7251d3a947985e2bfe652` | ICERIK_DOGRULANDI |
| 2026-01-18 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-18/CSV/GunlukYarisProgrami/18.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15048 | `8794c79f3874334522c4d933d7848ebf40373a82a0626fa59fe99ddcdb57360d` | ICERIK_DOGRULANDI |
| 2026-01-18 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-18/CSV/GunlukYarisSonuclari/18.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15036 | `b1d4279fc69e72979296ec5e7b7a4738b1d2760d61dd6f1d22bc1c8d9734c7a8` | ICERIK_DOGRULANDI |
| 2026-01-18 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-18/CSV/GunlukYarisProgrami/18.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 14995 | `75078c6dffa24178ceb1785e7f0c8f95baf1f49b4df750449778d5247947f5f7` | ICERIK_DOGRULANDI |
| 2026-01-18 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-18/CSV/GunlukYarisSonuclari/18.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 15601 | `a9c7a9bf38dae08147d737b5aa968912e86cfde9639a85e71be3bcdb1b38092e` | ICERIK_DOGRULANDI |
| 2026-01-19 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-19/CSV/GunlukYarisProgrami/19.01.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 16971 | `b145ff153cb6f34bdafc43445624443ee3d950384f072882f8e1c3cf787cbe16` | ICERIK_DOGRULANDI |
| 2026-01-19 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-19/CSV/GunlukYarisSonuclari/19.01.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 17213 | `c5dc03d2d79ae0f83d59fd523fcdba1f82fedc3694f9f8837f595b7b88df06f9` | ICERIK_DOGRULANDI |
| 2026-01-19 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-19/CSV/GunlukYarisProgrami/19.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 15248 | `f6ca8a7ac83698413b202524355f7c457f416eb807d3ea083a3bad0dff685b0d` | ICERIK_DOGRULANDI |
| 2026-01-19 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-19/CSV/GunlukYarisSonuclari/19.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 15931 | `5290e99d6b68a9235cf32323a82a2864099ed8234cb541e2ebe1b1f609d0ad07` | ICERIK_DOGRULANDI |
| 2026-01-20 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-20/CSV/GunlukYarisProgrami/20.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15153 | `5e8088004967256d92cc6ece78c9a6cc2dce2f1a099cb6481f6327baaacd4fbe` | ICERIK_DOGRULANDI |
| 2026-01-20 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-20/CSV/GunlukYarisSonuclari/20.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15262 | `905af412014da7f3138032062f32d9f5203848ce8c58b82a4b16949701924e5f` | ICERIK_DOGRULANDI |
| 2026-01-20 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-20/CSV/GunlukYarisProgrami/20.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 10619 | `b266b7ac15e507920e1ff96cdc74e1224d8c6f85a57d94c65ef76f93881d3aad` | ICERIK_DOGRULANDI |
| 2026-01-20 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-20/CSV/GunlukYarisSonuclari/20.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 10795 | `934c0c34cef686c0936fe9c982dc182c407b083c20c3abce78061aaa21b727af` | ICERIK_DOGRULANDI |
| 2026-01-21 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-21/CSV/GunlukYarisProgrami/21.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16145 | `b4acbdedccb0ca3b0876bc856ca1f43db54884e3d235122d197167ac7524ffaa` | ICERIK_DOGRULANDI |
| 2026-01-21 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-21/CSV/GunlukYarisSonuclari/21.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 16590 | `8727aecbfb1d0d62d827baef02f11381eeb975e9e7230431b7589c5f988c1033` | ICERIK_DOGRULANDI |
| 2026-01-21 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-21/CSV/GunlukYarisProgrami/21.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 16041 | `d46c1713d11d391d0a4a5c41e8cd05234547f2be566739a15b7bde738fadf048` | ICERIK_DOGRULANDI |
| 2026-01-21 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-21/CSV/GunlukYarisSonuclari/21.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 16379 | `45ac4e164d978fa80b1cc86840b17365304000102f9e22845eb0933292ac1b76` | ICERIK_DOGRULANDI |
| 2026-01-22 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-22/CSV/GunlukYarisProgrami/22.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 12691 | `09e0c82d7c6ae2eab1fdd05869bef1ddde03a0f51262868a2d43a45ae35d5566` | ICERIK_DOGRULANDI |
| 2026-01-22 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-22/CSV/GunlukYarisSonuclari/22.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 13235 | `7a063ae3aeb192db16fc717af5e1dfc499ee425e0094b34f02ff06d2f29dfb6d` | ICERIK_DOGRULANDI |
| 2026-01-22 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-22/CSV/GunlukYarisProgrami/22.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 12721 | `11fc1c809df1f52d198e9bf248aecaf6763f2b14e9c5feaaf5a2afafd86b643f` | ICERIK_DOGRULANDI |
| 2026-01-22 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-22/CSV/GunlukYarisSonuclari/22.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 13202 | `eef786ec85cf153a55f920564c4d8b8c3e0edfa3069d3f799304cef6511349a7` | ICERIK_DOGRULANDI |
| 2026-01-23 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-23/CSV/GunlukYarisProgrami/23.01.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 12592 | `241edff51096905055812d4d82b7487cd727b1ad0db344f1821ab4fee7f5794a` | ICERIK_DOGRULANDI |
| 2026-01-23 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-23/CSV/GunlukYarisSonuclari/23.01.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 12877 | `cfff3e26215bdab0ad680e5c0536491f3e59bde8e0106e968967768a286ecd42` | ICERIK_DOGRULANDI |
| 2026-01-23 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-23/CSV/GunlukYarisProgrami/23.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 9156 | `64ecc6c20e9c1065224c23a51e1858d225e51e0672b813a735a0002f49101e09` | ICERIK_DOGRULANDI |
| 2026-01-23 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-23/CSV/GunlukYarisSonuclari/23.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 9155 | `24c3fd202d8e2f2b4f6301fb9ec6b5752e1af608e263389826c0770c1238c273` | ICERIK_DOGRULANDI |
| 2026-01-24 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-24/CSV/GunlukYarisProgrami/24.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 16014 | `b97d01dd479ce11db5ce13fbe411fd18723d998e8350008ee5177134178e46fb` | ICERIK_DOGRULANDI |
| 2026-01-24 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-24/CSV/GunlukYarisSonuclari/24.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 16473 | `aa8db4965b33624bddf058f770989398e451970412c633839390f42efed05e6b` | ICERIK_DOGRULANDI |
| 2026-01-24 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-24/CSV/GunlukYarisProgrami/24.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17947 | `450439779dbd7b5fd6dc94aebea0f363226e0923590c280f551de192b3a041cb` | ICERIK_DOGRULANDI |
| 2026-01-24 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-24/CSV/GunlukYarisSonuclari/24.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 18503 | `d39a50ccfa242fc132f5f73603e630b7f2fa26373e574c149e793186e9b66827` | ICERIK_DOGRULANDI |
| 2026-01-25 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-25/CSV/GunlukYarisProgrami/25.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 14138 | `0f9cb892cd1ea8c70fb6c48cd576080eae60a5e84580ebf8439fd2184c0e13f6` | ICERIK_DOGRULANDI |
| 2026-01-25 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-25/CSV/GunlukYarisSonuclari/25.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14511 | `0536077d2a353fc09c4fffddf1dc65d019169ad457e6d5e1a95ec6fdb91c46cd` | ICERIK_DOGRULANDI |
| 2026-01-25 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-25/CSV/GunlukYarisProgrami/25.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16861 | `a4a743178662f0f265ad9fb6f3fee0fedb4c0099a352cc1cd42693dba223b0c2` | ICERIK_DOGRULANDI |
| 2026-01-25 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-25/CSV/GunlukYarisSonuclari/25.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 17210 | `1eec36c25b6718d703deb6c42f7e4925776a7dd13ab4a3882445096e82ea64a4` | ICERIK_DOGRULANDI |
| 2026-01-26 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-26/CSV/GunlukYarisProgrami/26.01.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 13066 | `6aee621d91685d7fb5193aed32bca86807ecd22a9a440f18af7473bae60db436` | ICERIK_DOGRULANDI |
| 2026-01-26 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-26/CSV/GunlukYarisSonuclari/26.01.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 13526 | `02fbae54652a1a1e73cba8d1a508115332cb973518154321865f9d78593cc2bb` | ICERIK_DOGRULANDI |
| 2026-01-26 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-26/CSV/GunlukYarisProgrami/26.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 14606 | `ad6e104f5a9b5727705d05b8af05a320a63933721536ce8f8f75cd9ad4ecec26` | ICERIK_DOGRULANDI |
| 2026-01-26 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-26/CSV/GunlukYarisSonuclari/26.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 15124 | `1ccffd8cb6a18b53e2a33f79801e63eaddfb35a255e1d2c00799ff29786d02b7` | ICERIK_DOGRULANDI |
| 2026-01-27 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-27/CSV/GunlukYarisProgrami/27.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15329 | `f33b948d748db9343baaec4954ee7eedfbd9bdc98038e0b643d8580a72e4a8be` | ICERIK_DOGRULANDI |
| 2026-01-27 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-27/CSV/GunlukYarisSonuclari/27.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15659 | `060335db347e3dac70468e2079e9d1c1daa8b17f7fe389cf6244483ed468c542` | ICERIK_DOGRULANDI |
| 2026-01-27 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-27/CSV/GunlukYarisProgrami/27.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 14962 | `dd113e4178e85ccc6dc38aef4c4402c40779dc225021b2bd91100be02e508173` | ICERIK_DOGRULANDI |
| 2026-01-27 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-27/CSV/GunlukYarisSonuclari/27.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15460 | `4a769d9dee46a3d711801f1a51295f16c543e8e0d61ab1f2a3eafdd84c86bae1` | ICERIK_DOGRULANDI |
| 2026-01-28 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-28/CSV/GunlukYarisProgrami/28.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16497 | `1257d3a30bfd92d41af3cd98e85e93a16ce89355c43d5533812e94e4afd527d5` | ICERIK_DOGRULANDI |
| 2026-01-28 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-28/CSV/GunlukYarisSonuclari/28.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17151 | `425bb6214b66932b95e92b5484697b1477e111a682c79c009bbf124ce19c0d8e` | ICERIK_DOGRULANDI |
| 2026-01-28 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-28/CSV/GunlukYarisProgrami/28.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 17943 | `341191c72757db2edce5018c273cc9cf7f324db05837441bd41fa3fe63b4a26c` | ICERIK_DOGRULANDI |
| 2026-01-28 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-28/CSV/GunlukYarisSonuclari/28.01.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 18736 | `11cda1f6cd5909830efa522a2c03b8448cb169c14c6cdbc42264c8c03f3cdfd4` | ICERIK_DOGRULANDI |
| 2026-01-29 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-29/CSV/GunlukYarisProgrami/29.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 15490 | `f622555afdfef7590d52c3023ddf8f8fdbec7d07dbabd2b9ebd55f67d8218546` | ICERIK_DOGRULANDI |
| 2026-01-29 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-29/CSV/GunlukYarisSonuclari/29.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15695 | `ebb7d0df2cda8a63aac0be48da5058e87a6b3d38ae403330892688cfeaa855c7` | ICERIK_DOGRULANDI |
| 2026-01-29 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-29/CSV/GunlukYarisProgrami/29.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 13942 | `5546c1ba80bbdff0d223d179a56d8794c8da8e72cf4b6c23addb66232fe96b2e` | ICERIK_DOGRULANDI |
| 2026-01-29 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-29/CSV/GunlukYarisSonuclari/29.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 14147 | `cf2d19a9e1b1c7969b2d2eec3ce7dd4fa43dc4d2e35526c50058a645cd19f1ab` | ICERIK_DOGRULANDI |
| 2026-01-30 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-30/CSV/GunlukYarisProgrami/30.01.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 14104 | `f1c1392bb32ca7d77e02529f3202cb4d94b462d60a7a86fb60b58686e63b17d1` | ICERIK_DOGRULANDI |
| 2026-01-30 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-30/CSV/GunlukYarisSonuclari/30.01.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 14675 | `34f93a090b11d9df58a8a6ffe3c6bb1992a2028f4023e853d3a8c6bbde3109fd` | ICERIK_DOGRULANDI |
| 2026-01-30 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-30/CSV/GunlukYarisProgrami/30.01.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 13879 | `08bbac50b7ec6558f30d08937b72275fd37fcb8d7b17e949cf7975da11651127` | ICERIK_DOGRULANDI |
| 2026-01-30 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-30/CSV/GunlukYarisSonuclari/30.01.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 14178 | `7a44d24be19a2e2dcede9ca0944e4e2677fe490fa6d19ebe18f9a321e1112a2f` | ICERIK_DOGRULANDI |
| 2026-01-31 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-31/CSV/GunlukYarisProgrami/31.01.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 14407 | `78ccdf2731951dc9bad77090b26a1fe4c037a60d87e5db5e015752bbe8c0f663` | ICERIK_DOGRULANDI |
| 2026-01-31 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-31/CSV/GunlukYarisSonuclari/31.01.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14780 | `4e29fcb410a7078f4a2b9ca7806d873b7f2e5dc66c56fa36cc71827f0b2583a5` | ICERIK_DOGRULANDI |
| 2026-01-31 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-31/CSV/GunlukYarisProgrami/31.01.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17039 | `d4c34fd14562fc232dfe9cae36f873127632c527a914e820ace9484e33a1b686` | ICERIK_DOGRULANDI |
| 2026-01-31 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-01-31/CSV/GunlukYarisSonuclari/31.01.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 17343 | `c6987e517bf104143eb505fd789f480b37b8cdb6a967af89972cd449d360df0e` | ICERIK_DOGRULANDI |

Doğrulanan ham CSV'ler `/home/user/TJK/kontrol/ham/2026-01` altında (`yerel_dosya` adlarıyla). Ayrıntılı kayıt: `/home/user/TJK/kontrol/yillik/listeler/2026-01_durum.tsv`.
