//! Placeholder: the legacy single-struct `AdaptConfig` +
//! `AdaptMetrics` shape retired in Pass 5 of runtime megaround
//! `202605101036` per the no-legacy-shims rule. The replacement is
//! the nine per-axis `AdaptAxis` `BuilderInput` configs in
//! `hilavitkutin_api::adapt` plus the nine per-axis metrics
//! Resources in `hilavitkutin_providers::metrics`. Pass 7 ships the
//! integration tests that exercise the new axis surface.

#[test]
fn _axis_surface_validated_in_pass_7() {
    // empty body; the new contract is the per-axis `BuilderInput`
    // configs and per-axis metrics Resources. Pass 7 integration
    // tests cover the wiring.
}
