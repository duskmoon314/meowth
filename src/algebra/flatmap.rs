use super::*;
use crate::EqT;

/// # FlatMap
///
/// `FlatMap` is a weaker version of [`Monad`]: no [`pure`](Applicative::pure)
/// method. Indeed, [`Monad`] is a "subtype" of `FlatMap`.
///
/// ## Example
///
/// ```
/// use cats::hkt::HKT1;
/// use cats::algebra::*;
///
/// #[derive(Debug, PartialEq)]
/// struct F<T>(T);
///
/// // Implementation of required traits are hidden here
/// # impl<A> HKT1 for F<A> {
/// #     type Unwrapped = A;
/// #     type Wrapped<T> = F<T>;
/// # }
/// # impl<T> Totality for F<T> {}
/// # impl<T> Associativity for F<T> {}
/// # impl<A> Magmaal for F<A> {
/// #     fn product<B, C>(x: F<B>, y: F<C>) -> F<(B, C)> {
/// #         F((x.0, y.0))
/// #     }
/// # }
/// # impl<T> Functor for F<T> {
/// #     fn fmap<B, G>(self, mut f: G) -> F<B>
/// #     where
/// #         G: FnMut(T) -> B,
/// #     {
/// #        F(f(self.0))
/// #     }
/// # }
/// # impl<T> Apply for F<T> {
/// #     fn apply<G, B>(mut self, ff: F<G>) -> F<B>
/// #     where
/// #         G: FnOnce(Self::Unwrapped) -> B,
/// #     {
/// #         F((ff.0)(self.0))
/// #     }
/// # }
/// impl<T> FlatMap for F<T> {
///     fn flat_map<B, G>(self, mut f: G) -> F<B>
///     where
///         G: FnMut(Self::Unwrapped) -> F<B>,
///     {
///         f(self.0)
///     }
/// }
///
/// let x = F(F(1));
/// let y = x.flatten();
/// assert_eq!(y, F(1));
/// ```
pub trait FlatMap: Apply {
    fn flat_map<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>;

    /// Flatten a nested structure `F<F<A>>` into a flat structure `F<A>`.
    fn flatten<A>(self) -> Self::Wrapped<A>
    where
        Self::Unwrapped: EqT<Self::Wrapped<A>>,
    {
        self.flat_map(|x| x.cast())
    }
}

#[cfg(feature = "instance")]
impl<T> FlatMap for Option<T> {
    fn flat_map<B, F>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
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
    fn test_flat_map() {
        let x = Some(1);
        let y = x.flat_map(|x| Some(x + 1));
        assert_eq!(y, Some(2));

        let x = Some(Some(1));
        let y = x.flat_map::<i32, _>(|x| x);
        assert_eq!(y, Some(1));
        let z = <Option<Option<i32>> as FlatMap>::flatten(x);
        assert_eq!(z, Some(1));
    }
}
