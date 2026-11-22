# Son veri testi

- **Durum:** BAŞARILI (dört istisna işlendi; 15.03 Adana dar kapsamla incelemede)
- **Tarih:** 2026-11-22
- **Araçlar:** `tjk_indir liste/indir` (curl; robots.txt HTTP 404, kural yok), `tjk_ayristirici` 0.12.0, `tjk-fetch karsilastir`
- **SQLite:** kullanılmadı · **Mevcut ham CSV'ler:** değiştirilmedi
- Ayrıntı: `kontrol/mart_istisnalar.md`

## Bu testte indirilen dosyalar

| Tarih / hipodrom | Rol | Kaynak URL | HTTP | Bayt | SHA-256 | İçerik doğrulaması |
|---|---|---|---|---|---|---|
| 16.03.2026 Adana | sonuç | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-16/CSV/GunlukYarisSonuclari/16.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 12192 | `70763d1a1258cc66801d786c043fe2eef9b5e8713f3d12666188a0a4a6343dd3` | ICERIK_DOGRULANDI: 2026-03-16 Adana sonuç, 32. yarış günü, koşu 3–8 |
| 23.03.2026 Adana | sonuç | https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-03-23/CSV/GunlukYarisSonuclari/23.03.2026-Adana-GunlukYarisSonuclari-TR.csv | 200 | 17779 | `2db0876bcaa528479cb7b75d0abfa08ee0103b8a95180bcac1b0f821bf2d34cb` | ICERIK_DOGRULANDI: 2026-03-23 Adana sonuç, 34. yarış günü, koşu 1–9 |

Ham dosyalar: `kontrol/ham/20260316-ADANA-sonuc.csv`, `kontrol/ham/20260323-ADANA-sonuc.csv`.

## Ayrıştırma sonuçları (dört gün)

| Gün | Girdi | Sonuç |
|---|---|---|
| 15.03.2026 Adana | program 15.03 + sonuç 15.03 (koşu 1–2) + `--ek-sonuc` 16.03 (koşu 3–8), `--ertelenme-notu "TJK duyuru 52071 …"` | 8 koşu, 77/77 at eşleşti; bahis 21/21 uyumlu; **İNCELEME GEREKLİ**: `GANYAN_SATIRI_BULUNAMADI: 3. koşu`, `ALTILI_ODEMESI_BULUNAMADI: A1` |
| 21.03.2026 Adana | program 21.03 + sonuç 23.03, `--ertelenme-notu` | 9 koşu (hepsi ERTELENDI 21.03→23.03), 89/89 at; bahis 25/25; 4. koşu pist Çim→Kum kayıtlı; **TAMAM** |
| 21.03.2026 İstanbul | program + sonuç 21.03 | LEONİDAS (`YP`) eküri doğrulandı; **TAMAM** |
| 31.03.2026 Antalya | program + sonuç 31.03 | 8. koşu ortak birinci 5 + 6; 2. altılı iki kombinasyon (2.093,81 TL / 1.686,12 TL); bahis 25/25; **TAMAM** |

## Regresyon

- `cargo test --release --offline`: 19 birim + 10 uçtan uca test geçti.
- 50 gün eski (0.11.0) ve yeni (0.12.0) sürümle karşılaştırıldı: 41 gün bayt bayt aynı; 8 günde yalnızca `YP`'li 10 at satırının
  adı/kodu; Antalya'da beklenen ortak birincilik değişiklikleri. Daha önce TAMAM olan 46 gün hâlâ TAMAM.
  Hücre listesi: `kontrol/regresyon_0.12.0_farklar.tsv`.

## Mart 2026 toplamı (resmî takvim: 61 tarih–hipodrom çifti)

| Durum | Çift |
|---|---|
| Tamamlandı (bu depoda doğrulandı) | 50 |
| İnceleme gerekli | 1 (15.03 Adana) |
| Eksik | 0 |
| Önceden işlendi, bu depoda kanıtı yok (06–10 Mart) | 10 |
| **Toplam** | **61** |

Bu depoda işlenen 51 çiftin 50'si tamamlandı, 1'i incelemede.

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
| 2026-03-06 | Bursa | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-06 | İzmir | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-07 | Adana | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-07 | İstanbul | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-08 | İzmir | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-08 | Adana | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-09 | Bursa | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-09 | Şanlıurfa | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-10 | Adana | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-10 | Antalya | ÖNCEDEN İŞLENDİ (bu depoda kanıtı yok) | 06–10 Mart çiftleri başka bir çalışmada işlendi; bu depoda yalnızca 06.03 İzmir program CSV'si var |
| 2026-03-11 | İstanbul | TAMAMLANDI |  |
| 2026-03-11 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-12 | İzmir | TAMAMLANDI |  |
| 2026-03-12 | Antalya | TAMAMLANDI |  |
| 2026-03-13 | Antalya | TAMAMLANDI |  |
| 2026-03-13 | İzmir | TAMAMLANDI |  |
| 2026-03-14 | Adana | TAMAMLANDI |  |
| 2026-03-14 | İstanbul | TAMAMLANDI |  |
| 2026-03-15 | İzmir | TAMAMLANDI |  |
| 2026-03-15 | Adana | İNCELEME GEREKLİ | 3. koşu GANYAN ve 1. altılı ödemesi dosyada yok (koşular 15.03+16.03 dosyalarıyla tam) |
| 2026-03-16 | Bursa | TAMAMLANDI |  |
| 2026-03-16 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-17 | Adana | TAMAMLANDI |  |
| 2026-03-17 | Antalya | TAMAMLANDI |  |
| 2026-03-18 | İstanbul | TAMAMLANDI |  |
| 2026-03-18 | Şanlıurfa | TAMAMLANDI |  |
| 2026-03-19 | İzmir | TAMAMLANDI |  |
| 2026-03-19 | Antalya | TAMAMLANDI |  |
| 2026-03-20 | İzmir | TAMAMLANDI |  |
| 2026-03-21 | Adana | TAMAMLANDI | Sonuçlar 23.03 dosyasında; 9 koşu ertelendi olarak kayıtlı; 4. koşu pisti Çim→Kum |
| 2026-03-21 | İstanbul | TAMAMLANDI | YP düzeltmesi; eküri doğrulandı |
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
| 2026-03-31 | Antalya | TAMAMLANDI | 8. koşu ortak birinci 5+6; iki 2. altılı ödemesi ayrı |
