#!/usr/bin/env bash
# For each branch: record files it adds (design_rounds, research, benches) whose blob is
# nowhere in dev's tree and nowhere in dev's history (deleted on purpose later).
r=${1:?the hilavitkutin clone}; shift
git -C "$r" ls-tree -r origin/dev | awk '{print $3}' | sort -u > ${TMPDIR:-/tmp}/devblobs.txt
git -C "$r" log --format= --raw --no-abbrev origin/dev | awk '{print $4}' | sort -u > ${TMPDIR:-/tmp}/devhistblobs.txt
for b in "$@"; do
  base=$(git -C "$r" merge-base origin/dev "origin/$b")
  intree=0; inhist=0; never=(); tot=0
  while IFS= read -r p; do
    tot=$((tot+1))
    blob=$(git -C "$r" rev-parse "origin/$b:$p")
    if grep -qxF "$blob" ${TMPDIR:-/tmp}/devblobs.txt; then intree=$((intree+1))
    elif grep -qxF "$blob" ${TMPDIR:-/tmp}/devhistblobs.txt; then inhist=$((inhist+1))
    else never+=("$p"); fi
  done < <(git -C "$r" diff --name-only --diff-filter=A "$base" "origin/$b" -- mock/design_rounds mock/research mock/benches)
  echo "== $b: added $tot; blob in dev tree $intree; once on dev then removed $inhist; never on dev ${#never[@]}"
  for p in "${never[@]:0:12}"; do echo "   $p"; done
done
