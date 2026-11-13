//! The native host: runs every case and prints its lines.
//!
//! One line per digest, `c<case> f<frame> <16 hex digits>`, the digest's
//! eight bytes read little-endian, then one `c<case> end written=<n>
//! refused=<frame|none>` line per case. `run.mjs` prints the same lines from
//! the wasm host.

use cross_target_loop::{CASES, FRAMES, run_case};
use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::USize;

fn main() {
    for case in 0 .. CASES.host() {
        let mut out = [Word::of([0; 8]); FRAMES];
        let report = run_case(USize::from_host(case), &mut out);
        for (frame, word) in out.iter().take(report.written.host()).enumerate() {
            println!(
                "c{case} f{} {:016x}",
                frame + 1,
                u64::from_le_bytes(word.bytes())
            );
        }
        let refused = match report.refused {
            Maybe::Is(frame) => frame.host().to_string(),
            Maybe::Isnt => "none".to_string(),
        };
        println!(
            "c{case} end written={} refused={refused}",
            report.written.host()
        );
    }
}
