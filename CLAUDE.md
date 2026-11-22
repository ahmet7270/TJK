# TJK proje kuralları

- Uygulama kodu yalnızca Rust (`tjk-fetch/`). HTTPS aktarımı için curl kullanılabilir.
- Veri testlerinde dosyaları her zaman ağdan indir; depodaki eski kopyaları kullanma.
- Önce robots.txt kontrolü. HTTP 403, 429 veya ağ geçidi hatası (502/503/504) alınırsa dur ve tam hata mesajını bildir.
- Kullanıcı açıkça istemedikçe veri tabanını değiştirme.

## Veri testi raporlama

Her veri testinden sonra:

1. Kısa sonucu `kontrol/son_test.md` dosyasına yaz (önceki içeriğin üzerine): denenen tarih ve hipodrom,
   kaynak URL'ler, HTTP durumları, Rust doğrulama sonucu (bayt, SHA-256, eşleşme).
2. Başarılıysa indirilen ham CSV'leri `kontrol/ham/` altına; başarısızsa tam hata mesajını
   `kontrol/son_hata.txt` dosyasına koyup rapor ile **aynı commit'e** ekle.
3. SQLite dosyalarını (`*.sqlite`, `*.sqlite3`, `*.db`) asla commit'leme.
4. Çalışma dalına push et; yanıtın sonunda doğrudan GitHub commit bağlantısını ver
   (`https://github.com/ahmet7270/TJK/commit/<tam SHA>`).
