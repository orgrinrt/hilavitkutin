//! A prototype names one set of pragmas per profile.
//!
//! A consumer's script names a prototype in [`script!`](crate::script)
//! and writes nothing of what it does, so no consumer holds a copy that
//! can drift. The crate's own are [`Default`], [`Deterministic`] and
//! [`Workspace`]; a consumer writes one of its own by implementing
//! [`Prototype`] in a crate among its build-dependencies.
//!
//! A set lists its pragmas in the order of their bits, which is the
//! order their flags are written in.

use crate::pragma::Mask;
use crate::roster::{
    Bolt,
    ExpandedLto,
    FastMath,
    LoopOptimization,
    MathPeephole,
    ParallelCodegen,
    Pgo,
    Profiling,
    SharedGenerics,
};
use crate::set::{Set, checked};

/// One set of pragmas per profile.
pub trait Prototype {
    /// The set of the `dev` profile.
    type Dev: Set;
    /// The set of the `dev-opt` profile.
    type DevOpt: Set;
    /// The set of the `release` profile.
    type Release: Set;
    /// The set of the `profiling` profile.
    type Profiling: Set;
    /// The set of the `ci` profile.
    type Ci: Set;
}

/// The crate's defaults, as [`configure`](crate::configure) takes them.
///
/// No set holds a pragma of the seven build choices: each choice is the
/// toolchain's own until a bench cell shows another value ahead on a
/// target.
#[derive(Debug, Clone, Copy)]
pub struct Default;

impl Prototype for Default {
    type Ci = (LoopOptimization, ParallelCodegen<0>);
    type Dev = (ParallelCodegen<0>, SharedGenerics);
    type DevOpt = (LoopOptimization, ParallelCodegen<0>, SharedGenerics);
    type Profiling = (
        LoopOptimization,
        MathPeephole,
        FastMath,
        Pgo,
        Bolt,
        Profiling,
        ParallelCodegen<0>,
    );
    type Release = (
        LoopOptimization,
        MathPeephole,
        FastMath,
        Pgo,
        Bolt,
        ParallelCodegen<0>,
    );
}

/// The defaults without `FastMath`, and without `MathPeephole`, which
/// stands on it: for a crate whose own code must give the same floats on
/// every machine.
#[derive(Debug, Clone, Copy)]
pub struct Deterministic;

impl Prototype for Deterministic {
    type Ci = (LoopOptimization, ParallelCodegen<0>);
    type Dev = (ParallelCodegen<0>, SharedGenerics);
    type DevOpt = (LoopOptimization, ParallelCodegen<0>, SharedGenerics);
    type Profiling = (LoopOptimization, Pgo, Bolt, Profiling, ParallelCodegen<0>);
    type Release = (LoopOptimization, Pgo, Bolt, ParallelCodegen<0>);
}

/// The defaults with the pragmas that reach the whole workspace, for the
/// one script that claims it: `ExpandedLto` in release and in ci.
#[derive(Debug, Clone, Copy)]
pub struct Workspace;

impl Prototype for Workspace {
    type Ci = (LoopOptimization, ExpandedLto, ParallelCodegen<0>);
    type Dev = (ParallelCodegen<0>, SharedGenerics);
    type DevOpt = (LoopOptimization, ParallelCodegen<0>, SharedGenerics);
    type Profiling = (
        LoopOptimization,
        MathPeephole,
        FastMath,
        Pgo,
        Bolt,
        Profiling,
        ParallelCodegen<0>,
    );
    type Release = (
        LoopOptimization,
        MathPeephole,
        FastMath,
        ExpandedLto,
        Pgo,
        Bolt,
        ParallelCodegen<0>,
    );
}

/// The assertion the script that claims the workspace makes of its
/// prototype: nothing is wrong with any of its sets, and every set holds
/// the same pragmas of the groups whose keys belong to no profile.
///
/// Such a key is in the generated cargo file once. Two sets that held
/// different pragmas of one such group would write it twice with two
/// values, which is a file cargo cannot read.
pub const fn claimed<P: Prototype>() {
    checked::<P::Dev>();
    checked::<P::DevOpt>();
    checked::<P::Release>();
    checked::<P::Profiling>();
    checked::<P::Ci>();
    let shared: Mask = <P::Dev as Set>::SHARED;
    assert!(
        shared == <P::DevOpt as Set>::SHARED
            && shared == <P::Release as Set>::SHARED
            && shared == <P::Profiling as Set>::SHARED
            && shared == <P::Ci as Set>::SHARED,
        "the sets of a prototype that claims the workspace hold the same link driver and the same C \
         compiler in every profile, since their keys belong to no profile, and these sets differ"
    );
}
