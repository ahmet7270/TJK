# Yıllık kapsam: çalıştırma ve kaldığı yerden devam

Hedef: 22.11.2025 – 22.11.2026 (dahil) Türkiye tarih–hipodrom çiftleri. Resmî çiftler: `takvim/takvim_hedef.tsv` (760).
SQLite ve ZIP'ler Git'e girmez (`.gitignore`: `*.sqlite`, `calisma/`, `*.zip`).

## 0. Başlangıç ve yedek

```bash
cargo build --release --offline --manifest-path tjk_ayristirici/Cargo.toml   # tjk_ayristirici, tjk_indir, tjk_sorgu, tjk_denetim
cargo build --release --manifest-path tjk-fetch/Cargo.toml                   # tjk-fetch (sha2, encoding_rs)
mkdir -p calisma/baslangic calisma/yedek
unzip tjk_calisma_v4_birlesik.zip -d calisma/baslangic
sha256sum calisma/baslangic/tjk_calisma_v4_birlesik.sqlite   # a4dce9ff4946cd74bb4ef23983eff4ff39acb33a3bd0aa40e94cce69da036a66
cp calisma/baslangic/tjk_calisma_v4_birlesik.sqlite calisma/yedek/baslangic_v4.sqlite && chmod a-w calisma/yedek/baslangic_v4.sqlite
cp calisma/baslangic/tjk_calisma_v4_birlesik.sqlite calisma/tjk_yillik.sqlite        # çalışma kopyası (TJK_DB ile değiştirilebilir)
```

Başlangıç: 63 gün, 497 koşu, 4901 katılım/sonuç, 109 altılı; tam döküm SHA-256 `d85bb4ab…850e` (`tjk_denetim`).

## 1. Her ay (sıra önemli değil; her adım kaldığı yerden devam eder)

```bash
bash kontrol/yillik/ay_isle.sh indir 2026-04     # takvim -> aday liste (tjk_indir liste) -> daraltma (tjk-fetch daralt)
                                                 # -> tjk-fetch toplu (curl, robots.txt, içerik doğrulama, ayrıştırma)
# İnceleme/eksik çiftler için: ertelenen koşuların başka tarihli dosyaları listeler/AY_ek_liste.tsv ile indirilir
#   (bkz. ay_isle.sh başı), eşleme listeler/AY_ozel.tsv'ye yazılır (aktar.sh biçimi: gun_id, program, sonuç, "--ek-sonuc DOSYA|not").
bash kontrol/yillik/ay_isle.sh aktar 2026-04     # tjk-fetch girdiler -> aktar.sh (1.) -> tjk_denetim -> aktar.sh (2.) -> tjk_denetim
                                                 # -> iki tam dökümün cmp'si; rapor: raporlar/AY_denetim.md
bash kontrol/yillik/ay_isle.sh denetim 2026-04   # yalnızca denetim (DB salt okunur)
```

- `indir` erişim hatasında (HTTP 403/429/502/503/504, bağlantı yok) çıkış kodu 3 verir, `kontrol/son_hata.txt` yazılır, döngü durur.
  Yeniden çalıştırınca indirme durumu (`listeler/AY_durum.tsv`) okunur; SHA-256'sı diskteki dosyayla eşleşen dosyalar yeniden indirilmez.
- Mart 2026 indirilmez: `kontrol/mart_sqlite_girdiler.tsv` + `kontrol/ham/` (içerik ve SHA-256 ile yeniden kullanım); URL/HTTP kaydı
  `kontrol/mart_indirme_durumu.tsv` ve `listeler/2026-03_durum_ek.tsv`.
- Ham dosyalar: `kontrol/ham/AY/` (Mart: `kontrol/ham/`). Aylık listeler/girdiler/durumlar: `kontrol/yillik/listeler/`.

## Dosyalar

| Dosya | İçerik |
|---|---|
| `listeler/AY_takvim.tsv` | ayın resmî çiftleri |
| `listeler/AY_liste.tsv` | çift başına program + sonuç URL'si (kalıptan üretilmiş aday) |
| `listeler/AY_durum.tsv` | her indirme: zaman, URL, HTTP, bayt, SHA-256, içerik doğrulaması |
| `listeler/AY_ek_liste.tsv`, `AY_ek_durum.tsv` | takvim dışı tarihli ek dosyalar (ertelenen koşular) |
| `listeler/AY_ozel.tsv` | aktarımda üretilen satırın yerine geçen özel girdi (ertelenen koşular) |
| `listeler/AY_girdiler.tsv` | SQLite aktarım girdisi |
| `raporlar/AY_indirme.md` | indirme + ayrıştırma raporu |
| `raporlar/AY_denetim.md` | ay sonu denetimi |
| `../yillik_durum.md` | ay bazında ilerleme |
