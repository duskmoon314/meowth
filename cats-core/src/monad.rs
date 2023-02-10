//! Monad

use crate::*;

/// `Monad` is an [`Applicative`] with [`flat_map`](Monad::flat_map).
pub trait Monad: Applicative {
    /// `flat_map` maps a function over the value
    ///
    /// # Examples
    ///
    /// ```
    /// use cats_core::Monad;
    ///
    /// let x = Some(1);
    /// let y = x.flat_map(|x| Some(x + 1));
    /// assert_eq!(y, Some(2));
    /// ```
    fn flat_map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> Self::Wrapped<B>;

    /// Flatten a nested structure `F<F<A>>` into a flat structure `F<A>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cats_core::Monad;
    ///
    /// let x = Some(Some(1));
    /// let y = x.flat_map(|x| x);
    /// let z = <Option<Option<_>> as Monad>::flatten(x);
    /// assert_eq!(y, Some(1));
    /// assert_eq!(z, Some(1));
    /// ```
    fn flatten<A>(self) -> Self::Wrapped<A>
    where
        Self::Unwrapped: Id<Self::Wrapped<A>>,
    {
        self.flat_map(|x| x.id())
    }
}

impl<T> Monad for Option<T> {
    fn flat_map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        match self {
            Some(a) => f(a),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monad() {
        let x = Some(Some(1));
        let y = x.flat_map(|x| x);
        let z = <Option<Option<_>> as Monad>::flatten(x);
        assert_eq!(y, Some(1));
        assert_eq!(z, Some(1));
    }
}
