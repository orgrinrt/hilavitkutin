//! The bridge from `StringInterner` to [`InternerApi`], on every build.
//!
//! It rests on nothing a feature gates, and coherence lets only this
//! crate write it, since a consumer owns neither the trait nor the type.
//! So a consumer with an arena of its own reaches its interner through
//! the contract on a default build.

use hilavitkutin_str::{ArenaInterner, Str, StringInterner};
use notko::Maybe;

use super::InternerApi;

/// Any `StringInterner<A>` serves as an [`InternerApi`] provider.
/// Consumers wire `Resource<StringInterner<A>>` and the `HasInterner`
/// accessor returns it.
impl<A> InternerApi for StringInterner<A>
where
    A: ArenaInterner + 'static,
{
    #[inline(always)]
    fn intern(
        &self,
        s: &str, // lint:allow(no-bare-string) reason: interner boundary mirror of the trait method; tracked: #72
    ) -> Maybe<Str> {
        StringInterner::intern(self, s)
    }

    #[inline(always)]
    #[rustfmt::skip] // keeps the allow on the signature it governs
    fn resolve(
        &self,
        s: Str,
    ) -> Maybe<&str> { // lint:allow(no-bare-string) reason: interner boundary mirror of the trait method; tracked: #72
        StringInterner::resolve(self, s)
    }
}
