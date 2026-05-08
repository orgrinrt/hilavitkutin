// Sketch: verify a macro_rules! macro can expand to a crate-level inner
// attribute (`#![recursion_limit = "1024"]`) when invoked at the top of
// the consumer crate root.
//
// Hypothesis: WORKS. macro_rules! macros invoked at the top of lib.rs
// before any items can produce inner attributes that apply to the crate
// root. This is the pattern used by `recursion_limit_for_kits!()` per
// round 202605042200's locked design.
//
// To run as a self-contained probe:
//   rustc --edition 2024 sketch.rs -o /tmp/recursion_limit_macro_sketch
//   /tmp/recursion_limit_macro_sketch  # should print OK

// Define the macro upstream-style.
macro_rules! recursion_limit_for_kits {
    () => { #![recursion_limit = "1024"] };
}

// Invoke at the very top of the crate root, before any items.
recursion_limit_for_kits!();

// Force the trait solver to walk a deep recursion chain to verify
// the expanded limit applies. If 1024 didn't apply, default 128 would
// crash on this depth.
trait Recurse<const N: usize> {}

// Base case.
impl Recurse<0> for () {}

// Recursive case. With recursion_limit = 1024, we can chain to depth ~512.
// Without it, ~32 to 64 typically fails.
impl<const N: usize> Recurse<N> for ()
where
    (): Recurse<{ N - 1 }>,
{
}

fn force<const N: usize>() where (): Recurse<N> {}

fn main() {
    // Force depth 200 — well past default 128.
    force::<200>();
    println!("OK: recursion_limit raised by macro expansion");
}
