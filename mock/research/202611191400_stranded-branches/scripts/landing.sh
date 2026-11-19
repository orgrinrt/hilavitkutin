#!/usr/bin/env bash
# For each branch: find dev commits carrying any of its subjects (squash landings), and
# diff the branch tip against that landing over the paths the branch changed.
r=${1:?the hilavitkutin clone}; shift
for b in "$@"; do
  base=$(git -C "$r" merge-base origin/dev "origin/$b")
  tip=$(git -C "$r" log -1 --format=%s "origin/$b")
  echo "== $b  (tip: $tip)"
  hit=""
  while IFS= read -r s; do
    h=$(git -C "$r" log origin/dev --format='%h' --fixed-strings --grep="$s" -n1 "$base..origin/dev" 2>/dev/null)
    if [ -n "$h" ]; then
      hs=$(git -C "$r" log -1 --format=%s "$h")
      if [ "$hs" = "$s" ]; then hit="$h"; echo "   landed as $h: $s"; break; fi
    fi
  done < <(git -C "$r" log --format=%s "$base..origin/$b")
  mapfile -t paths < <(git -C "$r" diff --name-only "$base" "origin/$b")
  if [ -n "$hit" ]; then
    n=$(git -C "$r" diff --name-only "$hit" "origin/$b" -- "${paths[@]}" | wc -l)
    m=$(git -C "$r" diff --name-only origin/dev "origin/$b" -- "${paths[@]}" | wc -l)
    echo "   paths ${#paths[@]}; differing from landing $n; from dev $m"
  else
    m=$(git -C "$r" diff --name-only origin/dev "origin/$b" -- "${paths[@]}" | wc -l)
    echo "   no landing found; paths ${#paths[@]}; differing from dev $m"
  fi
done
