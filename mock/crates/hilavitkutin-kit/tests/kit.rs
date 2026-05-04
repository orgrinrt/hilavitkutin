//! Crate-isolation smoke test for `hilavitkutin-kit`.
//!
//! Proves the `Kit<B>` trait shape compiles and is invocable
//! independently of the engine and the api crate. The Kit's `B`
//! parameter accepts any type; engine-side coupling
//! (`BuilderExtending<Self>`) lives at the engine call site, not
//! at the Kit declaration.

#![no_std]

use hilavitkutin_kit::Kit;

struct DummyBuilder;

struct DummyKit;

impl Kit<DummyBuilder> for DummyKit {
    type Output = DummyBuilder;
    fn install(self, builder: DummyBuilder) -> DummyBuilder {
        builder
    }
}

#[test]
fn kit_trait_shape_compiles_standalone() {
    let _: DummyBuilder = DummyKit.install(DummyBuilder);
}
