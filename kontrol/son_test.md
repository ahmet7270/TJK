# Son veri testi

- **Durum:** TAMAMLANDI (liste sonuna kadar işlendi)
- **Çalışma:** Mart 2026 toplu indirme, `mart_2026_kalan_102.tsv` (01.03.2026 Adana önceden doğrulandığı için atlandı)
- **Araçlar:** `tjk-fetch toplu` (curl ile HTTPS; içerik doğrulaması `tjk_ayristirici/src/ayristir.rs` ile), birlikte ayrıştırma `tjk_ayristirici` 0.11.0
- **SQLite:** kullanılmadı
- **Ön kontrol:** 50 çift, 100 satır, 100 benzersiz URL; her çiftte bir program ve bir sonuç
- **robots.txt:** HTTP 404: robots.txt yok, kural tanımlı değil (RFC 9309: izinli)

## Özet (bu çalışmadaki 50 çift)

| Durum | Çift |
|---|---|
| Tamamlandı | 46 |
| İnceleme gerekli | 4 |
| Eksik | 0 |
| Henüz denenmedi | 0 |
| **Toplam** | **50** |

Önceki çalışmadan SHA-256'sı eşleşerek yeniden kullanılan dosya: 99

## Çiftler

| # | Tarih | Hipodrom | Durum | Gerekçe |
|---|---|---|---|---|
| 1 | 2026-03-01 | İzmir | TAMAMLANDI |  |
| 2 | 2026-03-02 | Bursa | TAMAMLANDI |  |
| 3 | 2026-03-02 | Şanlıurfa | TAMAMLANDI |  |
| 4 | 2026-03-03 | Adana | TAMAMLANDI |  |
| 5 | 2026-03-03 | Antalya | TAMAMLANDI |  |
| 6 | 2026-03-04 | İstanbul | TAMAMLANDI |  |
| 7 | 2026-03-04 | Şanlıurfa | TAMAMLANDI |  |
| 8 | 2026-03-05 | Antalya | TAMAMLANDI |  |
| 9 | 2026-03-05 | İzmir | TAMAMLANDI |  |
| 10 | 2026-03-11 | İstanbul | TAMAMLANDI |  |
| 11 | 2026-03-11 | Şanlıurfa | TAMAMLANDI |  |
| 12 | 2026-03-12 | Antalya | TAMAMLANDI |  |
| 13 | 2026-03-12 | İzmir | TAMAMLANDI |  |
| 14 | 2026-03-13 | Antalya | TAMAMLANDI |  |
| 15 | 2026-03-13 | İzmir | TAMAMLANDI |  |
| 16 | 2026-03-14 | Adana | TAMAMLANDI |  |
| 17 | 2026-03-14 | İstanbul | TAMAMLANDI |  |
| 18 | 2026-03-15 | Adana | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: ESLESMEYEN_PROGRAM_KAYDI ×61 (ilk: ESLESMEYEN_PROGRAM_KAYDI: 20260315-ADANA-K03-A01 ATAKLI KIZ (aday satırlar: [])); EKURI_GRUPLARI_CELISIYOR ×4 (ilk: EKURI_GRUPLARI_CELISIYOR: 3. koşu program [[2, 11]] sonuç [] (üye numaraları, sıradan bağımsız)); BIRINCI_BULUNAMADI ×6 (ilk: BIRINCI_BULUNAMADI: 3. koşu (GANYAN [])); ALTILI_AYAGI_EKSIK ×10 (ilk: ALTILI_AYAGI_EKSIK: 20260315-ADANA-A1 3. ayak (3. koşu): sonuç dosyasında koşu yok); ALTILI_ODEMESI_BULUNAMADI ×2 (ilk: ALTILI_ODEMESI_BULUNAMADI: 20260315-ADANA-A1 son ayak (6. koşu) bahis satırında '1. 6'LI GANYAN' yok) |
| 19 | 2026-03-15 | İzmir | TAMAMLANDI |  |
| 20 | 2026-03-16 | Bursa | TAMAMLANDI |  |
| 21 | 2026-03-16 | Şanlıurfa | TAMAMLANDI |  |
| 22 | 2026-03-17 | Adana | TAMAMLANDI |  |
| 23 | 2026-03-17 | Antalya | TAMAMLANDI |  |
| 24 | 2026-03-18 | İstanbul | TAMAMLANDI |  |
| 25 | 2026-03-18 | Şanlıurfa | TAMAMLANDI |  |
| 26 | 2026-03-19 | Antalya | TAMAMLANDI |  |
| 27 | 2026-03-19 | İzmir | TAMAMLANDI |  |
| 28 | 2026-03-20 | İzmir | TAMAMLANDI |  |
| 29 | 2026-03-21 | Adana | INCELEME_GEREKLI | sonuc: BICIM_HATASI (dosya boş) |
| 30 | 2026-03-21 | İstanbul | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: EKURI_UYESI_DOGRULANAMADI ×2 (ilk: EKURI_UYESI_DOGRULANAMADI: 20260321-ISTANBUL-program.csv satır 113 (6. koşu): (6)LEONİDAS — programda 6 numara: LEONİDAS SKG SK YP) |
| 31 | 2026-03-22 | Adana | TAMAMLANDI |  |
| 32 | 2026-03-22 | İzmir | TAMAMLANDI |  |
| 33 | 2026-03-23 | Bursa | TAMAMLANDI |  |
| 34 | 2026-03-23 | Şanlıurfa | TAMAMLANDI |  |
| 35 | 2026-03-24 | Adana | TAMAMLANDI |  |
| 36 | 2026-03-24 | Antalya | TAMAMLANDI |  |
| 37 | 2026-03-25 | İstanbul | TAMAMLANDI |  |
| 38 | 2026-03-25 | Şanlıurfa | TAMAMLANDI |  |
| 39 | 2026-03-26 | Antalya | TAMAMLANDI |  |
| 40 | 2026-03-26 | İzmir | TAMAMLANDI |  |
| 41 | 2026-03-27 | Antalya | TAMAMLANDI |  |
| 42 | 2026-03-27 | İzmir | TAMAMLANDI |  |
| 43 | 2026-03-28 | Adana | TAMAMLANDI |  |
| 44 | 2026-03-28 | İstanbul | TAMAMLANDI |  |
| 45 | 2026-03-29 | Adana | TAMAMLANDI |  |
| 46 | 2026-03-29 | İzmir | TAMAMLANDI |  |
| 47 | 2026-03-30 | Bursa | TAMAMLANDI |  |
| 48 | 2026-03-30 | Şanlıurfa | TAMAMLANDI |  |
| 49 | 2026-03-31 | Adana | TAMAMLANDI |  |
| 50 | 2026-03-31 | Antalya | INCELEME_GEREKLI | ayrıştırıcı gün durumu INCELEME_GEREKLI: GANYAN_BIRINCI_CELISKISI ×1 (ilk: GANYAN_BIRINCI_CELISKISI: 8. koşu GANYAN [5, 6], sonuçta birinci [5]) |

## Dosyalar

| Tarih | Hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | Doğrulama |
|---|---|---|---|---|---|---|---|
| 2026-03-01 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-01/CSV/GunlukYarisProgrami/01.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 13932 | `6b068eaf437417959dc45de41f8df2b682c952a7a69c6c5ee1d90bf32fe00dea` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-01 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-01/CSV/GunlukYarisSonuclari/01.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 14401 | `de762c958ea99140c3774b6a2068f27e831241f571e07d3bf49d505bdeed473d` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-02 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-02/CSV/GunlukYarisProgrami/02.03.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 14957 | `c5f305fb0104e7abc6d178fd1547764191b1738554c5f00884a1c7bf9d7d4c35` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-02 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-02/CSV/GunlukYarisSonuclari/02.03.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 15397 | `dfa2030343b35fd15f5371b33b69f0b7f8cb1a7dc929bed9850351c56ce4df88` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-02 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-02/CSV/GunlukYarisProgrami/02.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 17632 | `9caea30e941291f4f5341a1664e269c8e5b684ff20ecae638d86a8e0e08a191f` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-02 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-02/CSV/GunlukYarisSonuclari/02.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 18169 | `f70f0a28976035f688febea5e66551fe5367bf97e2573782e0dd834605133536` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-03 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-03/CSV/GunlukYarisProgrami/03.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15795 | `233f008fc952435cfe01819707d328ec2693c349dadcc189ccd1682015dbc894` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-03 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-03/CSV/GunlukYarisSonuclari/03.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 16036 | `a22db5d78d9c252412f2151965cd3f980fc1bfc183999239e56dac8806c4454f` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-03 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-03/CSV/GunlukYarisProgrami/03.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 12645 | `09b62fd4742a5fdb811441f38011df802260d935c401e73c522f71b9ece504cf` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-03 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-03/CSV/GunlukYarisSonuclari/03.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 13253 | `a3a7f5e55ac45b45ef783289c0c0107efb6016ff5a8c49940eed8ed672e9cc0a` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-04 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-04/CSV/GunlukYarisProgrami/04.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 14928 | `55af06ceee40dc247a1a276326e6c63fc1f60e5d65c21fdc6b15918dfe888dde` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-04 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-04/CSV/GunlukYarisSonuclari/04.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 15131 | `3aacb532791469780e5317686b592cb558f5a3d4d225d40a4e0da6920ae4ca99` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-04 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-04/CSV/GunlukYarisProgrami/04.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 18426 | `ccfdfa97314138579498391e513767655a9621792805d41b27d2d829363d645f` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-04 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-04/CSV/GunlukYarisSonuclari/04.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 18810 | `c2b54bba7d15b1ff1ce897f8c359f2268a088924478a80fdf1853162fa5ff493` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-05 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-05/CSV/GunlukYarisProgrami/05.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 15256 | `32be6a9ecd65431b12a309d6625db985e80ad6ad6e41f89363445f367b482fc3` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-05 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-05/CSV/GunlukYarisSonuclari/05.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15792 | `97ec5bd1d393050d832d362dc07e38ad95b0d77799dfc3ab3eba359e4bda1d7a` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-05 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-05/CSV/GunlukYarisProgrami/05.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 14874 | `e3511d22b034fa1c44bb0e94953aa2aee4134d3ff1941a3f560e14e326010fb8` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-05 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-05/CSV/GunlukYarisSonuclari/05.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 15082 | `d25edfff657de93dcc284306c1e757428a830e81a44a83a9854cf8afa5105976` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-11 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-11/CSV/GunlukYarisProgrami/11.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 19575 | `78acb3f5d8ba68859bc95ef4f6ba8a92a8d02f7937b5c561abd84862de40fa71` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-11 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-11/CSV/GunlukYarisSonuclari/11.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 20223 | `ca3d82a62453d03bb0be5dc6cf6e345e96395c43b04253fc8aa12b45bf19d079` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-11 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-11/CSV/GunlukYarisProgrami/11.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 18504 | `1c92d1d2b6efdf63d098d95c33e369ed44ce83fcdf2968a10baae63a6aeeb698` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-11 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-11/CSV/GunlukYarisSonuclari/11.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 19022 | `74c74d633a01318a76d4bde5f7c1201eca788a10aba2551e64e52d5d256ab252` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-12 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-12/CSV/GunlukYarisProgrami/12.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 12006 | `555a4ee628d06d82611bd150aa336ab84c53b631b8f3ac141464484a89908f4a` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-12 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-12/CSV/GunlukYarisSonuclari/12.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 12278 | `2182d6328607ae9cc38b7570401612f663fd070c19cea3990b16dfaec3b46419` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-12 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-12/CSV/GunlukYarisProgrami/12.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 10388 | `6646085781ecadb71a7a959e63f19140acc7308980cd9b3e5fbbb7c304c3f4c4` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-12 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-12/CSV/GunlukYarisSonuclari/12.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 10425 | `a682fbde55569978d431567c4b5226e5e65b837d2b85ae8489e740f8be0d9e50` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-13 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-13/CSV/GunlukYarisProgrami/13.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 15619 | `eb04cd1a643d8a01617525a70a7736a61a23fa10ebbb2e1cae98e305b6c9b179` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-13 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-13/CSV/GunlukYarisSonuclari/13.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15977 | `22ef0378a5d1cc10c0d40bf5b68b2b530c8757d64dc6dae759937e715fafb730` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-13 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-13/CSV/GunlukYarisProgrami/13.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 10732 | `568121246131722745dd59121bed1c42d508566bda9bbbfc24f3e5ad47eada28` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-13 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-13/CSV/GunlukYarisSonuclari/13.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 10987 | `447d8ad8042422dabed816d9704d19585d7f43d6b9e74dd65b11c4811080efd8` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-14 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-14/CSV/GunlukYarisProgrami/14.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 16141 | `3f33a1c0dcd9c9c92f6b56b2551007dd2a86c84991ea29bd19f8b3685e347a42` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-14 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-14/CSV/GunlukYarisSonuclari/14.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 16614 | `06568acf2078a3c9d5cf8779df99bf3b16bc8b30fc032a0d8583320d9378fe46` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-14 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-14/CSV/GunlukYarisProgrami/14.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 12356 | `4dd2364dbdf462b85850032fdd27d70d82715bf5d259e947f0d0705ef57a18e9` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-14 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-14/CSV/GunlukYarisSonuclari/14.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 12596 | `1f9a860e24ba997b63ddfa26790fa1aaae4055637e1b155d477c19ae8335896e` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-15 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-15/CSV/GunlukYarisProgrami/15.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15803 | `78b0403a6190a0e3ab52844cc10ecb9afa214c33e7160a3a788483f691f0e008` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-15 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-15/CSV/GunlukYarisSonuclari/15.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 3178 | `038eb81a9d6d5dbd62a8d44a2a431e8a2841962db45570c677c111db6e10427e` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-15 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-15/CSV/GunlukYarisProgrami/15.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16085 | `22f229b6a2e3d8158460bd20999ab4b02ae35ccc964af9e680e092f60643772a` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-15 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-15/CSV/GunlukYarisSonuclari/15.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 16341 | `a76f6dab14b79110057badc281c23d960125cacba4c743872467f0f99e57e71e` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-16 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-16/CSV/GunlukYarisProgrami/16.03.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 17052 | `cd601010f8c49c13c30f0c6393d44af63c8857ed2fee56f85fb572f6bbcc0c1e` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-16 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-16/CSV/GunlukYarisSonuclari/16.03.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 17701 | `7018e786a3ecaf6c2bc84b7398b5d539edd37a1bbeaa4ed4f08d1ca359a5f150` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-16 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-16/CSV/GunlukYarisProgrami/16.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 19892 | `04c7ef43854fe6567ba5943aeb5fae0c7445c6b081d3989742b0dc493aa4219f` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-16 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-16/CSV/GunlukYarisSonuclari/16.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 20679 | `ac8148434e4b1b97ee895fd0ab044a667d56c851b46a29961b061901fb6f694f` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-17 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-17/CSV/GunlukYarisProgrami/17.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 14893 | `d2deb0e1c93e2b3cab4301ccf4567f81adbe8830ea8c8cc737930704d453ed47` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-17 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-17/CSV/GunlukYarisSonuclari/17.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 15206 | `b649912d9459a6986d8fafb1797c743a1677349e847e4aad7938f05ff56b9444` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-17 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-17/CSV/GunlukYarisProgrami/17.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 11659 | `0e48097d33127dc9645022b080b91f607095c75e876a57f6ab4f06a63fb3f626` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-17 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-17/CSV/GunlukYarisSonuclari/17.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 12164 | `432dc530aad47be09c0ab2c32b73851c0bc8e53cbaad6171623f5ac0db93ce87` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-18 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-18/CSV/GunlukYarisProgrami/18.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 12545 | `484ac60bb871e8cadb4da44723da02c49508c6f38ccb8cd019bbb60d126b7daa` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-18 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-18/CSV/GunlukYarisSonuclari/18.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 12882 | `f17358db6b5ffd7832cedecd681efeb8f9f110a73b84693d6b28837cfdac5fb0` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-18 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-18/CSV/GunlukYarisProgrami/18.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 13808 | `4194355ff3feba525e345d0de7587758b2d0427bad1452696b0377a417445344` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-18 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-18/CSV/GunlukYarisSonuclari/18.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 14379 | `934742fbacd8b038fa3c30ac7c00490397082fdf4c4de48fadc27268712cb9b9` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-19 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-19/CSV/GunlukYarisProgrami/19.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 14920 | `5271e7543a53beced219840fd625bdb0eec3ed5b4d50cb77006ddb591fb30302` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-19 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-19/CSV/GunlukYarisSonuclari/19.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15372 | `e40174b76ac80283c806fd96d838eae555eba107f80ae7f28f5856a30614dbb8` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-19 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-19/CSV/GunlukYarisProgrami/19.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 12342 | `f0f6c6d74737c136ba390878988028f0b62df1c311b5abf2c97684c6c5aab1a0` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-19 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-19/CSV/GunlukYarisSonuclari/19.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 12585 | `aa350f2b4fe2ea9d812c29d30f5292e4d8b16d80edb5e6b2acd8eff06bb9e933` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-20 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-20/CSV/GunlukYarisProgrami/20.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 16773 | `3d00df970b44a8763add529dff75fcec73fde3c9e3006cff26e16e8e22736ea4` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-20 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-20/CSV/GunlukYarisSonuclari/20.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 17227 | `bfcd034e56e3a04c304309c0f84a333b30062439dfe4730b4c17893df6d0533d` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-21 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisProgrami/21.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 17685 | `de660aa8547daa74529d915bd42b14d58461b88652b31f3cd947d62eef35f26d` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-21 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisSonuclari/21.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 0 | `` | BICIM_HATASI |
| 2026-03-21 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisProgrami/21.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 19584 | `e9163623cebb0f340bf763f432b075efa235ed9b7a9c2ce0056b04b5722c8f42` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-21 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-21/CSV/GunlukYarisSonuclari/21.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 19861 | `9a1092a745700e672f2403be57c7126eca72bd0bcc589e99b7a82667a31410e3` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-22 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-22/CSV/GunlukYarisProgrami/22.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 14143 | `32779a3346a68ac198d53b1eb696a977717a71e878b2830d2708957ea4c57d0b` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-22 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-22/CSV/GunlukYarisSonuclari/22.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 14247 | `60c0b9d9ef29178d2e36ef444e269890907ad77339e83bf86516f51c0be9a592` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-22 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-22/CSV/GunlukYarisProgrami/22.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 13446 | `e0eae1d2f1cb9af78d0d084b7485ebcd6bee133d3d1b8c27d5132457f0a2cabb` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-22 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-22/CSV/GunlukYarisSonuclari/22.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 13634 | `2ea1fe4c908db7cf4bb1ed8e4c2a915b1c6a6041ac0b1de132b6063e640d4c88` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-23 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-23/CSV/GunlukYarisProgrami/23.03.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 12701 | `78ece0ec758916e60f81394203b37d326c5388c72880a50da6243d3fe202240d` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-23 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-23/CSV/GunlukYarisSonuclari/23.03.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 13090 | `cd3992e937748f92b5e938c046dbf764fb23cf2f3c41fde68817a443e026ab16` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-23 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-23/CSV/GunlukYarisProgrami/23.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 20666 | `de9fca8da30ca7c368070cc281d5edc4611146a03bb49cf956891928c6e37807` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-23 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-23/CSV/GunlukYarisSonuclari/23.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 21420 | `57499d5a342273fcdea6fab8bfd49e7031b001f8ba69b1ceb3971c101b060fcf` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-24 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-24/CSV/GunlukYarisProgrami/24.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 12927 | `4c23ac0eedbff1b4831904a8ada2e88493aea79e3c8f2cdd912c34c10a385d99` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-24 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-24/CSV/GunlukYarisSonuclari/24.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 13109 | `724b99cdba0136716083ff9b4346713098b7dc66952850af89cdbab3568d8145` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-24 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-24/CSV/GunlukYarisProgrami/24.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 14103 | `327356497fe1c140a61c1ddf48517d183a84e393ae4d7369b473575c79e12307` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-24 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-24/CSV/GunlukYarisSonuclari/24.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 14374 | `9e226aa4bb9e2d99287e9edfd5515e2c121eed44b3a1b138f64025b34191b149` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-25 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-25/CSV/GunlukYarisProgrami/25.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 18145 | `bba29d297e7227fa701a55b91512744b29cb5c8e7af8a10f50d79c45e94abcd1` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-25 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-25/CSV/GunlukYarisSonuclari/25.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 18808 | `67d9370a6a1a213110ca944afaa63dc5b30e905b9f458cd9c9a35752c5d1481b` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-25 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-25/CSV/GunlukYarisProgrami/25.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 15735 | `705c7184afb815cfdf4cfeb302ba455c05c4e58b99d97e190f230fdddf2f577f` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-25 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-25/CSV/GunlukYarisSonuclari/25.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 16077 | `592940836cc69d40fc8587e67a8527224d9ed318ecf7e0f9242fbefa245b82e3` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-26 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-26/CSV/GunlukYarisProgrami/26.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 12914 | `925074882ca7e05b22be543e6256b9a2cc0d8e743735413c55866fd56778d4d9` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-26 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-26/CSV/GunlukYarisSonuclari/26.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 13098 | `d8487e1c2f0be35df62ebebf7c79d936ac31c5babc89499e97fcf12f85b38627` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-26 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-26/CSV/GunlukYarisProgrami/26.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 10788 | `992e611231530128c63eb421da3443addf4371202737acd3bb828e57cb269aa5` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-26 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-26/CSV/GunlukYarisSonuclari/26.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 10879 | `b754d3152084b79fa0ba3ca03f77c753e9c82eee5fecbe4a81625c39af7a2ec7` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-27 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-27/CSV/GunlukYarisProgrami/27.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 12169 | `9d443733d849be20eec57df2201e26fa990abe7311a0167d6981bb22ba145e02` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-27 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-27/CSV/GunlukYarisSonuclari/27.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 12545 | `373caf196864afc8bf63cd23fd42c653dddb8e2caf1e22b0e1937ad77f5059a1` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-27 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-27/CSV/GunlukYarisProgrami/27.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 14780 | `d07b72d4c39fc3cc3539f3e0bf001a25288161506971e93b183de5acff158f50` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-27 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-27/CSV/GunlukYarisSonuclari/27.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 15092 | `371dc15c9997123abf36d880b5b86671da66254bac81f9031612f92e522ea1b3` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-28 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-28/CSV/GunlukYarisProgrami/28.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 18925 | `b4a16a8fb57fa8f3664b42cc1ebd21ba6d5f57ea34d022d908cb8f67dfc30ac4` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-28 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-28/CSV/GunlukYarisSonuclari/28.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 19607 | `b415f139b77c855a21b549b5c3aa0ea764eb344642a71c6704602bf2ba678590` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-28 | İstanbul | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-28/CSV/GunlukYarisProgrami/28.03.2026-%C4%B0stanbul-GunlukYarisProgrami-TR.csv | 200 | 15834 | `4a89d2e067c7efe3d0876f923349c00f7011e11e79f0a47d167bb5f17f1fadf5` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-28 | İstanbul | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-28/CSV/GunlukYarisSonuclari/28.03.2026-%C4%B0stanbul-GunlukYarisSonuclari-TR.csv | 200 | 16551 | `d73843090ca978c7cfae3beb34fe6abd5be2664d5f289110cbb85e4d03b823a6` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-29 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-29/CSV/GunlukYarisProgrami/29.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 15519 | `45d09beb3af9ca9b76b239dd3d49b6e956ecb4f2c1ff905d219418c81c8f39ab` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-29 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-29/CSV/GunlukYarisSonuclari/29.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 16046 | `5139731bbdb9159f1477edea4161d2a911cae1af1d6b2596f933a96870bb821d` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-29 | İzmir | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-29/CSV/GunlukYarisProgrami/29.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv | 200 | 14625 | `c8c9f2a4c6356e75415c79cfd92f037d051d2675807343a3d31bd9a9669bbcb8` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-29 | İzmir | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-29/CSV/GunlukYarisSonuclari/29.03.2026-%C4%B0zmir-GunlukYarisSonuclari-TR.csv | 200 | 15070 | `a96ba04d7d4be89b46399d92c729b7fcc23bb9367f9c777085f28334741de936` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-30 | Bursa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-30/CSV/GunlukYarisProgrami/30.03.2026-Bursa-GunlukYarisProgrami-TR.csv | 200 | 15909 | `d0786afed19d36fcba0842206afd0cc99907f16d402c1b31cf0710f1309af754` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-30 | Bursa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-30/CSV/GunlukYarisSonuclari/30.03.2026-Bursa-GunlukYarisSonuclari-TR.csv | 200 | 16284 | `42172165c25b9a7a5531baac3f899642cc42fdb8c30764c7a51db45809845a75` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-30 | Şanlıurfa | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-30/CSV/GunlukYarisProgrami/30.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisProgrami-TR.csv | 200 | 17272 | `117b3baefcd18fba919a941fc5c8897c5a168e002bf4e363474ccacb1055b1f1` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-30 | Şanlıurfa | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-30/CSV/GunlukYarisSonuclari/30.03.2026-%C5%9Eanl%C4%B1urfa-GunlukYarisSonuclari-TR.csv | 200 | 17843 | `1ff56067f9b5f7ecc4af8ee653ab5fa79fb63020a4b1bc3ae180b041828e3cb2` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-31 | Adana | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-31/CSV/GunlukYarisProgrami/31.03.2026-Adana-GunlukYarisProgrami-TR.csv | 200 | 11980 | `a0d95232e267f4714ab3be0d49dbba75202a639ad5c1b03fafb9875e961ff735` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-31 | Adana | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-31/CSV/GunlukYarisSonuclari/31.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 12268 | `3882004ba97d11d7784d9e7a1f210b1256848fa72827b674b10bb0016ae29a84` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-31 | Antalya | program | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-31/CSV/GunlukYarisProgrami/31.03.2026-Antalya-GunlukYarisProgrami-TR.csv | 200 | 14541 | `276c63c3fd389940a46406dda602413588d05e0c38344a2ea162f665bb492799` | ICERIK_DOGRULANDI (yeniden kullanıldı) |
| 2026-03-31 | Antalya | sonuc | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-31/CSV/GunlukYarisSonuclari/31.03.2026-Antalya-GunlukYarisSonuclari-TR.csv | 200 | 15151 | `fb7546221b73f6cc2c24f856c34e199047b614618f34f2089569e5ecfb44ea2a` | ICERIK_DOGRULANDI (yeniden kullanıldı) |

Doğrulanan ham CSV'ler `kontrol/ham` altında (`yerel_dosya` adlarıyla). Ayrıntılı kayıt: `kontrol/mart_indirme_durumu.tsv`.

## Notlar

- Bu rapor ikinci (yeniden başlatılmış) çalıştırmadan üretildi: 99 dosya, durum kaydındaki SHA-256 diskteki dosyayla eşleştiği için yeniden indirilmedi.
- 21.03.2026 Adana sonuç adresi iki çalıştırmada da HTTP 200 ile **0 bayt** döndü; boş dosya ham klasöre alınmadı. İki deneme de `kontrol/mart_indirme_durumu.tsv` içinde kayıtlı.
- 15.03.2026 Adana sonuç dosyası içerikten doğrulandı (tarih, hipodrom, tür doğru) ama 3.–8. koşuları içermiyor; ayrıştırıcı bu yüzden eşleşmeyen program kayıtları ve eksik altılı ayakları bildiriyor.
- Erişim hatası (403, 429, 502/503/504, bağlantı) yaşanmadı; `kontrol/son_hata.txt` oluşturulmadı.
