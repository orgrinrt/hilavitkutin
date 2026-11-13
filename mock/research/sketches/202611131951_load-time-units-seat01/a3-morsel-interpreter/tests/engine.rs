//! A load-time program run by a compiled unit inside the trunk's schedule.
//!
//! Three schedulers over the same loader and the same observer: one with a
//! typed rule compiled in Rust, one with the interpreter unit evaluating a
//! program vector at a time over each morsel (`batch`), one evaluating it per
//! record (`each`). All three must agree record for record.
//!
//! Then the load-time check: a program naming a store outside the unit's
//! declared set, storing into a store it only reads, or loading from one it
//! only writes, is refused before any scheduler exists. Those are the cases
//! that must fail for the arm to be sound: without them the program would
//! reach a store the plan does not know the unit touches.

use std::cell::RefCell;

use a3_morsel_interpreter::{Program, Refused, Slot, eval_morsel, eval_record, load};
use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    BatchApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

struct ZeroedHeap;
unsafe impl Send for ZeroedHeap {}
unsafe impl Sync for ZeroedHeap {}
impl MemoryProviderApi for ZeroedHeap {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        unsafe {
            std::alloc::alloc_zeroed(std::alloc::Layout::from_size_align(len.0, align.0).unwrap())
        }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        unsafe {
            std::alloc::dealloc(
                ptr,
                std::alloc::Layout::from_size_align(len.0, align.0).unwrap(),
            )
        }
    }

    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}
fn store<M: MemoryProviderApi>(m: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(m)
}

#[derive(Copy, Clone)]
struct Pos(i32);
#[derive(Copy, Clone)]
struct Vel(i32);
#[derive(Copy, Clone)]
struct Kind(u32);
#[derive(Copy, Clone)]
struct Out(i32);

type Inputs = Cons<Column<Pos>, Cons<Column<Vel>, Cons<Column<Kind>, Empty>>>;
type InPtrs = ColPtrCons<Pos, ColPtrCons<Vel, ColPtrCons<Kind, ColPtrNil>>>;
type OutSet = Cons<Column<Out>, Empty>;
type OutPtrs = ColPtrCons<Out, ColPtrNil>;
type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

const N: usize = 10_000;
const KINDS: u32 = 4;
/// Compiled into the typed rule; handed to the interpreter as content data.
const COEFS: [i32; 4] = [2, 3, 5, 7];

thread_local! {
    static SEEN: RefCell<Vec<i32>> = RefCell::new(Vec::new());
}

struct Loader;
impl BuilderInput for Loader {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Loader {
    type Ctx<'f> = EngineCtx<'f, Empty, Inputs, SnapNil, ColPtrNil, InPtrs>;
    type Hint = Hints;
    type Read = Empty;
    type Write = Inputs;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        // `each` hands a morsel-relative index and the write adds the morsel
        // start. The values derive from the relative index, so they are the
        // same in every scheduler here, which all window the loader alike.
        EachApi::run(ctx.each(), |i| {
            let w = ctx.writer();
            let k = i.0;
            unsafe {
                w.write::<Pos, _>(i, Pos(k as i32 * 3 - 7));
                w.write::<Vel, _>(i, Vel((k % 11) as i32 - 5));
                w.write::<Kind, _>(i, Kind(k as u32 % KINDS));
            }
        });
    }
}

struct Typed;
impl BuilderInput for Typed {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Typed {
    type Ctx<'f> = EngineCtx<'f, Inputs, OutSet, SnapNil, InPtrs, OutPtrs>;
    type Hint = Hints;
    type Read = Inputs;
    type Write = OutSet;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| {
            let r = ctx.reader();
            let (p, v, k) = unsafe {
                (
                    r.read::<Pos, _>(i).0,
                    r.read::<Vel, _>(i).0,
                    r.read::<Kind, _>(i).0,
                )
            };
            let out = p.wrapping_add(v.wrapping_mul(COEFS[k as usize]) >> 1);
            unsafe { ctx.writer().write::<Out, _>(i, Out(out)) };
        });
    }
}

/// The slots the interpreter unit declares, in the order its Read and Write
/// sets name them. This table and the two associated types below are the
/// one compile-time fact a program is checked against.
const SLOTS: [Slot; 4] = [
    Slot {
        name:     "pos",
        readable: true,
        writable: false,
    },
    Slot {
        name:     "vel",
        readable: true,
        writable: false,
    },
    Slot {
        name:     "kind",
        readable: true,
        writable: false,
    },
    Slot {
        name:     "out",
        readable: false,
        writable: true,
    },
];

struct Interp {
    program:    Program,
    per_record: bool,
}
impl BuilderInput for Interp {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Interp {
    type Ctx<'f> = EngineCtx<'f, Inputs, OutSet, SnapNil, InPtrs, OutPtrs>;
    type Hint = Hints;
    type Read = Inputs;
    type Write = OutSet;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        let read = |s: u8, i: usize| -> i32 {
            let r = ctx.reader();
            let i = USize(i);
            unsafe {
                match s {
                    0 => r.read::<Pos, _>(i).0,
                    1 => r.read::<Vel, _>(i).0,
                    2 => r.read::<Kind, _>(i).0 as i32,
                    _ => unreachable!("refused at load"),
                }
            }
        };
        let write = |s: u8, i: usize, v: i32| {
            match s {
                3 => unsafe { ctx.writer().write::<Out, _>(USize(i), Out(v)) },
                _ => unreachable!("refused at load"),
            }
        };
        if self.per_record {
            EachApi::run(ctx.each(), |i| eval_record(&self.program, i.0, read, write));
        } else {
            BatchApi::run(ctx.batch(), |_, len| {
                eval_morsel(&self.program, len.0, read, write)
            });
        }
    }
}

struct Observer;
impl BuilderInput for Observer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Observer {
    type Ctx<'f> = EngineCtx<'f, OutSet, Empty, SnapNil, OutPtrs, ColPtrNil>;
    type Hint = Hints;
    type Read = OutSet;
    type Write = Empty;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| {
            let v = unsafe { ctx.reader().read::<Out, _>(i).0 };
            SEEN.with(|s| s.borrow_mut().push(v));
        });
    }
}

const RULE: &str = "load vel; load kind; coef; mul; shr 1; load pos; add; store out";

macro_rules! run_with {
    ($rule:expr) => {{
        SEEN.with(|s| s.borrow_mut().clear());
        let mut s = Scheduler::builder()
            .with(Column::<Pos>::new())
            .with(Column::<Vel>::new())
            .with(Column::<Kind>::new())
            .with(Column::<Out>::new())
            .with(Loader)
            .with($rule)
            .with(Observer)
            .build(store(ZeroedHeap), USize(N))
            .unwrap_or_else(|_| panic!("build"));
        assert!(matches!(s.run(), Outcome::Ok(())));
        SEEN.with(|s| s.borrow().clone())
    }};
}

#[test]
fn a_load_time_program_agrees_with_the_compiled_rule_in_both_forms() {
    let typed = run_with!(Typed);
    let program = load(RULE, &SLOTS, COEFS.to_vec()).expect("the rule loads");
    let morsel = run_with!(Interp {
        program:    program.clone(),
        per_record: false,
    });
    let record = run_with!(Interp {
        program,
        per_record: true
    });
    assert_eq!(typed.len(), N);
    let distinct: std::collections::BTreeSet<i32> = typed.iter().copied().collect();
    assert!(
        distinct.len() > 1000,
        "the rule's output is not near-constant: {} distinct values",
        distinct.len()
    );
    assert_eq!(
        morsel, typed,
        "vector-at-a-time interpretation matches the compiled rule"
    );
    assert_eq!(
        record, typed,
        "per-record interpretation matches the compiled rule"
    );
}

/// Control: the agreement above can fail. A program that ignores the kind's
/// coefficient must disagree with the compiled rule.
#[test]
fn a_different_program_disagrees_with_the_compiled_rule() {
    let typed = run_with!(Typed);
    let other = load(
        "load vel; const 3; mul; shr 1; load pos; add; store out",
        &SLOTS,
        COEFS.to_vec(),
    )
    .expect("loads");
    let got = run_with!(Interp {
        program:    other,
        per_record: false,
    });
    assert_ne!(got, typed);
}

#[test]
fn a_program_naming_an_undeclared_store_is_refused_at_load() {
    let r = load("load heat; store out", &SLOTS, vec![]);
    assert_eq!(r.unwrap_err(), Refused::UnknownStore {
        name: "heat".into(),
    });
}

#[test]
fn a_program_storing_into_a_read_only_store_is_refused_at_load() {
    let r = load("load vel; const 2; mul; store vel", &SLOTS, vec![]);
    assert_eq!(r.unwrap_err(), Refused::StoreIntoReadOnly {
        name: "vel".into(),
    });
}

#[test]
fn a_program_loading_a_write_only_store_is_refused_at_load() {
    let r = load("load out; store out", &SLOTS, vec![]);
    assert_eq!(r.unwrap_err(), Refused::LoadFromWriteOnly {
        name: "out".into(),
    });
}
