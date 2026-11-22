#!/bin/bash
# SQLite aktarım sürücüsü (Mart'taki aktar.sh ile aynı mantık; ham klasörü parametre).
# Kullanım: aktar.sh GIRDILER.tsv HAM_KLASORU DB CIKTI_KLASORU
# GIRDILER satırı: gun_id<TAB>program<TAB>sonuc<TAB>ek   (ek: "--ek-sonuc DOSYA [--ek-sonuc DOSYA2]|erteleme notu" ya da boş)
# Her gün mevcut aracın tek işleminde silinip yeniden yazılır (tjk_ayristirici ... --sqlite DB).
set -u
KOK="$(cd "$(dirname "$0")/../.." && pwd)"
AYR="$KOK/tjk_ayristirici/target/release/tjk_ayristirici"
G="$1"; HAM="$2"; DB="$3"; OUT="$4"
mkdir -p "$OUT"
toplam=0; hata=0; tamam=0; incele=0
while IFS=$'\t' read -r gun p r ek || [ -n "${gun:-}" ]; do
  [ -z "$gun" ] && continue
  case "$gun" in \#*) continue;; esac
  toplam=$((toplam+1))
  args=("$HAM/$p" "$HAM/$r" "$OUT/$gun" --sqlite "$DB")
  if [ -n "$ek" ]; then
    eks=${ek%%|*}; notu=${ek#*|}
    set -- $eks
    while [ $# -gt 0 ]; do
      if [ "$1" = "--ek-sonuc" ]; then args+=(--ek-sonuc "$HAM/$2"); shift 2; else echo "bilinmeyen ek: $1" >&2; exit 2; fi
    done
    [ -n "$notu" ] && args+=(--ertelenme-notu "$notu")
  fi
  if "$AYR" "${args[@]}" > "$OUT/$gun.log" 2>&1; then
    d=$(grep -m1 '^GÜN DURUMU' "$OUT/$gun.log" | sed 's/.*: *//')
    [ "$d" = "TAMAM" ] && tamam=$((tamam+1)) || incele=$((incele+1))
  else
    hata=$((hata+1)); echo "HATA $gun (çıkış $?): $(tail -3 "$OUT/$gun.log" | tr '\n' ' ')"
  fi
done < "$G"
echo "AKTARIM $G: $toplam gün, TAMAM $tamam, INCELEME $incele, HATA $hata"
[ "$hata" -eq 0 ]
