# Son veri testi

- **Durum:** TAMAMLANDI (liste sonuna kadar işlendi)
- **Çalışma:** Mart 2026 toplu indirme, `/home/user/TJK/kontrol/yillik/listeler/2026-10_liste.tsv`
- **Araçlar:** `tjk-fetch toplu` (curl ile HTTPS; içerik doğrulaması `tjk_ayristirici/src/ayristir.rs` ile), birlikte ayrıştırma `tjk_ayristirici`
- **SQLite:** kullanılmadı
- **Ön kontrol:** 67 çift, 134 satır, 134 benzersiz URL; her çiftte bir program ve bir sonuç
- **robots.txt:** HTTP 404: robots.txt yok, kural tanımlı değil (RFC 9309: izinli)

## Özet (bu çalışmadaki 67 çift)

| Durum | Çift |
|---|---|
| Tamamlandı | 64 |
| İnceleme gerekli | 3 |
| Eksik | 0 |
| Henüz denenmedi | 0 |
| **Toplam** | **67** |

Önceki çalışmadan SHA-256'sı eşleşerek yeniden kullanılan dosya: 0

## Çiftler

| # | Tarih | Hipodrom | Durum | Gerekçe |
|---|---|---|---|---|
| 1 | 2026-10-01 | Ankara | TAMAMLANDI |  |
| 2 | 2026-10-01 | İzmir | TAMAMLANDI |  |
| 3 | 2026-10-02 | Bursa | TAMAMLANDI |  |
| 4 | 2026-10-02 | İstanbul | TAMAMLANDI |  |
| 5 | 2026-10-03 | Ankara | TAMAMLANDI |  |
| 6 | 2026-10-03 | Diyarbakır | TAMAMLANDI |  |
| 7 | 2026-10-03 | İzmir | TAMAMLANDI |  |
| 8 | 2026-10-04 | Adana | TAMAMLANDI |  |
| 9 | 2026-10-04 | İstanbul | TAMAMLANDI |  |
| 10 | 2026-10-05 | Bursa | TAMAMLANDI |  |
| 11 | 2026-10-05 | Şanlıurfa | TAMAMLANDI |  |
| 12 | 2026-10-06 | Adana | TAMAMLANDI |  |
| 13 | 2026-10-06 | Kocaeli | TAMAMLANDI |  |
| 14 | 2026-10-07 | Elazığ | TAMAMLANDI |  |
| 15 | 2026-10-07 | İstanbul | TAMAMLANDI |  |
| 16 | 2026-10-08 | Ankara | TAMAMLANDI |  |
| 17 | 2026-10-08 | İzmir | TAMAMLANDI |  |
| 18 | 2026-10-09 | Bursa | TAMAMLANDI |  |
| 19 | 2026-10-09 | İstanbul | TAMAMLANDI |  |
| 20 | 2026-10-10 | Ankara | TAMAMLANDI |  |
| 21 | 2026-10-10 | Diyarbakır | TAMAMLANDI |  |
| 22 | 2026-10-10 | İzmir | TAMAMLANDI |  |
| 23 | 2026-10-11 | Adana | TAMAMLANDI |  |
| 24 | 2026-10-11 | İstanbul | TAMAMLANDI |  |
| 25 | 2026-10-12 | Bursa | TAMAMLANDI |  |
| 26 | 2026-10-12 | Şanlıurfa | TAMAMLANDI |  |
| 27 | 2026-10-13 | Adana | TAMAMLANDI |  |
| 28 | 2026-10-13 | Kocaeli | TAMAMLANDI |  |
| 29 | 2026-10-14 | Elazığ | TAMAMLANDI |  |
| 30 | 2026-10-14 | İstanbul | TAMAMLANDI |  |
| 31 | 2026-10-15 | Ankara | TAMAMLANDI |  |
| 32 | 2026-10-15 | İzmir | TAMAMLANDI |  |
| 33 | 2026-10-16 | Bursa | INCELEME_GEREKLI | sonuc: BICIM_HATASI (dosya boş) |
| 34 | 2026-10-16 | İstanbul | TAMAMLANDI |  |
| 35 | 2026-10-17 | Ankara | TAMAMLANDI |  |
| 36 | 2026-10-17 | Diyarbakır | TAMAMLANDI |  |
| 37 | 2026-10-17 | İzmir | TAMAMLANDI |  |
| 38 | 2026-10-18 | Adana | TAMAMLANDI |  |
| 39 | 2026-10-18 | İstanbul | TAMAMLANDI |  |
| 40 | 2026-10-19 | Bursa | INCELEME_GEREKLI | sonuc: BICIM_HATASI (dosya boş) |
| 41 | 2026-10-19 | Şanlıurfa | TAMAMLANDI |  |
| 42 | 2026-10-20 | Adana | TAMAMLANDI |  |
| 43 | 2026-10-20 | Kocaeli | TAMAMLANDI |  |
| 44 | 2026-10-21 | Elazığ | TAMAMLANDI |  |
| 45 | 2026-10-21 | İstanbul | TAMAMLANDI |  |
| 46 | 2026-10-22 | Ankara | TAMAMLANDI |  |
| 47 | 2026-10-22 | İzmir | TAMAMLANDI |  |
| 48 | 2026-10-23 | Bursa | TAMAMLANDI |  |
| 49 | 2026-10-23 | İstanbul | TAMAMLANDI |  |
| 50 | 2026-10-24 | Ankara | TAMAMLANDI |  |
| 51 | 2026-10-24 | Diyarbakır | TAMAMLANDI |  |
| 52 | 2026-10-24 | İzmir | TAMAMLANDI |  |
| 53 | 2026-10-25 | Adana | TAMAMLANDI |  |
| 54 | 2026-10-25 | İstanbul | TAMAMLANDI |  |
| 55 | 2026-10-26 | Bursa | TAMAMLANDI |  |
| 56 | 2026-10-26 | Şanlıurfa | TAMAMLANDI |  |
| 57 | 2026-10-27 | Adana | TAMAMLANDI |  |
| 58 | 2026-10-27 | Kocaeli | TAMAMLANDI |  |
| 59 | 2026-10-28 | Elazığ | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: ESLESMEYEN_PROGRAM_KAYDI ×69 (ilk: ESLESMEYEN_PROGRAM_KAYDI: 20261028-ELAZIG-K02-A01 DREAM CHASER (aday satırlar: [])); EKURI_GRUPLARI_CELISIYOR ×2 (ilk: EKURI_GRUPLARI_CELISIYOR: 3. koşu program [[5, 7]] sonuç [] (üye numaraları, sıradan bağımsız)); BIRINCI_BULUNAMADI ×7 (ilk: BIRINCI_BULUNAMADI: 2. koşu (GANYAN [])); ALTILI_AYAGI_EKSIK ×11 (ilk: ALTILI_AYAGI_EKSIK: 20261028-ELAZIG-A1 2. ayak (2. koşu): sonuç dosyasında koşu yok); ALTILI_ODEMESI_BULUNAMADI ×2 (ilk: ALTILI_ODEMESI_BULUNAMADI: 20261028-ELAZIG-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok) |
| 60 | 2026-10-28 | İstanbul | TAMAMLANDI |  |
| 61 | 2026-10-29 | Ankara | TAMAMLANDI |  |
| 62 | 2026-10-29 | İzmir | TAMAMLANDI |  |
| 63 | 2026-10-30 | Bursa | TAMAMLANDI |  |
| 64 | 2026-10-30 | İstanbul | TAMAMLANDI |  |
| 65 | 2026-10-31 | Ankara | TAMAMLANDI |  |
| 66 | 2026-10-31 | Diyarbakır | TAMAMLANDI |  |
| 67 | 2026-10-31 | İzmir | TAMAMLANDI |  |

## Dosyalar

| Tarih | Hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | Doğrulama |
|---|---|---|---|---|---|---|---|
| 2026-10-01 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-01/CSV/GunlukYarisProgrami/01.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 17421 | `6a85ef92fb5dc76abb242364d1bf81dcd4ba0a1623adfc1bac5a537781332ed9` | ICERIK_DOGRULANDI |
| 2026-10-01 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-01/CSV/GunlukYarisSonuclari/01.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 17025 | `4727c344eb4d286e51893e6d9db61264701113d07bfc104abc99c119b20a7345` | ICERIK_DOGRULANDI |
| 2026-10-01 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-01/CSV/GunlukYarisProgrami/01.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 18204 | `e0d4527913f691c46627372bd65689026896ea77a694e345c76a4aad7f8946e3` | ICERIK_DOGRULANDI |
| 2026-10-01 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-01/CSV/GunlukYarisSonuclari/01.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 17663 | `4fd2531138cb43a9d448a729ee4c9872db67603cf265585c8eb406418b0f4815` | ICERIK_DOGRULANDI |
| 2026-10-02 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-02/CSV/GunlukYarisProgrami/02.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 9473 | `26d08843d5ec4af1fc4c5fbc4f885f102177c662c98c80c183ccc08a52d0043a` | ICERIK_DOGRULANDI |
| 2026-10-02 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-02/CSV/GunlukYarisSonuclari/02.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 9050 | `44522bd6734cabdf29618bd51151f5511074f014922e7f755a59765fcf10e114` | ICERIK_DOGRULANDI |
| 2026-10-02 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-02/CSV/GunlukYarisProgrami/02.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 19937 | `199d354c2f58e87d8d85461f88c09e7dce662125189aea474dc9e2df33e034ee` | ICERIK_DOGRULANDI |
| 2026-10-02 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-02/CSV/GunlukYarisSonuclari/02.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 20325 | `dc569dcd17bd3fd5936e4cc1e30430d431f9232f65b5853b8818652776e1c059` | ICERIK_DOGRULANDI |
| 2026-10-03 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-03/CSV/GunlukYarisProgrami/03.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 17292 | `aceb73d73f22b4adf1e50252a1a1a0a533bdf28be59de685cfbd240f10dd0f81` | ICERIK_DOGRULANDI |
| 2026-10-03 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-03/CSV/GunlukYarisSonuclari/03.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 17421 | `eba2622a99b115e636c9b9d34286ac8ee9b894469a9a3daeef1c78c2a58bf500` | ICERIK_DOGRULANDI |
| 2026-10-03 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-03/CSV/GunlukYarisProgrami/03.10.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 13183 | `efd65e1440546eda491cd8d2c1252c3798a91791917251485762f228f5c56f7a` | ICERIK_DOGRULANDI |
| 2026-10-03 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-03/CSV/GunlukYarisSonuclari/03.10.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 12912 | `2ca08bbe9bd4d0157c09e1b265d8f776eeb44bd1fd0c56cc654629a29ba81b2f` | ICERIK_DOGRULANDI |
| 2026-10-03 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-03/CSV/GunlukYarisProgrami/03.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 19611 | `ae3a7002bb2fc8d4300b4f610a534d045272e302bce0f43e0b6fd995cb3f78c3` | ICERIK_DOGRULANDI |
| 2026-10-03 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-03/CSV/GunlukYarisSonuclari/03.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 19373 | `29eb730913c59f263db50972fb93a57f969ebef770eaeb6948fd5c148330c98d` | ICERIK_DOGRULANDI |
| 2026-10-04 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-04/CSV/GunlukYarisProgrami/04.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 17487 | `0add278e9f0bef4f3dbe8e0e47cfbae1a3c68763a932f6b3a0663d0b8e07cc75` | ICERIK_DOGRULANDI |
| 2026-10-04 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-04/CSV/GunlukYarisSonuclari/04.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 17284 | `9d4396afc118e11817ba1795c5dbe369aeab0eb531318246ccb4388dbbd69369` | ICERIK_DOGRULANDI |
| 2026-10-04 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-04/CSV/GunlukYarisProgrami/04.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 18819 | `1c343d2b8098440895ee6c041c81fb5aa870e4ad331e050f387df1fdf41ea9f3` | ICERIK_DOGRULANDI |
| 2026-10-04 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-04/CSV/GunlukYarisSonuclari/04.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 18453 | `6976753f3bdd392161f6fdab58f7b7ce02aa4c92a70bffb38277edbd84924945` | ICERIK_DOGRULANDI |
| 2026-10-05 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-05/CSV/GunlukYarisProgrami/05.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 19513 | `8b8bf1f41092d74f8ac1cd64ab87c8fe18ac5f44944fee6dabf69e78966e3db2` | ICERIK_DOGRULANDI |
| 2026-10-05 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-05/CSV/GunlukYarisSonuclari/05.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 19260 | `5c5ba17d77dbeb91e093f9a1eff025b58b708b5910702a1d21526fc76ea79927` | ICERIK_DOGRULANDI |
| 2026-10-05 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-05/CSV/GunlukYarisProgrami/05.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 20640 | `030159ba04f1a29cc3460d193abd4f24ac4a7cc64808e9d297ab3a0e11ae5e6a` | ICERIK_DOGRULANDI |
| 2026-10-05 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-05/CSV/GunlukYarisSonuclari/05.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 20652 | `54078ef5287df0f2fa5d68962b70ce40cc6ef0c1051faee2a2a984abfb54c310` | ICERIK_DOGRULANDI |
| 2026-10-06 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-06/CSV/GunlukYarisProgrami/06.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 17702 | `77f1ca26ec288e6635ed9ed23054852e754a0360b14b1838bcd0dff2443a2b75` | ICERIK_DOGRULANDI |
| 2026-10-06 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-06/CSV/GunlukYarisSonuclari/06.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 17687 | `9ebccda5f767cf452265c5e512475c719b584577f0e848c6c994a993aaf87515` | ICERIK_DOGRULANDI |
| 2026-10-06 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-06/CSV/GunlukYarisProgrami/06.10.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 17430 | `a0b1d1f8713e478fc3ddca0a4f8ecad95bbf5508bc4a5ea7de570cfe705648b9` | ICERIK_DOGRULANDI |
| 2026-10-06 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-06/CSV/GunlukYarisSonuclari/06.10.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 17326 | `354d26b8f607cb3a9d10a4c0ed592cffff7406a8fa5d59b7d4d471b9c436782e` | ICERIK_DOGRULANDI |
| 2026-10-07 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-07/CSV/GunlukYarisProgrami/07.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 17831 | `e70d71d0b262cd6a46cd96c268ec9fe7c79ad3e5fb11656f8ba3939c4d8493b0` | ICERIK_DOGRULANDI |
| 2026-10-07 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-07/CSV/GunlukYarisSonuclari/07.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 17665 | `937e628039df5ea04aaa0e5b9be31c655e9d32cf89e25bdaae0e75185f2b9f48` | ICERIK_DOGRULANDI |
| 2026-10-07 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-07/CSV/GunlukYarisProgrami/07.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 17382 | `6d1849b5ccfaa86aa42630a911039e1a7f986a327cf6eaacb30b9484b25101ca` | ICERIK_DOGRULANDI |
| 2026-10-07 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-07/CSV/GunlukYarisSonuclari/07.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 16952 | `c5d94232220f1200c465b7d55e1733fd3e91ab68ec475808abaae2880b3303ec` | ICERIK_DOGRULANDI |
| 2026-10-08 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-08/CSV/GunlukYarisProgrami/08.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 17703 | `840845dbc3400566489870213021d96b1bcd8bb612c63c8fddd251ff03248ceb` | ICERIK_DOGRULANDI |
| 2026-10-08 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-08/CSV/GunlukYarisSonuclari/08.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 17042 | `62b20a62a8d15f98d5ad9950c42f58a7a4bc1522bd957ce7323ca5627d8121b6` | ICERIK_DOGRULANDI |
| 2026-10-08 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-08/CSV/GunlukYarisProgrami/08.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 17945 | `f7d434df134c673c2b82f7d42f9f8163bfd908c3c1af2bf7930bc9088f2ad9e6` | ICERIK_DOGRULANDI |
| 2026-10-08 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-08/CSV/GunlukYarisSonuclari/08.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 17322 | `d9ba8c65e97d81ea571cdc75e686d66584a294ff94cf4b2602ece579723bcc52` | ICERIK_DOGRULANDI |
| 2026-10-09 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-09/CSV/GunlukYarisProgrami/09.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 17370 | `4754e556fbba3e002f31ce69dea32db8d6e744408bcbff7666833ccfccc54bde` | ICERIK_DOGRULANDI |
| 2026-10-09 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-09/CSV/GunlukYarisSonuclari/09.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 17072 | `49a5ce86ab15155b419afc00ffb56f36683f1c4e82099f5b0f8d3a5566ae26fb` | ICERIK_DOGRULANDI |
| 2026-10-09 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-09/CSV/GunlukYarisProgrami/09.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 21781 | `50f43c02b405b5199a9bf2247bbd772ff6e5cf441b88060c611503e708b545e2` | ICERIK_DOGRULANDI |
| 2026-10-09 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-09/CSV/GunlukYarisSonuclari/09.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 21385 | `116a7c074d2a6340eccee6032aa106cd3e6cc4337ba94db99cd9ec486cd00423` | ICERIK_DOGRULANDI |
| 2026-10-10 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-10/CSV/GunlukYarisProgrami/10.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 17520 | `3b51876803d972ab777fe70f34cb9056e252efa46d5cfd599a972e985dd98a30` | ICERIK_DOGRULANDI |
| 2026-10-10 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-10/CSV/GunlukYarisSonuclari/10.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 16990 | `083dcf652c745acb6dc8b6cb2def13194b07bb24d6f67c998172b217ceeeea70` | ICERIK_DOGRULANDI |
| 2026-10-10 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-10/CSV/GunlukYarisProgrami/10.10.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 13218 | `1069e6157c349e202c3b39e2a56b5c487fd1725796c2f70adf18aba05afde1cf` | ICERIK_DOGRULANDI |
| 2026-10-10 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-10/CSV/GunlukYarisSonuclari/10.10.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 12925 | `9c9090eebca10aff2707c547fc44fcab2d593ef9ac29c96786c0f782c86de197` | ICERIK_DOGRULANDI |
| 2026-10-10 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-10/CSV/GunlukYarisProgrami/10.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 17065 | `b34c67e827f0cf4c32a8cc34f918d7ebac36b4a5e32b5191489258a562d64650` | ICERIK_DOGRULANDI |
| 2026-10-10 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-10/CSV/GunlukYarisSonuclari/10.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16801 | `eb43e835daa5c80ab28ecb2a4d449a90a86b9e2428c48ab5b48380e0d35be95d` | ICERIK_DOGRULANDI |
| 2026-10-11 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-11/CSV/GunlukYarisProgrami/11.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 22880 | `d4cb5d37eb89fa420061b416340760ea4d938cf5503f450e9114c043bfe5728d` | ICERIK_DOGRULANDI |
| 2026-10-11 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-11/CSV/GunlukYarisSonuclari/11.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 23203 | `a8d3ce3125d08f656ea4b6a3815ac1f7fc2a280913708fc7b8218213b9d4e143` | ICERIK_DOGRULANDI |
| 2026-10-11 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-11/CSV/GunlukYarisProgrami/11.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16336 | `e0ebf5ff519724981c182e26a9c083c0a6b52a8fbd0747803eacd10fad6147d5` | ICERIK_DOGRULANDI |
| 2026-10-11 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-11/CSV/GunlukYarisSonuclari/11.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 15898 | `19c995d3a832a911a0727aea8fdec72a034b921ffbba35df3dbfe1241836970a` | ICERIK_DOGRULANDI |
| 2026-10-12 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-12/CSV/GunlukYarisProgrami/12.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 19377 | `c8b8d853b06bbc2bf692325c4c8220f33769661200c3c9ad96f5c811c2f1751c` | ICERIK_DOGRULANDI |
| 2026-10-12 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-12/CSV/GunlukYarisSonuclari/12.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 19239 | `ce7839eb9b8a9e516b3d7abd6a51e9ff3bfbc35e4e4bcb31a2c0efa41eb577f0` | ICERIK_DOGRULANDI |
| 2026-10-12 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-12/CSV/GunlukYarisProgrami/12.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 20791 | `dc88a14d9b3f8c354355330845f648179cfeb205283834b1ddc94b407299f076` | ICERIK_DOGRULANDI |
| 2026-10-12 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-12/CSV/GunlukYarisSonuclari/12.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 21083 | `c69f13bb5af9d0c3d6920b4e0d3ddf6cec65f3bd230c2a939e371758304c57ac` | ICERIK_DOGRULANDI |
| 2026-10-13 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-13/CSV/GunlukYarisProgrami/13.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 22251 | `e4c582a24f3c8fa448a3eab71cad22c7f0f59951477516ed01d11c9762c272b9` | ICERIK_DOGRULANDI |
| 2026-10-13 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-13/CSV/GunlukYarisSonuclari/13.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 22483 | `9e8b2beeaf4aed7f9536bacd7d2fab1681535e543597791d6c4c6a421d2f5824` | ICERIK_DOGRULANDI |
| 2026-10-13 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-13/CSV/GunlukYarisProgrami/13.10.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 18597 | `c48ffa3a62080559453ffc0be0b9aab24843fc5cdaad3f7602ceb1dcf397aea9` | ICERIK_DOGRULANDI |
| 2026-10-13 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-13/CSV/GunlukYarisSonuclari/13.10.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 18639 | `c31960df4c55deb782aa4fae9e45f5885ccf1e82108250032edc1671409aeb32` | ICERIK_DOGRULANDI |
| 2026-10-14 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-14/CSV/GunlukYarisProgrami/14.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 17801 | `9fcdbf64ead65741909458e248c871d70f11b80f3693498aa54ca637ef7acd0c` | ICERIK_DOGRULANDI |
| 2026-10-14 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-14/CSV/GunlukYarisSonuclari/14.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 17497 | `7357ab84e692d4b7c09a9a26b668f4c0eb491115ec46c604f21828213ec3addc` | ICERIK_DOGRULANDI |
| 2026-10-14 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-14/CSV/GunlukYarisProgrami/14.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16890 | `aba97707b2827b98912f5069f96effa8bdff6ae54cbf98e619c86bca44881816` | ICERIK_DOGRULANDI |
| 2026-10-14 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-14/CSV/GunlukYarisSonuclari/14.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 16634 | `091eac1f746a5efe25760113ddad6646ef687e27e64b04af0a732a7121a9aec1` | ICERIK_DOGRULANDI |
| 2026-10-15 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-15/CSV/GunlukYarisProgrami/15.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 19412 | `0cd8abe2cdd045f91f197b656141ed0e471ad842dfdaaeb36f3a240e68b96ccb` | ICERIK_DOGRULANDI |
| 2026-10-15 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-15/CSV/GunlukYarisSonuclari/15.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 19318 | `cd651c2159067048406ec8a11f75b6358e1a3634998c8bd0dbdb5bbce1243566` | ICERIK_DOGRULANDI |
| 2026-10-15 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-15/CSV/GunlukYarisProgrami/15.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16714 | `c4f42982be8e40f291677f513e7e675fdb692046641545d316c0d1c5c70e5ad9` | ICERIK_DOGRULANDI |
| 2026-10-15 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-15/CSV/GunlukYarisSonuclari/15.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16587 | `f7129f81c075e63f9430a49cb0dda2c9bd259984f735b2ef328a70727e40a1a6` | ICERIK_DOGRULANDI |
| 2026-10-16 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-16/CSV/GunlukYarisProgrami/16.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 11194 | `4cffcb634ce78de7e053cf9eab07c9a690e386b1079cd21d3389903083ee5469` | ICERIK_DOGRULANDI |
| 2026-10-16 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-16/CSV/GunlukYarisSonuclari/16.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 0 | `` | BICIM_HATASI |
| 2026-10-16 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-16/CSV/GunlukYarisProgrami/16.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16429 | `4719a99fee91d399b116ad3721098f6d675393195f44e2b5916f1a0641f8dfcf` | ICERIK_DOGRULANDI |
| 2026-10-16 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-16/CSV/GunlukYarisSonuclari/16.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 15974 | `1094ef68c05da3d125e6310ffbe9336eec428db249516e9fd1ebcc91f4588a7d` | ICERIK_DOGRULANDI |
| 2026-10-17 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-17/CSV/GunlukYarisProgrami/17.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 17032 | `320d48606f97c27d23612762cc1e29c145d338f21709d43def01f834bd742486` | ICERIK_DOGRULANDI |
| 2026-10-17 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-17/CSV/GunlukYarisSonuclari/17.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 16646 | `a287efe75329a57d4b728b3ad860a7e6e936df9fd19b60513bd7cfa3f67ceb29` | ICERIK_DOGRULANDI |
| 2026-10-17 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-17/CSV/GunlukYarisProgrami/17.10.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 12762 | `7cb194c244a1f0389479aa218507f3f0935f8357748c74d201fb1d11b908161a` | ICERIK_DOGRULANDI |
| 2026-10-17 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-17/CSV/GunlukYarisSonuclari/17.10.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 12368 | `b8e21dbbcf1be0ec2f9781a5326906169dff31f5f1d6ce6cebf95c2ee9eba94c` | ICERIK_DOGRULANDI |
| 2026-10-17 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-17/CSV/GunlukYarisProgrami/17.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 18239 | `e32a1d9dbe6fe99c4204e495c3394c4e1471b6d865c76104cf960d0c6d53c023` | ICERIK_DOGRULANDI |
| 2026-10-17 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-17/CSV/GunlukYarisSonuclari/17.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 18153 | `e8a284690bca4d7963ad6b34f02ace3180cf097e6fcf364f6ba180b5bc7088f6` | ICERIK_DOGRULANDI |
| 2026-10-18 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-18/CSV/GunlukYarisProgrami/18.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 16898 | `3370eae644728ba79190c00dc73fea518e60f0a284ecb69a7b5d47b6163782d1` | ICERIK_DOGRULANDI |
| 2026-10-18 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-18/CSV/GunlukYarisSonuclari/18.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 16986 | `3b6ab69d39c64cb6b1c8196ff4d230f0b5ddf3cd181d191649d19518fe67ed89` | ICERIK_DOGRULANDI |
| 2026-10-18 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-18/CSV/GunlukYarisProgrami/18.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 15093 | `e8da1db27e8e46e33651a3ef509185925111a1b87ea5ea97143ceae0e262d09f` | ICERIK_DOGRULANDI |
| 2026-10-18 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-18/CSV/GunlukYarisSonuclari/18.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 14721 | `dd57f743bde2fc9ff89b17650748fcccaa77ef0fac8b5f8c8a10caeb385db36c` | ICERIK_DOGRULANDI |
| 2026-10-19 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-19/CSV/GunlukYarisProgrami/19.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 13689 | `365ce913124fde13e29bc69bd200a28ac192f42e1b0db5f54e493edf62c6ee50` | ICERIK_DOGRULANDI |
| 2026-10-19 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-19/CSV/GunlukYarisSonuclari/19.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 0 | `` | BICIM_HATASI |
| 2026-10-19 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-19/CSV/GunlukYarisProgrami/19.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 20687 | `b0000d5300ca9615c4c0ca36cedcd0418051adbc1de14c9fe70aa2e84271a8a6` | ICERIK_DOGRULANDI |
| 2026-10-19 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-19/CSV/GunlukYarisSonuclari/19.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 20818 | `251190ccc3b42d9c187af75a69ba7280fece87aa1cb24613aa71cfd751472f7e` | ICERIK_DOGRULANDI |
| 2026-10-20 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-20/CSV/GunlukYarisProgrami/20.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 18494 | `d45cb028b8ea8f73488031a415807fecee073b8929dc105f8aa24cf5d05ebc93` | ICERIK_DOGRULANDI |
| 2026-10-20 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-20/CSV/GunlukYarisSonuclari/20.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 18472 | `cdc9aba916897a161f24a4144e8572e591f7cb7655ec05e6a36126e4565a8abc` | ICERIK_DOGRULANDI |
| 2026-10-20 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-20/CSV/GunlukYarisProgrami/20.10.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 17492 | `cd01f996fddfcbf2834a2679c324a3a160e60079b8c31abd1d1c7dee435055ba` | ICERIK_DOGRULANDI |
| 2026-10-20 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-20/CSV/GunlukYarisSonuclari/20.10.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 17281 | `16963a4c73e74def594776e6ec75ba591b7045691a41e146aa2a52a581e04a23` | ICERIK_DOGRULANDI |
| 2026-10-21 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-21/CSV/GunlukYarisProgrami/21.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 13996 | `7cc4715937f854904286baa5abc6ae2f7c8bedb39d70ceee126875727d03e97f` | ICERIK_DOGRULANDI |
| 2026-10-21 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-21/CSV/GunlukYarisSonuclari/21.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 13845 | `72fff003f03885b529a0e9ad1e5a5c4fc298c4709cc58d7416a6a70c1b8aa7d9` | ICERIK_DOGRULANDI |
| 2026-10-21 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-21/CSV/GunlukYarisProgrami/21.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 21361 | `5a058a42745ba06b9a04e882a117ddbdc64a38329000c1ada9351ec5384d90b9` | ICERIK_DOGRULANDI |
| 2026-10-21 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-21/CSV/GunlukYarisSonuclari/21.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 21011 | `5c0137ea71e1e920e3dbc0e0c37399e75d289aca765c9f9ab28d0be5b774282b` | ICERIK_DOGRULANDI |
| 2026-10-22 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-22/CSV/GunlukYarisProgrami/22.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 16695 | `5081c0a129673f7994f9857509314bf578a3f82b38620d3a70574a35b5470d2c` | ICERIK_DOGRULANDI |
| 2026-10-22 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-22/CSV/GunlukYarisSonuclari/22.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 16206 | `3a8646cf3582539d7aa95b2a3ad84ec43add55b7ee51174b1f4fafa3af957d18` | ICERIK_DOGRULANDI |
| 2026-10-22 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-22/CSV/GunlukYarisProgrami/22.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16719 | `1ac95a1788cbde94e8b43b49abe070623b0b3b17c91dd848cc1bba691c25f830` | ICERIK_DOGRULANDI |
| 2026-10-22 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-22/CSV/GunlukYarisSonuclari/22.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16597 | `7a4679ae702bf18dff5f8d289c558ae1ccc9279436c74375bc94b98da945fe51` | ICERIK_DOGRULANDI |
| 2026-10-23 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-23/CSV/GunlukYarisProgrami/23.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 17145 | `97df9b039a2b082bc80da3e3009b56e81fe32bfa19a8053ae80306532b882c66` | ICERIK_DOGRULANDI |
| 2026-10-23 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-23/CSV/GunlukYarisSonuclari/23.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 16964 | `be0019c72458e3a878a5febee04c77a42b96a8704bc0c11e61a29c2e7df8ce70` | ICERIK_DOGRULANDI |
| 2026-10-23 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-23/CSV/GunlukYarisProgrami/23.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 20404 | `a32451d4f7fab0d3c27bc32bdf64748a52184ea5ed51d12d35ba035cc74caed4` | ICERIK_DOGRULANDI |
| 2026-10-23 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-23/CSV/GunlukYarisSonuclari/23.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 20152 | `d44d10c86e42a6d355795249b97478c9246da227e7db9c0a696185a483876188` | ICERIK_DOGRULANDI |
| 2026-10-24 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-24/CSV/GunlukYarisProgrami/24.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 19341 | `2847d2d502d82cc09a965dfb90b535f3850de730d5807f029a176e37cb4abb2a` | ICERIK_DOGRULANDI |
| 2026-10-24 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-24/CSV/GunlukYarisSonuclari/24.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 18803 | `b3cfc1904fce31136c0784b81489edd9d6bb50a5a73ef26e5d87447359c93d08` | ICERIK_DOGRULANDI |
| 2026-10-24 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-24/CSV/GunlukYarisProgrami/24.10.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 13897 | `eea18b294f2bc78bc5a301fc92e616d7097450ca6157752e0125d5a315ed29b3` | ICERIK_DOGRULANDI |
| 2026-10-24 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-24/CSV/GunlukYarisSonuclari/24.10.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 13669 | `b9bf981c06fb624a1e8cc6a53b9b3964455e5871f3d50aed0ba9c19ae8c0704d` | ICERIK_DOGRULANDI |
| 2026-10-24 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-24/CSV/GunlukYarisProgrami/24.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 21321 | `1b2123847db5f81a8f526a422623c01f159acc67acb716401ce4b77beeae5cee` | ICERIK_DOGRULANDI |
| 2026-10-24 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-24/CSV/GunlukYarisSonuclari/24.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 20737 | `17251ae08f5fda60bca3872ba44abdd149a6c373180e4cb4cdea7e3258facaeb` | ICERIK_DOGRULANDI |
| 2026-10-25 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-25/CSV/GunlukYarisProgrami/25.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 22064 | `7b965297f5ddd91badf83cff8bb30ccd4253a23aa97fb8bd3fef5be4336bc1b9` | ICERIK_DOGRULANDI |
| 2026-10-25 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-25/CSV/GunlukYarisSonuclari/25.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 22325 | `1c1ef508b5ff12b65a26f1ca4939789f0411ea3b4415922e89088cedd95c1bfb` | ICERIK_DOGRULANDI |
| 2026-10-25 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-25/CSV/GunlukYarisProgrami/25.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16231 | `928c29a0d367235c58fd96d566a8002888e72d3b32df88e7ef86800c00f0dbb3` | ICERIK_DOGRULANDI |
| 2026-10-25 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-25/CSV/GunlukYarisSonuclari/25.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 15860 | `f105cc64a3caf5e457d9de40daedf2ea7d72f22945ec2c8e1fb9a318a4c9ba19` | ICERIK_DOGRULANDI |
| 2026-10-26 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-26/CSV/GunlukYarisProgrami/26.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 12953 | `3e93d6acc4324847db1cdbb2359064a41b29a2b74f318736c63b5869b9558715` | ICERIK_DOGRULANDI |
| 2026-10-26 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-26/CSV/GunlukYarisSonuclari/26.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 12738 | `a78865210cb82ba44c81a536d6db1d9b73bd85c72f2c4dd0d443af4ebe2b110e` | ICERIK_DOGRULANDI |
| 2026-10-26 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-26/CSV/GunlukYarisProgrami/26.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 21992 | `6a66253c6b4ca432af642f691ffc6f7067032775fb4be6633a8dae0df49b3adf` | ICERIK_DOGRULANDI |
| 2026-10-26 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-26/CSV/GunlukYarisSonuclari/26.10.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 22240 | `acd49e687727a2e12d75814708ea3291236cb1f60a7a6acab7feccdc3e4fdc78` | ICERIK_DOGRULANDI |
| 2026-10-27 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-27/CSV/GunlukYarisProgrami/27.10.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 20548 | `2f17d711f7dfdb1fa6220924879954e6141eb6d357434c5e331cb404e90a9ce1` | ICERIK_DOGRULANDI |
| 2026-10-27 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-27/CSV/GunlukYarisSonuclari/27.10.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 20667 | `bfc6bf745cfad04e8c248560dfceba3f53403198024a210384d75d36ceaa051e` | ICERIK_DOGRULANDI |
| 2026-10-27 | Kocaeli | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-27/CSV/GunlukYarisProgrami/27.10.2026-Kocaeli-GunlukYarisProgrami-TR.csv | 200 | 16366 | `c58bbb41fe240d0ad32a8fbd4c4483c2bc68f42c05c44bd35c5f8b84eb41a8ed` | ICERIK_DOGRULANDI |
| 2026-10-27 | Kocaeli | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-27/CSV/GunlukYarisSonuclari/27.10.2026-Kocaeli-GunlukYarisSonuclari-TR.csv | 200 | 16010 | `0596fbf9601b887e4c50dd188ba3ce5c576c7b62e3e55a845ddef9ccd2f71e62` | ICERIK_DOGRULANDI |
| 2026-10-28 | Elazığ | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-28/CSV/GunlukYarisProgrami/28.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisProgrami-TR.csv | 200 | 15885 | `9dcc1da8bf0e26f93e379fcab9fbdb33435a4f5e02f841c4f921a48cba75b0f8` | ICERIK_DOGRULANDI |
| 2026-10-28 | Elazığ | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-28/CSV/GunlukYarisSonuclari/28.10.2026-Elaz%C4%B1%C4%9F-GunlukYarisSonuclari-TR.csv | 200 | 1592 | `82007f66ef1409f67f95d3e88ca56dbd5308b4eb40342b5ba488090ed313f8b3` | ICERIK_DOGRULANDI |
| 2026-10-28 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-28/CSV/GunlukYarisProgrami/28.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 16947 | `5b7acf61d825f28b4b3ed33e0d6df58811f4bdcc999116c1c3c8165bae462a9f` | ICERIK_DOGRULANDI |
| 2026-10-28 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-28/CSV/GunlukYarisSonuclari/28.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 16632 | `3712c0d7378626314eefddd86bccfa72d94ac6b4c316664c45899d8f6b558ad6` | ICERIK_DOGRULANDI |
| 2026-10-29 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-29/CSV/GunlukYarisProgrami/29.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 15729 | `dbdcf4082eb567e8c2b5b1dabd92360b45abbbfe5497afa5cc213b4d70bae023` | ICERIK_DOGRULANDI |
| 2026-10-29 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-29/CSV/GunlukYarisSonuclari/29.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 15335 | `77e2b5f12b9dcb2a9895ff5b31ed470cb39cf3a32e432e4b82b1e00198efff85` | ICERIK_DOGRULANDI |
| 2026-10-29 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-29/CSV/GunlukYarisProgrami/29.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 19598 | `74623faea1c95d8402804228f4760cf1fe8c87209f3b8587f184bd9881e8f7e6` | ICERIK_DOGRULANDI |
| 2026-10-29 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-29/CSV/GunlukYarisSonuclari/29.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 19334 | `fe195b199c0d082eb04017ca7d077e0344c6a911d79856bdc77c95ef2bf85ba0` | ICERIK_DOGRULANDI |
| 2026-10-30 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-30/CSV/GunlukYarisProgrami/30.10.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 15355 | `b1bee731dcf444d868bc218b572f43f08faa033a269ac82907944a3c7a07a419` | ICERIK_DOGRULANDI |
| 2026-10-30 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-30/CSV/GunlukYarisSonuclari/30.10.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 15080 | `2cd4a7afaf44de7e89e71423cd1e824d1702fb4fa78d7cc6f7dbfeeaf2bc61f3` | ICERIK_DOGRULANDI |
| 2026-10-30 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-30/CSV/GunlukYarisProgrami/30.10.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 20985 | `5c808c3e97721da88671c1960e44ac8c0a2a724f80691a8517839992e9f61d07` | ICERIK_DOGRULANDI |
| 2026-10-30 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-30/CSV/GunlukYarisSonuclari/30.10.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 20512 | `9ccca19c50953e96402d5a14982ab42fdb321b840b963bbad02a55b3ddff4b1f` | ICERIK_DOGRULANDI |
| 2026-10-31 | Ankara | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-31/CSV/GunlukYarisProgrami/31.10.2026-Ankara-GunlukYarisProgrami-TR.csv | 200 | 20487 | `639ef8cd8225a7949b7b2f30148c1edb8accdc77445b683cf6039f3ec00f2cc6` | ICERIK_DOGRULANDI |
| 2026-10-31 | Ankara | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-31/CSV/GunlukYarisSonuclari/31.10.2026-Ankara-GunlukYarisSonuclari-TR.csv | 200 | 20249 | `de80a216e0968c73f3fc852aab8b1d8f307432b39adf64921d4e8c263408081a` | ICERIK_DOGRULANDI |
| 2026-10-31 | Diyarbakır | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-31/CSV/GunlukYarisProgrami/31.10.2026-Diyarbak%C4%B1r-GunlukYarisProgrami-TR.csv | 200 | 12008 | `895ad18dfef8fc43a8ddef6eba3928807c31137116d20dafde442496adb2de65` | ICERIK_DOGRULANDI |
| 2026-10-31 | Diyarbakır | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-31/CSV/GunlukYarisSonuclari/31.10.2026-Diyarbak%C4%B1r-GunlukYarisSonuclari-TR.csv | 200 | 11634 | `24193a26c7a45861f042a0eb6e54301d1c6bbcc8eec90a721e06fb535d996488` | ICERIK_DOGRULANDI |
| 2026-10-31 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-31/CSV/GunlukYarisProgrami/31.10.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 17160 | `c88fc56cebfe8251eccb9e91f62a7765236774086b6b2a5594b8f00fe958fbcb` | ICERIK_DOGRULANDI |
| 2026-10-31 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-10-31/CSV/GunlukYarisSonuclari/31.10.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16824 | `fe148c35929626043a299d43f693e805fce3b5b1286634005f5bb27e065b3e52` | ICERIK_DOGRULANDI |

Doğrulanan ham CSV'ler `/home/user/TJK/kontrol/ham/2026-10` altında (`yerel_dosya` adlarıyla). Ayrıntılı kayıt: `/home/user/TJK/kontrol/yillik/listeler/2026-10_durum.tsv`.
