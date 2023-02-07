use super::*;

/// # Monad
///
/// `Monad` is a [`Applicative`] that also implements [`FlatMap`].
pub trait Monad: FlatMap + Applicative {
    // TODO: Add Monad's own methods
}

impl<T: FlatMap + Applicative> Monad for T {}
