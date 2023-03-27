use crate::utils::AttributeExt;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;
use quote::{format_ident, quote};
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::DeriveInput;
use syn::Result;

/// Wrapper around [`enum_ref_impl`] for error conversions.
pub fn enum_ref(input: DeriveInput) -> TokenStream2 {
    match enum_ref_impl(input) {
        Ok(result) => result,
        Err(error) => error.to_compile_error(),
    }
}

/// Implements the `#[derive(EnumRef)]` functionality for the given `input`.
fn enum_ref_impl(input: DeriveInput) -> Result<TokenStream2> {
    let data = extract_enum(&input)?;
    let ident = &input.ident;
    let repr = repr_attr(&input);
    let ref_ident = format_ident!("{}Ref", ident);
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let (ref_generics, ref_lifetime) = make_ref_generics(&input.generics);
    let (impl_ref_generics, type_ref_generics, _) = ref_generics.split_for_impl();
    let variants = data.variants.iter().map(make_ref);
    let arms = data.variants.iter().map(make_arm);
    Ok(quote! {
        const _: () = {
            #[derive(::core::fmt::Debug)]
            #repr
            pub enum #ref_ident #impl_ref_generics {
                #( #variants ),*
            }

            impl #impl_generics ::enum_ref::EnumRef for #ident #type_generics #where_clause {
                type Ref<#ref_lifetime> where Self: #ref_lifetime = #ref_ident #type_ref_generics
                where
                    Self: #ref_lifetime;

                fn as_ref(&self) -> <Self as ::enum_ref::EnumRef>::Ref<'_> {
                    // This type alias is a workaround for a Rust compiler bug.
                    //
                    // # Note
                    //
                    // This is required for a workaround for this issue:
                    // https://github.com/rust-lang/rust/issues/86935#issuecomment-1484160404
                    //
                    // The problem is that we cannot use associated type paths to
                    // disambiguate enum variants with named fields.
                    type __enum_ref_EnumRef_Ref #impl_ref_generics =
                        <#ident #type_generics as ::enum_ref::EnumRef>::Ref<#ref_lifetime>;
                    match self {
                        #(
                            Self::#arms => __enum_ref_EnumRef_Ref::#arms,
                        )*
                    }
                }
            }
        };
    })
}

/// Wrapper around [`enum_mut_impl`] for error conversions.
pub fn enum_mut(input: DeriveInput) -> TokenStream2 {
    match enum_mut_impl(input) {
        Ok(result) => result,
        Err(error) => error.to_compile_error(),
    }
}

/// Implements the `#[derive(EnumMut)]` functionality for the given `input`.
fn enum_mut_impl(input: DeriveInput) -> Result<TokenStream2> {
    let data = extract_enum(&input)?;
    let ident = &input.ident;
    let repr = repr_attr(&input);
    let mut_ident = format_ident!("{}Mut", ident);
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let (ref_generics, ref_lifetime) = make_ref_generics(&input.generics);
    let (impl_ref_generics, type_ref_generics, _) = ref_generics.split_for_impl();
    let variants = data.variants.iter().map(make_mut);
    let arms = data.variants.iter().map(make_arm);
    Ok(quote! {
        const _: () = {
            #[derive(::core::fmt::Debug)]
            #repr
            pub enum #mut_ident #impl_ref_generics {
                #( #variants ),*
            }

            impl #impl_generics ::enum_ref::EnumMut for #ident #type_generics #where_clause {
                type Mut<#ref_lifetime> where Self: #ref_lifetime = #mut_ident #type_ref_generics
                where
                    Self: #ref_lifetime;

                fn as_mut(&mut self) -> <Self as ::enum_ref::EnumMut>::Mut<'_> {
                    // This type alias is a workaround for a Rust compiler bug.
                    //
                    // # Note
                    //
                    // This is required for a workaround for this issue:
                    // https://github.com/rust-lang/rust/issues/86935#issuecomment-1484160404
                    //
                    // The problem is that we cannot use associated type paths to
                    // disambiguate enum variants with named fields.
                    type __enum_ref_EnumMut_Mut #impl_ref_generics =
                        <#ident #type_generics as ::enum_ref::EnumMut>::Mut<#ref_lifetime>;
                    match self {
                        #(
                            Self::#arms => __enum_ref_EnumMut_Mut::#arms,
                        )*
                    }
                }
            }
        };
    })
}

/// Sanitizes the input to the `EnumRef` and `EnumMut` derive macros.
fn extract_enum(input: &DeriveInput) -> Result<&syn::DataEnum> {
    let data = match &input.data {
        syn::Data::Enum(data) => data,
        syn::Data::Struct(_) => bail_spanned!(
            input,
            "derive(EnumRef) only works on `enum` types but found struct"
        ),
        syn::Data::Union(_) => bail_spanned!(
            input,
            "derive(EnumRef) only works on `enum` types but found union"
        ),
    };
    Ok(data)
}

fn make_ref(variant: &syn::Variant) -> syn::Variant {
    make_ref_variant(variant, Mutability::Ref)
}

fn make_mut(variant: &syn::Variant) -> syn::Variant {
    make_ref_variant(variant, Mutability::Mut)
}

#[derive(Debug, Clone, Copy)]
enum Mutability {
    Ref,
    Mut,
}

/// Adds a special `'__enum_ref_lt` lifetime parameter to the start of the given `generics`.
///
/// Returns the adjusted generics as well as the added lifetime.
fn make_ref_generics(generics: &syn::Generics) -> (syn::Generics, syn::Lifetime) {
    let mut generics = generics.clone();
    let lifetime = make_ref_lifetime();
    generics.params.insert(0, parse_quote!(#lifetime));
    (generics, lifetime)
}

fn make_ref_lifetime() -> syn::Lifetime {
    parse_quote!('__enum_ref_lt)
}

fn make_ref_variant(variant: &syn::Variant, mutable: Mutability) -> syn::Variant {
    let lt = make_ref_lifetime();
    let mutability = matches!(mutable, Mutability::Mut).then_some(quote!(mut));
    let mut fields = variant.fields.clone();
    for field in &mut fields {
        let ty = &field.ty;
        let ref_ty: syn::TypeReference = parse_quote!(&#lt #mutability #ty);
        field.ty = syn::Type::Reference(ref_ty);
    }
    syn::Variant {
        fields,
        ..variant.clone()
    }
}

fn repr_attr(input: &DeriveInput) -> Option<syn::Attribute> {
    input
        .attrs
        .iter()
        .cloned()
        .find(AttributeExt::is_repr_attribute)
}

fn make_arm(variant: &syn::Variant) -> TokenStream2 {
    let span = variant.span();
    let ident = &variant.ident;
    match &variant.fields {
        syn::Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| {
                f.ident
                    .as_ref()
                    .expect("named fields must have identifiers")
            });
            quote_spanned!(span=> #ident { #(#names),* })
        }
        syn::Fields::Unnamed(fields) => {
            let underscores = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(n, _field)| format_ident!("_{n}"));
            quote_spanned!(span=> #ident (#(#underscores),*))
        }
        syn::Fields::Unit => {
            quote_spanned!(span=> #ident)
        }
    }
}
