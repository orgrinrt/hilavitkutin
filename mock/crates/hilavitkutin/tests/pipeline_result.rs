//! PipelineResult variant tests.

use hilavitkutin::scheduler::PipelineResult;

#[test]
fn variants_are_distinct() {
    assert_ne!(PipelineResult::Completed, PipelineResult::Failed);
    assert_ne!(PipelineResult::Completed, PipelineResult::Poisoned);
    assert_ne!(PipelineResult::Failed, PipelineResult::Poisoned);
}

#[test]
fn copy_and_clone() {
    let a = PipelineResult::Completed;
    let b = a;
    let c = a.clone();
    assert_eq!(b, PipelineResult::Completed);
    assert_eq!(c, PipelineResult::Completed);
}
