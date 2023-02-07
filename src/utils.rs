//! Utilities for the crate.

/// # EqT
///
/// This trait is used to assert that two types are equal.
///
/// No other implementations are allowed.
///
/// REF: https://docs.rs/functional/0.0.5/functional/trait.Equals.html
#[doc(hidden)]
pub trait EqT<T> {
    fn cast(self) -> T;
}

impl<T> EqT<T> for T {
    #[inline(always)]
    fn cast(self) -> T {
        self
    }
}
