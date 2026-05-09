//! Tests for the v1.1 descriptor validation steps: tag check,
//! descriptor_size check, abi_version check, length-bounds checks.
//!
//! `validate_descriptor` is part of the public surface; consumers
//! probing descriptors outside the standard `ExtensionHost::load`
//! path call it directly. These tests exercise it the same way:
//! construct a descriptor literal in test memory, hand it to the
//! helper, assert the surfaced outcome.

use core::ffi::c_void;

use hilavitkutin_extensions::{
    AbiVersion, ProviderEntry, ProviderId, EXTENSION_DESCRIPTOR_TAG,
    ExtensionAbiStatus, ExtensionDescriptor, ExtensionError, ExtensionVersion,
    HOST_ABI_VERSION, MAX_DESCRIPTOR_LIST_LEN, validate_descriptor,
};
use notko::Outcome;

fn good_descriptor() -> ExtensionDescriptor {
    ExtensionDescriptor {
        tag: EXTENSION_DESCRIPTOR_TAG,
        descriptor_size: core::mem::size_of::<ExtensionDescriptor>() as u32,
        abi_version: HOST_ABI_VERSION,
        name_ptr: core::ptr::null(),
        name_len: 0,
        version: ExtensionVersion {
            major: 1,
            minor: 0,
            patch: 0,
            _reserved: 0,
        },
        providers_ptr: core::ptr::null(),
        providers_len: 0,
        required_host_providers_ptr: core::ptr::null(),
        required_host_providers_len: 0,
        init_fn: None,
        shutdown_fn: None,
    }
}

#[test]
fn valid_descriptor_passes() {
    let descriptor = good_descriptor();
    match validate_descriptor(&descriptor) {
        Outcome::Ok(()) => {}
        Outcome::Err(e) => panic!("expected Ok, got error: {:?}", error_label(&e)),
    }
}

#[test]
fn tag_mismatch_surfaces_descriptor_tag_mismatch() {
    let mut descriptor = good_descriptor();
    descriptor.tag = 0xDEADBEEF;
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::DescriptorTagMismatch { found }) => {
            assert_eq!(found, 0xDEADBEEF);
        }
        other => panic!("expected DescriptorTagMismatch, got {:?}", error_label_outcome(&other)),
    }
}

#[test]
fn size_too_small_surfaces_descriptor_size_too_small() {
    let mut descriptor = good_descriptor();
    descriptor.descriptor_size = 16;
    let host_size = core::mem::size_of::<ExtensionDescriptor>() as u32;
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::DescriptorSizeTooSmall { advertised, minimum }) => {
            assert_eq!(advertised, 16);
            assert_eq!(minimum, host_size);
        }
        other => panic!("expected DescriptorSizeTooSmall, got {:?}", error_label_outcome(&other)),
    }
}

#[test]
fn abi_mismatch_surfaces_abi_version_mismatch() {
    let mut descriptor = good_descriptor();
    descriptor.abi_version = AbiVersion(99);
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::AbiVersionMismatch { expected, got }) => {
            assert_eq!(expected.0, HOST_ABI_VERSION.0);
            assert_eq!(got.0, 99);
        }
        other => panic!("expected AbiVersionMismatch, got {:?}", error_label_outcome(&other)),
    }
}

#[test]
fn bounds_exceeded_name_surfaces_descriptor_bounds_exceeded() {
    let mut descriptor = good_descriptor();
    descriptor.name_len = MAX_DESCRIPTOR_LIST_LEN + 1;
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::DescriptorBoundsExceeded { field, len }) => {
            assert_eq!(field, "name");
            assert_eq!(len, MAX_DESCRIPTOR_LIST_LEN + 1);
        }
        other => panic!("expected DescriptorBoundsExceeded(name), got {:?}", error_label_outcome(&other)),
    }
}

#[test]
fn bounds_exceeded_providers_surfaces_descriptor_bounds_exceeded() {
    let mut descriptor = good_descriptor();
    descriptor.providers_len = MAX_DESCRIPTOR_LIST_LEN + 1;
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::DescriptorBoundsExceeded { field, len }) => {
            assert_eq!(field, "providers");
            assert_eq!(len, MAX_DESCRIPTOR_LIST_LEN + 1);
        }
        other => panic!("expected DescriptorBoundsExceeded(providers), got {:?}", error_label_outcome(&other)),
    }
}

#[test]
fn bounds_exceeded_required_host_providers_surfaces_descriptor_bounds_exceeded() {
    let mut descriptor = good_descriptor();
    descriptor.required_host_providers_len = MAX_DESCRIPTOR_LIST_LEN + 1;
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::DescriptorBoundsExceeded { field, len }) => {
            assert_eq!(field, "required_host_providers");
            assert_eq!(len, MAX_DESCRIPTOR_LIST_LEN + 1);
        }
        other => panic!("expected DescriptorBoundsExceeded(required_host_providers), got {:?}", error_label_outcome(&other)),
    }
}

#[test]
fn validation_order_tag_first() {
    // A descriptor with both wrong tag and wrong size surfaces tag
    // mismatch first because the tag check runs before size check.
    let mut descriptor = good_descriptor();
    descriptor.tag = 0xDEADBEEF;
    descriptor.descriptor_size = 0;
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::DescriptorTagMismatch { .. }) => {}
        other => panic!(
            "expected DescriptorTagMismatch (tag check runs first), got {:?}",
            error_label_outcome(&other)
        ),
    }
}

#[test]
fn validation_order_size_before_version() {
    // A descriptor with valid tag, wrong size, wrong version surfaces
    // size mismatch before the version check fires.
    let mut descriptor = good_descriptor();
    descriptor.descriptor_size = 0;
    descriptor.abi_version = AbiVersion(99);
    match validate_descriptor(&descriptor) {
        Outcome::Err(ExtensionError::DescriptorSizeTooSmall { .. }) => {}
        other => panic!(
            "expected DescriptorSizeTooSmall (size check runs before version), got {:?}",
            error_label_outcome(&other)
        ),
    }
}

#[test]
fn forward_evolution_size_greater_than_host_passes() {
    // A descriptor advertising a larger size (forward-evolution case)
    // passes the size check; the host trusts the v1.1 prefix and
    // ignores trailing bytes.
    let mut descriptor = good_descriptor();
    let host_size = core::mem::size_of::<ExtensionDescriptor>() as u32;
    descriptor.descriptor_size = host_size + 64;
    match validate_descriptor(&descriptor) {
        Outcome::Ok(()) => {}
        Outcome::Err(e) => panic!("expected Ok for size > host_size, got {:?}", error_label(&e)),
    }
}

// Helper to render error variants as static labels for panic messages.
// Avoids requiring Debug on ExtensionError.
fn error_label(e: &ExtensionError) -> &'static str {
    match e {
        ExtensionError::LinkFailed { .. } => "LinkFailed",
        ExtensionError::DescriptorMissing => "DescriptorMissing",
        ExtensionError::DescriptorInvalid => "DescriptorInvalid",
        ExtensionError::DescriptorTagMismatch { .. } => "DescriptorTagMismatch",
        ExtensionError::DescriptorSizeTooSmall { .. } => "DescriptorSizeTooSmall",
        ExtensionError::AbiVersionMismatch { .. } => "AbiVersionMismatch",
        ExtensionError::DescriptorBoundsExceeded { .. } => "DescriptorBoundsExceeded",
        ExtensionError::ExtensionVersionUnsupported => "ExtensionVersionUnsupported",
        ExtensionError::RequiredHostProviderMissing { .. } => "RequiredHostProviderMissing",
        ExtensionError::InitFailed { .. } => "InitFailed",
        ExtensionError::ShutdownFailed { .. } => "ShutdownFailed",
        // ExtensionError is #[non_exhaustive]; wildcard satisfies the
        // closure should new variants land in a future round.
        _ => "<unknown>",
    }
}

fn error_label_outcome(o: &Outcome<(), ExtensionError>) -> &'static str {
    match o {
        Outcome::Ok(()) => "Ok(())",
        Outcome::Err(e) => error_label(e),
    }
}

// Suppress unused-import warning; these symbols ship and the test uses
// them indirectly via the validation helper, which closes over them
// internally. Keeping them in the import list documents the contract
// the test exercises.
#[allow(dead_code)]
fn _signature_witness() -> (ProviderEntry, ProviderId, ExtensionAbiStatus, *mut c_void) {
    (
        ProviderEntry {
            id: ProviderId(0),
            vtable_ptr: core::ptr::null(),
        },
        ProviderId(0),
        ExtensionAbiStatus::Ok,
        core::ptr::null_mut(),
    )
}
