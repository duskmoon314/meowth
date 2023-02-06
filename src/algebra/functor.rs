use crate::hkt::HKT1;

/// # Functor
///
/// See [Functor](https://en.wikipedia.org/wiki/Functor) for more information.
///
/// ## Example
///
/// ```
/// use cats::algebra::Functor;
///
/// let x = Some(1);
/// let y = x.fmap(|x| x as f64 / 2.0);
/// assert_eq!(y, Some(0.5));
///
/// let mut f = Option::lift(|x: i32| x as f64 / 2.0);
/// assert_eq!(f(Some(1)), Some(0.5));
/// ```
pub trait Functor: HKT1 + Sized {
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B;

    fn lift<B, F>(f: F) -> Box<dyn FnMut(Self) -> Self::Wrapped<B>>
    where
        F: FnMut(Self::Unwrapped) -> B + 'static + Clone,
    {
        Box::new(move |x| x.fmap(f.clone()))
    }

    // TODO: Add more methods like in Scala Cats
}

#[cfg(feature = "instance")]
impl<T> Functor for Option<T> {
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B,
    {
        self.map(f)
    }
}

#[cfg(feature = "instance")]
impl<T> Functor for Vec<T> {
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functor() {
        assert_eq!(Option::fmap(Some(1), |x| x as f64 / 2.0), Some(0.5));
        assert_eq!(Some(1).fmap(f64::from), Some(1.0));

        let mut f = Option::lift(|x: i32| x as f64 / 2.0);
        assert_eq!(f(Some(1)), Some(0.5));
    }
}
