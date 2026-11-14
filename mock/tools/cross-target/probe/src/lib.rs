//! The four frozen files and the frozen harness's `lend.rs`, taken by path.
//!
//! The bytes the ledger holds are the bytes every target compiles. The files
//! are the public modules of this library, so what the program in `main.rs`
//! leaves unused is still the library's surface and is compiled the same.

#[path = "../../../../crates/hilavitkutin-frozen-authors/tests/common/lend.rs"]
pub mod lend;
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/model.rs"]
pub mod model;
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/record.rs"]
pub mod record;
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/units.rs"]
pub mod units;
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/values.rs"]
pub mod values;
