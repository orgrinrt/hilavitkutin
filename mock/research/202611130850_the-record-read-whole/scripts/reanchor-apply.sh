#!/usr/bin/env bash
# reanchor-apply.sh <moved map> <file>... : rewrites every cited `path:a-b` the
# map moves, whole tokens only, in one pass so no new range is rewritten again.
set -euo pipefail
map=$1; shift
MAP="$map" perl -i -pe '
  BEGIN { open(my $m, "<", $ENV{MAP}) or die; while (<$m>) { chomp; my ($o, $n) = split /\t/; $h{$o} = $n } close $m;
          $re = join("|", map { quotemeta } sort { length($b) <=> length($a) } keys %h) }
  s/($re)(?![0-9]|-[0-9])/$h{$1}/g if length $re;
' "$@"
