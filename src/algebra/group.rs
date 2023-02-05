use super::*;

/// # Group
///
/// A `Group` is a [`Monoid`] which has [`Inverse`]. That is, the operation
/// [`combine`](Magma::combine) must be associative and there must be an element
/// [`IDENTITY`](Identity::IDENTITY) such that `combine(x, IDENTITY) = x` for
/// all `x`. And for all `x`, there must be an element `y` such that
/// `combine(x, y) = IDENTITY`. The element `y` is called the
/// [inverse](Inverse::inverse) of `x`.
///
/// See [Group](https://en.wikipedia.org/wiki/Group_(mathematics)) for more
/// information.
///
/// ## Example
///
/// Below is an example of implementing `Group` for `i32`. Since Rust has orphan
/// rule, we wrap `i32` in a newtype `MyI32` to implement `Group` for `i32`.
///
/// ```
/// use cats::algebra::*;
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct MyI32(pub i32);
///
/// // To implement `Group` for `MyI32`, we need to implement `Magma`,
/// // `Totality`, `Associativity`, `Identity` and `Inverse` for `MyI32`.
/// impl Magma for MyI32 {
///     fn combine(x: MyI32, y: MyI32) -> MyI32 {
///         MyI32(x.0 + y.0)
///     }
/// }
/// impl Totality for MyI32 {}
/// impl Associativity for MyI32 {}
/// impl Identity for MyI32 {
///     const IDENTITY: MyI32 = MyI32(0);
/// }
/// impl Inverse for MyI32 {
///     fn inverse(x: MyI32) -> MyI32 {
///         MyI32(-x.0)
///     }
/// }
/// // Once we have implemented `Magma`, `Associativity`, `Identity` and
/// // `Inverse` for `MyI32`, we already have implemented `Group` for `MyI32`.
///
/// assert_eq!(MyI32::combine(MyI32(1), MyI32(2)), MyI32(3));
/// assert_eq!(MyI32::combine_all(vec![MyI32(1), MyI32(2), MyI32(3)]), MyI32(6));
/// assert_eq!(MyI32::remove(MyI32(3), MyI32(2)), MyI32(1));
/// assert_eq!(MyI32::is_inverse(MyI32(1), MyI32(-1)), true);
/// ```
pub trait Group<T = Self>: Monoid<T> + Inverse<T> {
    fn is_inverse(x: T, y: T) -> bool
    where
        T: PartialEq + Sized,
    {
        Self::combine(x, y) == Self::IDENTITY
    }

    fn remove(x: T, y: T) -> T
    where
        T: Sized,
    {
        Self::combine(x, Self::inverse(y))
    }
}

impl<T, S: Monoid<T> + Inverse<T>> Group<T> for S {}

/// # GroupK
///
/// A `GroupK` is a [`MonoidK`] which has [`Inverse`]. That is, the operation
/// [`combine_k`](MagmaK::combine_k) must be associative and there must be an
/// element [`IDENTITY`](Identity::IDENTITY) such that `combine_k(x, IDENTITY) =
/// x` for all `x`. And for all `x`, there must be an element `y` such that
/// `combine_k(x, y) = IDENTITY`. The element `y` is called the
/// [inverse](Inverse::inverse) of `x`.
///
/// `GroupK` is also a set of `F<_>` (HKT). See [`MagmaK`] for more details
/// about how to implement `GroupK`.
///
/// See [Group](https://en.wikipedia.org/wiki/Group_(mathematics)) for more
/// information.
///
/// ## Example
///
/// TODO: Find an example of `GroupK`. I can't think of one for now.
pub trait GroupK: MonoidK + Inverse {
    fn is_inverse_k<T>(x: Self::F<T>, y: Self::F<T>) -> bool
    where
        Self: Totality<Self::F<T>>
            + Associativity<Self::F<T>>
            + Identity<Self::F<T>>
            + Inverse<Self::F<T>>,
        Self::F<T>: PartialEq,
    {
        Self::combine_k(x, y) == Self::IDENTITY
    }

    fn remove_k<T>(x: Self::F<T>, y: Self::F<T>) -> Self::F<T>
    where
        Self: Totality<Self::F<T>>
            + Associativity<Self::F<T>>
            + Identity<Self::F<T>>
            + Inverse<Self::F<T>>,
    {
        Self::combine_k(x, Self::inverse(y))
    }
}

impl<S: MonoidK + Inverse> GroupK for S {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group() {
        use std::ops::{Add, Neg};

        struct Addition;

        impl<T: Add<Output = T>> Magma<T> for Addition {
            fn combine(x: T, y: T) -> T {
                x + y
            }
        }

        impl<T: Add<Output = T>> Totality<T> for Addition {}
        impl<T: Add<Output = T>> Associativity<T> for Addition {}

        impl Identity<i32> for Addition {
            const IDENTITY: i32 = 0;
        }

        impl<T: Add<Output = T> + Neg<Output = T>> Inverse<T> for Addition {
            fn inverse(x: T) -> T {
                -x
            }
        }

        assert_eq!(Addition::combine(1, 2), 3);
        assert_eq!(Addition::combine_all(vec![1, 2, 3]), 6);
        assert_eq!(Addition::remove(3, 2), 1);
        assert_eq!(Addition::is_inverse(1, -1), true);
    }
}
