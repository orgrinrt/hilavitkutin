//! What the length probes share: 128 frame lanes, a list builder, a kernel
//! writing a given list and reading nothing, and the probe itself.

#![allow(dead_code)]

use engine::{FrameLane, Kernel, Lane, Registry};

macro_rules! lanes {
    ($($n:ident)*) => {$(
        pub struct $n;
        impl Lane for $n {
            type Item = i32;
            const INIT: i32 = 0;
        }
        impl FrameLane for $n {}
    )*};
}

/// A list of the named lanes, one recursion per lane.
#[macro_export]
macro_rules! list {
    () => { notko_hlist::Empty };
    ($h:ident $($t:ident)*) => { notko_hlist::Cons<$h, $crate::list!($($t)*)> };
}

lanes! {
    L0 L1 L2 L3 L4 L5 L6 L7 L8 L9 L10 L11 L12 L13 L14 L15
    L16 L17 L18 L19 L20 L21 L22 L23 L24 L25 L26 L27 L28 L29 L30 L31
    L32 L33 L34 L35 L36 L37 L38 L39 L40 L41 L42 L43 L44 L45 L46 L47
    L48 L49 L50 L51 L52 L53 L54 L55 L56 L57 L58 L59 L60 L61 L62 L63
    L64 L65 L66 L67 L68 L69 L70 L71 L72 L73 L74 L75 L76 L77 L78 L79
    L80 L81 L82 L83 L84 L85 L86 L87 L88 L89 L90 L91 L92 L93 L94 L95
    L96 L97 L98 L99 L100 L101 L102 L103 L104 L105 L106 L107 L108 L109 L110 L111
    L112 L113 L114 L115 L116 L117 L118 L119 L120 L121 L122 L123 L124 L125 L126 L127
}

/// Writes the lanes `W` and reads nothing.
pub struct Writes<W>(core::marker::PhantomData<W>);

impl<W> Writes<W> {
    pub const fn new() -> Self {
        Writes(core::marker::PhantomData)
    }
}

/// `Writes<W>` as a block kernel of the registry, for each `W` named.
#[macro_export]
macro_rules! writes_in {
    ($reg:ty: $($w:ty),*) => {$(
        impl engine::BlockKernel<$reg> for $crate::common::Writes<$w> {
            type Reads = notko_hlist::Empty;
            type Writes = $w;

            fn run<'s>(_view: engine::View<'s, $reg, notko_hlist::Empty>, _out: engine::Staged<'s, $w>) {}
        }
    )*};
}

/// The mask and the length of a kernel's write list, with its positions
/// inferred the way the runner infers them.
pub const fn mask_of<Reg: Registry, K: Kernel<Reg, Ps>, Ps>(_kernel: &K) -> (u128, u32) {
    (K::MASK, K::LEN)
}

/// The lowest `n` bits set.
pub fn low_bits(n: u32) -> u128 {
    if n == 128 { u128::MAX } else { (1u128 << n) - 1 }
}

/// One probe: the registry's whole list as one kernel, and its two halves as
/// two kernels of one step. Each mask is read back, and each kernel is
/// instantiated on a frame so the runner's `const` block is evaluated.
#[macro_export]
macro_rules! at_length {
    ($name:ident, $reg:ty, $lo:ty, $hi:ty, $n:expr) => {
        #[test]
        fn $name() {
            use $crate::common::{Writes, low_bits, mask_of};

            let all = engine::Block(Writes::<$reg>::new());
            let lo = engine::Block(Writes::<$lo>::new());
            let hi = engine::Block(Writes::<$hi>::new());

            assert_eq!(mask_of::<$reg, _, _>(&all), (low_bits($n), $n));
            let (lo_mask, lo_len) = mask_of::<$reg, _, _>(&lo);
            let (hi_mask, hi_len) = mask_of::<$reg, _, _>(&hi);
            assert_eq!((lo_mask, lo_len), (low_bits($n / 2), $n / 2));
            assert_eq!((lo_mask | hi_mask, lo_len + hi_len), (low_bits($n), $n));
            assert_eq!(lo_mask & hi_mask, 0);

            let mut frame: engine::Frame<$reg, 1> = engine::Frame::new();
            frame.step(all);
            frame.step2(lo, hi);
        }
    };
}
