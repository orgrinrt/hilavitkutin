//! A content program and its load-time check.
//!
//! The program language is a stack machine over i32 lanes, small enough to
//! read whole: `load <store>`, `const <n>`, `coef` (pop a kind id, push the
//! content table's coefficient for it), `add`, `mul`, `shr <n>`, `store <store>`.
//! Store names resolve against the slots the hosting unit declared in Rust.
//! A program that names a store outside them, or stores into one the unit
//! only reads, is refused when it loads, as a value, before any schedule
//! exists. That refusal is what keeps the compiled access set true.
//!
//! The spike uses std (Vec, String) for the loader; the unit body it feeds is
//! the part that would have to be no_std, and it allocates per morsel here
//! (one Vec per register), which a real one would not.

/// A store slot the hosting unit declared. Its index is its position in the
/// unit's declared set; `readable` and `writable` say whether the unit's
/// Read and Write sets hold it.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub name:     &'static str,
    pub readable: bool,
    pub writable: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Load(u8),
    Const(i32),
    Coef,
    Add,
    Mul,
    Shr(u8),
    Store(u8),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refused {
    UnknownStore {
        name: String,
    },
    LoadFromWriteOnly {
        name: String,
    },
    StoreIntoReadOnly {
        name: String,
    },
    BadToken {
        token: String,
    },
    StackUnderflow {
        at: usize,
    },
    NoStore,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub ops:   Vec<Op>,
    /// Coefficient per content kind, filled from content at load.
    pub coefs: Vec<i32>,
}

/// Parse and check a program against the slots the hosting unit declared.
pub fn load(text: &str, slots: &[Slot], coefs: Vec<i32>) -> Result<Program, Refused> {
    let find = |name: &str| slots.iter().position(|s| s.name == name);
    let mut ops = Vec::new();
    let mut depth: i32 = 0;
    let mut stored = false;
    for (at, stmt) in text
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .enumerate()
    {
        let mut it = stmt.split_whitespace();
        let word = it.next().unwrap_or("");
        let arg = it.next();
        let op = match (word, arg) {
            ("load", Some(n)) => {
                let i = find(n).ok_or(Refused::UnknownStore {
                    name: n.into(),
                })?;
                if !slots[i].readable {
                    return Err(Refused::LoadFromWriteOnly {
                        name: n.into(),
                    });
                }
                depth += 1;
                Op::Load(i as u8)
            },
            ("store", Some(n)) => {
                let i = find(n).ok_or(Refused::UnknownStore {
                    name: n.into(),
                })?;
                if !slots[i].writable {
                    return Err(Refused::StoreIntoReadOnly {
                        name: n.into(),
                    });
                }
                depth -= 1;
                stored = true;
                Op::Store(i as u8)
            },
            ("const", Some(v)) => {
                depth += 1;
                Op::Const(v.parse().map_err(|_| {
                    Refused::BadToken {
                        token: v.into(),
                    }
                })?)
            },
            ("shr", Some(v)) => {
                Op::Shr(v.parse().map_err(|_| {
                    Refused::BadToken {
                        token: v.into(),
                    }
                })?)
            },
            ("coef", None) => Op::Coef,
            ("add", None) => {
                depth -= 1;
                Op::Add
            },
            ("mul", None) => {
                depth -= 1;
                Op::Mul
            },
            _ => {
                return Err(Refused::BadToken {
                    token: stmt.into(),
                });
            },
        };
        if depth < 0 {
            return Err(Refused::StackUnderflow {
                at,
            });
        }
        ops.push(op);
    }
    if !stored {
        return Err(Refused::NoStore);
    }
    Ok(Program {
        ops,
        coefs,
    })
}

/// Evaluate a program over one morsel, vector at a time: each op runs over
/// every record of the morsel before the next op starts, so the op dispatch
/// is paid once per morsel and the inner loop is a plain lane loop.
/// `read(slot, i)` and `write(slot, i, v)` are the unit's typed accessors.
pub fn eval_morsel(
    p: &Program,
    len: usize,
    mut read: impl FnMut(u8, usize) -> i32,
    mut write: impl FnMut(u8, usize, i32),
) {
    let mut stack: Vec<Vec<i32>> = Vec::new();
    for op in &p.ops {
        match *op {
            Op::Load(s) => stack.push((0 .. len).map(|i| read(s, i)).collect()),
            Op::Const(v) => stack.push(vec![v; len]),
            Op::Coef => {
                let top = stack.last_mut().expect("checked at load");
                for x in top.iter_mut() {
                    *x = p.coefs.get(*x as usize).copied().unwrap_or(0);
                }
            },
            Op::Add | Op::Mul => {
                let b = stack.pop().expect("checked at load");
                let a = stack.last_mut().expect("checked at load");
                for (x, y) in a.iter_mut().zip(b.iter()) {
                    *x = if *op == Op::Add { x.wrapping_add(*y) } else { x.wrapping_mul(*y) };
                }
            },
            Op::Shr(k) => {
                for x in stack.last_mut().expect("checked at load").iter_mut() {
                    *x >>= k;
                }
            },
            Op::Store(s) => {
                let v = stack.pop().expect("checked at load");
                for (i, x) in v.iter().enumerate() {
                    write(s, i, *x);
                }
            },
        }
    }
}

/// Evaluate a program for one record, op dispatch per record: the form a
/// per-record call into an interpreter takes.
pub fn eval_record(
    p: &Program,
    i: usize,
    mut read: impl FnMut(u8, usize) -> i32,
    mut write: impl FnMut(u8, usize, i32),
) {
    let mut stack: [i32; 16] = [0; 16];
    let mut sp = 0usize;
    for op in &p.ops {
        match *op {
            Op::Load(s) => {
                stack[sp] = read(s, i);
                sp += 1;
            },
            Op::Const(v) => {
                stack[sp] = v;
                sp += 1;
            },
            Op::Coef => stack[sp - 1] = p.coefs.get(stack[sp - 1] as usize).copied().unwrap_or(0),
            Op::Add => {
                sp -= 1;
                stack[sp - 1] = stack[sp - 1].wrapping_add(stack[sp]);
            },
            Op::Mul => {
                sp -= 1;
                stack[sp - 1] = stack[sp - 1].wrapping_mul(stack[sp]);
            },
            Op::Shr(k) => stack[sp - 1] >>= k,
            Op::Store(s) => {
                sp -= 1;
                write(s, i, stack[sp]);
            },
        }
    }
}
