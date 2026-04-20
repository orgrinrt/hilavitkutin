//! Monomorphised WU function-pointer shape (domain 17).
//!
//! Matches `WorkUnit::execute(&self, &Ctx)` from hilavitkutin-api,
//! with the `&self` receiver closed over at monomorphisation time.
//! Skeleton ships just the alias; emission of the closed-over form
//! is a follow-up round (see BACKLOG → `codegen_fiber`).

/// Function-pointer shape used by dispatch records.
pub type WuFn<Ctx> = fn(&Ctx);
