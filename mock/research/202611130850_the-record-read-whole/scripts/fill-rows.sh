#!/usr/bin/env bash
# Fill `quote` (and, on a tenet, `provenance`) of registry rows from their
# `source` locator, so the quoted lines are the file's lines by construction.
#
# usage: fill-rows.sh <registry toml>...
#
# A row is authored with `source = "<path>:<first>-<last>[, ...]"` and without
# `quote` or `provenance`; a re-run drops the two and writes them again. A path
# led by `<branch>:` is read from that branch (`git show`), every other path
# from the working tree.
set -eu
export LC_ALL=C.UTF-8
repo=$(git rev-parse --show-toplevel)
ledger_anchor='mock::research::202611130850_the-record-read-whole::LEDGER::#read-from-the-recovery-branch'

lines_of() { # <branch or empty> <path> <first> <last>
  if [ -n "$1" ]; then
    git -C "$repo" show "$1:$2" | sed -n "${3},${4}p"
  else
    sed -n "${3},${4}p" "$repo/$2"
  fi
}

slug_of() { # heading title on stdin -> mockspace's heading slug
  sed -E 's/[^[:alnum:]]+/-/g; s/^-+//; s/-+$//' | tr '[:upper:]' '[:lower:]'
}

provenance_of() { # <branch or empty> <path> <first>
  local branch=$1 path=$2 first=$3 cite head slug hits
  if [ -n "$branch" ]; then printf '%s' "$ledger_anchor"; return; fi
  cite=${path%.tmpl}; cite=${cite%.md}
  case $cite in mock/*) cite="mock::${cite#mock/}" ;; *) cite="live::$cite" ;; esac
  cite=$(printf '%s' "$cite" | sed 's#/#::#g')
  while IFS= read -r head; do
    slug=$(printf '%s' "$head" | slug_of)
    [ -n "$slug" ] || continue
    hits=$(grep -E '^ *#+ ' "$repo/$path" | sed -E 's/^ *#+ *//' | slug_of | grep -c -x -F -- "$slug" || true)
    if [ "$hits" = 1 ]; then printf '%s::#%s' "$cite" "$slug"; return; fi
  done < <(sed -n "1,${first}p" "$repo/$path" | grep -E '^ *#+ ' | sed -E 's/^ *#+ *//' | tac)
  printf '%s::%s' "$cite" "$first"
}

fill() {
  local file=$1 tmp kind="" skipping=0 line src
  tmp=$(mktemp)
  while IFS= read -r line || [ -n "$line" ]; do
    if [ "$skipping" = 1 ]; then
      [ "$line" = "'''" ] && skipping=0
      continue
    fi
    case $line in
      '[[tenet]]') kind=tenet ;;
      '[[question]]') kind=question ;;
      '[['*) kind=other ;;
    esac
    case $line in
      "quote = '''") skipping=1; continue ;;
      'provenance = '*) [ "$kind" = tenet ] && continue ;;
    esac
    printf '%s\n' "$line" >> "$tmp"
    case $line in
      'source = "'*'"')
        src=${line#source = \"}; src=${src%\"}
        local first_done=0 quote="" part branch path range a b text
        local IFS_old=$IFS
        while [ -n "$src" ]; do
          part=${src%%, *}
          if [ "$part" = "$src" ]; then src=""; else src=${src#*, }; fi
          range=${part##*:}; path=${part%:*}; branch=""
          case $path in *:*) branch=${path%%:*}; path=${path#*:} ;; esac
          a=${range%-*}; b=${range#*-}
          text=$(lines_of "$branch" "$path" "$a" "$b"; printf x); text=${text%x}
          [ -n "$text" ] || { echo "fill-rows: $file: nothing at $part" >&2; rm -f "$tmp"; exit 1; }
          case $text in *"'''"*) echo "fill-rows: $file: $part holds a triple quote" >&2; rm -f "$tmp"; exit 1 ;; esac
          if [ "$first_done" = 0 ]; then
            [ "$kind" = tenet ] && printf 'provenance = ["%s"]\n' "$(provenance_of "$branch" "$path" "$a")" >> "$tmp"
            first_done=1
          else
            quote="${quote}---
"
          fi
          quote="${quote}${text}"
        done
        IFS=$IFS_old
        printf "quote = '''\n%s'''\n" "$quote" >> "$tmp"
        ;;
    esac
  done < "$file"
  if cmp -s "$tmp" "$file"; then rm -f "$tmp"; else cat "$tmp" > "$file"; rm -f "$tmp"; echo "filled $file"; fi
}

for f in "$@"; do fill "$f"; done
