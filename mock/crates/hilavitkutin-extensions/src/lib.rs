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
