//! Placeholder: `ThreadPool::new(core_count, wake_strategy)`
//! retired in Pass 4 of runtime megaround `202605101036` in favour
//! of the `ThreadPool::builder()` flow with
//! `.with_wake_strategy(...)`. Pass 7 ships the integration tests
//! that exercise the new builder shape.

#[test]
fn _threadpool_builder_validated_in_pass_7() {
    // empty body; the new contract is `ThreadPool::builder()`. The
    // legacy two-arg `::new(core_count, wake_strategy)` retired
    // per the no-legacy-shims rule.
}
