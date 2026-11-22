#!/bin/bash
# Açık çiftler için resmî CDN'deki alternatif belge adreslerini yoklar (HEAD). 403/429/5xx'te durur.
# Çıktı: tarih, hipodrom, klasör, rol, dil, HTTP, bayt, content-type
CIKTI="$1"; : > "$CIKTI"
B=https://medya-cdn.tjk.org/raporftp/TJKPDF
while IFS='|' read -r T H; do
  Y=${T:0:4}; D="${T:8:2}.${T:5:2}.${T:0:4}"
  for K in CSV PDF PDFOzetSonuc; do
   for R in GunlukYarisProgrami GunlukYarisSonuclari; do
    for L in TR EN; do
      [ "$K" = CSV ] && [ "$L" = EN ] && continue
      [ "$K" = PDFOzetSonuc ] && [ "$R" = GunlukYarisProgrami ] && continue
      E=$([ "$K" = CSV ] && echo csv || echo pdf)
      U="$B/$Y/$T/$K/$R/$D-$H-$R-$L.$E"
      O=$(curl -sS -A "tjk-arsiv-araci/0.1" -o /dev/null -I -w "%{http_code}\t%{size_download}\t%{content_type}\t%header{content-length}" "$U" 2>&1)
      c=${O%%$'\t'*}
      printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$T" "$H" "$K/$R/$L" "$c" "$O" "$U" >> "$CIKTI"
      case $c in 403|429|502|503|504) echo "DUR $c $U" >> "$CIKTI"; exit 3;; esac
      sleep 0.7
    done
   done
  done
done <<'LIST'
2026-01-11|Adana
2026-02-28|İstanbul
2026-03-15|Adana
2026-04-03|İzmir
2026-10-28|Elazığ
2025-12-31|Şanlıurfa
2026-04-18|Ankara
2026-09-28|Bursa
2026-09-29|Kocaeli
2026-10-16|Bursa
2026-10-19|Bursa
LIST
echo DONE >> "$CIKTI"
