//! Magma and generalized concept

use crate::Hkt1;

/// `Magma` is a type with a binary operation [`combine`](Magma::combine) that
/// must be closed.
///
/// REF:
/// - [nLab](https://ncatlab.org/nlab/show/magma)
///
/// # Implementation
///
/// `cats-core` provides default implementations of numeric types, string,
/// option, etc. based on the `+` operator (or [`Add`](std::ops::Add)).
pub trait Magma: Sized {
    /// Combines two values
    ///
    /// The operation must be closed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cats_core::Magma;
    ///
    /// assert_eq!(1.combine(2), 3);
    /// assert_eq!(Some(1).combine(Some(2)), Some(3));
    /// assert_eq!(Some(1).combine(None), Some(1));
    /// assert_eq!("Hello".to_string().combine("World".to_string()), "HelloWorld".to_string());
    /// ```
    fn combine(self, rhs: Self) -> Self;

    /// Combines self with itself
    ///
    /// The name `square` is chosen because it is the square of `x` if the operation is multiplication. i.e. `square(x) = x * x`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cats_core::Magma;
    ///
    /// assert_eq!(1.square(), 2);
    /// assert_eq!(Some(1).square(), Some(2));
    /// assert_eq!("Hello".to_string().square(), "HelloHello".to_string());
    /// ```
    fn square(self) -> Self
    where
        Self: Clone,
    {
        self.clone().combine(self)
    }
}

macro_rules! impl_magma_for_numeric {
    ($($t:ty),*) => {
        $(
            impl Magma for $t {
                fn combine(self, rhs: $t) -> $t {
                    self + rhs
                }
            }
        )*
    };
}

impl_magma_for_numeric!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl Magma for String {
    fn combine(self, rhs: String) -> String {
        self + &rhs
    }
}

impl<T: Magma> Magma for Option<T> {
    fn combine(self, rhs: Option<T>) -> Option<T> {
        match (self, rhs) {
            (Some(x), Some(y)) => Some(x.combine(y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }
}

/// `MagmaK` is a type constructor with a binary operation [`combine_k`](MagmaK::combine_k) that
/// must be closed.
///
/// Different from [`Magma`], `MagmaK` is about type constructor. Thus, the
/// inner type is not required to implement [`Magma`]. For example, `Option<T>`
/// is still a `MagmaK` even if `T` does not implement `Magma`.
pub trait MagmaK: Hkt1 + Sized {
    /// Combines two values
    ///
    /// The operation must be closed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cats_core::MagmaK;
    ///
    /// assert_eq!(Option::combine_k(Some(1), Some(2)), Some(1));
    /// assert_eq!(Option::combine_k(Some(1), None), Some(1));
    /// assert_eq!(Option::combine_k(None, Some(2)), Some(2));
    /// assert_eq!(Option::combine_k(None::<i32>, None), None);
    /// ```
    fn combine_k(self, rhs: Self) -> Self;
}

impl<T> MagmaK for Option<T> {
    fn combine_k(self, rhs: Option<T>) -> Option<T> {
        self.or(rhs)
    }
}

/// `Magmoidal` is a categorification of [`Magma`], which provides a functor
/// [`product`](Magmoidal::product).
///
/// [`product`](Magmoidal::product) combines two objects of the same
/// type constructor `F` into a single object of the same type constructor `F`.
/// For example, `fa: Option<A>` and `fb: Option<B>` can be combined into
/// `fc: Option<(A, B)>`.
///
/// REF:
/// - [nLab](https://ncatlab.org/nlab/show/magmoidal+category)
pub trait Magmoidal: Hkt1 + Sized {
    /// `product` combines two values
    ///
    /// # Example
    ///
    /// ```
    /// use cats_core::*;
    ///
    /// let x = Some(1);
    /// let y = Some(2.0);
    /// let z = x.product(y);
    /// assert_eq!(z, Some((1, 2.0)));
    /// ```
    fn product<B>(self, b: Self::Wrapped<B>) -> Self::Wrapped<(Self::Unwrapped, B)>
    where
        for<'a> B: 'a;
}

impl<A> Magmoidal for Option<A> {
    fn product<B>(self, b: Option<B>) -> Option<(A, B)> {
        match (self, b) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }
}

mod tests {
    #[test]
    fn test_magma() {
        use super::*;

        /// Test `Magma` helper function
        fn test_magma_helper<T: Magma + PartialEq + Clone + std::fmt::Debug>(x: T, y: T, z: T) {
            assert_eq!(x.clone().combine(y.clone()), z.clone());
            assert_eq!(
                x.clone().combine(y.clone()).combine(y.clone()),
                z.clone().combine(y.clone())
            );
            assert_eq!(
                x.clone().combine(y.clone()).combine(y.clone()),
                x.clone().combine(y.clone().square())
            );
            assert_eq!(
                x.clone().combine(y.clone()).combine(x.clone()),
                z.clone().combine(x.clone())
            );
            assert_eq!(
                x.clone().combine(y.clone()).combine(x.clone()),
                x.clone().combine(y.clone().combine(x.clone()))
            );
        }

        test_magma_helper(1_i8, 2_i8, 3_i8);
        test_magma_helper(
            "Hello".to_string(),
            "World".to_string(),
            "HelloWorld".to_string(),
        );
        test_magma_helper(Some(1_i8), Some(2_i8), Some(3_i8));
        test_magma_helper(None, Some(2_i8), Some(2_i8));
        test_magma_helper(Some(1_i8), None, Some(1_i8));
        test_magma_helper(None::<i8>, None, None);

        // Test impl of newtype
        #[derive(Debug, Clone, PartialEq)]
        struct MulI32(i32);

        impl Magma for MulI32 {
            fn combine(self, rhs: MulI32) -> MulI32 {
                MulI32(self.0 * rhs.0)
            }
        }

        test_magma_helper(MulI32(1), MulI32(2), MulI32(2));
    }

    #[test]
    fn test_magma_k() {
        use super::*;

        /// Test `MagmaK` helper function
        fn test_magma_k_helper<T: MagmaK + PartialEq + Clone + std::fmt::Debug>(x: T, y: T, z: T) {
            assert_eq!(x.clone().combine_k(y.clone()), z.clone());
            assert_eq!(
                x.clone().combine_k(y.clone()).combine_k(y.clone()),
                z.clone().combine_k(y.clone())
            );
            assert_eq!(
                x.clone().combine_k(y.clone()).combine_k(x.clone()),
                z.clone().combine_k(x.clone())
            );
            assert_eq!(
                x.clone().combine_k(y.clone()).combine_k(x.clone()),
                x.clone().combine_k(y.clone().combine_k(x.clone()))
            );
        }

        test_magma_k_helper(Some(1_i8), Some(2_i8), Some(1_i8));
        test_magma_k_helper(None, Some(2_i8), Some(2_i8));
        test_magma_k_helper(Some(1_i8), None, Some(1_i8));
        test_magma_k_helper(None::<i8>, None, None);
    }

    #[test]
    fn test_magmoidal() {
        use super::*;

        let x = Some(1);
        let y = Some(2.0);
        let z = x.product(y);
        assert_eq!(z, Some((1, 2.0)));

        let x = Some(1);
        let y = None::<f64>;
        let z = x.product(y);
        assert_eq!(z, None);

        let x = None::<i32>;
        let y = Some(2.0);
        let z = x.product(y);
        assert_eq!(z, None);
    }
}
