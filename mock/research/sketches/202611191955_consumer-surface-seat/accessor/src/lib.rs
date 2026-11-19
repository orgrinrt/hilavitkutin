//! Where can the interner accessor of a unit's context be implemented?
//!
//! `HasInterner` lives in providers (src/interner.rs:47-53). The engine does
//! not depend on providers and providers does not depend on the engine, so an
//! impl for the engine's context type can be written in neither. The only
//! place left is a blanket over the api's resource accessor, which providers
//! does depend on. Arm `fixed` tries that with the shipped trait shape; arm
//! `method` tries it as a generic method.
#![no_std]


#[cfg(feature = "fixed")]
impl<R, C, A, I> hilavitkutin_providers::HasInterner for C
where
    R: AccessSet + Holds<Resource<StringInterner<A>>>,
    C: ResourceProviderApi<R> + ResolveResource<StringInterner<A>, I>,
    A: ArenaInterner + 'static,
{
    type Provider = StringInterner<A>;

    fn interner(&self) -> &Self::Provider {
        self.resource::<StringInterner<A>, I>()
    }
}

#[cfg(feature = "method")]
pub trait InternerReach<R: AccessSet>: ResourceProviderApi<R> {
    fn interner<A: ArenaInterner + 'static, I>(&self) -> &StringInterner<A>
    where
        R: Holds<Resource<StringInterner<A>>>,
        Self: ResolveResource<StringInterner<A>, I>,
    {
        self.resource::<StringInterner<A>, I>()
    }
}

#[cfg(feature = "method")]
impl<R: AccessSet, C: ResourceProviderApi<R>> InternerReach<R> for C {}
