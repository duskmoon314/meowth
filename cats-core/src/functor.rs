//! Functor

use crate::Hkt1;

/// `Functor` is a map from one category to another.
///
/// In `cats`, it provides two methods [`map`](Functor::fmap) (or
/// [`fmap`](Functor::fmap) as an alias) and [`lift`](Functor::lift) to map a
/// value of type `A` to a value of type `B` or a function from `A` to `B`.
///
/// REF
/// - [Wikipedia](https://en.wikipedia.org/wiki/Functor)
/// - [nLab](https://ncatlab.org/nlab/show/functor)
///
/// # Example
///
/// ```
/// use cats_core::Functor;
///
/// let x = Some(1);
/// let y = x.fmap(|x| x as f64 / 2.0);
/// assert_eq!(y, Some(0.5));
///
/// let mut f = Option::lift(|x: i32| x as f64 / 2.0);
/// assert_eq!(f(Some(1)), Some(0.5));
/// ```
pub trait Functor: Hkt1 + Sized {
    /// Maps a function over the wrapped value.
    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B;

    /// Some types have a `map` method already, so we use `fmap` as an alias of
    /// [`map`](Functor::map).
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        self.map(f)
    }

    /// Lifts a function of `A -> B` to a function of `F<A> -> F<B>`.
    fn lift<B, F>(f: F) -> Box<dyn Fn(Self) -> Self::Wrapped<B>>
    where
        for<'a> F: Fn(Self::Unwrapped) -> B + 'a + Clone,
    {
        Box::new(move |x: Self| x.map(f.clone()))
    }
}

impl<T> Functor for Option<T> {
    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        self.map(f)
    }
}

impl<T> Functor for Vec<T> {
    fn map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functor() {
        // test Option
        let x = Some(1);
        let y = x.fmap(|x| x as f64 / 2.0);
        assert_eq!(y, Some(0.5));

        let f = Option::lift(|x: i32| x as f64 / 2.0);
        assert_eq!(f(Some(1)), Some(0.5));

        // test Vec
        let x = vec![1, 2, 3];
        let y = x.fmap(|x| x as f64 / 2.0);
        assert_eq!(y, vec![0.5, 1.0, 1.5]);

        let f = Vec::lift(|x: i32| x as f64 / 2.0);
        assert_eq!(f(vec![1, 2, 3]), vec![0.5, 1.0, 1.5]);
    }
}
