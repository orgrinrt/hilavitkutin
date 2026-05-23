//! Placeholder: `PipelineResult` retired in Pass 6 of runtime
//! megaround `202605101036` per the no-legacy-shims rule. Pipeline
//! result now flows through `RunCfg::Out` directly; Pass 7 ships
//! the integration tests that exercise it on the new shape.

#[test]
fn _runcfg_out_shape_validated_in_pass_7() {
    // empty body; the new contract is the `RunCfg::Out` associated
    // type carried by `Scheduler::run`. Pass 7 integration tests
    // exercise it end-to-end.
}
