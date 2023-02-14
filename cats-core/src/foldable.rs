//! Foldable

use crate::*;

/// Data structures that can be folded to a summary value.
pub trait Foldable: Hkt1 + Sized {
    /// Given a structure with elements whose type is a [`Monoid`], combine them
    /// via [`combine`](Magma::combine).
    fn fold(self) -> Self::Unwrapped
    where
        Self::Unwrapped: Monoid,
    {
        self.fold_map(|x| x)
    }

    /// Map each element of the structure to a [`Monoid`] and combine them via
    /// [`combine`](Magma::combine).
    fn fold_map<M, F>(self, f: F) -> M
    where
        M: Monoid,
        F: Fn(Self::Unwrapped) -> M,
    {
        self.fold_right(M::IDENTITY, |a, b| M::combine(f(a), b))
    }

    /// Left associative fold of a structure.
    fn fold_left<B, F>(self, b: B, f: F) -> B
    where
        F: Fn(B, Self::Unwrapped) -> B;

    /// Right associative fold of a structure.
    fn fold_right<B, F>(self, b: B, f: F) -> B
    where
        F: Fn(Self::Unwrapped, B) -> B;
}

impl<T> Foldable for Vec<T> {
    fn fold_left<B, F>(self, b: B, f: F) -> B
    where
        F: Fn(B, T) -> B,
    {
        self.into_iter().fold(b, f)
    }

    fn fold_right<B, F>(self, b: B, f: F) -> B
    where
        F: Fn(T, B) -> B,
    {
        let mut b = b;
        for x in self.into_iter().rev() {
            b = f(x, b);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foldable_vec() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.clone().fold(), 15);
        assert_eq!(v.clone().fold_map(|x| x * 2), 30);
        assert_eq!(v.clone().fold_left(0, |a, b| a + b), 15);
        assert_eq!(v.fold_right(0, |a, b| a + b), 15);
    }
}
