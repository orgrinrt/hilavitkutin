# The shared half of every member's `run-checks.sh`, sourced by it after its
# own `set -eu`: `-v` handling, the silent `run`, and the member directory as
# the working directory.
#
# A member's checks are one command from its directory, silent on success and
# printing the failed command's whole output on failure. `stage.sh` runs them
# before it builds a member for timing.
#
#   -v  print each command and its output rather than only failures
#
# A member script calls `run "<label>" <command>...` once per check and ends
# with `checks_done`.

verbose=0
if [ "${1:-}" = "-v" ]; then
    verbose=1
fi

checks_log=$(mktemp)
trap 'rm -f "$checks_log"' EXIT INT TERM

run() {
    label="$1"
    shift
    if [ "$verbose" = 1 ]; then
        echo "+ $*"
        "$@" || { echo "run-checks: $label failed" >&2; exit 1; }
        return
    fi
    if ! "$@" >"$checks_log" 2>&1; then
        echo "run-checks: $label failed" >&2
        cat "$checks_log" >&2
        exit 1
    fi
}

checks_done() {
    if [ "$verbose" = 1 ]; then
        echo "run-checks: ok"
    fi
}

CARGO_INCREMENTAL=0
export CARGO_INCREMENTAL
