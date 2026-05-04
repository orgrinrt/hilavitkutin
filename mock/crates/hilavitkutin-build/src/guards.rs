//! `compile_error!` helpers for ISA / vendor / arch mismatches.
//!
//! DESIGN Q2 bans custom cargo features: `compile_error!` guards
//! are the replacement mechanism when a pragma or consumer needs to
//! assert a target property. Only `require_isa!` ships this round;
//! the wrapper-script follow-up adds vendor / arch variants per
//! BACKLOG.

/// Emit a `compile_error!` stating which `-C target-feature` flag
/// the consumer needs. The macro always expands to a hard error , 
/// call it inside a `#[cfg(...)]` that only fires on mismatch.
///
/// ```ignore
/// #[cfg(not(target_feature = "avx2"))]
/// hilavitkutin_build::require_isa!("avx2");
/// ```
#[macro_export]
macro_rules! require_isa {
    ($feat:literal) => {
        ::core::compile_error!(::core::concat!(
            "hilavitkutin-build: target must support `",
            $feat,
            "` (add --target-feature=+",
            $feat,
            " to RUSTFLAGS or use a -C target-cpu=... that enables it)"
        ));
    };
}
