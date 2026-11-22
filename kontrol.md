# TJK arşiv erişim testi — konuşma kaydı

Dal: `claude/tjk-archive-download-test-hqotls` · Tarih: 22.11.2026

---

## 1. İstek: TJK arşivine erişim testi

**Kullanıcı:** TJK arşivine bu bulut oturumundan erişilebildiğini gerçek bir indirmeyle test et. Komutları
kendin çalıştır, benden dosya indirmemi isteme. Uygulama kodu yalnızca Rust olsun; HTTPS için curl kullanılabilir.

1. `https://medya-cdn.tjk.org/robots.txt` üzerinden erişim kurallarını kontrol et; engellenirse dur ve tam hatayı ver.
2. 6 Mart 2026 İzmir yarış programı CSV'sini doğrudan ağdan indir (depodaki eski kopyayı kullanma).
3. HTTP durumu, bayt sayısı, SHA-256; ilk satır ve ilk koşudan bir at adı. Karşılaştırma:
   12660 bayt, `45801a1bafe2fdf3bd299759844c25d59766b8d5f753cc2ebf89b0064ffbf594`.
4. Eşleşirse 2026 resmî takvim PDF'sini indir, Mart'taki Türkiye tarih–hipodrom çiftlerini
   `mart_2026_resmi_takvim.tsv` dosyasına yaz, sütun hizasını doğrula, çift sayısını bildir.
   403/429/ağ geçidi hatasında dur. Veri tabanını değiştirme.

### Yapılanlar ve sonuçlar

- Rust aracı `tjk-fetch/` yazıldı (komutlar: `robots`, `fetch`, `csv`, `takvim`; aktarım curl, PDF kelime
  koordinatları `pdftotext -bbox`).
- **robots.txt:** HTTP 404 (dosya yok). RFC 9309'a göre kısıtlama yok → devam edildi.
- **İzmir CSV** (`.../2026-03-06/CSV/GunlukYarisProgrami/06.03.2026-%C4%B0zmir-GunlukYarisProgrami-TR.csv`):
  HTTP 200, **12660 bayt**, SHA-256 `45801a1b…bf594` → **birebir eşleşti**. UTF-8, 107 satır.
  - İlk satır: `İzmir;(29. Yarış Günü);06/03/2026`
  - 1. koşu, 1 numaralı at: **ATİK MEHMET** (ham: `ATİK MEHMET KG DB SK GKR`; sondakiler ekipman kısaltması)
- **Takvim PDF** (`/haberftp/2025/2026takvim161225.pdf`): HTTP 200, 134596 bayt,
  SHA-256 `36a2318a2c8f13e8df78883cf2ff686ab38acd4eec1dcf5ebc2777b1713a52e6`. Tek sayfalık ızgara.
  - Düz metinde gün numaraları ile kodlar kaydığı için her kod sayfadaki konumuna göre tarihine bağlandı.
  - Hizalama doğrulaması (hata yok): tarih–gün uyumu; sol/sağ yuva çakışması yok; tüm yıl sayımı PDF
    başlığındaki toplamlarla aynı (ADA 89, ANK 83, ANT 54, BUR 92, DYB 30, ELZ 49, İST 135, İZM 125,
    KOC 44, URF 57; genel toplam 758).
  - **Mart: 61 tarih–hipodrom çifti** → `mart_2026_resmi_takvim.tsv`. 20.03'teki `RMZ` (Ramazan Bayramı)
    hipodrom olmadığı için hariç. Dağılım: Adana 14, İzmir 13, Antalya 11, Şanlıurfa 9, İstanbul 8, Bursa 6.
  - 06.03 takvimde Bursa ve İzmir → CSV ile tutarlı. PDF'de yurt dışı koşusu yok.
- Veri tabanı yoktu, hiçbir şey değiştirilmedi.
- Commit `0a36f28` oluşturuldu; **push GitHub'dan 403 aldı** (Claude'un depoya erişimi yok).

---

## 2. İstek: raporlama kuralı

**Kullanıcı:** Bundan sonraki veri testlerinde kısa sonucu `kontrol/son_test.md` dosyasına yaz (tarih,
hipodrom, kaynak URL'ler, HTTP durumu, Rust doğrulama sonucu). Başarılıysa ham CSV'leri, başarısızsa tam
hata mesajını aynı commit'e ekle. SQLite dosyasını ekleme. Çalışma dalına push et, yanıtın sonunda
GitHub commit bağlantısını ver.

### Yapılanlar

- `kontrol/son_test.md` yazıldı (06.03.2026 İzmir, BAŞARILI).
- Ham CSV `kontrol/ham/06.03.2026-İzmir-GunlukYarisProgrami-TR.csv` eklendi (SHA-256 yeniden doğrulandı).
- `.gitignore`: SQLite dosyaları hariç tutuldu.
- `CLAUDE.md`: kural kalıcı hale getirildi (başarısız testte hata `kontrol/son_hata.txt`'ye gidecek).
- Commit `2240f3934592e28006d3147c583924c26bc23851` oluşturuldu.
- **Push yine 403 aldı:**
  ```
  remote: Claude doesn't have GitHub access to ahmet7270/TJK for your organization.
  fatal: unable to access 'https://github.com/ahmet7270/TJK/': The requested URL returned error: 403
  ```
  Çözüm: https://claude.ai/connect-github üzerinden GitHub'ı yeniden bağlamak ve gerekirse Claude GitHub
  App'i `ahmet7270/TJK` deposuna kurmak.

---

## 3. İstek: bu konuşmayı paylaş

**Kullanıcı:** Bu konuşmalarımızı kontrol.md'ye yaz, bana gönder.

→ Bu dosya.

## Bekleyen iş

GitHub erişimi düzelince daldaki commit'ler push edilecek. Commit bağlantısı (push sonrası geçerli olur):
https://github.com/ahmet7270/TJK/commit/2240f3934592e28006d3147c583924c26bc23851
