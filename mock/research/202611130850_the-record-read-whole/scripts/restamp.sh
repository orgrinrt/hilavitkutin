#!/usr/bin/env bash
# Move one tenet from `ratified` to `provisional` on a stamp, or give a
# stamped tenet a new stamp.
#
# usage: restamp.sh <registry toml> <tenet id> <stamp file> [<superseded_by id>]
#
# The stamp file holds the stamp's text on one line, as the row will carry it.
# `ratified_by` and `ratified_from` go, since a stamped row rests on no
# ratification; the stamp names the passages instead. `stamped_by = "experts"`
# and `stamp` are written after `standing`, replacing ones that are there. The
# stamp names its question ticket as ticket `<id>`, as the lint
# `a-provisional-stamp-is-not-a-ratification` holds it to, and one that does
# not is refused. With a fourth argument the row gains
# `superseded_by = ["<id>"]`, unless it has the field already. Refuses a row that
# is not in the file, and a stamp holding a single quote, which the literal
# string the rows use cannot carry.
set -eu
export LC_ALL=C.UTF-8
file=$1 id=$2 stampfile=$3 heir=${4:-}
stamp=$(cat "$stampfile")
case $stamp in *"'"*) echo "restamp: the stamp holds a single quote" >&2; exit 2 ;; esac
case $stamp in *"ticket \`"*) ;; *) echo "restamp: the stamp names no question ticket as ticket \`<id>\`" >&2; exit 2 ;; esac
grep -q "^id = \"$id\"$" "$file" || { echo "restamp: no tenet $id in $file" >&2; exit 1; }
tmp=$(mktemp)
awk -v id="$id" -v stamp="$stamp" -v heir="$heir" '
  /^\[\[/ { if (inrow && heir != "" && !hassup) print "superseded_by = [\"" heir "\"]"; inrow = 0; hassup = 0 }
  $0 == "id = \"" id "\"" { inrow = 1 }
  inrow && /^ratified_by = / { next }
  inrow && /^ratified_from = / { next }
  inrow && /^stamped_by = / { next }
  inrow && /^stamp = / { next }
  inrow && /^superseded_by = / { hassup = 1 }
  inrow && /^standing = / { print "standing = \"provisional\""; print "stamped_by = \"experts\""; print "stamp = \x27" stamp "\x27"; next }
  { print }
  END { if (inrow && heir != "" && !hassup) print "superseded_by = [\"" heir "\"]" }
' "$file" > "$tmp"
mv "$tmp" "$file"
