//! An arm with one export, so the cdylib is a real one to stage.

#[no_mangle]
pub extern "C" fn stage_fixture_arm() -> u32 {
    7
}
