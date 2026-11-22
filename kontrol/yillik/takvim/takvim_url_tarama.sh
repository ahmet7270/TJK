#!/bin/bash
# HEAD probe for TJK calendar PDFs on medya-cdn (robots.txt 404 = no rules)
out=$1; shift
for pat in "$@"; do
 for m in 01 02 03 04 05 06 07 08 09 10 11 12; do for d in $(seq -w 1 31); do
  u=$(echo "$pat" | sed "s/DD/$d/;s/MM/$m/")
  c=$(curl -sS -o /dev/null -I -w '%{http_code}' "https://medya-cdn.tjk.org/haberftp/$u")
  echo "$u $c" >> $out
  case $c in 403|429|502|503|504) echo "STOP $u $c" >> $out; exit 3;; esac
  sleep 0.3
 done; done
done
echo DONE >> $out
