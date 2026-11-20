//! The OS error code the loader reports: what goes in by `from_host` is what
//! comes out by `host`, whatever the adaptation of the alias would do to a
//! value it had to complete.

use hilavitkutin_linking::{LinkError, OsCode};

#[test]
fn a_code_comes_out_as_it_went_in() {
    // zero is the value the loader reports on a unix it cannot read errno on, 2 is
    // ENOENT, 1314 is a Windows privilege error, and the largest is the edge of the
    // range, where a wrapping or a saturating completion would show
    for code in [0, 2, 13, 1314, usize::MAX] {
        assert_eq!(OsCode::from_host(code).host(), code);
    }
}

#[test]
fn a_load_failure_carries_the_code_it_was_built_with() {
    let failure = LinkError::LoadFailed {
        platform_code: OsCode::from_host(2),
    };
    match failure {
        LinkError::LoadFailed {
            platform_code,
        } => assert_eq!(platform_code.host(), 2),
        _ => panic!("a load failure matched another variant"),
    }
}
