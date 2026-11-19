#!/usr/bin/env bash
# Is branch A contained in branch B (ancestor, or every patch of A cherry-equivalent in B)?
r=${1:?the hilavitkutin clone}; shift
a=$1; shift
for b in "$@"; do
  if git -C "$r" merge-base --is-ancestor "origin/$a" "origin/$b"; then echo "$a ancestor of $b"; continue; fi
  plus=$(git -C "$r" cherry "origin/$b" "origin/$a" | grep -c '^+')
  echo "$a vs $b: $plus patches not in $b"
done
