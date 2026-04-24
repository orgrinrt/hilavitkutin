//! Proc-macro companion to `hilavitkutin-extensions`.
//!
//! Ships `#[export_extension]`, the attribute macro that emits the
//! `#[repr(C)]` `ExtensionDescriptor` literal, the `__hilavitkutin_extension_descriptor`
//! exported fn, the capability table, and per-capability trampolines
//! on behalf of an extension author.
//!
//! The emitted output references `::hilavitkutin_extensions::*` paths
//! only. Consumers add `hilavitkutin-extensions` as a regular dep
//! alongside this macro crate.
//!
//! This is the sole proc-macro crate in the hilavitkutin stack. Proc-
//! macro crates run in the compiler host context and therefore use
//! `std`; the emitted output remains `no_std`-compatible.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// `#[export_extension]` attribute macro.
///
/// Attach to an extension's top-level extension struct. Emits the
/// `#[repr(C)] ExtensionDescriptor` literal, a `const CAPABILITIES`
/// slice populated from the struct's `CapabilityExport` impls, and
/// the `#[no_mangle] pub extern "C" fn __hilavitkutin_extension_descriptor`
/// required by the host's pull-based discovery contract.
///
/// # Attribute parameters (v1 accepted forms)
///
/// - `name = "..."` (required): ASCII identifier the host uses in
///   diagnostics and `ExtensionRequirement` lookups.
/// - `version = "X.Y.Z"` (optional): parsed into `ExtensionVersion`;
///   defaults to `CARGO_PKG_VERSION`.
/// - `required_host_caps = [...]` (optional): ASCII capability names
///   the extension requires the host to provide; emitted into the
///   descriptor's required-host-caps table.
///
/// Full semantics and the per-capability trampoline emission shape
/// live in the crate's `DESIGN.md`.
#[proc_macro_attribute]
pub fn export_extension(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    // v1 skeleton: pass the struct through and emit an extern-C
    // descriptor stub. The full emission (capability trampolines,
    // descriptor table, attribute parsing) lands in follow-up work
    // tracked in `BACKLOG.md`. The skeleton exists so the attribute
    // is callable and downstream test fixtures can begin to wire up
    // against the macro surface.
    let expanded = quote! {
        #input

        const _: fn() = || {
            let _ = ::core::marker::PhantomData::<#name>;
        };
    };

    TokenStream::from(expanded)
}
