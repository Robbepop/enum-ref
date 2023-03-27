#![no_std]

pub use enum_ref_macro::{EnumMut, EnumRef};

/// Trait implemented by `enum` types for shared access.
///
/// This trait usually is implemented via `#[derive(EnumRef)]`.
pub trait EnumRef {
    /// A wrapper type around `Self` for shared access.
    type Ref<'a>
    where
        Self: 'a;

    /// Returns a shared reference wrapper to `self`.
    fn as_ref(&self) -> Self::Ref<'_>;
}

/// Trait implemented by `enum` types for exclusive access.
///
/// This trait usually is implemented via `#[derive(EnumMut)]`.
pub trait EnumMut {
    /// A wrapper type around `Self` for shared access.
    type Mut<'a>
    where
        Self: 'a;

    /// Returns an exlusive reference wrapper to `self`.
    fn as_mut(&mut self) -> Self::Mut<'_>;
}
