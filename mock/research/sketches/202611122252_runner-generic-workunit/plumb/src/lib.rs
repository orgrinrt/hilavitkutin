//! Runner-side plumbing both arms share: the world's size, the seeded permutation, the
//! ordered event log, and the two read cores (columns, and records in permuted slots).
//! Nothing here implements a vocabulary trait: each runner crate does that itself, as an
//! independent implementor would, so the growth probe counts honest edits.

#![no_std]

use rg_owed::{ByRow, Col, Elem, Has, Place, Project, SubOf, Val};

/// The world's key has two axes: `east` across a row of the grid, `north` between rows.
pub const EAST: usize = 16;
pub const NORTH: usize = 16;
/// Rows in the world, one per key; row `north * EAST + east`.
pub const N: usize = EAST * NORTH;
/// Steps a run takes.
pub const TICKS: u32 = 6;
/// Events a run can hold.
pub const CAP: usize = N * TICKS as usize;
/// The seed of the permuted runner's visiting order and slot layout.
pub const SEED: u64 = 0x5EED_1234_ABCD_0001;

/// The neighbour of `row` at `(east, north)`, each offset saturated to its reach first,
/// each axis wrapping around the world.
#[inline(always)]
pub fn wrap(row: usize, off: (i32, i32), reach: (i32, i32)) -> usize {
    let e = (row % EAST) as i32 + off.0.clamp(-reach.0, reach.0);
    let n = (row / EAST) as i32 + off.1.clamp(-reach.1, reach.1);
    n.rem_euclid(NORTH as i32) as usize * EAST + e.rem_euclid(EAST as i32) as usize
}

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

/// A seeded permutation of `0..N`: `slot_of[row]` is where a row's record lives.
pub fn permutation(seed: u64) -> [u32; N] {
    let mut p = [0u32; N];
    for (i, x) in p.iter_mut().enumerate() {
        *x = i as u32;
    }
    let mut s = seed | 1;
    for i in (1 .. N).rev() {
        let j = (xorshift(&mut s) % (i as u64 + 1)) as usize;
        p.swap(i, j);
    }
    p
}

/// The inverse of a permutation: `row_of[slot]`.
pub fn invert(p: &[u32; N]) -> [u32; N] {
    let mut q = [0u32; N];
    for (row, slot) in p.iter().enumerate() {
        q[*slot as usize] = row as u32;
    }
    q
}

/// A seeded draw for one key at one tick: a pure function of both, so every layout and
/// every split of the rows draws the same value for the same key.
#[inline(always)]
pub const fn draw(tick: u32, row: u32) -> u32 {
    let mut s = SEED ^ ((tick as u64) << 32 | row as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    s ^= s >> 29;
    s = s.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    s ^= s >> 32;
    s as u32
}

/// Events in the order (tick, row, arrival), whatever order the runner produced them.
pub struct EvLog<E, const C: usize> {
    items: [Option<(u32, u32, E)>; C],
    len:   usize,
}

impl<E, const C: usize> EvLog<E, C> {
    pub fn new() -> Self {
        EvLog {
            items: [const { None }; C],
            len:   0,
        }
    }

    pub fn push(&mut self, tick: u32, row: u32, e: E) {
        self.items[self.len] = Some((tick, row, e));
        self.len += 1;
    }

    pub fn append(&mut self, other: &mut Self) {
        for i in 0 .. other.len {
            self.items[self.len] = other.items[i].take();
            self.len += 1;
        }
        other.len = 0;
    }

    /// Stable insertion sort by (tick, row): the canonical order every runner ends on.
    pub fn settle(&mut self) {
        for i in 1 .. self.len {
            let mut j = i;
            while j > 0 && key(&self.items[j - 1]) > key(&self.items[j]) {
                self.items.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, i: usize) -> Option<&(u32, u32, E)> {
        self.items[.. self.len].get(i).and_then(Option::as_ref)
    }
}

impl<E, const C: usize> Default for EvLog<E, C> {
    fn default() -> Self {
        Self::new()
    }
}

fn key<E>(x: &Option<(u32, u32, E)>) -> (u32, u32) {
    match x {
        Some((t, r, _)) => (*t, *r),
        None => (u32::MAX, u32::MAX),
    }
}

/// What a run ends on: the state product and the settled events.
pub struct Outcome<S, E> {
    pub state:  S,
    pub events: EvLog<E, CAP>,
}

/// Reads of list `L` at one row, out of columns.
pub struct SoaCore<'a, L: Project> {
    pub cols: L::Prod<Col<'a>>,
    pub row:  usize,
}

impl<'a, L: Project> SoaCore<'a, L> {
    #[inline(always)]
    pub fn get<M: Elem, P: Place>(&self) -> M::Value
    where
        L: Has<M, P>,
    {
        Col::at::<M>(L::pick::<Col<'a>>(&self.cols), self.row)
    }

    #[inline(always)]
    pub fn at<M: Elem, P: Place>(&self, off: (i32, i32), reach: (i32, i32)) -> M::Value
    where
        L: Has<M, P>,
    {
        Col::at::<M>(L::pick::<Col<'a>>(&self.cols), wrap(self.row, off, reach))
    }
}

/// Reads of list `L` at one row, out of whole records of list `S` held in permuted
/// slots. The places of `L` in `S` are a type parameter, which a handle chosen per call
/// can carry.
pub struct AosCore<'a, S: Project, L: Project, Ps> {
    pub recs:    &'a [S::Prod<Val>],
    pub slot_of: &'a [u32],
    pub row:     usize,
    pub here:    L::Prod<Val>,
    pub _ps:     core::marker::PhantomData<Ps>,
}

impl<'a, S: Project, L: SubOf<S, Ps>, Ps> AosCore<'a, S, L, Ps> {
    pub fn new(recs: &'a [S::Prod<Val>], slot_of: &'a [u32], row: usize) -> Self {
        let here = L::sub::<Val>(&recs[slot_of[row] as usize]);
        AosCore {
            recs,
            slot_of,
            row,
            here,
            _ps: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn get<M: Elem, P: Place>(&self) -> M::Value
    where
        L: Has<M, P>,
    {
        *L::pick::<Val>(&self.here)
    }

    #[inline(always)]
    pub fn at<M: Elem, P: Place>(&self, off: (i32, i32), reach: (i32, i32)) -> M::Value
    where
        L: Has<M, P>,
    {
        let r = wrap(self.row, off, reach);
        let there = L::sub::<Val>(&self.recs[self.slot_of[r] as usize]);
        *L::pick::<Val>(&there)
    }
}

/// The same reads where the places cannot be a type parameter (a context type fixed
/// by its declaration alone): the projection is erased into a function pointer taken
/// where the places are known.
pub struct AosFnCore<'a, S: Project, L: Project> {
    pub recs:    &'a [S::Prod<Val>],
    pub slot_of: &'a [u32],
    pub row:     usize,
    pub sub:     fn(&S::Prod<Val>) -> L::Prod<Val>,
    pub here:    L::Prod<Val>,
}

impl<'a, S: Project, L: Project> AosFnCore<'a, S, L> {
    pub fn new(
        recs: &'a [S::Prod<Val>],
        slot_of: &'a [u32],
        row: usize,
        sub: fn(&S::Prod<Val>) -> L::Prod<Val>,
    ) -> Self {
        let here = sub(&recs[slot_of[row] as usize]);
        AosFnCore {
            recs,
            slot_of,
            row,
            sub,
            here,
        }
    }

    #[inline(always)]
    pub fn get<M: Elem, P: Place>(&self) -> M::Value
    where
        L: Has<M, P>,
    {
        *L::pick::<Val>(&self.here)
    }

    #[inline(always)]
    pub fn at<M: Elem, P: Place>(&self, off: (i32, i32), reach: (i32, i32)) -> M::Value
    where
        L: Has<M, P>,
    {
        let r = wrap(self.row, off, reach);
        let there = (self.sub)(&self.recs[self.slot_of[r] as usize]);
        *L::pick::<Val>(&there)
    }
}

/// The records of a column store, one product of values per slot.
pub fn to_records<S: Project, const NN: usize>(
    soa: &S::Prod<rg_owed::Arr<NN>>,
    slot_of: &[u32],
) -> [S::Prod<Val>; NN]
where
    S::Prod<Val>: Copy,
{
    let first = S::read_row::<rg_owed::Arr<NN>>(soa, 0);
    let mut recs = [first; NN];
    for row in 0 .. NN {
        recs[slot_of[row] as usize] = S::read_row::<rg_owed::Arr<NN>>(soa, row);
    }
    recs
}

/// The column store of records held in permuted slots.
pub fn to_columns<S: Project, const NN: usize>(
    recs: &[S::Prod<Val>; NN],
    slot_of: &[u32],
    into: &mut S::Prod<rg_owed::Arr<NN>>,
) {
    for row in 0 .. NN {
        S::write_row::<rg_owed::Arr<NN>>(into, row, &recs[slot_of[row] as usize]);
    }
}

#[doc(hidden)]
pub use rg_owed as __owed;

/// A type list written flat.
#[macro_export]
macro_rules! hlist {
    () => { $crate::__owed::Empty };
    ($h:ty $(, $t:ty)* $(,)?) => { $crate::__owed::Cons<$h, $crate::hlist!($($t),*)> };
}

macro_rules! fillers {
    ($($n:ident : $t:ty),* $(,)?) => {
        $(
            pub struct $n;
            impl Elem for $n {
                type Value = $t;
            }
        )*
    };
}

/// The registry's own stores, which no unit reads, of mixed value types.
pub mod filler {
    use rg_owed::Elem;
    fillers! {
        F00: u8, F01: u16, F02: u32, F03: u64, F04: i8, F05: i16, F06: i64, F07: i32,
        F08: u8, F09: u16, F10: u32, F11: u64, F12: i8, F13: i16, F14: i64, F15: i32,
        F16: u8, F17: u16, F18: u32, F19: u64, F20: i8, F21: i16, F22: i64, F23: i32,
        F24: u8, F25: u16, F26: u32, F27: u64, F28: i8, F29: i16, F30: i64, F31: i32,
        F32: u8, F33: u16, F34: u32, F35: u64, F36: i8, F37: i16, F38: i64, F39: i32,
        F40: u8, F41: u16, F42: u32, F43: u64, F44: i8, F45: i16, F46: i64, F47: i32,
        F48: u8, F49: u16, F50: u32, F51: u64, F52: i8, F53: i16, F54: i64, F55: i32,
        F56: u8, F57: u16, F58: u32, F59: u64,
    }
}

/// A registry of 64 stores: 60 of its own, and four a unit reads at places 3, 20, 41
/// and 63.
#[macro_export]
macro_rules! registry {
    ($a:ty, $b:ty, $c:ty, $d:ty) => {
        $crate::hlist![
            $crate::filler::F00,
            $crate::filler::F01,
            $crate::filler::F02,
            $a,
            $crate::filler::F03,
            $crate::filler::F04,
            $crate::filler::F05,
            $crate::filler::F06,
            $crate::filler::F07,
            $crate::filler::F08,
            $crate::filler::F09,
            $crate::filler::F10,
            $crate::filler::F11,
            $crate::filler::F12,
            $crate::filler::F13,
            $crate::filler::F14,
            $crate::filler::F15,
            $crate::filler::F16,
            $crate::filler::F17,
            $crate::filler::F18,
            $b,
            $crate::filler::F19,
            $crate::filler::F20,
            $crate::filler::F21,
            $crate::filler::F22,
            $crate::filler::F23,
            $crate::filler::F24,
            $crate::filler::F25,
            $crate::filler::F26,
            $crate::filler::F27,
            $crate::filler::F28,
            $crate::filler::F29,
            $crate::filler::F30,
            $crate::filler::F31,
            $crate::filler::F32,
            $crate::filler::F33,
            $crate::filler::F34,
            $crate::filler::F35,
            $crate::filler::F36,
            $crate::filler::F37,
            $crate::filler::F38,
            $c,
            $crate::filler::F39,
            $crate::filler::F40,
            $crate::filler::F41,
            $crate::filler::F42,
            $crate::filler::F43,
            $crate::filler::F44,
            $crate::filler::F45,
            $crate::filler::F46,
            $crate::filler::F47,
            $crate::filler::F48,
            $crate::filler::F49,
            $crate::filler::F50,
            $crate::filler::F51,
            $crate::filler::F52,
            $crate::filler::F53,
            $crate::filler::F54,
            $crate::filler::F55,
            $crate::filler::F56,
            $crate::filler::F57,
            $crate::filler::F58,
            $crate::filler::F59,
            $d
        ]
    };
}

/// Carriers are named here so a runner never spells a projection by hand.
pub type ValsOf<L> = <L as Project>::Prod<Val>;
pub type ColsOf<'a, L> = <L as Project>::Prod<Col<'a>>;
pub type ArrOf<L, const NN: usize> = <L as Project>::Prod<rg_owed::Arr<NN>>;

#[cfg(test)]
mod tests {
    use super::{EAST, NORTH, wrap};

    const REACH: (i32, i32) = (3, 2);

    /// The neighbour with no clamp: what a window that does not saturate would read.
    fn unclamped(row: usize, off: (i32, i32)) -> usize {
        let e = (row % EAST) as i32 + off.0;
        let n = (row / EAST) as i32 + off.1;
        n.rem_euclid(NORTH as i32) as usize * EAST + e.rem_euclid(EAST as i32) as usize
    }

    #[test]
    fn an_offset_past_the_reach_saturates_to_it_per_axis() {
        for row in [0, 5, EAST - 1, EAST * NORTH - 1] {
            assert_eq!(wrap(row, (9, 0), REACH), wrap(row, (3, 0), REACH));
            assert_eq!(wrap(row, (0, -7), REACH), wrap(row, (0, -2), REACH));
            assert_eq!(wrap(row, (-40, 40), REACH), wrap(row, (-3, 2), REACH));
            assert_ne!(wrap(row, (9, 0), REACH), unclamped(row, (9, 0)));
            assert_ne!(wrap(row, (0, -7), REACH), unclamped(row, (0, -7)));
        }
    }

    #[test]
    fn an_offset_within_the_reach_reads_the_neighbour_wrapping_both_axes() {
        for row in [0, 5, EAST - 1, EAST * NORTH - 1] {
            for e in -REACH.0 ..= REACH.0 {
                for n in -REACH.1 ..= REACH.1 {
                    assert_eq!(wrap(row, (e, n), REACH), unclamped(row, (e, n)));
                }
            }
        }
        assert_eq!(wrap(0, (-1, -1), REACH), EAST * NORTH - 1);
    }
}
