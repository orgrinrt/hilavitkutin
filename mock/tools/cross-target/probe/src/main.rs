//! The frozen authors under the reference runner, one digest a frame, on
//! whatever target this is built for.
//!
//! The four frozen files are taken by path, so the bytes the ledger holds are
//! the bytes every target compiles, and the world is lent by the macro the
//! frozen crate's own tests lend it with. On the host a digest is a line of
//! sixteen hex digits. On wasm it is handed to the `emit` the embedder
//! imports, which prints the same line.

#[allow(dead_code)]
#[path = "../../../../crates/hilavitkutin-frozen-authors/tests/common/lend.rs"]
mod lend;
#[allow(dead_code)]
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/model.rs"]
mod model;
#[allow(dead_code)]
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/record.rs"]
mod record;
#[allow(dead_code)]
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/units.rs"]
mod units;
#[allow(dead_code)]
#[path = "../../../../crates/hilavitkutin-frozen-authors/src/values.rs"]
mod values;

use hilavitkutin_api::store::{Accum, Column, Virtual};
use hilavitkutin_conformance::{Ascending, Faithful, Fold, Lent, Oracle, Word};
use notko::Outcome;

use crate::model::Model;
use crate::units::{Alarm, Crest, Cross, Drift, Fade, Smooth, Spread, Tally, Warm};
use crate::values::{
    Alarms,
    Ash,
    Crossed,
    Edge,
    Field,
    Glow,
    Heat,
    Level,
    Num,
    Overheat,
    Peak,
    Pool,
    Pos,
    RECORDS,
    Vel,
};

/// How many frames the stream is, the same count the frozen crate's tests
/// hold the scheduler over.
const FRAMES: usize = 8;

#[cfg(target_family = "wasm")]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    /// The embedder's sink for one digest.
    fn emit(digest: u64);
}

/// Hand one frame's digest out.
fn out(digest: u64) {
    #[cfg(target_family = "wasm")]
    // SAFETY: the embedder's `emit` takes one 64-bit integer and returns.
    unsafe {
        emit(digest)
    };
    #[cfg(not(target_family = "wasm"))]
    println!("{digest:016x}");
}

fn main() {
    crate::lend!(world, 0);
    let Outcome::Ok(mut oracle) = Oracle::<_, Faithful, Ascending>::new(world, RECORDS) else {
        panic!("the reference runner refused the stores");
    };
    let units = (
        Drift,
        (
            Spread,
            (
                Cross,
                (Alarm, (Tally, (Smooth, (Crest, (Warm, (Fade, ())))))),
            ),
        ),
    );
    for frame in 0 .. FRAMES {
        let Outcome::Ok(()) = oracle.frame(&units) else {
            panic!("the reference runner refused frame {frame}");
        };
        let mut fold = Fold::new();
        crate::kept!(oracle, record::words, |word| fold.word(word));
        if cfg!(all(feature = "skew", target_family = "wasm")) && frame + 1 == FRAMES {
            fold.word(Word::from_raw(1));
        }
        out(fold.finish().to_raw());
    }
}
