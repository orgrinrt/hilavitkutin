//! Proc-macro companion to `hilavitkutin-api`.
//!
//! Ships `#[derive(ResourceFootprint)]`, the derive that sums a resource value
//! type's `Field`/`Seq`/`Map` field footprints into its L1-morsel write-collection
//! budget (canonical R5). The engine reads `ResourceFootprint::L1_BYTES` for a
//! written `Resource<T>` store when computing the per-fiber morsel window.
//!
//! The derive is a syntactic field-walk: for each struct field whose type path's
//! last segment is `Field`/`Seq`/`Map`, it sums
//! `<ty as ::hilavitkutin_api::footprint::CollectionBytes>::BYTES`. Plain fields
//! contribute nothing, so a resource of plain fields derives to a zero footprint
//! without rewriting its field types; a resource holding `Seq`/`Map` collections
//! gets the real `N * elem` budget. `Field` fields are `CollectionBytes`-zero
//! (register-cached scalars), so summing them is correct and explicit.
//!
//! The emitted output references `::hilavitkutin_api::*` and `::arvo::*` paths
//! only. Consumers add `hilavitkutin-api` (and `arvo`) as regular dependencies
//! alongside this macro crate.
//!
//! One of two proc-macro crates in the hilavitkutin stack (with
//! `hilavitkutin-extensions-macros`). Proc-macro crates run in the compiler host
//! context and therefore use `std`; the emitted output remains `no_std`-compatible.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Type};

/// Derive `ResourceFootprint` for a resource value type.
///
/// Sums the `CollectionBytes` footprint of each `Field`/`Seq`/`Map`-typed field;
/// other field types contribute nothing. Structs only (named or tuple); deriving
/// on an enum or union is a compile error.
#[proc_macro_derive(ResourceFootprint)]
pub fn derive_resource_footprint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data = match &input.data {
        Data::Struct(s) => s,
        _ => {
            return syn::Error::new_spanned(
                name,
                "ResourceFootprint can only be derived for structs (a resource value type is a struct of Field/Seq/Map fields)",
            )
            .to_compile_error()
            .into();
        }
    };

    // syn's `Fields::iter` yields `&Field` across named / unnamed / unit, so the
    // field walk needs no manual collection.
    let terms = data.fields.iter().filter_map(|f| {
        is_collection_field(&f.ty).then(|| {
            let ty = &f.ty;
            quote! { + <#ty as ::hilavitkutin_api::footprint::CollectionBytes>::BYTES.0 }
        })
    });

    let expanded = quote! {
        impl #impl_generics ::hilavitkutin_api::footprint::ResourceFootprint
            for #name #ty_generics #where_clause
        {
            const L1_BYTES: ::arvo::USize = ::arvo::USize(0 #(#terms)*);
        }
    };
    expanded.into()
}

/// Whether the field type is one of the R5 resource collection kinds
/// (`Field`/`Seq`/`Map`), matched by the type path's last segment.
fn is_collection_field(ty: &Type) -> bool {
    let Type::Path(tp) = ty else { return false };
    match tp.path.segments.last() {
        Some(seg) => {
            let id = seg.ident.to_string();
            id == "Field" || id == "Seq" || id == "Map"
        }
        None => false,
    }
}
