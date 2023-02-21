//! Either

use crate::core::{Applicative, Functor, Hkt1, Id, Magmoidal, Monad, Monoidal};

/// `Either`
#[derive(Debug, Clone, PartialEq)]
pub enum Either<L, R> {
    /// `Left`
    Left(L),
    /// `Right`
    Right(R),
}

pub use Either::Left;
pub use Either::Right;

impl<L, R> Either<L, R> {
    /// Return `true` if the `Either` is a `Left`, `false` otherwise.
    pub fn is_left(&self) -> bool {
        match self {
            Either::Left(_) => true,
            Either::Right(_) => false,
        }
    }

    /// Return `true` if the `Either` is a `Right`, `false` otherwise.
    pub fn is_right(&self) -> bool {
        match self {
            Either::Left(_) => false,
            Either::Right(_) => true,
        }
    }

    /// Return the left value or a default.
    pub fn left_or(self, default: L) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => default,
        }
    }

    /// Return the right value or a default.
    pub fn right_or(self, default: R) -> R {
        match self {
            Either::Left(_) => default,
            Either::Right(r) => r,
        }
    }
}

impl<L, R> Hkt1 for Either<L, R> {
    type Unwrapped = R;
    type Wrapped<T> = Either<L, T>;
}

impl<L, R> Functor for Either<L, R> {
    fn map<B, F>(self, f: F) -> Either<L, B>
    where
        for<'a> F: Fn(R) -> B + 'a,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }
}

impl<L, R> Magmoidal for Either<L, R>
where
    for<'a> R: Clone + 'a,
{
    fn product<B>(self, b: Either<L, B>) -> Either<L, (R, B)>
    where
        for<'a> B: 'a,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => b.map(move |b| (r.clone(), b)),
        }
    }
}

impl<L, R> Monoidal for Either<L, R>
where
    for<'a> R: Clone + 'a,
{
    fn unit() -> Either<L, ()> {
        Either::Right(())
    }
}

impl<L, R> Applicative for Either<L, R>
where
    for<'a> R: Clone + 'a,
{
    fn pure<A>(a: A) -> Either<L, A>
    where
        Self: Id<Either<L, A>>,
        for<'a> A: Clone + 'a,
    {
        Either::Right(a)
    }

    fn ap<B, F>(self, ff: Either<L, F>) -> Either<L, B>
    where
        for<'a> F: Fn(R) -> B + 'a,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => ff.map(move |f| f(r.clone())),
        }
    }
}

impl<L, R> Monad for Either<L, R>
where
    for<'a> R: Clone + 'a,
{
    fn flat_map<B, F>(self, f: F) -> Either<L, B>
    where
        for<'a> F: Fn(R) -> Either<L, B> + 'a,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => f(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_either() {
        let s: Either<String, i32> = Left("foo".to_string());
        let n: Either<String, i32> = Right(3);

        assert_eq!(s.is_left(), true);
        assert_eq!(s.is_right(), false);
        assert_eq!(n.is_left(), false);
        assert_eq!(n.is_right(), true);

        let s = s.fmap(|x| x * 2);
        let n = n.fmap(|x| x * 2);

        assert_eq!(s, Left("foo".to_string()));
        assert_eq!(n, Right(6));
    }
}
