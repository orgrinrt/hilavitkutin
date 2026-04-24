#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

//! Contract-bound host orchestration over `hilavitkutin-linking`.
//!
//! This crate layers the pull-based discovery contract, lifecycle
//! management, capability dispatch, and failure policy on top of the
//! cross-platform dynamic library loader in `hilavitkutin-linking`.
//! Extensions are `cdylib` artefacts that export a single well-known
//! symbol `__hilavitkutin_extension_descriptor` returning a pointer
//! to a `#[repr(C)]` `ExtensionDescriptor`.
//!
//! # Arbitrary-time linking invariant
//!
//! Any extension may be loaded, invoked, and dropped at any point in
//! the host's execution, independent of siblings. No global registry,
//! no ecosystem-wide init gate. The contract enforces this by
//! construction: lifecycle is strictly per-extension and descriptor
//! shapes never cross-reference sibling extensions.

mod descriptor;
mod error;
mod extension;
mod host;
mod traits;

pub use descriptor::{
    CapabilityEntry, CapabilityId, DESCRIPTOR_SYMBOL, ExtensionAbiStatus,
    ExtensionDescriptor, ExtensionVersion, HOST_ABI_VERSION,
};
pub use error::ExtensionError;
pub use extension::Extension;
pub use host::{
    ExtensionHost, ExtensionRequirement, FailurePolicyFn, PolicyVerdict,
    default_policy,
};
pub use traits::{CapabilityExport, ExtensionMeta, InitHandler, ShutdownHandler};

#[doc(hidden)]
pub use notko::MaybeNull;

#[cfg(test)]
mod tests {
    // Tests in this module validate the ABI layout itself: FNV-1a hash
    // widths, `#[repr(u32)]` discriminants, struct sizes, transparent
    // repr equivalence. They necessarily reference raw primitives
    // because they verify the ABI contract. All tracked: #206.
    use super::*;

    #[test]
    fn capability_id_from_name_is_const_fnv_1a() {
        // FNV-1a over "cap.a": stable bit-equality across platforms.
        const CAP: CapabilityId = CapabilityId::from_name("cap.a");
        // Recompute with an independent FNV-1a run to cross-check.
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test verifies FNV-1a algorithm width; tracked: #206
        const FNV_PRIME: u64 = 0x100000001b3; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test verifies FNV-1a algorithm width; tracked: #206
        let mut h: u64 = FNV_OFFSET_BASIS; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test FNV-1a state; tracked: #206
        for &b in b"cap.a" {
            h ^= b as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test FNV-1a step cast; tracked: #206
            h = h.wrapping_mul(FNV_PRIME);
        }
        assert_eq!(CAP.0, h);
    }

    #[test]
    fn capability_id_distinct_names_differ() {
        assert_ne!(
            CapabilityId::from_name("cap.a").0,
            CapabilityId::from_name("cap.b").0,
        );
    }

    #[test]
    fn host_abi_version_is_one() {
        assert_eq!(HOST_ABI_VERSION, 1);
    }

    #[test]
    fn default_policy_aborts_on_required() {
        let err = ExtensionError::DescriptorMissing;
        assert_eq!(
            default_policy(&err, ExtensionRequirement::Required),
            PolicyVerdict::Abort,
        );
    }

    #[test]
    fn default_policy_continues_on_optional() {
        let err = ExtensionError::DescriptorMissing;
        assert_eq!(
            default_policy(&err, ExtensionRequirement::Optional),
            PolicyVerdict::Continue,
        );
    }

    #[test]
    fn host_advertises_declared_capabilities() {
        const CAP_X: CapabilityId = CapabilityId::from_name("cap.x");
        const CAP_Y: CapabilityId = CapabilityId::from_name("cap.y");
        const CAP_Z: CapabilityId = CapabilityId::from_name("cap.z");
        static CAPS: &[CapabilityId] = &[CAP_X, CAP_Y];
        let host = ExtensionHost::new(CAPS);
        assert!(host.has_capability(CAP_X).0);
        assert!(host.has_capability(CAP_Y).0);
        assert!(!host.has_capability(CAP_Z).0);
    }

    #[test]
    fn descriptor_symbol_is_null_terminated() {
        assert_eq!(DESCRIPTOR_SYMBOL.last(), Some(&0));
        assert_eq!(
            &DESCRIPTOR_SYMBOL[..DESCRIPTOR_SYMBOL.len() - 1],
            b"__hilavitkutin_extension_descriptor",
        );
    }

    #[test]
    fn extension_version_layout_has_reserved_field() {
        let v = ExtensionVersion { major: 1, minor: 2, patch: 3, _reserved: 0 };
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v._reserved, 0);
        // Four u16 fields => 8 bytes.
        assert_eq!(core::mem::size_of::<ExtensionVersion>(), 8);
    }

    #[test]
    fn capability_id_is_transparent_u64() {
        assert_eq!(
            core::mem::size_of::<CapabilityId>(),
            core::mem::size_of::<u64>(), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test asserts #[repr(transparent)] equivalence to u64; tracked: #206
        );
        assert_eq!(
            core::mem::align_of::<CapabilityId>(),
            core::mem::align_of::<u64>(), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test asserts #[repr(transparent)] equivalence to u64; tracked: #206
        );
    }

    #[test]
    fn extension_abi_status_is_u32_repr() {
        assert_eq!(ExtensionAbiStatus::Ok as u32, 0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test asserts #[repr(u32)] discriminant; tracked: #206
        assert_eq!(ExtensionAbiStatus::InitFailed as u32, 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #206
        assert_eq!(ExtensionAbiStatus::InvalidArg as u32, 2); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #206
        assert_eq!(ExtensionAbiStatus::NotSupported as u32, 3); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #206
        assert_eq!(ExtensionAbiStatus::Internal as u32, 4); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #206
        assert_eq!(core::mem::size_of::<ExtensionAbiStatus>(), 4);
    }
}
