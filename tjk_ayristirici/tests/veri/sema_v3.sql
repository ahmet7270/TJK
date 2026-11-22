-- tjk_pilot.sqlite şeması (sürüm 2: devir kaydı ve bahis uyarıları; sürüm 1'den göç: veritabani.rs GOC_1_2)
-- İlke: ARŞİV tabloları kaynakta görüneni saklar; TAHMİN girdisi yalnızca v_tahmin_girdisi görünümünden okunur.
-- Her satır kaynak dosyaya (kaynak_dosya: ad + SHA-256) ve kaynak satır numarasına bağlıdır.
-- Bir koşu ve bir at katılımı, kaç altılıda yer alırsa alsın, TEK satırdır; altılı bağlantısı altili_ayak üzerinden kurulur.

PRAGMA user_version = 3;

CREATE TABLE IF NOT EXISTS kaynak_dosya (
    kaynak_id   INTEGER PRIMARY KEY,
    sha256      TEXT NOT NULL UNIQUE,
    dosya_adi   TEXT NOT NULL,
    rol         TEXT NOT NULL CHECK (rol IN ('program','sonuc')),
    kodlama     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS yaris_gunu (
    gun_id              TEXT PRIMARY KEY,              -- YYYYAAGG-HIPODROM
    tarih               TEXT NOT NULL,
    hipodrom            TEXT NOT NULL,
    yaris_gunu_no       INTEGER NOT NULL,
    gun_durumu          TEXT NOT NULL CHECK (gun_durumu IN ('TAMAM','INCELEME_GEREKLI')),
    program_kaynak_id   INTEGER NOT NULL REFERENCES kaynak_dosya(kaynak_id),
    sonuc_kaynak_id     INTEGER NOT NULL REFERENCES kaynak_dosya(kaynak_id),
    ayristirici_surumu  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS inceleme_nedeni (
    gun_id  TEXT NOT NULL REFERENCES yaris_gunu(gun_id) ON DELETE CASCADE,
    sira    INTEGER NOT NULL,
    neden   TEXT NOT NULL,
    PRIMARY KEY (gun_id, sira)
);

CREATE TABLE IF NOT EXISTS kosu (
    kosu_id             TEXT PRIMARY KEY,              -- gun_id-Kkk
    gun_id              TEXT NOT NULL REFERENCES yaris_gunu(gun_id) ON DELETE CASCADE,
    kosu_no             INTEGER NOT NULL,
    saat                TEXT, kosu_cinsi TEXT, yas_grubu TEXT, kilo_sartlari TEXT,
    mesafe_m            INTEGER, pist TEXT, rekor_derece TEXT,
    ikramiye_1_tl REAL, ikramiye_2_tl REAL, ikramiye_3_tl REAL, ikramiye_4_tl REAL, ikramiye_5_tl REAL,
    yetistirici_primi_1_tl REAL, yetistirici_primi_2_tl REAL, yetistirici_primi_3_tl REAL,
    yetistirici_primi_4_tl REAL, yetistirici_primi_5_tl REAL,
    kosu_bahisleri      TEXT,
    program_kaynak_satir INTEGER NOT NULL,
    kosu_adi            TEXT,                          -- başlıkta yazıyorsa koşunun adı; adsız koşuda NULL (şema 3)
    UNIQUE (gun_id, kosu_no)
);

-- Atın kariyer boyu kimliği. TJK AtId kaynakta yok: geçici anahtar ad+baba+anne ile kurulur ve işaretlenir.
-- Katılım kimliği (katilim_id) ASLA at kimliği olarak kullanılmaz.
CREATE TABLE IF NOT EXISTS at (
    at_id           INTEGER PRIMARY KEY,
    tjk_at_id       INTEGER,                           -- bilinmiyor (NULL)
    gecici_anahtar  TEXT NOT NULL UNIQUE,              -- AD|BABA|ANNE
    at_adi          TEXT NOT NULL,
    baba            TEXT,
    anne            TEXT,
    kimlik_dayanagi TEXT NOT NULL DEFAULT 'GECICI_AD_BABA_ANNE'
);

-- ARŞİV: programda görünen katılım (yarış öncesi türünde bilgi, ancak arşiv anlık görüntüsü)
CREATE TABLE IF NOT EXISTS katilim (
    katilim_id      TEXT PRIMARY KEY,                  -- kosu_id-Aaa (yalnızca bu yarıştaki katılım)
    kosu_id         TEXT NOT NULL REFERENCES kosu(kosu_id) ON DELETE CASCADE,
    at_id           INTEGER NOT NULL REFERENCES at(at_id),
    program_at_no   INTEGER NOT NULL,
    at_isim_ham TEXT, isim_kodlari TEXT, yas_ham TEXT, yas INTEGER,
    kilo_ham TEXT, kilo REAL, kilo_ek TEXT,
    jokey_ham TEXT, jokey TEXT, jokey_ap_ibaresi INTEGER,
    sahip TEXT, antrenor TEXT,
    start_ham TEXT, start_no INTEGER, start_ek TEXT,
    h INTEGER, son6_ham TEXT, kgs INTEGER, s20 INTEGER, en_iyi_derece_ham TEXT, en_iyi_derece_sn REAL,
    kosmaz_ibaresi_programda INTEGER NOT NULL,
    ekuri_grubu     TEXT,
    agf_ham         TEXT,                              -- AGF hücresi olduğu gibi
    jokey_oncesi_dogrulandi  INTEGER NOT NULL,
    kilo_oncesi_dogrulandi   INTEGER NOT NULL,
    kosmaz_zamani_dogrulandi INTEGER NOT NULL,
    program_kaynak_satir INTEGER NOT NULL,
    UNIQUE (kosu_id, program_at_no)
);

-- ARŞİV: AGF hücresindeki her değer ayrı satır. altili_id yalnızca eşleşme belirliyse doludur; tahmin edilmez.
CREATE TABLE IF NOT EXISTS agf_gozlem (
    katilim_id      TEXT NOT NULL REFERENCES katilim(katilim_id) ON DELETE CASCADE,
    deger_sirasi    INTEGER NOT NULL,                  -- hücredeki konum (1, 2)
    agf_yuzde       REAL NOT NULL,
    agf_sira        INTEGER NOT NULL,
    altili_id       TEXT REFERENCES altili(altili_id) ON DELETE SET NULL,
    eslesme_dayanagi TEXT NOT NULL CHECK (eslesme_dayanagi IN ('TEK_ALTILI_CIKARIM','RESMI_TABLO','DOGRULANMADI')),
    zaman_dogrulandi INTEGER NOT NULL,                 -- yarış öncesi zaman doğrulaması (ayrı bilgi)
    PRIMARY KEY (katilim_id, deger_sirasi)
);

-- ARŞİV: yarış sonrası
CREATE TABLE IF NOT EXISTS sonuc (
    katilim_id          TEXT PRIMARY KEY REFERENCES katilim(katilim_id) ON DELETE CASCADE,
    sonuc_liste_sirasi  INTEGER,
    bitis_sirasi        INTEGER,
    kosu_durumu         TEXT NOT NULL,
    derece_ham TEXT, derece_sn REAL, ganyan_oran REAL, fark_ham TEXT,
    eslesme_durumu      TEXT NOT NULL,
    capraz_kontrol_farklari TEXT,
    sonuc_kaynak_satir  INTEGER
);

CREATE TABLE IF NOT EXISTS bahis_sonucu (
    kosu_id         TEXT NOT NULL REFERENCES kosu(kosu_id) ON DELETE CASCADE,
    sira            INTEGER NOT NULL,
    oyun            TEXT NOT NULL,
    kombinasyon_ham TEXT NOT NULL,
    tutar_ham       TEXT,                              -- kazanana ödeme (ham); devirde NULL
    tutar_tl        REAL,                              -- kazanana ödeme; devirde NULL
    teselli_ham     TEXT,
    sonuc_kaynak_satir INTEGER NOT NULL,
    sonuc_durumu    TEXT NOT NULL DEFAULT 'ODENDI' CHECK (sonuc_durumu IN ('ODENDI','DEVIR')),
    devir_ham       TEXT,                              -- ör. "1,007,862.90 TL"
    devir_tl        REAL,
    ham_ibare       TEXT,                              -- kaynaktaki ibare olduğu gibi
    PRIMARY KEY (kosu_id, sira)
);

-- Okunamayan bahis parçaları (ham metin ve kaynak satırı)
CREATE TABLE IF NOT EXISTS bahis_uyarisi (
    kosu_id         TEXT NOT NULL REFERENCES kosu(kosu_id) ON DELETE CASCADE,
    sira            INTEGER NOT NULL,
    ham_metin       TEXT NOT NULL,
    sonuc_kaynak_satir INTEGER NOT NULL,
    PRIMARY KEY (kosu_id, sira)
);

CREATE TABLE IF NOT EXISTS ganyan_kontrol (
    kosu_id         TEXT PRIMARY KEY REFERENCES kosu(kosu_id) ON DELETE CASCADE,
    ganyan_numaralari TEXT NOT NULL,                   -- kaynaktaki gibi, virgülle
    birinciler      TEXT NOT NULL,
    durum           TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS altili (
    altili_id           TEXT PRIMARY KEY,              -- gun_id-An
    gun_id              TEXT NOT NULL REFERENCES yaris_gunu(gun_id) ON DELETE CASCADE,
    oyun_adi            TEXT NOT NULL,
    baslangic_kosu_no   INTEGER NOT NULL,
    kombinasyon_ham     TEXT,
    odeme_ham           TEXT,
    odeme_tl            REAL,
    program_kaynak_satir INTEGER NOT NULL,
    sonuc_kaynak_satir  INTEGER,
    sonuc_durumu        TEXT,                          -- ODENDI / DEVIR (sonuç yoksa NULL)
    devir_tl            REAL
);

CREATE TABLE IF NOT EXISTS altili_ayak (
    altili_id   TEXT NOT NULL REFERENCES altili(altili_id) ON DELETE CASCADE,
    ayak_no     INTEGER NOT NULL CHECK (ayak_no BETWEEN 1 AND 6),
    kosu_id     TEXT NOT NULL REFERENCES kosu(kosu_id) ON DELETE CASCADE,
    PRIMARY KEY (altili_id, ayak_no),
    UNIQUE (altili_id, kosu_id)
);

-- Ayakta geçerli her numara ayrı satır (ör. 1,2 → iki satır)
CREATE TABLE IF NOT EXISTS altili_ayak_gecerli (
    altili_id       TEXT NOT NULL,
    ayak_no         INTEGER NOT NULL,
    program_at_no   INTEGER NOT NULL,
    katilim_id      TEXT REFERENCES katilim(katilim_id) ON DELETE CASCADE,
    neden           TEXT NOT NULL CHECK (neden IN ('BIRINCI','KOSMAZ','EKURI','ACIKLANMADI')),
    PRIMARY KEY (altili_id, ayak_no, program_at_no),
    FOREIGN KEY (altili_id, ayak_no) REFERENCES altili_ayak(altili_id, ayak_no) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS ix_katilim_at ON katilim(at_id);
CREATE INDEX IF NOT EXISTS ix_ayak_kosu ON altili_ayak(kosu_id);
CREATE INDEX IF NOT EXISTS ix_agf_altili ON agf_gozlem(altili_id);

-- Kaynak izi: her katılımın program ve sonuç kaynağı
CREATE VIEW IF NOT EXISTS v_kaynak_izi AS
SELECT k.katilim_id,
       kp.dosya_adi AS program_dosyasi, kp.sha256 AS program_sha256, k.program_kaynak_satir,
       ks.dosya_adi AS sonuc_dosyasi,   ks.sha256 AS sonuc_sha256,   s.sonuc_kaynak_satir
FROM katilim k
JOIN kosu ko       ON ko.kosu_id = k.kosu_id
JOIN yaris_gunu g  ON g.gun_id = ko.gun_id
JOIN kaynak_dosya kp ON kp.kaynak_id = g.program_kaynak_id
JOIN kaynak_dosya ks ON ks.kaynak_id = g.sonuc_kaynak_id
LEFT JOIN sonuc s  ON s.katilim_id = k.katilim_id;

-- TAHMİN GİRDİSİ: altılı ayağındaki her katılım için yalnızca yarış öncesi türündeki alanlar.
-- Sonuç tablosuna dokunmaz. AGF yalnızca bu altılıya eşleşmesi belirliyse gelir (aksi hâlde NULL).
-- egitime_uygun = 0: arşivdeki değerlerin yarış öncesi zamanı doğrulanmadı.
CREATE VIEW IF NOT EXISTS v_tahmin_girdisi AS
SELECT a.altili_id, a.ayak_no, ko.kosu_id, ko.kosu_no, ko.mesafe_m, ko.pist, ko.kosu_cinsi,
       k.katilim_id, k.at_id, k.program_at_no, k.yas, k.kilo, k.jokey, k.antrenor, k.start_no, k.h,
       k.son6_ham, k.kgs, k.s20, k.en_iyi_derece_sn, k.ekuri_grubu,
       ag.agf_yuzde AS agf_bu_altili_yuzde, ag.agf_sira AS agf_bu_altili_sira, ag.eslesme_dayanagi AS agf_eslesme,
       k.agf_ham,
       COALESCE(ag.zaman_dogrulandi, 0) AS agf_zaman_dogrulandi,
       k.jokey_oncesi_dogrulandi, k.kilo_oncesi_dogrulandi, k.kosmaz_zamani_dogrulandi,
       CASE WHEN COALESCE(ag.zaman_dogrulandi,0)=1 AND k.jokey_oncesi_dogrulandi=1
                 AND k.kilo_oncesi_dogrulandi=1 AND k.kosmaz_zamani_dogrulandi=1 THEN 1 ELSE 0 END AS egitime_uygun
FROM altili_ayak a
JOIN kosu ko    ON ko.kosu_id = a.kosu_id
JOIN katilim k  ON k.kosu_id = ko.kosu_id
LEFT JOIN agf_gozlem ag ON ag.katilim_id = k.katilim_id AND ag.altili_id = a.altili_id;

CREATE VIEW IF NOT EXISTS v_altili_gecerli AS
SELECT g.altili_id, g.ayak_no, ko.kosu_no, g.program_at_no, k.at_isim_ham, g.neden, g.katilim_id
FROM altili_ayak_gecerli g
JOIN altili_ayak a ON a.altili_id = g.altili_id AND a.ayak_no = g.ayak_no
JOIN kosu ko ON ko.kosu_id = a.kosu_id
LEFT JOIN katilim k ON k.katilim_id = g.katilim_id;
