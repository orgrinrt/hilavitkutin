#!/usr/bin/env bash
# workflow-shape.sh : puts the tenet rows in the shape the pack's registry lints hold.
#   cites    keeps only the other tenets the row's `what` and `because` name by id,
#            in the order named; every other entry moves to `related` as `tenet::<id>`
#   related  drops a `tenet::<id>` the words name, the row itself and repeats
#   rules_out is renamed `rejects`: the readers wrote prose there, and the lint
#            `a-retired-word-stays-out` reads the field as words banned from the tree
#   subject  moves up beside `id`, so a quote line opening with `[` cannot hide it
#            from a reader that takes such a line for a table header
# Idempotent. Run from the hilavitkutin clone. Prints counts on stderr.
set -euo pipefail
ids=$(mktemp)
awk '
  { c = gsub(/\047\047\047/, "&"); if (c % 2 == 1) { inq = !inq; next } }
  inq { next }
  /^\[\[tenet\]\]$/ { t = 1; next }
  /^\[\[/ { t = 0; next }
  t && /^id = "/ { s = $0; sub(/^id = "/, "", s); sub(/"$/, "", s); print s }
' mock/registry/tenet/*.toml > "$ids"
for f in mock/registry/tenet/*.toml; do
  awk -v IDS="$ids" '
    BEGIN { while ((getline line < IDS) > 0) isid[line] = 1 }
    function items(line, arr,   s, n, i) {
      s = line; sub(/^[a-z_]+ = \[/, "", s); sub(/\] *$/, "", s)
      if (s ~ /^ *$/) return 0
      sub(/^ *"/, "", s); sub(/" *$/, "", s)
      n = split(s, arr, /", *"/)
      return n
    }
    function flush(   i, n, a, text, tok, m, named, nn, seen, id, cl, rl, rest, nr, keep, k, v, out, ci, ri, si, ii, anchor, skip) {
      if (nb == 0) return
      id = ""; ci = 0; ri = 0; si = 0; ii = 0; text = ""
      q = 0
      for (i = 1; i <= nb; i++) {
        l = buf[i]
        c = gsub(/\047\047\047/, "&", l)
        if (q) { if (c % 2 == 1) q = 0; continue }
        if (c % 2 == 1) { q = 1; continue }
        if (l ~ /^id = "/) { ii = i; id = l; sub(/^id = "/, "", id); sub(/"$/, "", id) }
        else if (l ~ /^(what|because) = /) text = text " " l
        else if (l ~ /^cites = \[/) ci = i
        else if (l ~ /^related = \[/) ri = i
        else if (l ~ /^subject = /) si = i
        else if (l ~ /^rules_out = /) { sub(/^rules_out = /, "rejects = ", buf[i]); renamed++ }
      }
      if (id != "" && istenet) {
        # the ids the words name, in order, each once, never the row itself
        nn = 0; split("", seen)
        rest = text
        while (match(rest, /[A-Za-z0-9_]+/)) {
          tok = substr(rest, RSTART, RLENGTH); rest = substr(rest, RSTART + RLENGTH)
          if ((tok in isid) && tok != id && !(tok in seen)) { seen[tok] = 1; named[++nn] = tok }
        }
        # related: what stands, less what the words name, the row itself and repeats
        nr = 0; split("", inrel)
        if (ri) {
          n = items(buf[ri], a)
          for (k = 1; k <= n; k++) {
            v = a[k]
            if (v == "tenet::" id) continue
            if (v ~ /^tenet::/ && (substr(v, 8) in seen)) continue
            if (v in inrel) continue
            inrel[v] = 1; rel[++nr] = v
          }
        }
        if (ci) {
          n = items(buf[ci], a)
          for (k = 1; k <= n; k++) {
            v = a[k]
            if (v == id || (v in seen)) continue
            if (("tenet::" v) in inrel) continue
            inrel["tenet::" v] = 1; rel[++nr] = "tenet::" v; moved++
          }
        }
        cl = ""
        if (nn) { cl = "cites = ["; for (k = 1; k <= nn; k++) cl = cl (k > 1 ? ", " : "") "\"" named[k] "\""; cl = cl "]" }
        rl = ""
        if (nr) { rl = "related = ["; for (k = 1; k <= nr; k++) rl = rl (k > 1 ? ", " : "") "\"" rel[k] "\""; rl = rl "]" }
        if (ci) { buf[ci] = cl; drop[ci] = (cl == "") }
        if (ri) { buf[ri] = rl; drop[ri] = (rl == "") }
        if (!ci && cl != "") extra = cl
        if (!ri && rl != "") extra = (extra != "" ? extra "\n" : "") rl
      }
      anchor = ci ? ci : (ri ? ri : (si ? si : ii))
      for (i = 1; i <= nb; i++) {
        skip = ((i in drop) && drop[i]) || (si && i == si && si != ii + 1)
        if (!skip) print buf[i]
        if (si && i == ii && si != ii + 1) print buf[si]
        if (i == anchor && extra != "") printf "%s\n", extra
      }
      nb = 0; extra = ""; done = 0; split("", drop); split("", buf)
    }
    {
      line = $0
      c = gsub(/\047\047\047/, "&", line)
      if (inq) { buf[++nb] = $0; if (c % 2 == 1) inq = 0; next }
      if ($0 ~ /^\[\[/) { flush(); istenet = ($0 == "[[tenet]]") }
      buf[++nb] = $0
      if (c % 2 == 1) inq = 1
    }
    END { flush(); printf "%s moved=%d renamed=%d\n", FILENAME, moved, renamed > "/dev/stderr" }
  ' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
done 2>&1 | awk '{ for (i = 2; i <= NF; i++) { split($i, kv, "="); t[kv[1]] += kv[2] } } END { printf "cites moved to related: %d; rules_out renamed: %d\n", t["moved"], t["renamed"] }'
rm -f "$ids"
