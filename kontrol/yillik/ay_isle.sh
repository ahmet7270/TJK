#!/bin/bash
# Aylık iş akışı (kaldığı yerden devam eder). Kullanım:
#   ay_isle.sh indir AY     # takvim -> aday liste -> daraltma -> toplu indirme + içerik doğrulama + ayrıştırma
#   ay_isle.sh aktar AY     # girdiler -> 1. aktarım -> denetim -> 2. aktarım -> denetim -> döküm karşılaştırması
#   ay_isle.sh denetim AY   # yalnızca denetim (veri tabanını değiştirmez)
# AY: YYYY-AA. Takvim: kontrol/yillik/takvim/takvim_hedef.tsv (22.11.2025–22.11.2026 resmî çiftler).
# Özel girdiler (ertelenen koşular vb.): kontrol/yillik/listeler/AY_ozel.tsv (aktar.sh biçimi).
# Mart 2026: indirme yok; kontrol/mart_sqlite_girdiler.tsv + kontrol/ham (içerik/SHA-256 ile yeniden kullanım).
set -u
ISLEM="$1"; AY="$2"
KOK="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="$KOK/tjk_ayristirici/target/release"; FETCH="$KOK/tjk-fetch/target/release/tjk-fetch"
DB="${TJK_DB:-$KOK/calisma/tjk_yillik.sqlite}"
TAKVIM="$KOK/kontrol/yillik/takvim/takvim_hedef.tsv"
L="$KOK/kontrol/yillik/listeler"; R="$KOK/kontrol/yillik/raporlar"; W="$KOK/calisma/$AY"
mkdir -p "$L" "$R" "$W"
if [ "$AY" = "2026-03" ]; then
  HAM="$KOK/kontrol/ham"; GIRDI="$KOK/kontrol/mart_sqlite_girdiler.tsv"
  DURUM=(--durum "$KOK/kontrol/mart_indirme_durumu.tsv" --durum "$L/2026-03_durum_ek.tsv")
else
  HAM="$KOK/kontrol/ham/$AY"; GIRDI="$L/${AY}_girdiler.tsv"
  DURUM=(--durum "$L/${AY}_durum.tsv")
fi

denetim() {  # $1 = etiket
  "$BIN/tjk_denetim" --db "$DB" --ay "$AY" --takvim "$TAKVIM" "${DURUM[@]}" \
     --cikti "$W/denetim_$1.md" --dokum "$W/dokum_$1.txt"
}

case "$ISLEM" in
indir)
  [ "$AY" = "2026-03" ] && { echo "Mart: indirme yok (yeniden kullanım)"; exit 0; }
  { head -1 "$TAKVIM"; grep "^$AY-" "$TAKVIM"; } > "$L/${AY}_takvim.tsv"
  n=$(($(wc -l < "$L/${AY}_takvim.tsv") - 1))
  gunler=$(grep "^$AY-" "$TAKVIM" | cut -f1 | cut -c9-10 | sed 's/^0//' | sort -n -u | paste -sd,)
  echo "$AY: takvimde $n çift; günler $gunler"
  "$BIN/tjk_indir" liste --ay "$AY" --gunler "$gunler" --cikti "$W/aday.tsv" || exit 1
  "$FETCH" daralt "$L/${AY}_takvim.tsv" "$W/aday.tsv" 0000-00-00 0000-00-00 "$n" 0 "$L/${AY}_liste.tsv" > "$W/daralt.txt" || { cat "$W/daralt.txt"; exit 1; }
  tail -3 "$W/daralt.txt"
  mkdir -p "$HAM"
  "$FETCH" toplu --liste "$L/${AY}_liste.tsv" --ham "$HAM" --durum "$L/${AY}_durum.tsv" \
     --ayristirici "$BIN/tjk_ayristirici" --cikti "$W/toplu" --rapor "$R/${AY}_indirme.md" \
     --hata "$W/hata.txt" --beklenen-cift "$n" --bekle-ms "${BEKLE_MS:-1500}" > "$W/toplu.txt" 2>&1
  rc=$?
  grep -E "^ÖZET|^robots" "$W/toplu.txt"
  if [ $rc -eq 3 ]; then cp "$W/hata.txt" "$KOK/kontrol/son_hata.txt"; echo "ERİŞİM HATASI — indirme durdu:"; cat "$W/hata.txt"; exit 3; fi
  grep -E ": (INCELEME_GEREKLI|EKSIK)" "$W/toplu.txt" || true
  exit $rc
  ;;
aktar)
  if [ "$AY" != "2026-03" ]; then
    "$FETCH" girdiler --liste "$L/${AY}_liste.tsv" --durum "$L/${AY}_durum.tsv" --ham "$HAM" \
       --cikti "$GIRDI" --ozel "$L/${AY}_ozel.tsv" || exit 1
  fi
  mkdir -p "$KOK/calisma/yedek"
  sqlite_kopya() { cp "$DB" "$1" && sha256sum "$1" | cut -c1-64; }
  echo "aktarım öncesi yedek: $(sqlite_kopya "$KOK/calisma/yedek/once_${AY}.sqlite")"
  bash "$KOK/kontrol/yillik/aktar.sh" "$GIRDI" "$HAM" "$DB" "$W/aktar1" || exit 1
  denetim 1 > "$W/denetim_1.txt"; d1=$?
  bash "$KOK/kontrol/yillik/aktar.sh" "$GIRDI" "$HAM" "$DB" "$W/aktar2" || exit 1
  denetim 2 > "$W/denetim_2.txt"; d2=$?
  cat "$W/denetim_2.txt"
  if cmp -s "$W/dokum_1.txt" "$W/dokum_2.txt"; then echo "İKİNCİ AKTARIM: 16 tablonun tam dökümü AYNI"; else echo "İKİNCİ AKTARIM: DÖKÜM FARKLI"; exit 1; fi
  cp "$W/denetim_2.md" "$R/${AY}_denetim.md"
  { echo; echo "## İkinci aktarım"; echo; echo "Aynı girdilerle ikinci aktarımdan sonra tüm tabloların sıralı tam dökümü birinciyle bayt bayt aynı (\`cmp\`)."; } >> "$R/${AY}_denetim.md"
  echo "aktarım sonrası yedek: $(sqlite_kopya "$KOK/calisma/yedek/sonra_${AY}.sqlite")"
  [ $d1 -eq 0 ] && [ $d2 -eq 0 ]
  ;;
denetim)
  denetim 0; cp "$W/denetim_0.md" "$R/${AY}_denetim.md"
  ;;
*) echo "işlem: indir|aktar|denetim"; exit 2;;
esac
