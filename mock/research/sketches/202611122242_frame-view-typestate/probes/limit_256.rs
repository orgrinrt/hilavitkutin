//! The 128-lane probe of `author/tests/lengths_128.rs` at a recursion limit of
//! 256, where the trait solver stops with `E0275`. The test file holds at 300.
//! Output: `out/limit_256.check.txt`.
#![recursion_limit = "256"]

#[path = "../author/tests/common/mod.rs"]
mod common;

use common::*;

type Lo128 = list!(
    L0 L1 L2 L3 L4 L5 L6 L7 L8 L9 L10 L11 L12 L13 L14 L15
    L16 L17 L18 L19 L20 L21 L22 L23 L24 L25 L26 L27 L28 L29 L30 L31
    L32 L33 L34 L35 L36 L37 L38 L39 L40 L41 L42 L43 L44 L45 L46 L47
    L48 L49 L50 L51 L52 L53 L54 L55 L56 L57 L58 L59 L60 L61 L62 L63
);
type Hi128 = list!(
    L64 L65 L66 L67 L68 L69 L70 L71 L72 L73 L74 L75 L76 L77 L78 L79
    L80 L81 L82 L83 L84 L85 L86 L87 L88 L89 L90 L91 L92 L93 L94 L95
    L96 L97 L98 L99 L100 L101 L102 L103 L104 L105 L106 L107 L108 L109 L110 L111
    L112 L113 L114 L115 L116 L117 L118 L119 L120 L121 L122 L123 L124 L125 L126 L127
);
type R128 = list!(
    L0 L1 L2 L3 L4 L5 L6 L7 L8 L9 L10 L11 L12 L13 L14 L15
    L16 L17 L18 L19 L20 L21 L22 L23 L24 L25 L26 L27 L28 L29 L30 L31
    L32 L33 L34 L35 L36 L37 L38 L39 L40 L41 L42 L43 L44 L45 L46 L47
    L48 L49 L50 L51 L52 L53 L54 L55 L56 L57 L58 L59 L60 L61 L62 L63
    L64 L65 L66 L67 L68 L69 L70 L71 L72 L73 L74 L75 L76 L77 L78 L79
    L80 L81 L82 L83 L84 L85 L86 L87 L88 L89 L90 L91 L92 L93 L94 L95
    L96 L97 L98 L99 L100 L101 L102 L103 L104 L105 L106 L107 L108 L109 L110 L111
    L112 L113 L114 L115 L116 L117 L118 L119 L120 L121 L122 L123 L124 L125 L126 L127
);

writes_in!(R128: R128, Lo128, Hi128);

fn main() {
    let all = engine::Block(Writes::<R128>::new());
    let lo = engine::Block(Writes::<Lo128>::new());
    let hi = engine::Block(Writes::<Hi128>::new());
    let mut frame: engine::Frame<R128, 1> = engine::Frame::new();
    frame.step(all);
    frame.step2(lo, hi);
}
