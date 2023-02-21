//! Monoid and generalized concept

use crate::core::*;

/// `Monoid` is a [`Semigroup`] with an identity element.
///
/// REF
/// - [nLab](https://ncatlab.org/nlab/show/monoid)
pub trait Monoid: Semigroup {
    /// The identity element of `combine`
    const IDENTITY: Self;

    /// `combine_n_or_id` combines `n` elements of `I` into one.
    /// If `n` is zero, return `Self::IDENTITY`.
    fn combine_n_or_id(self, n: usize) -> Self
    where
        Self: Clone,
    {
        if n == 0 {
            Self::IDENTITY
        } else {
            self.combine_n(n)
        }
    }

    /// `combine_all` combines all elements of `I` into one.
    /// If `I` is empty, return `Self::IDENTITY`.
    fn combine_all<I>(xs: I) -> Self
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        xs.into_iter().fold(Self::IDENTITY, Self::combine)
    }
}

macro_rules! impl_monoid_for_numeric {
    ($($t:ty),*) => ($(
        impl Monoid for $t {
            const IDENTITY: Self = 0;
        }
    )*)
}

impl_monoid_for_numeric!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Monoid for String {
    const IDENTITY: Self = String::new();
}

impl<T: Monoid> Monoid for Option<T> {
    const IDENTITY: Self = None;
}

/// `MonoidK` is a [`SemigroupK`] with an identity element.
///
/// Different from [`Monoid`], `MonoidK` is about type constructor. Thus,
/// the inner type is not required to implement [`Monoid`]. For example,
/// `Option<T>` is still a `MonoidK` even if `T` does not implement
/// `Monoid`.
pub trait MonoidK: SemigroupK {
    /// The identity element of `combine_k`
    const IDENTITY: Self;

    /// `combine_n_or_id` combines `n` elements of `I` into one.
    /// If `n` is zero, return `Self::IDENTITY`.
    fn combine_n_or_id_k(self, n: usize) -> Self
    where
        Self: Clone,
    {
        if n == 0 {
            Self::IDENTITY
        } else {
            self.combine_n_k(n)
        }
    }

    /// `combine_all` combines all elements of `I` into one.
    /// If `I` is empty, return `Self::IDENTITY`.
    fn combine_all_k<I>(xs: I) -> Self
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        xs.into_iter().fold(Self::IDENTITY, Self::combine_k)
    }
}

impl<T> MonoidK for Option<T> {
    const IDENTITY: Self = None;
}

/// `Monoidal` is a [`Magmoidal`] with an unit object.
pub trait Monoidal: Magmoidal {
    /// The unit object of `combine`
    // const UNIT: Self::Wrapped<()>;
    fn unit() -> Self::Wrapped<()>;
}

impl<T> Monoidal for Option<T> {
    // const UNIT: Option<()> = Some(());
    fn unit() -> Option<()> {
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monoid() {
        assert_eq!(1.combine_n(3), 3);
        assert_eq!(i32::combine_all(vec![1, 2, 3]), 6);
        assert_eq!(i32::combine_all(vec![]), 0);
        assert_eq!(1.combine_n_or_id(0), 0);
        assert_eq!(1.combine_n_or_id(3), 3);
    }

    #[test]
    fn test_monoidk() {
        assert_eq!(Some(1).combine_n_k(3), Some(1));
        assert_eq!(
            Option::<i32>::combine_all_k(vec![Some(1), Some(2), Some(3)]),
            Some(1)
        );
        assert_eq!(Option::<i32>::combine_all_k(vec![]), None);
        assert_eq!(Some(1).combine_n_or_id_k(0), None);
        assert_eq!(Some(1).combine_n_or_id_k(3), Some(1));
    }

    #[test]
    fn test_monoidal() {
        assert_eq!(Option::<i32>::unit(), Some(()));
    }
}
