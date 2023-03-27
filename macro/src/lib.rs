use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[macro_use]
mod error;
mod derive;
mod utils;

/// Proc. macro to derive [`EnumRef`] trait for the Rust `enum`.
/// 
/// This generates a new `enum` that mirrors all variants of the
/// original `enum` type but wraps all variant field in a shared
/// reference.
/// Furthermore it implement the [`EnumRef`] trait for the original
/// `enum` in order to make the generated `enum` accessible.
/// 
/// # Example
/// 
/// ```
/// use enum_ref::EnumRef;
/// 
/// #[derive(EnumRef)]
/// #[repr(u8)] // Rust requires this for `B = 42`
/// enum Test {
///     A,
///     B = 42,
///     C(i32),
///     D(i32, i64),
///     E { a: i32 },
///     F { a: i32, b: i64 },
/// }
/// 
/// // Access and name the generated `enum` as follows:
/// type TestRef<'a> = <Test as EnumRef>::Ref<'a>;
/// ```
/// 
/// The `#[derive(EnumRef)]` in the above example will generate roughly the following Rust code.
/// 
/// ```
/// # #[repr(u8)] // Rust requires this for `B = 42`
/// # enum Test {
/// #     A,
/// #     B = 42,
/// #     C(i32),
/// #     D(i32, i64),
/// #     E { a: i32 },
/// #     F { a: i32, b: i64 },
/// # }
/// #
/// const _: () = {
///     #[derive(::core::fmt::Debug)]
///     #[repr(u8)]
///     pub enum TestRef<'__enum_ref_lt> {
///         A,
///         B = 42,
///         C(&'__enum_ref_lt i32),
///         D(&'__enum_ref_lt i32, &'__enum_ref_lt i64),
///         E {
///             a: &'__enum_ref_lt i32,
///         },
///         F {
///             a: &'__enum_ref_lt i32,
///             b: &'__enum_ref_lt i64,
///         },
///     }
/// 
///     impl ::enum_ref::EnumRef for Test {
///         type Ref<'__enum_ref_lt> where Self: '__enum_ref_lt =
///                 TestRef<'__enum_ref_lt> where Self: '__enum_ref_lt;
///         fn as_ref(&self) -> <Self as ::enum_ref::EnumRef>::Ref<'_> {
///             type __enum_ref_EnumRef_Ref<'__enum_ref_lt> =
///                 <Test as ::enum_ref::EnumRef>::Ref<'__enum_ref_lt>;
///             match self {
///                 Self::A => __enum_ref_EnumRef_Ref::A,
///                 Self::B => __enum_ref_EnumRef_Ref::B,
///                 Self::C(_0) => __enum_ref_EnumRef_Ref::C(_0),
///                 Self::D(_0, _1) => __enum_ref_EnumRef_Ref::D(_0, _1),
///                 Self::E { a } => __enum_ref_EnumRef_Ref::E { a },
///                 Self::F { a, b } => __enum_ref_EnumRef_Ref::F { a, b },
///             }
///         }
///     }
/// };
/// ```
#[proc_macro_derive(EnumRef)]
pub fn enum_ref(input: TokenStream) -> TokenStream {
    derive::enum_ref(parse_macro_input!(input as DeriveInput)).into()
}

/// Proc. macro to derive [`EnumMut`] trait for the Rust `enum`.
/// 
/// This generates a new `enum` that mirrors all variants of the
/// original `enum` type but wraps all variant field in an exclusive
/// reference.
/// Furthermore it implement the [`EnumMut`] trait for the original
/// `enum` in order to make the generated `enum` accessible.
/// 
/// # Example
/// 
/// ```
/// use enum_ref::EnumMut;
/// 
/// #[derive(EnumMut)]
/// #[repr(u8)] // Rust requires this for `B = 42`
/// enum Test {
///     A,
///     B = 42,
///     C(i32),
///     D(i32, i64),
///     E { a: i32 },
///     F { a: i32, b: i64 },
/// }
/// 
/// // Access and name the generated `enum` as follows:
/// type TestMut<'a> = <Test as EnumMut>::Mut<'a>;
/// ```
/// 
/// The `#[derive(EnumMut)]` in the above example will generate roughly the following Rust code.
/// 
/// ```
/// # #[repr(u8)] // Rust requires this for `B = 42`
/// # enum Test {
/// #     A,
/// #     B = 42,
/// #     C(i32),
/// #     D(i32, i64),
/// #     E { a: i32 },
/// #     F { a: i32, b: i64 },
/// # }
/// #
/// const _: () = {
///     #[derive(::core::fmt::Debug)]
///     #[repr(u8)]
///     pub enum TestMut<'__enum_ref_lt> {
///         A,
///         B = 42,
///         C(&'__enum_ref_lt mut i32),
///         D(&'__enum_ref_lt mut i32, &'__enum_ref_lt mut i64),
///         E {
///             a: &'__enum_ref_lt mut i32,
///         },
///         F {
///             a: &'__enum_ref_lt mut i32,
///             b: &'__enum_ref_lt mut i64,
///         },
///     }
/// 
///     impl ::enum_ref::EnumMut for Test {
///         type Mut<'__enum_ref_lt> where Self: '__enum_ref_lt =
///                 TestMut<'__enum_ref_lt> where Self: '__enum_ref_lt;
///         fn as_mut(&mut self) -> <Self as ::enum_ref::EnumMut>::Mut<'_> {
///             type __enum_ref_EnumMut_Mut<'__enum_ref_lt> =
///                 <Test as ::enum_ref::EnumMut>::Mut<'__enum_ref_lt>;
///             match self {
///                 Self::A => __enum_ref_EnumMut_Mut::A,
///                 Self::B => __enum_ref_EnumMut_Mut::B,
///                 Self::C(_0) => __enum_ref_EnumMut_Mut::C(_0),
///                 Self::D(_0, _1) => __enum_ref_EnumMut_Mut::D(_0, _1),
///                 Self::E { a } => __enum_ref_EnumMut_Mut::E { a },
///                 Self::F { a, b } => __enum_ref_EnumMut_Mut::F { a, b },
///             }
///         }
///     }
/// };
/// ```
#[proc_macro_derive(EnumMut)]
pub fn enum_mut(input: TokenStream) -> TokenStream {
    derive::enum_mut(parse_macro_input!(input as DeriveInput)).into()
}
