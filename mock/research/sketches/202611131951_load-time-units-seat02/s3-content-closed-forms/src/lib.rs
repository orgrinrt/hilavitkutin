//! Spike s3, seat 02: content is data, units fixed.
//!
//! A compiled unit holds a closed set of forms; content rows, read at load, name a
//! form, the stores it reads and writes, and its parameters. The binding-time
//! split: the set of forms and each form's loop are build-time; which rows exist,
//! which stores they bind and their constants are load-time. The unit dispatches on
//! the form once a morsel, so each form's loop is a monomorphised loop.
//!
//! Checks: rows binding stores outside the unit's envelope are refused at load; a
//! row asking for a form the closed set does not hold has no representation and is
//! refused; rows compose to what a hand-written unit computes; and the shape of the
//! code, once-a-morsel dispatch against once-a-record (`asm/`).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    /// out = in * k >> 16
    Scale { k: i64 },
    /// out = in + k
    Offset { k: i64 },
    /// out = clamp(in, lo, hi)
    Clamp { lo: i64, hi: i64 },
    /// out = a + b
    Sum,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rule {
    pub form: Form,
    pub a: usize,
    pub b: usize,
    pub out: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoadRefusal {
    UnknownForm(String),
    StoreOutsideEnvelope(String),
    WrongArity(String),
    BadNumber(String),
}

/// The stores the compiled unit declared at build, by name, in slot order. The
/// first `reads` are read only; the rest are written.
pub struct Envelope {
    pub names: &'static [&'static str],
    pub writable_from: usize,
}

fn slot(env: &Envelope, name: &str, write: bool) -> Result<usize, LoadRefusal> {
    match env.names.iter().position(|n| *n == name) {
        Some(i) if !write || i >= env.writable_from => Ok(i),
        _ => Err(LoadRefusal::StoreOutsideEnvelope(name.to_string())),
    }
}

fn num(s: &str) -> Result<i64, LoadRefusal> {
    s.parse().map_err(|_| LoadRefusal::BadNumber(s.to_string()))
}

/// One content row per line: `<form> <out> <in> [<in2>|<k>|<lo> <hi>]`.
pub fn load(env: &Envelope, text: &str) -> Result<Vec<Rule>, LoadRefusal> {
    let mut rules = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let w: Vec<&str> = line.split_whitespace().collect();
        let rule = match (w[0], w.len()) {
            ("scale", 4) => Rule { form: Form::Scale { k: num(w[3])? }, a: slot(env, w[2], false)?, b: 0, out: slot(env, w[1], true)? },
            ("offset", 4) => Rule { form: Form::Offset { k: num(w[3])? }, a: slot(env, w[2], false)?, b: 0, out: slot(env, w[1], true)? },
            ("clamp", 5) => Rule { form: Form::Clamp { lo: num(w[3])?, hi: num(w[4])? }, a: slot(env, w[2], false)?, b: 0, out: slot(env, w[1], true)? },
            ("sum", 4) => Rule { form: Form::Sum, a: slot(env, w[2], false)?, b: slot(env, w[3], false)?, out: slot(env, w[1], true)? },
            ("scale" | "offset" | "clamp" | "sum", _) => return Err(LoadRefusal::WrongArity(line.to_string())),
            (other, _) => return Err(LoadRefusal::UnknownForm(other.to_string())),
        };
        rules.push(rule);
    }
    Ok(rules)
}

/// The compiled unit's body over one morsel `[s, e)`: the rule's form is matched
/// once, then its own loop runs.
pub fn run_morsel(rules: &[Rule], cols: &mut [Vec<i64>], s: usize, e: usize) {
    for r in rules {
        match r.form {
            Form::Scale { k } => s3_scale(&cols[r.a][s..e].to_vec(), k, &mut cols[r.out][s..e]),
            Form::Offset { k } => {
                let a = cols[r.a][s..e].to_vec();
                for (o, x) in cols[r.out][s..e].iter_mut().zip(a) {
                    *o = x.wrapping_add(k);
                }
            }
            Form::Clamp { lo, hi } => {
                let a = cols[r.a][s..e].to_vec();
                for (o, x) in cols[r.out][s..e].iter_mut().zip(a) {
                    *o = x.clamp(lo, hi);
                }
            }
            Form::Sum => {
                let (a, b) = (cols[r.a][s..e].to_vec(), cols[r.b][s..e].to_vec());
                for ((o, x), y) in cols[r.out][s..e].iter_mut().zip(a).zip(b) {
                    *o = x.wrapping_add(y);
                }
            }
        }
    }
}

pub fn run_pass(rules: &[Rule], cols: &mut [Vec<i64>], morsel: usize) {
    let n = cols[0].len();
    let mut s = 0;
    while s < n {
        let e = (s + morsel).min(n);
        run_morsel(rules, cols, s, e);
        s = e;
    }
}

/// The Scale form's loop, as the compiled unit holds it.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn s3_scale(a: &[i64], k: i64, out: &mut [i64]) {
    let n = a.len().min(out.len());
    for i in 0..n {
        out[i] = a[i].wrapping_mul(k) >> 16;
    }
}

/// The same rule set dispatched on the form at every record, the interpreter shape.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn s3_per_record(form: &Form, a: &[i64], out: &mut [i64]) {
    let form = core::hint::black_box(form);
    let n = a.len().min(out.len());
    for i in 0..n {
        out[i] = match *form {
            Form::Scale { k } => a[i].wrapping_mul(k) >> 16,
            Form::Offset { k } => a[i].wrapping_add(k),
            Form::Clamp { lo, hi } => a[i].clamp(lo, hi),
            Form::Sum => a[i],
        };
    }
}
