#!/usr/bin/env bash
# reanchor-map.sh <old commit> <ranges file> <changed files> : for every cited
# `path:a-b` whose path changed since <old commit>, prints `path:a-b<TAB>path:c-d`
# where the old lines stand verbatim, once, at c-d in the working tree,
# `path:a-b<TAB>SAME` where they did not move, and `path:a-b<TAB>LOST` otherwise.
set -euo pipefail
old=$1; ranges=$2; changed=$3
blk=$(mktemp)
while IFS= read -r r; do
  path=${r%:*}; span=${r##*:}
  grep -qxF -- "$path" "$changed" || continue
  a=${span%-*}; b=${span#*-}
  [ -f "$path" ] || { printf '%s\tLOST\n' "$r"; continue; }
  git show "$old:$path" 2>/dev/null | sed -n "${a},${b}p" > "$blk" || { printf '%s\tLOST\n' "$r"; continue; }
  if [ ! -s "$blk" ]; then printf '%s\tLOST\n' "$r"; continue; fi
  if sed -n "${a},${b}p" "$path" | cmp -s - "$blk"; then printf '%s\tSAME\n' "$r"; continue; fi
  hit=$(awk -v B="$blk" '
    BEGIN { while ((getline l < B) > 0) blk[++n] = l }
    { f[NR] = $0 }
    END { for (i = 1; i + n - 1 <= NR; i++) { ok = 1; for (j = 1; j <= n; j++) if (f[i + j - 1] != blk[j]) { ok = 0; break } if (ok) { c++; at = i } }
          if (c == 1) print at; else print "x" c }' "$path")
  case $hit in
    x*) printf '%s\tLOST\n' "$r" ;;
    *) n=$((b - a)); if [ "$a" = "$b" ] && [ "${span#*-}" = "$span" ]; then printf '%s\t%s:%s\n' "$r" "$path" "$hit"; else printf '%s\t%s:%s-%s\n' "$r" "$path" "$hit" "$((hit + n))"; fi ;;
  esac
done < "$ranges"
rm -f "$blk"
