//! Proc-macro companion to `hilavitkutin-extensions`.
//!
//! Ships `#[export_extension]`, the attribute macro that emits the
//! `#[repr(C)] ExtensionDescriptor` static, the
//! `__hilavitkutin_extension_descriptor` exported fn, the capability
//! table, and optional init / shutdown trampolines on behalf of an
//! extension author.
//!
//! The emitted output references `::hilavitkutin_extensions::*` paths
//! only. Consumers add `hilavitkutin-extensions` as a regular dep
//! alongside this macro crate.
//!
//! This is the sole proc-macro crate in the hilavitkutin stack. Proc-
//! macro crates run in the compiler host context and therefore use
//! `std`; the emitted output remains `no_std`-compatible.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    Expr, ExprArray, ExprLit, Ident, ItemStruct, Lit, LitStr, Path, Token,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
};

/// `#[export_extension]` attribute macro.
///
/// Attach to an extension's top-level struct. Emits the descriptor,
/// exported-fn, and capability table required by the host's pull-based
/// discovery contract.
///
/// # Attribute parameters
///
/// - `name = "..."`: ASCII name. Defaults to the struct ident in
///   kebab-case.
/// - `version = "MAJOR.MINOR.PATCH"`: explicit version triple.
///   Defaults to `env!("CARGO_PKG_VERSION")` parsed at emission time.
/// - `required_host_caps = [PATH, PATH, ...]`: const `CapabilityId`
///   expressions the extension requires from the host. Defaults to
///   empty.
/// - `capabilities = [TypePath, TypePath, ...]`: type paths that each
///   implement `CapabilityExport`. Emission reads
///   `<T as CapabilityExport>::ID` and `::VTABLE_PTR` into the
///   capability table. Defaults to empty.
/// - `init = TypePath`: type implementing `InitHandler`. When present,
///   the descriptor's `init_fn` slot is populated with a trampoline.
/// - `shutdown = TypePath`: type implementing `ShutdownHandler`. Same
///   conditional emission rule as `init`.
///
/// See `DESIGN.md` for the full emission shape.
#[proc_macro_attribute]
pub fn export_extension(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let attrs = match ExtAttrs::parse_from(attr.into()) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };

    let struct_ident = &input.ident;

    // Resolve attribute defaults.
    let name_bytes_lit = match attrs.name {
        Some(name) => name.value().into_bytes(),
        None => kebab_case(&struct_ident.to_string()).into_bytes(),
    };
    let name_lit = LitStr::new(
        // quote emits a byte-string via the byte-vec path below; keep
        // the Rust-level ident available for diagnostics.
        &String::from_utf8_lossy(&name_bytes_lit),
        Span::call_site(),
    );
    let _ = name_lit; // kept for potential future diagnostic use

    let name_literal_bytes = {
        // Build `b"..."` token.
        let s = core::str::from_utf8(&name_bytes_lit).unwrap_or("");
        let lit = syn::LitByteStr::new(s.as_bytes(), Span::call_site());
        quote! { #lit }
    };

    let version_expr = match attrs.version {
        Some(v) => match parse_semver(&v.value()) {
            Ok((maj, min, pat)) => quote! {
                ::hilavitkutin_extensions::ExtensionVersion {
                    major: #maj,
                    minor: #min,
                    patch: #pat,
                    _reserved: 0,
                }
            },
            Err(msg) => {
                return syn::Error::new(v.span(), msg)
                    .to_compile_error()
                    .into();
            }
        },
        None => quote! {
            {
                const fn __ext_parse_env_semver() -> ::hilavitkutin_extensions::ExtensionVersion {
                    // `env!("CARGO_PKG_VERSION")` is a compile-time
                    // &'static str. Walk its bytes to produce the
                    // four-field version record.
                    let bytes = env!("CARGO_PKG_VERSION").as_bytes();
                    let mut i = 0usize;
                    let mut major: u16 = 0;
                    while i < bytes.len() && bytes[i] != b'.' {
                        major = major * 10 + (bytes[i] - b'0') as u16;
                        i += 1;
                    }
                    i += 1;
                    let mut minor: u16 = 0;
                    while i < bytes.len() && bytes[i] != b'.' {
                        minor = minor * 10 + (bytes[i] - b'0') as u16;
                        i += 1;
                    }
                    i += 1;
                    let mut patch: u16 = 0;
                    while i < bytes.len() && bytes[i] != b'-' && bytes[i] != b'+' {
                        patch = patch * 10 + (bytes[i] - b'0') as u16;
                        i += 1;
                    }
                    ::hilavitkutin_extensions::ExtensionVersion {
                        major, minor, patch, _reserved: 0,
                    }
                }
                __ext_parse_env_semver()
            }
        },
    };

    // required_host_caps const slice.
    let required_caps_init = if attrs.required_host_caps.is_empty() {
        quote! { const __EXT_REQUIRED_CAPS: &[::hilavitkutin_extensions::CapabilityId] = &[]; }
    } else {
        let caps = &attrs.required_host_caps;
        quote! {
            const __EXT_REQUIRED_CAPS: &[::hilavitkutin_extensions::CapabilityId] = &[
                #( #caps ),*
            ];
        }
    };

    // capabilities const slice.
    let capabilities_init = if attrs.capabilities.is_empty() {
        quote! { const __EXT_CAPABILITIES: &[::hilavitkutin_extensions::CapabilityEntry] = &[]; }
    } else {
        let caps = attrs.capabilities.iter().map(|ty| {
            quote! {
                ::hilavitkutin_extensions::CapabilityEntry {
                    id: <#ty as ::hilavitkutin_extensions::CapabilityExport>::ID,
                    vtable_ptr: <#ty as ::hilavitkutin_extensions::CapabilityExport>::VTABLE_PTR,
                }
            }
        });
        quote! {
            const __EXT_CAPABILITIES: &[::hilavitkutin_extensions::CapabilityEntry] = &[
                #( #caps ),*
            ];
        }
    };

    // init / shutdown trampolines.
    let (init_trampoline, init_slot) = match attrs.init {
        Some(path) => {
            let fn_ident =
                format_ident!("__ext_init_trampoline_{}", struct_ident);
            (
                quote! {
                    unsafe extern "C" fn #fn_ident(
                        host_ctx: *mut ::core::ffi::c_void,
                    ) -> ::hilavitkutin_extensions::ExtensionAbiStatus {
                        // SAFETY: caller is the host, which passes the
                        // pointer it allocated for this specific load.
                        unsafe {
                            <#path as ::hilavitkutin_extensions::InitHandler>::init(host_ctx)
                        }
                    }
                },
                quote! { ::hilavitkutin_extensions::MaybeNull::new(#fn_ident) },
            )
        }
        None => (quote! {}, quote! { ::hilavitkutin_extensions::MaybeNull::isnt() }),
    };

    let (shutdown_trampoline, shutdown_slot) = match attrs.shutdown {
        Some(path) => {
            let fn_ident =
                format_ident!("__ext_shutdown_trampoline_{}", struct_ident);
            (
                quote! {
                    unsafe extern "C" fn #fn_ident(
                        host_ctx: *mut ::core::ffi::c_void,
                    ) -> ::hilavitkutin_extensions::ExtensionAbiStatus {
                        // SAFETY: caller is the host, which passes the
                        // pointer it threaded through at load time.
                        unsafe {
                            <#path as ::hilavitkutin_extensions::ShutdownHandler>::shutdown(host_ctx)
                        }
                    }
                },
                quote! { ::hilavitkutin_extensions::MaybeNull::new(#fn_ident) },
            )
        }
        None => (quote! {}, quote! { ::hilavitkutin_extensions::MaybeNull::isnt() }),
    };

    let expanded = quote! {
        #input

        const __EXT_NAME: &[u8] = #name_literal_bytes;

        const __EXT_VERSION: ::hilavitkutin_extensions::ExtensionVersion = #version_expr;

        #required_caps_init

        #capabilities_init

        #init_trampoline
        #shutdown_trampoline

        #[used]
        static __HILAVITKUTIN_EXT_DESCRIPTOR:
            ::hilavitkutin_extensions::ExtensionDescriptor =
        ::hilavitkutin_extensions::ExtensionDescriptor {
            abi_version: ::hilavitkutin_extensions::HOST_ABI_VERSION,
            name_ptr: __EXT_NAME.as_ptr(),
            name_len: __EXT_NAME.len(),
            version: __EXT_VERSION,
            capabilities_ptr: __EXT_CAPABILITIES.as_ptr(),
            capabilities_len: __EXT_CAPABILITIES.len(),
            required_host_caps_ptr: __EXT_REQUIRED_CAPS.as_ptr(),
            required_host_caps_len: __EXT_REQUIRED_CAPS.len(),
            init_fn: #init_slot,
            shutdown_fn: #shutdown_slot,
        };

        #[unsafe(no_mangle)]
        pub extern "C" fn __hilavitkutin_extension_descriptor()
            -> *const ::hilavitkutin_extensions::ExtensionDescriptor
        {
            &__HILAVITKUTIN_EXT_DESCRIPTOR
        }
    };

    expanded.into()
}

/// Attribute parameters to the `#[export_extension]` macro.
struct ExtAttrs {
    name: Option<LitStr>,
    version: Option<LitStr>,
    required_host_caps: Vec<Expr>, // lint:allow(no-alloc) reason: proc-macro host-context std; tracked: #205
    capabilities: Vec<Path>, // lint:allow(no-alloc) reason: proc-macro host-context std; tracked: #205
    init: Option<Path>,
    shutdown: Option<Path>,
}

impl ExtAttrs {
    fn parse_from(tokens: proc_macro2::TokenStream) -> syn::Result<Self> {
        let mut out = Self {
            name: None,
            version: None,
            required_host_caps: Vec::new(),
            capabilities: Vec::new(),
            init: None,
            shutdown: None,
        };

        if tokens.is_empty() {
            return Ok(out);
        }

        let entries: Punctuated<AttrEntry, Token![,]> =
            Punctuated::<AttrEntry, Token![,]>::parse_terminated
                .parse2(tokens)?;

        for entry in entries {
            let key_str = entry.key.to_string();
            match key_str.as_str() {
                "name" => {
                    out.name = Some(entry.expect_lit_str()?);
                }
                "version" => {
                    out.version = Some(entry.expect_lit_str()?);
                }
                "required_host_caps" => {
                    out.required_host_caps = entry.expect_expr_array()?;
                }
                "capabilities" => {
                    out.capabilities = entry.expect_path_array()?;
                }
                "init" => {
                    out.init = Some(entry.expect_path()?);
                }
                "shutdown" => {
                    out.shutdown = Some(entry.expect_path()?);
                }
                _ => {
                    return Err(syn::Error::new(
                        entry.key.span(),
                        format!(
                            "unknown #[export_extension] parameter `{}`. Supported keys: name, version, required_host_caps, capabilities, init, shutdown.",
                            key_str
                        ),
                    ));
                }
            }
        }
        Ok(out)
    }
}

struct AttrEntry {
    key: Ident,
    _eq: Token![=],
    value: Expr,
}

impl Parse for AttrEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let value: Expr = input.parse()?;
        Ok(Self { key, _eq, value })
    }
}

impl AttrEntry {
    fn expect_lit_str(self) -> syn::Result<LitStr> {
        match self.value {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s),
            other => Err(syn::Error::new_spanned(
                other,
                format!(
                    "expected string literal for `{}`",
                    self.key
                ),
            )),
        }
    }

    fn expect_path(self) -> syn::Result<Path> {
        match self.value {
            Expr::Path(p) => Ok(p.path),
            other => Err(syn::Error::new_spanned(
                other,
                format!("expected a type path for `{}`", self.key),
            )),
        }
    }

    fn expect_expr_array(self) -> syn::Result<Vec<Expr>> { // lint:allow(no-alloc) reason: proc-macro host-context std; tracked: #205
        match self.value {
            Expr::Array(ExprArray { elems, .. }) => {
                Ok(elems.into_iter().collect())
            }
            other => Err(syn::Error::new_spanned(
                other,
                format!("expected `[...]` array for `{}`", self.key),
            )),
        }
    }

    fn expect_path_array(self) -> syn::Result<Vec<Path>> { // lint:allow(no-alloc) reason: proc-macro host-context std; tracked: #205
        match self.value {
            Expr::Array(ExprArray { elems, .. }) => {
                let mut out = Vec::with_capacity(elems.len());
                for e in elems {
                    match e {
                        Expr::Path(p) => out.push(p.path),
                        other => {
                            return Err(syn::Error::new_spanned(
                                other,
                                "expected a type path inside `capabilities = [...]`",
                            ));
                        }
                    }
                }
                Ok(out)
            }
            other => Err(syn::Error::new_spanned(
                other,
                format!("expected `[...]` array for `{}`", self.key),
            )),
        }
    }
}

fn kebab_case(ident: &str) -> String { // lint:allow(no-alloc) reason: proc-macro host-context std; tracked: #205
    let mut out = String::with_capacity(ident.len());
    for (i, c) in ident.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i != 0 {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn parse_semver(s: &str) -> Result<(u16, u16, u16), &'static str> {
    let parts: Vec<&str> = s.split('.').collect(); // lint:allow(no-alloc) reason: proc-macro host-context std; tracked: #205
    if parts.len() != 3 {
        return Err(
            "version must be MAJOR.MINOR.PATCH (three dot-separated integers)",
        );
    }
    let major = parts[0]
        .parse::<u16>()
        .map_err(|_| "major is not a u16 integer")?;
    let minor = parts[1]
        .parse::<u16>()
        .map_err(|_| "minor is not a u16 integer")?;
    // patch may carry a pre-release or build suffix; drop it.
    let patch_raw = parts[2];
    let patch_num_end = patch_raw
        .find(|c: char| c == '-' || c == '+')
        .unwrap_or(patch_raw.len());
    let patch = patch_raw[..patch_num_end]
        .parse::<u16>()
        .map_err(|_| "patch is not a u16 integer")?;
    Ok((major, minor, patch))
}
