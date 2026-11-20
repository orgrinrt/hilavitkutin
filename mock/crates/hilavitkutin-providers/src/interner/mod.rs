//! Interner provider surface.
//!
//! Provides the [`InternerApi`] provider trait and the [`HasInterner`]
//! accessor, the two contracts this crate declares, on every build.
//! WorkUnits that handle [`Str`] declare `HasInterner` in their `Ctx`
//! tuple bound and call `ctx.interner().intern(s)` /
//! `ctx.interner().resolve(handle)` from `execute()`. The default
//! implementation, the inline `MemoryArena`, `default_interner` and
//! `InternerKit`, is under the `portable` feature. The bridge that lets a
//! `StringInterner` serve as an `InternerApi` provider is not.

use hilavitkutin_str::Str;
use notko::Maybe;

mod bridge;

#[cfg(feature = "portable")]
mod arena;

#[cfg(feature = "portable")]
pub use arena::{InternerKit, MemoryArena, default_interner};

/// Provider-shape trait for the interner.
///
/// Parallels the platform contracts [`MemoryProviderApi`] /
/// [`ClockApi`] / [`ThreadPoolApi`] in `hilavitkutin-api`. The
/// `'static` bound lets the value live in a `Resource<T>`. v0 does
/// not require `Send + Sync`; the engine's resource access during
/// single-thread morsel dispatch is sequential. The Sync arena
/// variant (BACKLOG) generalises this for multi-thread schedules.
///
/// [`MemoryProviderApi`]: hilavitkutin_api::MemoryProviderApi
/// [`ClockApi`]: hilavitkutin_api::ClockApi
/// [`ThreadPoolApi`]: hilavitkutin_api::ThreadPoolApi
pub trait InternerApi: 'static {
    /// Intern a string. Const-table short-circuit applies before
    /// the arena lookup. `Maybe::Isnt` when the arena has no room for it.
    fn intern(&self, s: &str) -> Maybe<Str>; // lint:allow(no-bare-string) reason: interner boundary; the input is the &str the consumer passes; tracked: #72

    /// Resolve a handle back to a string. Returns `Maybe::Isnt` for a
    /// const handle with no string behind it; a runtime id the arena
    /// did not issue does not come back as `Maybe::Isnt` yet.
    fn resolve(&self, s: Str) -> Maybe<&str>; // lint:allow(no-bare-string) reason: interner boundary; resolved &str; tracked: #72
}

/// Accessor for the interner provider in a Context tuple.
///
/// Mirrors `HasMemoryProvider` / `HasClock` / `HasThreadPool` from
/// `hilavitkutin-api`. WorkUnits that need to intern or resolve
/// strings declare `HasInterner` in their `Ctx` tuple bound.
pub trait HasInterner {
    /// Concrete interner implementation type.
    type Provider: InternerApi;

    /// Borrow the interner provider.
    fn interner(&self) -> &Self::Provider;
}
