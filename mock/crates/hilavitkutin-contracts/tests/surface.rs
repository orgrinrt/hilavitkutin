//! The surface of `num`, every door used once: in a const context where the door is const, and
//! by the value it gives.

use core::cmp::Ordering;

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::num::{Additive, Bool, Count, Identity, Index, Int, Lane, USize, Wide};

const BOOL: [bool; 7] = [
    Bool::FALSE.get(),
    Bool::TRUE.get(),
    Bool::of(true).get(),
    Bool::TRUE.and(Bool::FALSE).get(),
    Bool::TRUE.or(Bool::FALSE).get(),
    Bool::FALSE.not().get(),
    Bool::TRUE.not().get(),
];

#[test]
fn bool_doors() {
    assert_eq!(BOOL, [false, true, true, false, true, true, false]);
}

const U3: USize = USize::from_host(3);
const U5: USize = USize::from_host(5);
const USIZE: [usize; 4] = [
    U3.host(),
    U3.add(U5).host(),
    U5.sub(U3).host(),
    U3.min(U5).add(U3.max(U5)).host(),
];
const USIZE_CMP: [bool; 6] = [
    U3.eq(U3).get(),
    U3.lt(U5).get(),
    U3.le(U3).get(),
    U5.gt(U3).get(),
    U5.ge(U5).get(),
    U3.gt(U5).get(),
];

#[test]
fn usize_doors() {
    assert_eq!(USIZE, [3, 8, 2, 8]);
    assert_eq!(USIZE_CMP, [true, true, true, true, true, false]);
    assert_eq!(U3.cmp(U5), Ordering::Less);
    assert_eq!(<USize as Identity<Additive>>::IDENTITY, USize::from_host(0));
    assert_eq!(
        USize::from_host(0).sub(U3).host(),
        usize::MAX - 2,
        "sub wraps"
    );
}

const I2: Index = Index::from_host(2);
const C3: Count = Count::from_host(3);
const INDEX: [usize; 5] = [
    Index::FIRST.host(),
    I2.after(C3).host(),
    I2.next().host(),
    I2.min(Index::FIRST).host(),
    I2.max(Index::FIRST).host(),
];
const INDEX_CMP: [bool; 8] = [
    I2.eq(I2).get(),
    I2.lt(I2.next()).get(),
    I2.le(I2).get(),
    I2.gt(Index::FIRST).get(),
    I2.ge(I2).get(),
    I2.within(C3).get(),
    I2.within(Count::from_host(2)).get(),
    I2.gt(I2).get(),
];

#[test]
fn index_doors() {
    assert_eq!(INDEX, [0, 5, 3, 0, 2]);
    assert_eq!(INDEX_CMP, [
        true, true, true, true, true, true, false, false
    ]);
    assert_eq!(I2.cmp(Index::FIRST), Ordering::Greater);
    assert_eq!(I2.before(Count::ONE), Maybe::Is(Index::from_host(1)));
    assert_eq!(I2.before(C3), Maybe::Isnt);
    assert_eq!(I2.since(Index::FIRST), Maybe::Is(Count::from_host(2)));
    assert_eq!(Index::FIRST.since(I2), Maybe::Isnt);
}

const COUNT: [usize; 8] = [
    Count::ZERO.host(),
    Count::ONE.host(),
    C3.add(Count::ONE).host(),
    match C3.sub(Count::ONE) {
        Maybe::Is(n) => n.host(),
        Maybe::Isnt => usize::MAX,
    },
    C3.next().host(),
    C3.end().host(),
    C3.min(Count::ONE).host(),
    C3.max(Count::ONE).host(),
];
const COUNT_CMP: [bool; 5] = [
    C3.eq(C3).get(),
    Count::ONE.lt(C3).get(),
    C3.le(C3).get(),
    C3.gt(Count::ZERO).get(),
    C3.ge(C3.next()).get(),
];

#[test]
fn count_doors() {
    assert_eq!(COUNT, [0, 1, 4, 2, 4, 3, 1, 3]);
    assert_eq!(COUNT_CMP, [true, true, true, true, false]);
    assert_eq!(C3.cmp(C3), Ordering::Equal);
    assert_eq!(<Count as Identity<Additive>>::IDENTITY, Count::ZERO);
    assert_eq!(
        Count::ZERO.sub(Count::ONE),
        Maybe::Isnt,
        "a count does not go below zero"
    );
    assert_eq!(C3.sub(C3), Maybe::Is(Count::ZERO));
}

type I8 = Int<8>;

const INT: [i64; 6] = [
    I8::MIN.raw(),
    I8::MAX.raw(),
    I8::constant::<-5>().raw(),
    I8::from_raw(1_000).raw(),
    I8::from_raw(-1_000).raw(),
    I8::MAX.widen().add(I8::MAX.widen()).narrow().raw(),
];

#[test]
fn int_doors() {
    assert_eq!(INT, [-128, 127, -5, 127, -128, 127]);
    assert_eq!(<I8 as Identity<Additive>>::IDENTITY, I8::from_raw(0));
    assert_eq!(Wide::<8>::ZERO, I8::from_raw(0).widen());
    assert_eq!(Wide::<8>::MAX_TERMS, USize::from_host(255));
    assert_eq!(<I8 as Lane>::MAX_TERMS, USize::from_host(255));
    assert_eq!(<Int<64> as Lane>::MAX_TERMS, USize::from_host(usize::MAX));
}

#[test]
fn the_term_bound_is_exact() {
    // 255 subtracted minimums fill the register to its last representable value below zero's
    // mirror; the 256th does not fit.
    let mut sum = Wide::<8>::ZERO;
    for _ in 0 .. 255 {
        assert!(sum.fits_sub(I8::MIN.widen()).get());
        sum = sum.sub(I8::MIN.widen());
    }
    assert!(!sum.fits_sub(I8::MIN.widen()).get());
    let mut low = Wide::<8>::ZERO;
    for _ in 0 .. 255 {
        assert!(low.fits_add(I8::MIN.widen()).get());
        low = low.add(I8::MIN.widen());
    }
    assert!(
        low.fits_add(I8::MIN.widen()).get(),
        "adding minimums reaches the register's minimum one later"
    );
}

#[test]
fn lane_doors() {
    let a = I8::from_raw(100);
    let b = I8::from_raw(-100);
    let w = <I8 as Lane>::wide_sub(
        <I8 as Lane>::wide_add(<I8 as Lane>::WIDE_ZERO, Lane::widen(a)),
        Lane::widen(b),
    );
    assert_eq!(<I8 as Lane>::narrow(w), I8::MAX);
}

/// At `B` bits the register holds `MAX_TERMS` subtracted minimums, the most a sum of members can
/// reach, and not one more. The sum is built by doubling, so 64 bits costs 64 steps.
fn term_bound<const B: u32>() {
    let min = Int::<B>::MIN.widen();
    let one = Wide::<B>::ZERO.sub(min);
    let mut half = one;
    for _ in 1 .. B {
        assert!(half.fits_add(half).get(), "{B} bits");
        half = half.add(half);
    }
    let all = half.sub(one).add(half);
    assert_eq!(
        Wide::<B>::MAX_TERMS.host() as u128,
        (1u128 << B) - 1,
        "{B} bits: the bound counts every term the register holds"
    );
    assert!(
        all.sub(one).fits_sub(min).get(),
        "{B} bits: the last term fits"
    );
    assert!(!all.fits_sub(min).get(), "{B} bits: one term more does not");
    let low = Wide::<B>::ZERO.sub(all);
    assert!(
        low.fits_add(min).get(),
        "{B} bits: the low side holds one more"
    );
    assert!(
        !low.add(min).fits_add(min).get(),
        "{B} bits: and no further"
    );
}

macro_rules! every_width {
    ($($b:literal)*) => {
        #[test]
        fn the_term_bound_holds_at_every_width() {
            $(term_bound::<$b>();)*
        }
    };
}

every_width!(
    1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
    33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
);
