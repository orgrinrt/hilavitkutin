// Validate the bundle-walk level: a BundleProject over a WU cons-list,
// carrying a PARALLEL nested witness list (cons-list of (ReadIdx, WriteIdx)
// pairs, one per WU) as a trait param (dodging E0207, like Project<R,Indices>),
// solver-inferred at the call site. This is the part the first sketch did NOT
// cover (it validated one access set + one inferred index list).
use core::marker::PhantomData;
struct Empty;
struct Cons<H, T>(PhantomData<(H, T)>);
struct Here;
struct There<I>(PhantomData<I>);

trait WitnessIndex {
    const I: usize;
}
impl WitnessIndex for Here {
    const I: usize = 0;
}
impl<X: WitnessIndex> WitnessIndex for There<X> {
    const I: usize = 1 + X::I;
}

trait Locate<Target, Index> {}
impl<Target, Tail> Locate<Target, Here> for Cons<Target, Tail> {}
impl<Target, Head, Tail, X> Locate<Target, There<X>> for Cons<Head, Tail> where
    Tail: Locate<Target, X>
{
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct Mask(u64);
impl Mask {
    fn set(self, b: usize) -> Self {
        Mask(self.0 | (1u64 << b))
    }
}

trait MaskProject<Stores, Indices> {
    fn project(m: Mask) -> Mask;
}
impl<Stores> MaskProject<Stores, Empty> for Empty {
    fn project(m: Mask) -> Mask {
        m
    }
}
impl<Stores, M, Tail, X, XT> MaskProject<Stores, Cons<X, XT>> for Cons<M, Tail>
where
    Stores: Locate<M, X>,
    X: WitnessIndex,
    Tail: MaskProject<Stores, XT>,
{
    fn project(m: Mask) -> Mask {
        let m = m.set(X::I);
        <Tail as MaskProject<Stores, XT>>::project(m)
    }
}

trait WorkUnit {
    type Read;
    type Write;
    const COMM: bool;
}

#[derive(Debug)]
struct Inputs {
    reads: [Mask; 8],
    writes: [Mask; 8],
    comm: [bool; 8],
    count: usize,
}
impl Inputs {
    fn new() -> Self {
        Inputs { reads: [Mask::default(); 8], writes: [Mask::default(); 8], comm: [false; 8], count: 0 }
    }
}

// BundleProject<Stores, Witnesses>: Witnesses is a parallel cons-list of
// (ReadIdx, WriteIdx) pairs (one per WU). Trait param => no E0207.
trait BundleProject<Stores, Witnesses> {
    fn project(inp: &mut Inputs, i: usize);
}
impl<Stores> BundleProject<Stores, Empty> for Empty {
    fn project(_: &mut Inputs, _: usize) {}
}
impl<Stores, W, T, RI, WI, WT> BundleProject<Stores, Cons<(RI, WI), WT>> for Cons<W, T>
where
    W: WorkUnit,
    W::Read: MaskProject<Stores, RI>,
    W::Write: MaskProject<Stores, WI>,
    T: BundleProject<Stores, WT>,
{
    fn project(inp: &mut Inputs, i: usize) {
        inp.reads[i] = <W::Read as MaskProject<Stores, RI>>::project(Mask::default());
        inp.writes[i] = <W::Write as MaskProject<Stores, WI>>::project(Mask::default());
        inp.comm[i] = W::COMM;
        inp.count = i + 1;
        <T as BundleProject<Stores, WT>>::project(inp, i + 1);
    }
}

fn build_inputs<Wus, Stores, Witnesses>() -> Inputs
where
    Wus: BundleProject<Stores, Witnesses>,
{
    let mut inp = Inputs::new();
    <Wus as BundleProject<Stores, Witnesses>>::project(&mut inp, 0);
    inp
}

struct SA;
struct SB;
struct SC;
struct SD;
type Stores = Cons<SA, Cons<SB, Cons<SC, Cons<SD, Empty>>>>;
struct W0;
impl WorkUnit for W0 {
    type Read = Cons<SA, Cons<SC, Empty>>;
    type Write = Cons<SB, Empty>;
    const COMM: bool = false;
}
struct W1;
impl WorkUnit for W1 {
    type Read = Cons<SB, Empty>;
    type Write = Cons<SD, Empty>;
    const COMM: bool = true;
}
type Wus = Cons<W0, Cons<W1, Empty>>;

fn main() {
    let inp = build_inputs::<Wus, Stores, _>();
    assert_eq!(inp.count, 2);
    assert_eq!(inp.reads[0], Mask(0b0101), "W0 reads SA(0),SC(2)");
    assert_eq!(inp.writes[0], Mask(0b0010), "W0 writes SB(1)");
    assert_eq!(inp.reads[1], Mask(0b0010), "W1 reads SB(1)");
    assert_eq!(inp.writes[1], Mask(0b1000), "W1 writes SD(3)");
    assert_eq!(inp.comm[1], true);
    assert_ne!(inp.writes[0].0 & inp.reads[1].0, 0, "W0 write SB overlaps W1 read SB");
    println!("BUNDLE WORKS: nested witness list inferred, per-WU masks correct");
}
