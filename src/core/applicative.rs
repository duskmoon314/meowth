//! Applicative

use crate::core::*;

/// `Applicative` is a [`Functor`] with a [`pure`](Applicative::pure) method
///
/// REF
/// - [nLab](https://ncatlab.org/nlab/show/applicative+functor)
pub trait Applicative: Functor + Monoidal {
    /// `pure` lifts a value into the applicative functor.
    ///
    /// # Example
    ///
    /// ```
    /// use cats::core::*;
    ///
    /// let x = Option::pure(1);
    /// assert_eq!(x, Some(1));
    /// ```
    fn pure<A>(a: A) -> Self::Wrapped<A>
    where
        Self: Id<Self::Wrapped<A>>,
        for<'a> A: Clone + 'a;

    /// `ap` applies a function to the value
    fn ap<B, F>(self, ff: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        for<'a> F: Fn(Self::Unwrapped) -> B + 'a;

    /// `ap2` applies a function to two values
    ///
    /// This should be really simple to implement, but not easy to provide a
    /// default implementation due to the type system.
    ///
    /// # Example
    ///
    /// ```
    /// use cats::core::*;
    ///
    /// let x = Some(1);
    /// let y = Some(2.0);
    /// let z = Some(|a: i32, b: f64| a as f64 + b);
    /// let w = x.ap2(y, z);
    /// assert_eq!(w, Some(3.0));
    /// ```
    fn ap2<B, C, F>(self, _b: Self::Wrapped<B>, _f: Self::Wrapped<F>) -> Self::Wrapped<C>
    where
        for<'a> F: Fn(Self::Unwrapped, B) -> C + 'a,
        for<'a> B: 'a,
    {
        unimplemented!()
    }
}

impl<T> Applicative for Option<T> {
    fn pure<A>(a: A) -> Option<A> {
        Some(a)
    }

    fn ap<B, F>(self, ff: Option<F>) -> Option<B>
    where
        F: Fn(T) -> B,
    {
        match (self, ff) {
            (Some(a), Some(f)) => Some(f(a)),
            _ => None,
        }
    }

    fn ap2<B, C, F>(self, b: Option<B>, f: Option<F>) -> Option<C>
    where
        for<'a> F: Fn(T, B) -> C + 'a,
        for<'a> B: 'a,
    {
        match self.product(b).product(f) {
            Some(((a, b), f)) => Some(f(a, b)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_applicative() {
        let x = Option::pure(1);
        assert_eq!(x, Some(1));

        let x = Some(1);
        let y = Some(2.0);
        let z = x.product(y);
        assert_eq!(z, Some((1, 2.0)));

        let x = Some(1);
        let y = Some(|x: i32| x as f64 / 2.0);
        let z = x.ap(y);
        assert_eq!(z, Some(0.5));

        let x = None;
        let y = Some(|x: i32| x as f64 / 2.0);
        let z = x.ap(y);
        assert_eq!(z, None);

        let x = Some(1);
        let y = Some(2.0);
        let z = Some(|a: i32, b: f64| a as f64 + b);
        let w = x.ap2(y, z);
        assert_eq!(w, Some(3.0));
    }
}
