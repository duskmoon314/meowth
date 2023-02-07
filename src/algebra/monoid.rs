use super::*;

/// # Monoid
///
/// A `Monoid` is a [`Semigroup`] which has [`Identity`]. That is, the operation
/// [`combine`](Magma::combine) must be associative and there must be an element
/// [`IDENTITY`](Identity::IDENTITY) such that `combine(x, IDENTITY) = x` for
/// all `x`.
///
/// See [Monoid](https://en.wikipedia.org/wiki/Monoid) for more information.
///
/// ## Example
///
/// Below is an example of implementing `Monoid` for `i32`. Since
/// Rust has orphan rule, we wrap `i32` in a newtype `MyI32` to implement
/// `Monoid` for `i32`.
///
/// ```
/// use cats::algebra::*;
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct MyI32(pub i32);
///
/// // To implement `Monoid` for `MyI32`, we need to implement `Magma`,
/// // `Totality`, `Associativity` and `Identity` for `MyI32`.
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
/// // Once we have implemented `Magma`, `Associativity` and `Identity` for
/// // `MyI32`, we already have implemented `Monoid` for `MyI32`.
///
/// assert_eq!(MyI32::combine(MyI32(1), MyI32(2)), MyI32(3));
/// assert_eq!(MyI32::combine_n(MyI32(2), 3), MyI32(6));
/// assert_eq!(MyI32::combine_all(vec![MyI32(1), MyI32(2), MyI32(3)]), MyI32(6));
/// assert_eq!(MyI32::combine_all::<Vec<MyI32>>(vec![]), MyI32(0));
/// assert_eq!(MyI32::is_identity(MyI32(0)), true);
/// ```
pub trait Monoid<T = Self>: Semigroup<T> + Identity<T> {
    fn combine_all<I>(xs: I) -> T
    where
        I: IntoIterator<Item = T>,
        T: Sized,
    {
        xs.into_iter().fold(Self::IDENTITY, Self::combine)
    }
}

impl<T, S: Semigroup<T> + Identity<T>> Monoid<T> for S {}

/// # MonoidK
///
/// A `MonoidK` is a [`SemigroupK`] which has [`Identity`]. That is, the
/// operation [`combine_k`](MagmaK::combine_k) must be associative and there
/// must be an element [`IDENTITY`](Identity::IDENTITY) such that
/// `combine_k(x, IDENTITY) = x` for all `x`.
///
/// `MonoidK` is also a set of `F<_>` (HKT). [`MagmaK`] (which [`SemigroupK`]
/// extends) uses GAT to implement this. See [`MagmaK`] for more details about
/// how to implement `MonoidK`.
///
/// See [Monoid](https://en.wikipedia.org/wiki/Monoid) for more information.
///
/// ## Example
///
/// ```
/// use cats::algebra::*;
///
/// assert_eq!(Vec::combine_all_k(vec![vec![1], vec![1], vec![1]]), vec![1, 1, 1]);
/// ```
pub trait MonoidK: SemigroupK + Identity {
    fn combine_all_k<T, I>(xs: I) -> Self::Wrapped<T>
    where
        I: IntoIterator<Item = Self::Wrapped<T>>,
        Self: Totality<Self::Wrapped<T>>
            + Associativity<Self::Wrapped<T>>
            + Identity<Self::Wrapped<T>>,
    {
        xs.into_iter().fold(Self::IDENTITY, Self::combine_k)
    }
}

impl<S: SemigroupK + Identity> MonoidK for S {}

pub trait Monoidal: Semigroupal + Identity {
    fn unit() -> Self::Wrapped<()>;
}

#[cfg(feature = "instance")]
impl<T> Monoidal for Option<T> {
    fn unit() -> Option<()> {
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monoid() {
        use std::ops::Add;

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
        impl Identity<f32> for Addition {
            const IDENTITY: f32 = 0.0;
        }

        assert_eq!(Addition::combine(1, 2), 3);
        assert_eq!(Addition::combine(2.0, 3.0), 5.0);
        assert_eq!(Addition::combine_all(vec![1, 2, 3]), 6);
        assert_eq!(Addition::combine_all::<Vec<i32>>(vec![]), 0);

        let i32_identity: i32 = Addition::identity();
        assert_eq!(i32_identity, 0);
        assert_eq!(Addition::is_identity(0.0), true);
    }

    #[test]
    fn test_monoidk() {
        let xs = vec![vec![1], vec![1], vec![1]];
        assert_eq!(Vec::combine_all_k(xs), vec![1, 1, 1])
    }
}
