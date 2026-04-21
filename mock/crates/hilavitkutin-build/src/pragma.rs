//! Pragma enum + fixed-size `PragmaSet` tag-set.
//!
//! Pure data. `PragmaSet` is a 13-variant bit mask over the base
//! pragma variants; `ParallelCodegen(u8)`'s parameter is stored
//! separately because it carries a runtime value. The mask reserves
//! room for future growth (u16, 16 slots).

/// One of the 13 built-in compilation pragmas documented in
/// `DESIGN.md` (§Pragma system, Q4f).
///
/// `ParallelCodegen` carries a `u8` parameter — `0` means auto-detect
/// via `std::thread::available_parallelism()` at wrapper time.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Pragma {
    LoopOptimization,
    Polly,
    MathPeephole,
    FastMath,
    ExpandedLto,
    Pgo,
    Bolt,
    Profiling,
    BuildStd,
    ParallelCodegen(u8), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pragma parameter: unit count, bounded [0, 255]; tracked: #72
    SharedGenerics,
    LoopFusion,
    MimallocAllocator,
}

impl Pragma {
    /// Stable bit index for this pragma. `ParallelCodegen` collapses
    /// to a single slot because its param is stored separately in
    /// `PragmaSet::parallel_codegen_units`.
    const fn bit(self) -> u16 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 13-pragma bit-mask algorithmic width; build-dep only; tracked: #72
        let idx: u8 = match self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pragma discriminant; tracked: #72
            Pragma::LoopOptimization => 0,
            Pragma::Polly => 1,
            Pragma::MathPeephole => 2,
            Pragma::FastMath => 3,
            Pragma::ExpandedLto => 4,
            Pragma::Pgo => 5,
            Pragma::Bolt => 6,
            Pragma::Profiling => 7,
            Pragma::BuildStd => 8,
            Pragma::ParallelCodegen(_) => 9,
            Pragma::SharedGenerics => 10,
            Pragma::LoopFusion => 11,
            Pragma::MimallocAllocator => 12,
        };
        1u16 << idx
    }

    /// Inverse of `bit`: rebuild the canonical `Pragma` value for a
    /// given bit index. `ParallelCodegen`'s param is supplied
    /// separately by the caller (from `PragmaSet::parallel_codegen_units`).
    const fn from_index(idx: u8, parallel_units: u8) -> Option<Pragma> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-option) reason: internal pragma reconstructor; u8 discriminant + parallel unit count; Option is internal; tracked: #72
        match idx {
            0 => Some(Pragma::LoopOptimization),
            1 => Some(Pragma::Polly),
            2 => Some(Pragma::MathPeephole),
            3 => Some(Pragma::FastMath),
            4 => Some(Pragma::ExpandedLto),
            5 => Some(Pragma::Pgo),
            6 => Some(Pragma::Bolt),
            7 => Some(Pragma::Profiling),
            8 => Some(Pragma::BuildStd),
            9 => Some(Pragma::ParallelCodegen(parallel_units)),
            10 => Some(Pragma::SharedGenerics),
            11 => Some(Pragma::LoopFusion),
            12 => Some(Pragma::MimallocAllocator),
            _ => None,
        }
    }
}

/// Tag-set of pragmas. Fixed-size bit mask over the 13 base variants;
/// `ParallelCodegen`'s `u8` parameter stored separately.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PragmaSet {
    mask: u16, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: 13-pragma bit-mask algorithmic width; tracked: #72
    parallel_codegen_units: Option<u8>, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-option) lint:allow(no-public-raw-field) reason: pragma parameter storage; Option because pragma may be absent from the set; tracked: #72
}

impl PragmaSet {
    /// Empty set.
    pub const fn new() -> Self {
        PragmaSet {
            mask: 0,
            parallel_codegen_units: None,
        }
    }

    /// Add `p` to the set. For `ParallelCodegen`, stores the unit
    /// count (overwriting any previous value).
    pub const fn with(mut self, p: Pragma) -> Self {
        self.mask |= p.bit();
        if let Pragma::ParallelCodegen(n) = p {
            self.parallel_codegen_units = Some(n);
        }
        self
    }

    /// Remove `p` from the set. For `ParallelCodegen`, also clears
    /// the stored unit count.
    pub const fn without(mut self, p: Pragma) -> Self {
        self.mask &= !p.bit();
        if let Pragma::ParallelCodegen(_) = p {
            self.parallel_codegen_units = None;
        }
        self
    }

    /// Check membership. For `ParallelCodegen(n)`, matches on the
    /// slot (ignores `n`) — use `parallel_codegen_units()` to read
    /// the stored param.
    pub const fn contains(self, p: Pragma) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: build-time predicate; consumer output flows into `build.rs` stdout; tracked: #72
        (self.mask & p.bit()) != 0
    }

    /// The stored `ParallelCodegen` unit count, if the pragma is set.
    pub const fn parallel_codegen_units(self) -> Option<u8> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-option) reason: stored pragma parameter; Option because pragma may be absent; tracked: #72
        self.parallel_codegen_units
    }

    /// Iterate the set in stable bit-index order.
    pub fn iter(self) -> PragmaIter {
        PragmaIter {
            mask: self.mask,
            parallel_units: self.parallel_codegen_units.unwrap_or(0),
            idx: 0,
        }
    }
}

/// Iterator over the pragmas in a `PragmaSet`. Yields in ascending
/// bit-index order (matches the order of the `Pragma` enum's declared
/// variants).
pub struct PragmaIter {
    mask: u16, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: PragmaIter scans the 13-pragma bit-mask; mirrors PragmaSet width; tracked: #72
    parallel_units: u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: unit count carried through iteration; tracked: #72
    idx: u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: iterator cursor over 13 pragma bits; tracked: #72
}

impl Iterator for PragmaIter {
    type Item = Pragma;

    fn next(&mut self) -> Option<Pragma> { // lint:allow(no-bare-option) reason: std `Iterator::next` trait return; cannot deviate; tracked: #72
        while self.idx < 13 {
            let bit = 1u16 << self.idx;
            let i = self.idx;
            self.idx += 1;
            if (self.mask & bit) != 0 {
                return Pragma::from_index(i, self.parallel_units);
            }
        }
        None
    }
}
