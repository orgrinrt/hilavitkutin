#!/bin/sh
# The fixture member's checks, for `tests/stage_script.rs`. Touches the file
# `$STAGE_FIXTURE_CHECKS_RAN` names when it is set, which is what tells a
# refusal that never reached the checks from one that ran them, and fails when
# `$STAGE_FIXTURE_CHECKS_FAIL` is set.
set -eu
if [ -n "${STAGE_FIXTURE_CHECKS_RAN:-}" ]; then
    : >"$STAGE_FIXTURE_CHECKS_RAN"
fi
[ -z "${STAGE_FIXTURE_CHECKS_FAIL:-}" ]
