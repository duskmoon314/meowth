//! Semigroup and generalized concept

use crate::*;

/// `Semigroup` is a [`Magma`] whose [`combine`](Magma::combine) operation is
/// associative.
///
/// REF
/// - [nLab](https://ncatlab.org/nlab/show/semigroup)
pub trait Semigroup: Magma {
    /// `combine_n` combines `n` elements of `Self` into one.
    fn combine_n(self, n: usize) -> Self
    where
        Self: Clone,
    {
        let mut result = self.clone();
        if n == 0 {
            panic!("n must be positive in Semigroup::combine_n (n > 0)")
        }
        for _ in 1..n {
            result = Self::combine(result, self.clone());
        }
        result
    }

    /// `combine_all_option` combines all elements of `I` into one.
    /// If `I` is empty, return `None`.
    /// Otherwise, return `Some(Self)`.
    fn combine_all_option<I>(xs: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        xs.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(y) => Some(Self::combine(y, x)),
        })
    }
}

macro_rules! impl_semigroup_for_numeric {
    ($($t:ty),*) => ($(
        impl Semigroup for $t {}
    )*)
}

impl_semigroup_for_numeric!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Semigroup for String {}

impl<T: Semigroup> Semigroup for Option<T> {}

/// `SemigroupK` is a [`MagmaK`] whose [`combine_k`](MagmaK::combine_k)
/// operation is associative.
///
/// Different from [`Semigroup`], `SemigroupK` is about type constructor. Thus,
/// the inner type is not required to implement [`Semigroup`]. For example,
/// `Option<T>` is still a `SemigroupK` even if `T` does not implement
/// `Semigroup`.
pub trait SemigroupK: MagmaK {
    /// `combine_n_k` combines `n` elements of `Self` into one.
    fn combine_n_k(self, n: usize) -> Self
    where
        Self: Clone,
    {
        let mut result = self.clone();
        if n == 0 {
            panic!("n must be positive in SemigroupK::combine_n_k (n > 0)")
        }
        for _ in 1..n {
            result = Self::combine_k(result, self.clone());
        }
        result
    }

    /// `combine_all_option_k` combines all elements of `I` into one.
    /// If `I` is empty, return `None`.
    /// Otherwise, return `Some(Self)`.
    fn combine_all_option_k<I>(xs: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        xs.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(y) => Some(Self::combine_k(y, x)),
        })
    }
}

impl<T> SemigroupK for Option<T> {}

// Maybe there should be a `Semigroupal` between `Magmoidal` and `Monoidal`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroup() {
        /// Test `Semigroup` helper function
        fn test_semigroup_helper<T: Semigroup + PartialEq + Clone + std::fmt::Debug>(
            x: T,
            n: usize,
            y: T,
        ) {
            assert_eq!(x.clone().combine_n(n), y.clone());
            assert_eq!(x.clone().combine_n(n + 1), y.clone().combine(x.clone()));
            assert_eq!(x.clone().combine_n(n + 1), x.clone().combine(y.clone()));

            let xs = vec![x.clone(); n];
            assert_eq!(T::combine_all_option(xs), Some(y.clone()));

            let xs: Vec<T> = vec![];
            assert_eq!(T::combine_all_option(xs), None);
        }

        test_semigroup_helper(1, 1, 1);
        test_semigroup_helper(1, 3, 3);
        test_semigroup_helper("a".to_string(), 3, "aaa".to_string());
        test_semigroup_helper(Some(1), 3, Some(3));
    }

    #[test]
    #[should_panic]
    fn test_semigroup_should_panic() {
        1.combine_n(0);
    }

    #[test]
    fn test_semigroupk() {
        /// Test `SemigroupK` helper function
        fn test_semigroupk_helper<T: SemigroupK + PartialEq + Clone + std::fmt::Debug>(
            x: T,
            n: usize,
            y: T,
        ) {
            assert_eq!(x.clone().combine_n_k(n), y.clone());
            assert_eq!(x.clone().combine_n_k(n + 1), y.clone().combine_k(x.clone()));
            assert_eq!(x.clone().combine_n_k(n + 1), x.clone().combine_k(y.clone()));

            let xs = vec![x.clone(); n];
            assert_eq!(T::combine_all_option_k(xs), Some(y.clone()));

            let xs: Vec<T> = vec![];
            assert_eq!(T::combine_all_option_k(xs), None);
        }

        test_semigroupk_helper(Some(1), 3, Some(1));

        let xs = vec![Some(1), Some(2), Some(3)];
        assert_eq!(Option::<i32>::combine_all_option_k(xs), Some(Some(1)));
    }
}
