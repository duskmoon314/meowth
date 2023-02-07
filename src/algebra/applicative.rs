use super::*;

/// # Apply
///
/// Weaker version of [`Applicative`]: has [`apply`](Apply::apply) but not
/// pure.
pub trait Apply: Functor + Semigroupal {
    /// # apply
    ///
    /// Given a value and a function in the Apply context, applies the function
    /// to the value.
    ///
    /// ## Example
    ///
    /// ```
    /// use cats::algebra::*;
    ///
    /// let x = Some(1);
    /// let y = Some(|x: i32| x as f64 / 2.0);
    /// let z = x.apply(y);
    /// assert_eq!(z, Some(0.5));
    /// ```
    fn apply<F, B>(self, ff: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B;
}

#[cfg(feature = "instance")]
impl<T> Apply for Option<T> {
    fn apply<F, B>(self, ff: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B,
    {
        match (self, ff) {
            (Some(a), Some(f)) => Some(f(a)),
            _ => None,
        }
    }
}

/// # Applicative
///
/// `Applicative` is a [`Functor`] with a [`pure`](Applicative::pure) method
/// and an [`apply`](Apply::apply) method.
///
/// ## Example
///
/// ```
/// use cats::algebra::*;
///
/// let x = Option::pure(1);
/// let y = Option::pure(|x: i32| x as f64 / 2.0);
/// let z = x.apply(y);
/// assert_eq!(z, Some(0.5));
/// ```
pub trait Applicative: Apply + Monoidal {
    fn pure<A>(a: A) -> Self::Wrapped<A>
    where
        // TODO: This is a trick to let Rust infer the type
        Self: Totality<Self::Wrapped<A>>;
}

#[cfg(feature = "instance")]
impl<T> Applicative for Option<T> {
    fn pure<A>(a: A) -> Self::Wrapped<A>
    where
        Self: Totality<Self::Wrapped<A>>,
    {
        Some(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let x = Some(1);
        let y = Some(|x: i32| x as f64 / 2.0);
        let z = x.apply(y);
        assert_eq!(z, Some(0.5));
    }

    #[test]
    fn test_applicative() {
        let x = Option::pure(1);
        let y = Option::pure(|x: i32| x as f64 / 2.0);
        let z = x.apply(y);
        assert_eq!(z, Some(0.5));
    }
}
