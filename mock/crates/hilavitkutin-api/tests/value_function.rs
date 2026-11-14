//! The value function on a real `EachApi`.
//!
//! A runner in the test gathers the read product at each record, applies the
//! function `map` was handed, and stores what it returns. The bodies are
//! written as a unit author writes them: `r.get(MEMBER)` for a read, `rec!`
//! for the writes.

#![no_std]

use core::cell::Cell;

use arvo::{Bool, Cap, USize};
use hilavitkutin_api::values::{RCons, RNil};
use hilavitkutin_api::{
    Accum,
    Column,
    EachApi,
    Reach,
    ReadVals,
    ResolveMap,
    Resource,
    ResourceRead,
    Virtual,
    Window,
    WriteVals,
    Written,
    read,
    rec,
    write,
};
use notko::Maybe;

const N: usize = 4;
const ONE: Cap = arvo_tensor::cap(1);

// --- A resource read by value, two columns, one column written ------------

#[derive(Clone, Copy, PartialEq, Debug)]
struct Pos(u32);
#[derive(Clone, Copy, PartialEq, Debug)]
struct Vel(u32);
#[derive(Clone, Copy, PartialEq, Debug)]
struct Step(u32);

impl ResourceRead for Step {
    type Val<'a> = Step;

    fn read<'a>(&'a self) -> Step {
        *self
    }
}

impl Pos {
    fn moved(self, v: Vel, dt: Step) -> Pos {
        Pos(self.0 + v.0 * dt.0)
    }
}

type IntegrateRead = read![Column<Pos>, Column<Vel>, Resource<Step>];
type IntegrateWrite = write![Column<Pos>];

struct IntegrateMorsel {
    pos:  Cell<[Pos; N]>,
    vel:  [Vel; N],
    step: Step,
}

impl EachApi<IntegrateRead, IntegrateWrite> for IntegrateMorsel {
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize),
    {
        for i in 0 .. N {
            f(USize(i));
        }
    }
}

impl ResolveMap<IntegrateRead, IntegrateWrite> for IntegrateMorsel {
    fn resolve_map<F>(&self, f: F)
    where
        F: for<'a> Fn(&ReadVals<'a, IntegrateRead>) -> WriteVals<IntegrateWrite>,
    {
        let mut pos = self.pos.get();
        for i in 0 .. N {
            let read = ReadVals::<IntegrateRead>(RCons {
                head: pos[i],
                tail: RCons {
                    head: self.vel[i],
                    tail: RCons {
                        head: self.step.read(),
                        tail: RNil,
                    },
                },
            });
            let out = f(&read);
            pos[i] = out.0.head;
        }
        self.pos.set(pos);
    }
}

#[test]
fn map_applies_the_function_at_every_record() {
    let m = IntegrateMorsel {
        pos:  Cell::new([Pos(0), Pos(10), Pos(20), Pos(30)]),
        vel:  [Vel(1), Vel(2), Vel(3), Vel(4)],
        step: Step(5),
    };
    EachApi::<IntegrateRead, IntegrateWrite>::map(&m, |r| {
        rec![r.get(Column::<Pos>::new()).moved(
            r.get(Column::<Vel>::new()),
            r.get(Resource::<Step>::default())
        )]
    });
    assert_eq!(m.pos.get(), [Pos(5), Pos(20), Pos(35), Pos(50)]);
}

// A function that returned the read unchanged would leave the column as it
// was: the test above fails for it, shown here by the same runner.
#[test]
fn map_stores_what_the_function_returns_and_nothing_else() {
    let m = IntegrateMorsel {
        pos:  Cell::new([Pos(0), Pos(10), Pos(20), Pos(30)]),
        vel:  [Vel(1), Vel(2), Vel(3), Vel(4)],
        step: Step(5),
    };
    EachApi::<IntegrateRead, IntegrateWrite>::map(&m, |r| rec![r.get(Column::<Pos>::new())]);
    assert_eq!(m.pos.get(), [Pos(0), Pos(10), Pos(20), Pos(30)]);
}

// --- A reach, a resource read by reference, and the three write kinds -----

#[derive(Clone, Copy, PartialEq, Debug)]
struct Height(u32);
/// No comparison: listed under `Written`.
#[derive(Clone, Copy)]
struct Slope(i64);
#[derive(Clone, Copy, PartialEq, Debug)]
struct Peak(u32);
struct Seen;

/// Holds a collection and is not `Copy`, so a function reads it in place.
struct Table {
    marks: [u32; 2],
}

impl ResourceRead for Table {
    type Val<'a> = &'a Table;

    fn read<'a>(&'a self) -> &'a Table {
        self
    }
}

type SurveyRead = read![Reach<Column<Height>, ONE>, Resource<Table>];
type SurveyWrite = write![Written<Column<Slope>>, Accum<Peak>, Virtual<Seen>];

struct SurveyMorsel {
    height: [Height; N],
    table:  Table,
    slope:  Cell<[Slope; N]>,
    peaks:  Cell<[Peak; N]>,
    kept:   Cell<usize>,
    fired:  Cell<u32>,
}

impl EachApi<SurveyRead, SurveyWrite> for SurveyMorsel {
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize),
    {
        for i in 0 .. N {
            f(USize(i));
        }
    }
}

impl ResolveMap<SurveyRead, SurveyWrite> for SurveyMorsel {
    fn resolve_map<F>(&self, f: F)
    where
        F: for<'a> Fn(&ReadVals<'a, SurveyRead>) -> WriteVals<SurveyWrite>,
    {
        let mut slope = self.slope.get();
        for i in 0 .. N {
            // SAFETY: `height` holds `N` initialised records, nothing writes
            // them while the window lives, and `i < N`.
            let window: Window<'_, Height, ONE> =
                unsafe { Window::over(self.height.as_ptr(), USize(i), USize(N)) };
            let read = ReadVals::<SurveyRead>(RCons {
                head: window,
                tail: RCons {
                    head: self.table.read(),
                    tail: RNil,
                },
            });
            let out = f(&read);
            // A column value lands at the record.
            slope[i] = out.0.head;
            // A present accumulator value is appended, an absent one is not.
            if let Maybe::Is(p) = out.get(Accum::<Peak>::new()) {
                let mut peaks = self.peaks.get();
                peaks[self.kept.get()] = p;
                self.peaks.set(peaks);
                self.kept.set(self.kept.get() + 1);
            }
            // A set virtual is fired, an unset one is not.
            if out.get(Virtual::<Seen>::new()) == Bool::TRUE {
                self.fired.set(self.fired.get() + 1);
            }
        }
        self.slope.set(slope);
    }
}

fn survey() -> SurveyMorsel {
    SurveyMorsel {
        height: [Height(3), Height(9), Height(4), Height(6)],
        table:  Table {
            marks: [9, 6],
        },
        slope:  Cell::new([Slope(0); N]),
        peaks:  Cell::new([Peak(0); N]),
        kept:   Cell::new(0),
        fired:  Cell::new(0),
    }
}

#[test]
fn map_reads_a_window_and_a_resource_by_reference() {
    let m = survey();
    EachApi::<SurveyRead, SurveyWrite>::map(&m, |r| {
        let w = r.get(Reach::<Column<Height>, ONE>::new());
        let table: &Table = r.get(Resource::<Table>::default());
        let here = w.here();
        // A neighbour past either end of the column reads as the record.
        let before = match w.before::<ONE>() {
            Maybe::Is(h) => h,
            Maybe::Isnt => here,
        };
        let after = match w.after::<ONE>() {
            Maybe::Is(h) => h,
            Maybe::Isnt => here,
        };
        let peak = if here.0 > before.0 && here.0 > after.0 {
            Maybe::Is(Peak(here.0))
        } else {
            Maybe::Isnt
        };
        let seen = Bool(table.marks[0] == here.0 || table.marks[1] == here.0);
        rec![Slope(after.0 as i64 - before.0 as i64), peak, seen]
    });

    let slope = m.slope.get();
    // 9 - 3, 4 - 3, 6 - 9, 6 - 4: the two ends read themselves on one side.
    assert_eq!([slope[0].0, slope[1].0, slope[2].0, slope[3].0], [
        6, 1, -3, 2
    ]);
    // One interior peak, at 9. The last record is no peak: nothing is read
    // past the end, so its neighbour on that side is itself.
    assert_eq!(m.kept.get(), 1);
    assert_eq!(m.peaks.get()[0], Peak(9));
    // Heights 9 and 6 are the table's marks.
    assert_eq!(m.fired.get(), 2);
}

#[test]
fn a_window_reads_nothing_past_either_end() {
    let height = [Height(3), Height(9), Height(4), Height(6)];
    // SAFETY: four initialised records, unwritten while the windows live.
    let first: Window<'_, Height, ONE> =
        unsafe { Window::over(height.as_ptr(), USize(0), USize(N)) };
    // SAFETY: as above, and record 3 is one of the four.
    let last: Window<'_, Height, ONE> =
        unsafe { Window::over(height.as_ptr(), USize(3), USize(N)) };

    assert_eq!(first.here(), Height(3));
    assert!(matches!(first.before::<ONE>(), Maybe::Isnt));
    assert!(matches!(first.after::<ONE>(), Maybe::Is(Height(9))));
    assert_eq!(last.here(), Height(6));
    assert!(matches!(last.before::<ONE>(), Maybe::Is(Height(4))));
    assert!(matches!(last.after::<ONE>(), Maybe::Isnt));
}

// --- The empty write list ---------------------------------------------------

type CountRead = read![Column<Pos>];
type CountWrite = write![];

struct CountMorsel {
    pos:     [Pos; N],
    applied: Cell<u32>,
}

impl EachApi<CountRead, CountWrite> for CountMorsel {
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize),
    {
        for i in 0 .. N {
            f(USize(i));
        }
    }
}

impl ResolveMap<CountRead, CountWrite> for CountMorsel {
    fn resolve_map<F>(&self, f: F)
    where
        F: for<'a> Fn(&ReadVals<'a, CountRead>) -> WriteVals<CountWrite>,
    {
        for i in 0 .. N {
            let read = ReadVals::<CountRead>(RCons {
                head: self.pos[i],
                tail: RNil,
            });
            let _nothing = f(&read);
            self.applied.set(self.applied.get() + 1);
        }
    }
}

#[test]
fn a_function_with_no_writes_returns_the_empty_product() {
    let m = CountMorsel {
        pos:     [Pos(1), Pos(2), Pos(3), Pos(4)],
        applied: Cell::new(0),
    };
    EachApi::<CountRead, CountWrite>::map(&m, |_r| rec![]);
    assert_eq!(m.applied.get(), 4);
}
