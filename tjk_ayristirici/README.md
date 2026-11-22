# tjk_ayristirici

TJK günlük **Program** ve **Sonuç** CSV dosyalarını (aynı gün, aynı hipodrom) üç çıktıya dönüştürür:

| Çıktı | İçerik |
|---|---|
| `program.csv` | Her satır bir koşudaki bir at; yalnızca program dosyasından gelen alanlar (yarış öncesi) |
| `sonuclar.csv` | Aynı `kayit_id` ile bitiriş sırası, koşu durumu, derece, ganyan, fark, eşleşme kontrolleri (yarış sonrası) |
| `altili.json` | Günün altılı(ları): ayak koşuları, ayak başına geçerli numaralar, ödeme, kanıt satırları, koşmaz kural testi, açık sorular |

Bağımlılık yok (yalnızca Rust standart kütüphanesi); internetsiz derlenir.

## Derleme ve test

```bash
cargo build --release --offline
cargo test --release --offline
```

## Kullanım

```bash
./target/release/tjk_ayristirici PROGRAM.csv SONUC.csv CIKTI_KLASORU \
  [--kontrol-komb "6 / 1,4 / 6 / 4 / 7 / 4"] [--kontrol-odeme "29.789,50 TL"] \
  [--ilgili-url URL] [--ilgili-not METIN]
```

- `--kontrol-*` seçenekleri tekrarlanabilir; k. değer günün k. altılısına uygulanır.
- Kontrol özeti standart çıktıya yazılır (kayıt sayıları, eşleşme sorunları, bahis satırı kontrolü, derece sırası).
- Hata durumunda çıkış kodu 1.

## Kurallar

- Sonuç dosyasından program çıktısına değer taşınmaz.
- Dosyada olmayan değer üretilmez; boş bırakılır, gerekçesi `altili.json` içinde yazılır.
- Sonuç CSV'sindeki "At No" sütunu programdaki at numarası değil, liste sırasıdır. Eşleşme
  tarih + hipodrom + koşu no + at adı ile yapılır ve bahis satırlarındaki (GANYAN, SIRALI İKİLİ,
  ÜÇLÜ, TABELA, SIRALI 5'Lİ) program numaralarıyla bağımsız olarak doğrulanır.
- Kodlama: UTF-8 (BOM'lu/BOM'suz); geçersiz UTF-8 ise Windows-1254 olarak çözülür.
- Kayıt anahtarı: `YYYYAAGG-HIPODROM-Kkk-Aaa` (ör. `20260306-IZMIR-K03-A04`).

## Aynı günde birden çok altılı (v0.3)

- Altılılar programdaki "N. 6'LI GANYAN Bu koşudan başlar" ibaresinden çıkarılır; her biri ardışık 6 koşudur.
- `program.csv` → `altili_baglantilari`: tek katılım kaydının bağlı olduğu altılı ve ayak
  (ör. `20251228-ADANA-A1:3|20251228-ADANA-A2:1`).
- AGF hücresi birden çok değer taşıyabilir (`%33.82(1)  %35.48(1)`): ham değer `agf_ham`'da korunur,
  1. değer `agf_yuzde/agf_sira`, 2. değer `agf_2_yuzde/agf_2_sira`. Değerlerin hangi altılıya ait olduğu
  dosyada yazmadığı için `agf_altili_eslesmesi = SIRA_DOGRULANMADI…` olarak işaretlenir.
- `ekuri_grubu`: programdaki "[(3)…,(7)…,] eküridir." ibaresinden; bu önek bahis satırlarından ayrılır.

## Testler

`cargo test` hem birim testlerini hem `tests/pilotlar.rs` uçtan uca testlerini çalıştırır:
İzmir 06.03.2026 çıktıları `tests/veri/izmir/beklenen_v0_2/` ile karşılaştırılır; Adana 28.12.2025
iki altılı, ortak koşu bağlantıları, AGF ve eküri ayrımı için sınanır.

## Modüller

`ayristir.rs` (dosya ve alan ayrıştırma) · `main.rs` (eşleştirme, altılı, çıktılar) ·
`json.rs` · `csvw.rs` · `sha256.rs` · `kodlama.rs`

## v0.4: gün durumu ve altılıya özel AGF

- `altili.json` → `gun_durumu`: `TAMAM` ya da `INCELEME_GEREKLI`; nedenler `inceleme_nedenleri` listesinde
  (`AYNI_SONUC_SATIRI_BIRDEN_FAZLA_PROGRAM_KAYDINA`, `ESLESMEYEN_PROGRAM_KAYDI`, `ALTILI_AYAGI_EKSIK`,
  `ESLESMEYEN_SONUC_SATIRI`, `ALTILI_ODEMESI_BULUNAMADI`, `ALTILI_BULUNAMADI`).
- `altili.json` → `kosmaz_ozeti`: `benzersiz_kosmaz_katilim` ve `altililar_icindeki_kosmaz_kaydi` ayrı sayılır.
- `program.csv` → `agf_altili1_*`, `agf_altili2_*`: yalnızca eşleşme belirliyse (tek değer + tek altılı) doludur.
  Çok değerli hücrede, resmî 1./2. AGF tablosuyla doğrulanmadıkça boş kalır; ham hücre ve iki değer korunur.
  Yarış öncesi zaman doğrulaması ayrı sütundadır (`agf_oncesi_dogrulandi`).

## v0.12: ertelenen koşular, ortak birincilik, YP eki

- `--ek-sonuc DOSYA` (tekrarlanabilir) ve `--ertelenme-notu METİN`: aynı yarış gününün (hipodrom + yarış günü no) başka
  sonuç dosyalarındaki koşular birleştirilir. Farklı tarihli sonuç yalnızca not verilirse kabul edilir; aynı koşu iki
  dosyada varsa hata (ikinci kez sayılmaz). Ertelenen koşunun cinsi/yaş grubu/kilo/adı/mesafesi programla karşılaştırılır.
- Yeni çıktı `kosu_tarihleri.csv`: koşu başına planlanan ve gerçekleşen tarih/saat, pist, kaynak dosya, erteleme notu.
- Ortak birincilik yalnızca birden çok ayrı GANYAN ödemesi + ilk N sıra + ganyan oranı = ödeme + `At Başı` farkı + aynı derece
  birlikteyse kabul edilir (aynı derece tek başına yetmez). `altili.json` → `ortak_birincilikler`; bitiş sırası ortak 1.
  Birden çok altılı ödemesi `kazanan_kombinasyonlar` içinde her biri kendi ödemesiyle tutulur.
- `YP` at adı kod belirteçlerine eklendi (ham ad `at_isim_ham`'da korunur).
- Yeni alanlar yalnızca ilgili durum varsa yazılır; normal günlerin `program.csv`/`sonuclar.csv`/`altili.json` çıktısı değişmez.
- Uçtan uca testler: `tests/mart_istisnalar.rs` (girdi: `../kontrol/ham`).
