//! hilavitkutin-ctx — provider-gated context framework.
//!
//! no_std, zero deps. Provides `Context<P>`, `provider!` macro
//! for accessor trait generation, and `tuple!` macro for
//! positional accessor impls on tuples.

#![no_std]

/// Context wraps a provider set P. Methods appear on Context<P>
/// based on which accessor traits P implements.
pub struct Context<P> {
    pub providers: P,
}

/// Generates an accessor trait and a Context delegation impl.
///
/// Usage:
/// ```ignore
/// provider!(ConnectorApi as HasConnector => connection);
/// ```
///
/// Generates:
/// - `trait HasConnector { type Connector: ConnectorApi; fn connection(&self) -> &Self::Connector; }`
/// - `impl<P: HasConnector> Context<P> { fn connection(&self) -> &P::Connector { ... } }`
#[macro_export]
macro_rules! provider {
    ($ApiTrait:ident as $AccTrait:ident => $method:ident) => {
        pub trait $AccTrait {
            type Provider: $ApiTrait;
            fn $method(&self) -> &Self::Provider;
        }

        impl<P: $AccTrait> $crate::Context<P> {
            pub fn $method(&self) -> &<P as $AccTrait>::Provider {
                self.providers.$method()
            }
        }
    };
}

/// Generates accessor trait impls on a tuple for a specific layout.
///
/// Each position gets ONE bound — no coherence conflicts.
///
/// Usage:
/// ```ignore
/// tuple!(ConnectorApi: HasConnector => connection,
///        WriterApi: HasWriter => writer);
/// ```
#[macro_export]
macro_rules! tuple {
    // 2-tuple
    (
        $Api0:ident : $Acc0:ident => $m0:ident,
        $Api1:ident : $Acc1:ident => $m1:ident $(,)?
    ) => {
        impl<A: $Api0, B> $Acc0 for (A, B) {
            type Provider = A;
            fn $m0(&self) -> &A { &self.0 }
        }
        impl<A, B: $Api1> $Acc1 for (A, B) {
            type Provider = B;
            fn $m1(&self) -> &B { &self.1 }
        }
    };

    // 3-tuple
    (
        $Api0:ident : $Acc0:ident => $m0:ident,
        $Api1:ident : $Acc1:ident => $m1:ident,
        $Api2:ident : $Acc2:ident => $m2:ident $(,)?
    ) => {
        impl<A: $Api0, B, C> $Acc0 for (A, B, C) {
            type Provider = A;
            fn $m0(&self) -> &A { &self.0 }
        }
        impl<A, B: $Api1, C> $Acc1 for (A, B, C) {
            type Provider = B;
            fn $m1(&self) -> &B { &self.1 }
        }
        impl<A, B, C: $Api2> $Acc2 for (A, B, C) {
            type Provider = C;
            fn $m2(&self) -> &C { &self.2 }
        }
    };

    // 4-tuple
    (
        $Api0:ident : $Acc0:ident => $m0:ident,
        $Api1:ident : $Acc1:ident => $m1:ident,
        $Api2:ident : $Acc2:ident => $m2:ident,
        $Api3:ident : $Acc3:ident => $m3:ident $(,)?
    ) => {
        impl<A: $Api0, B, C, D> $Acc0 for (A, B, C, D) {
            type Provider = A;
            fn $m0(&self) -> &A { &self.0 }
        }
        impl<A, B: $Api1, C, D> $Acc1 for (A, B, C, D) {
            type Provider = B;
            fn $m1(&self) -> &B { &self.1 }
        }
        impl<A, B, C: $Api2, D> $Acc2 for (A, B, C, D) {
            type Provider = C;
            fn $m2(&self) -> &C { &self.2 }
        }
        impl<A, B, C, D: $Api3> $Acc3 for (A, B, C, D) {
            type Provider = D;
            fn $m3(&self) -> &D { &self.3 }
        }
    };
}

/// Generates an accessor trait for an API trait parameterised over
/// a single type parameter.
///
/// Usage:
/// ```ignore
/// provider_generic!(<R: AccessSet> ColumnReaderApi as HasColumnReader => reader);
/// ```
///
/// Assumes the API trait and accessor trait both take one type
/// parameter matching `$R`. Generic methods on the API trait may
/// carry further bounds on `$R` (e.g. `where R: Contains<...>`) —
/// those flow through unchanged.
///
/// Consumers call the accessor method on the provider tuple
/// (bringing the accessor trait into scope). Unlike `provider!`,
/// this macro does NOT emit a `Context<P>` inherent delegation:
/// Rust's orphan rule forbids inherent impls on foreign types from
/// downstream crates. If you need `ctx.method()` sugar, wrap the
/// provider tuple in your own `MyCtx` newtype and implement
/// accessor methods on it directly.
#[macro_export]
macro_rules! provider_generic {
    (
        < $R:ident : $RBound:path >
        $ApiTrait:ident as $AccTrait:ident => $method:ident
    ) => {
        pub trait $AccTrait<$R: $RBound> {
            type Provider: $ApiTrait<$R>;
            fn $method(&self) -> &Self::Provider;
        }
    };
}

/// Two-parameter variant of `provider_generic!`.
///
/// Usage:
/// ```ignore
/// provider_generic2!(<R: AccessSet, W: AccessSet>
///                    EachApi as HasEach => each);
/// ```
///
/// Separate macro because Rust declarative macros can't cleanly
/// match a variable-length generic parameter list. Extend with
/// `provider_generic3!` etc. if a 3+ parameter need surfaces.
/// Same orphan-rule caveat as `provider_generic!` — no
/// `Context<P>` delegation is emitted.
#[macro_export]
macro_rules! provider_generic2 {
    (
        < $R:ident : $RBound:path, $W:ident : $WBound:path >
        $ApiTrait:ident as $AccTrait:ident => $method:ident
    ) => {
        pub trait $AccTrait<$R: $RBound, $W: $WBound> {
            type Provider: $ApiTrait<$R, $W>;
            fn $method(&self) -> &Self::Provider;
        }
    };
}

/// Facade macro: declares providers and tuple layouts in one call.
///
/// Usage:
/// ```ignore
/// define_providers! {
///     providers {
///         ConnectorApi as HasConnector => connection,
///         QueryApi as HasQuery => querier,
///         WriterApi as HasWriter => writer,
///     }
///     layouts {
///         (ConnectorApi: HasConnector => connection,
///          WriterApi: HasWriter => writer),
///         (ConnectorApi: HasConnector => connection,
///          QueryApi: HasQuery => querier,
///          WriterApi: HasWriter => writer),
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_providers {
    (
        providers {
            $( $ApiTrait:ident as $AccTrait:ident => $method:ident ),+ $(,)?
        }
        layouts {
            $( ( $($layout_api:ident : $layout_acc:ident => $layout_method:ident),+ ) ),+ $(,)?
        }
    ) => {
        $(
            $crate::provider!($ApiTrait as $AccTrait => $method);
        )+

        $(
            $crate::tuple!( $($layout_api : $layout_acc => $layout_method),+ );
        )+
    };
}
