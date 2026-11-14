//! The frozen authors under the reference runner, one digest a frame, on
//! whatever target this is built for.
//!
//! The four frozen files come from this package's library, which takes them
//! by path, and the world is lent by the macro the frozen crate's own tests
//! lend it with. On the host a digest is a line of sixteen hex digits. On
//! wasm it is handed to the `emit` the embedder imports, which prints the
//! same line.

use cross_target_probe::model::Model;
use cross_target_probe::record;
use cross_target_probe::units::{Alarm, Crest, Cross, Drift, Fade, Smooth, Spread, Tally, Warm};
use cross_target_probe::values::{
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
use hilavitkutin_api::store::{Accum, Column, Virtual};
use hilavitkutin_conformance::{Ascending, Faithful, Fold, Lent, Oracle, Word};
use notko::Outcome;

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
    cross_target_probe::lend!(world, 0);
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
        cross_target_probe::kept!(oracle, record::words, |word| fold.word(word));
        if cfg!(all(feature = "skew", target_family = "wasm")) && frame + 1 == FRAMES {
            fold.word(Word::from_raw(1));
        }
        out(fold.finish().to_raw());
    }
}
