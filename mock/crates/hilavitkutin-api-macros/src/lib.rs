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
//! From the same fields the derive writes `ResourceRead`, the form a value
//! function reads the resource in: by reference where a field is a `Seq` or a
//! `Map`, and by value otherwise, which asks the value type to be `Copy`.
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
use syn::{Data, DeriveInput, Type, parse_macro_input};

/// Derive `ResourceFootprint` for a resource value type.
///
/// Sums the `CollectionBytes` footprint of each `Field`/`Seq`/`Map`-typed field;
/// other field types contribute nothing. Also writes `ResourceRead`: `&Self`
/// where a field is a `Seq` or a `Map`, and `Self` otherwise, for which the
/// value type is `Copy`. Structs only (named or tuple); deriving on an enum or
/// union is a compile error.
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
        },
    };

    // syn's `Fields::iter` yields `&Field` across named / unnamed / unit, so the
    // field walk needs no manual collection.
    let terms = data
        .fields
        .iter()
        .filter(|&f| is_collection_field(&f.ty))
        .map(|f| {
            let ty = &f.ty;
            quote! { + <#ty as ::hilavitkutin_api::footprint::CollectionBytes>::BYTES.0 }
        });

    // The form a value function reads the resource in, from the same fields:
    // by reference where a field is a `Seq` or a `Map`, by value otherwise.
    let read = if data.fields.iter().any(|f| holds_collection(&f.ty)) {
        quote! {
            impl #impl_generics ::hilavitkutin_api::values::ResourceRead
                for #name #ty_generics #where_clause
            {
                type Val<'a> = &'a Self where Self: 'a;

                #[inline(always)]
                fn read<'a>(&'a self) -> &'a Self {
                    self
                }
            }
        }
    } else {
        // Read by value, so the value type is `Copy`. The bound sits on the
        // impl, and a value type that is not `Copy` is refused there by name.
        let mut by_value = input.generics.clone();
        by_value
            .make_where_clause()
            .predicates
            .push(syn::parse_quote!(Self: ::core::marker::Copy + 'static));
        let (_, _, by_value_where) = by_value.split_for_impl();
        quote! {
            impl #impl_generics ::hilavitkutin_api::values::ResourceRead
                for #name #ty_generics #by_value_where
            {
                type Val<'a> = Self where Self: 'a;

                #[inline(always)]
                fn read<'a>(&'a self) -> Self {
                    *self
                }
            }
        }
    };

    let expanded = quote! {
        impl #impl_generics ::hilavitkutin_api::footprint::ResourceFootprint
            for #name #ty_generics #where_clause
        {
            const L1_BYTES: ::arvo::USize = ::arvo::USize(0 #(#terms)*);
        }

        #read
    };
    expanded.into()
}

/// The last segment of the field type's path, as written.
fn last_segment(ty: &Type) -> Option<String> {
    let Type::Path(tp) = ty else { return None };
    tp.path.segments.last().map(|seg| seg.ident.to_string())
}

/// Whether the field type is one of the R5 resource collection kinds
/// (`Field`/`Seq`/`Map`), matched by the type path's last segment.
fn is_collection_field(ty: &Type) -> bool {
    matches!(last_segment(ty).as_deref(), Some("Field" | "Seq" | "Map"))
}

/// Whether the field type is a `Seq` or a `Map`, the two kinds that make a
/// resource one a value function reads by reference.
fn holds_collection(ty: &Type) -> bool {
    matches!(last_segment(ty).as_deref(), Some("Seq" | "Map"))
}
