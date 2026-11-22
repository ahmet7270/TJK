#!/bin/bash
# kontrol/yillik_durum.md dosyasını üretir: her ay için tjk_denetim (salt okunur) + kontrol/yillik/notlar.md.
set -u
KOK="$(cd "$(dirname "$0")/../.." && pwd)"
AYLAR="2025-11 2025-12 2026-01 2026-02 2026-03 2026-04 2026-05 2026-06 2026-07 2026-08 2026-09 2026-10 2026-11"
O="$KOK/kontrol/yillik_durum.md"; R="$KOK/kontrol/yillik/raporlar"
tablo=""; ayrinti=""; T=(0 0 0 0 0 0)
for AY in $AYLAR; do
  out=$(bash "$KOK/kontrol/yillik/ay_isle.sh" ozet "$AY" 2>&1); rc=$?
  md="$KOK/calisma/$AY/denetim_ozet.md"
  d=$(echo "$out" | grep '^DENETIM'); s=$(echo "$out" | grep '^SAYILAR'); b=$(echo "$out" | grep '^BUTUNLUK')
  num() { echo "$d" | grep -o "$1 [0-9]*" | grep -o '[0-9]*$'; }
  cift=$(echo "$d" | grep -o 'takvim [0-9]* çift' | grep -o '[0-9]*')
  ta=$(num TAMAMLANDI); ig=$(num INCELEME_GEREKLI); ek=$(num EKSIK); ak=$(num AKTARILMADI); hd=$(num HENUZ_DENENMEDI)
  dis=$(echo "$d" | grep -o 'takvim dışı DB günü [0-9]*' | grep -o '[0-9]*$')
  sv() { echo "$s" | grep -o "$1=[0-9]*" | cut -d= -f2; }
  fk=$(echo "$b" | grep -o 'fk=[0-9]*' | cut -d= -f2); ic=$(echo "$b" | grep -o 'integrity=[a-z]*' | cut -d= -f2)
  ikinci="—"; [ -f "$R/${AY}_denetim.md" ] && grep -q "## İkinci aktarım" "$R/${AY}_denetim.md" && ikinci="aynı"
  sert="yok"; [ $rc -ne 0 ] && sert="**VAR**"
  durum="tamamlandı"
  [ "$ig" != 0 ] && durum="inceleme var"
  [ "$ek" != 0 ] && durum="eksik var"
  [ "$ak" != 0 ] && durum="aktarım bekliyor"
  [ "$hd" != 0 ] && durum="denenmedi"
  [ "$hd" = "$cift" ] && durum="henüz başlanmadı"
  tablo+="| $AY | $cift | $ta | $ig | $ek | $ak | $hd | $dis | $(sv kosu) | $(sv katilim) | $(sv altili) | $fk | $ic | $ikinci | $sert | $durum |"$'\n'
  T[0]=$((T[0]+cift)); T[1]=$((T[1]+ta)); T[2]=$((T[2]+ig)); T[3]=$((T[3]+ek)); T[4]=$((T[4]+hd)); T[5]=$((T[5]+ak))
  # TAMAM dışındaki çiftler (denetim §1 tablosundan)
  sat=$(sed -n '/^## 1\./,/^## 2\./p' "$md" | grep -E '^\| [0-9]{4}-' | grep -v '| TAMAMLANDI |' | grep -v '| HENUZ_DENENMEDI |' | grep -v '| AKTARILMADI |')
  hdl=$(sed -n '/^## 1\./,/^## 2\./p' "$md" | grep -E '\| HENUZ_DENENMEDI \|' | awk -F'|' '{gsub(/ /,"",$2); gsub(/^ +| +$/,"",$3); printf "%s %s, ", $2, $3}' | sed 's/, $//')
  ayrinti+="### $AY"$'\n\n'
  [ -n "$sat" ] && ayrinti+="| Tarih | Hipodrom | Durum | Yarış günü no | Ayrıntı |"$'\n'"|---|---|---|---|---|"$'\n'"$sat"$'\n\n'
  [ -n "$hdl" ] && ayrinti+="Henüz denenmeyen: $hdl"$'\n\n'
  [ -z "$sat" ] && [ -z "$hdl" ] && ayrinti+="Takvimdeki tüm çiftler TAMAMLANDI."$'\n\n'
  er=$(sed -n '/^## 5\./,/^## 6\./p' "$md" | grep -c '^- `')
  [ "$er" != 0 ] && ayrinti+="Planlandığı gün dışında koşulan koşu: $er (ayrıntı: \`raporlar/${AY}_denetim.md\` §5)."$'\n\n'
  ob=$(sed -n '/^## 7\./,/^## 8\./p' "$md" | grep -o 'birden çok kazanan kombinasyonlu: [^;]*')
  [ -n "$ob" ] && [ "$ob" != "birden çok kazanan kombinasyonlu: yok" ] && ayrinti+="Altılı, ${ob} (her kombinasyon kendi ödemesine bağlı)."$'\n\n'
done
DB="${TJK_DB:-$KOK/calisma/tjk_yillik.sqlite}"
{
echo "# Yıllık durum: 22.11.2025 – 22.11.2026"
echo
echo "Üretim: \`kontrol/yillik/durum_yaz.sh\` (her ay için \`tjk_denetim\`, veri tabanı salt okunur) · $(date -u +%Y-%m-%dT%H:%MZ)"
echo "Veri tabanı: \`$(basename "$DB")\` · SHA-256 \`$(sha256sum "$DB" | cut -c1-64)\`"
echo
echo "Takvim: resmî 2025 ve 2026 PDF'leri (\`kontrol/yillik/takvim/KAYNAKLAR.md\`), 760 çift. Sınıflar: TAMAMLANDI (gün TAMAM),"
echo "INCELEME_GEREKLI (gün aktarıldı, gerekçeli açık kayıt var), EKSIK (denendi, dosya alınamadı/doğrulanamadı),"
echo "AKTARILMADI (iki dosya doğrulandı, aktarım bekliyor), HENUZ_DENENMEDI (indirme kaydı yok)."
echo
echo "| Ay | Takvim çifti | Tamam | İnceleme | Eksik | İndirildi, aktarılmadı | Denenmedi | Takvim dışı | Koşu | Katılım | Altılı | FK | integrity | 2. aktarım | Sert hata | Durum |"
echo "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
printf "%s" "$tablo"
echo "| **Toplam** | **${T[0]}** | **${T[1]}** | **${T[2]}** | **${T[3]}** | **${T[5]}** | **${T[4]}** | | | | | | | | | |"
echo
echo "## Tamamlanmamış çiftler (ay bazında)"
echo
printf "%s" "$ayrinti"
[ -f "$KOK/kontrol/yillik/notlar.md" ] && cat "$KOK/kontrol/yillik/notlar.md"
} > "$O"
echo "yazıldı: $O (toplam ${T[0]} çift: tamam ${T[1]}, inceleme ${T[2]}, eksik ${T[3]}, aktarılmadı ${T[5]}, denenmedi ${T[4]})"
