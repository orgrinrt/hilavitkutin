//! The instruments a runner of work units is held to.
//!
//! A one-thread reference runner that states what a frame means, a digest that
//! writes a frame's stores down as one word, and the laws, which are functions
//! over what runs wrote down.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(adt_const_params)]

pub mod digest;
pub mod laws;
pub mod oracle;

pub use digest::{Fold, Word};
pub use laws::{Breach, Law, Verdict, isolation, model, split};
pub use oracle::{
    Ascending,
    Descending,
    Faithful,
    Keeping,
    Lent,
    Oracle,
    Refusal,
    Visiting,
    mutant,
};
