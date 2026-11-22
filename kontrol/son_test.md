# Son kontrol: yıllık veri tabanı yedeği (2026-11-22)

- **Yedek:** `calisma/tjk_yillik.sqlite` salt okunur açılıp sqlite3 backup API ile kopyalandı; tam tablo dökümü kaynakla aynı (`a969e87c…fdaed0`).
- **Yedek SQLite SHA-256:** `7462e59990a5318388e1edd81082809e260334f2f48663c869e1101907bbf057` (kaynak dosya baytları farklıdır: `ab997fc3…bae5`; içerik aynı).
- **Salt okunur doğrulama:** integrity_check ok, foreign_key_check 0 satır, user_version 4; 754 yarış günü, 6198 koşu, 63598 katılım/sonuç, 1356 altılı, 1359 altılı ödemesi, 1510 kaynak dosyası; gün aralığı 2025-11-22 – 2026-11-22.
- **Kapsam:** 760 çift: 749 TAMAM, 5 INCELEME_GEREKLI, 6 EKSIK. Yıl tamamlanmış değil; açık 11 çift `kontrol/yillik/acik_11_cift.txt`.
- **Paket:** `tjk_yillik_kontrol.zip` (SQLite, .sha256, kontrol özeti, açık çift listesi) yeniden açıldı; SHA-256 `OK`, integrity ok, FK 0. ZIP SHA-256 `26437a46d3eef04ffbaafca3531cc57989a2d3c9558deb991090e968950e75fc`. SQLite ve ZIP Git'e eklenmedi.
- Ağ isteği yapılmadı; HTTP durumu yok.
