# Son veri testi: açık çiftler (2026-11-22)

- **Denenen:** 22.11.2026 Adana sonuç CSV'si (yeniden alım) + 11 açık çift için alternatif belgeler (CSV/PDF/özet PDF). robots.txt: medya-cdn HTTP 404 (kural yok); www.tjk.org kullanılmadı (`Disallow: /`).
- **Kaynak:** https://medya-cdn.tjk.org/raporftp/TJKPDF/2026/2026-11-22/CSV/GunlukYarisSonuclari/22.11.2026-Adana-GunlukYarisSonuclari-TR.csv — HTTP 200.
- **Rust doğrulaması (tjk-fetch/tjk_ayristirici):** 17554 bayt, SHA-256 `87f54203c487c945f775e7399d2ea7f30afd31fa72fed0d6a1950e5cc5a5e29f`; 8 koşu, içerik doğrulandı; aktarım 46/46 gün TAMAM, 2. aktarım dökümü aynı, FK 0, integrity ok. (CDN aynı adreste 14200/16872 baytlık eksik sürümler de döndürdü.)
- **Sonuç:** 22.11 Adana tamamlandı. Takvim: 749 TAMAM / 5 INCELEME / 6 EKSIK. Diğer 11 çiftte iptal ya da erteleme (11.01 Adana hariç) kanıtlanamadı; ayrıntı `kontrol/yillik/acik12/RAPOR.md`.
- HTTP 403/429/5xx alınmadı. Ham dosya: `kontrol/ham/2026-11/20261122-ADANA-sonuc.csv`.
