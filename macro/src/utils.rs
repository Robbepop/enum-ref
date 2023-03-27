/// Extension methods for [`struct@syn::Attribute`].
pub trait AttributeExt {
    /// Returns `true` if the [`struct@syn::Attribute`] is a Rust `#[repr(uN)]` attribute.
    fn is_repr_attribute(&self) -> bool;

    /// Returns `Some` if the [`struct@syn::Attribute`] is a Rust `#[repr(uN)]` attribute.
    fn filter_repr(&self) -> Option<&syn::Attribute>;
}

impl AttributeExt for syn::Attribute {
    fn is_repr_attribute(&self) -> bool {
        self.path().is_ident("repr")
    }

    fn filter_repr(&self) -> Option<&syn::Attribute> {
        if self.is_repr_attribute() {
            return Some(self);
        }
        None
    }
}
