//! The seal on `Stage` and `Kernel`.
//!
//! The runner's one unsafe cut trusts `MASK` and `LEN`. Those are safe
//! associated consts, so an impl outside the engine could state any mask it
//! liked. Each trait therefore has a required method taking `Token`, a type in
//! this private module: a crate outside cannot name `Token`, so it cannot
//! write the method, and so it cannot implement either trait, whatever self
//! type or position list it picks.
//!
//! A supertrait seal is not enough here. The engine has to implement it for
//! every `Cons<H, T>`, `Block<K>` and `Value<K>`, and the orphan rule then lets
//! a crate outside implement `Stage` for `Cons<Pos, Empty>` at a position list
//! of its own, or `Kernel` for `Block` of a kernel of its own (fixtures 12b and
//! 12c, which built under that seal).

/// Named only in this crate. No constructor is needed: the method taking it is
/// never called.
pub struct Token(());
