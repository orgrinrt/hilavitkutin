#!/bin/sh
# Build one bench member and copy what it built into a directory of its own,
# so a measurement reads binaries nobody else can replace.
#
# Every worktree of this repository builds into the one shared target
# directory, so a build in any of them replaces the member's driver and every
# arm cdylib while a measurement in another is reading them. The run does not
# fail and does not notice: the arms still load, the bridge hash still matches,
# and the artifact it writes names a build that no longer exists. A run reads a
# staged directory instead, and checks the checksums written at staging before
# it starts and again when it ends.
#
# The staged layout is `<dir>/release/<file>`, the same shape as the target
# directory, so a driver run from there finds its arms beside itself the way
# it does in the target directory. Beside `release/` are `binaries.sha256`,
# checked by `--verify`, and `build.txt`, naming the commit the binaries were
# built from and every path that differed from it, since a result from a
# dirty tree is only worth what the record of its dirt is.
#
# usage: stage.sh <member-dir> [<out-dir>]
#        stage.sh --verify <out-dir>
#
# The first form builds `<member-dir>` in release (`--workspace`, so a member
# that is one workspace stages every arm in one call) and prints the staged
# directory, a fresh temporary one unless `<out-dir>` is given, which must not
# exist yet. Nothing else is printed on success; the build's own output goes to
# stderr only when it fails. The second form is silent when every checksum
# holds and names the files that changed when one does not.

set -eu

die() {
    echo "stage: $*" >&2
    exit 1
}

if [ "${1:-}" = --verify ]; then
    out="${2:?usage: stage.sh --verify <out-dir>}"
    [ -f "$out/binaries.sha256" ] || die "$out was not staged by this script (no binaries.sha256)"
    if ! changed=$(cd "$out" && sha256sum --quiet -c binaries.sha256 2>&1); then
        die "a staged binary changed after staging:
$changed"
    fi
    exit 0
fi

member="${1:?usage: stage.sh <member-dir> [<out-dir>]}"
member=$(cd "$member" && pwd)
[ -f "$member/Cargo.toml" ] || die "$member has no Cargo.toml"

if [ -n "${2:-}" ]; then
    out="$2"
    [ ! -e "$out" ] || die "$out exists; a stage is written once, into a directory nothing else holds"
    mkdir -p "$out"
else
    out=$(mktemp -d "${TMPDIR:-/tmp}/bench-stage.XXXXXX")
fi
out=$(cd "$out" && pwd)
mkdir "$out/release"

log=$(mktemp)
trap 'rm -f "$log"' EXIT INT TERM

# The artifacts cargo reports for this member's own packages, read from the
# build itself rather than from a target directory guessed before it: an
# executable, and every cdylib a variant crate emits.
if ! (cd "$member" && CARGO_INCREMENTAL=0 cargo build --release --workspace \
        --message-format=json-render-diagnostics) >"$log" 2>"$log.err"; then
    cat "$log.err" >&2
    rm -f "$log.err"
    die "the release build of $member failed"
fi
rm -f "$log.err"

artifacts=$(grep '"reason":"compiler-artifact"' "$log" \
    | grep "\"package_id\":\"path+file://$member[/#]" \
    | grep -o -e '"executable":"[^"]*"' -e '"[^"]*\.so"' -e '"[^"]*\.dylib"' \
    | sed -e 's/^"executable"://' -e 's/^"//' -e 's/"$//' \
    | sort -u)
[ -n "$artifacts" ] || die "the build of $member reported no executable and no cdylib"

for f in $artifacts; do
    cp -p "$f" "$out/release/"
done

(cd "$out" && find release -type f | sort | xargs sha256sum > binaries.sha256)

repo=$(git -C "$member" rev-parse --show-toplevel)
{
    echo "member: ${member#"$repo"/}"
    echo "commit: $(git -C "$repo" rev-parse HEAD)"
    echo "staged: $(date -u '+%Y-%m-%d %H:%M UTC')"
    echo "profile: release, as $member/Cargo.toml declares it"
    dirty=$(git -C "$repo" status --porcelain)
    if [ -n "$dirty" ]; then
        echo "dirty, against that commit:"
        echo "$dirty" | sed 's/^/  /'
    else
        echo "dirty: no"
    fi
} > "$out/build.txt"

echo "$out"
