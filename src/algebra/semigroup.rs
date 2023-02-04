use super::*;

/// # SemiGroup
///
/// A `Semigroup` is a [`Magma`] which has [`Associativity`]. That is, the
/// operation [`combine`](Magma::combine) must be associative.
///
/// See [Semigroup](https://en.wikipedia.org/wiki/Semigroup) for more
/// information.
///
/// ## Example
///
/// Below is an example of implementing `Semigroup` for `Addition`.
///
/// ```
/// use std::ops::Add;
/// use cats::algebra::*;
///
/// struct Addition;
///
/// // To implement `Semigroup` for `Addition`, we need to implement `Magma` and
/// // `Associativity` for `Addition`.
/// impl<T: Add<Output = T>> Magma<T> for Addition {
///     fn combine(x: T, y: T) -> T {
///         x + y
///     }
/// }
/// impl<T: Add<Output = T>> Associativity<T> for Addition {}
/// // Once we have implemented `Magma` and `Associativity` for `Addition`, we
/// // already have implemented `Semigroup` for `Addition`.
///
/// assert_eq!(Addition::combine(1, 2), 3);
/// assert_eq!(Addition::combine(2.0, 3.0), 5.0);
/// assert_eq!(Addition::combine_n(2, 3), 6);
/// assert_eq!(Addition::combine_all_option(vec![1, 2, 3]), Some(6));
/// assert_eq!(Addition::combine_all_option::<Vec<i32>>(vec![]), None);
/// ```
///
/// We can also implement `Semigroup` for `i32` directly. Note that this
/// forbids other semigroup such as `Multiplication` to be implemented for
/// `i32`.
///
/// Since Rust has orphan rule, we wrap `i32` in a newtype `MyI32` to implement
/// `Semigroup` for `i32`.
///
/// ```
/// use cats::algebra::*;
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct MyI32(pub i32);
///
/// impl Magma for MyI32 {
///     fn combine(x: MyI32, y: MyI32) -> MyI32 {
///         MyI32(x.0 + y.0)
///     }
/// }
/// impl Associativity for MyI32 {}
///
/// assert_eq!(MyI32::combine(MyI32(1), MyI32(2)), MyI32(3));
/// assert_eq!(MyI32::combine_n(MyI32(2), 3), MyI32(6));
/// ```
pub trait Semigroup<T = Self>: Magma<T> + Associativity<T> {
    fn combine_n(x: T, n: usize) -> T
    where
        T: Sized + Clone,
    {
        let mut result = x.clone();
        for _ in 1..n {
            result = Self::combine(result, x.clone());
        }
        result
    }

    fn combine_all_option<I>(xs: I) -> Option<T>
    where
        I: IntoIterator<Item = T>,
        T: Sized,
    {
        xs.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(y) => Some(Self::combine(y, x)),
        })
    }
}

impl<T, S: Magma<T> + Associativity<T>> Semigroup<T> for S {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroup() {
        use std::ops::Add;

        struct Addition;

        impl<T: Add<Output = T>> Magma<T> for Addition {
            fn combine(x: T, y: T) -> T {
                x + y
            }
        }
        impl<T: Add<Output = T>> Associativity<T> for Addition {}

        assert_eq!(Addition::combine(1, 2), 3);
        assert_eq!(Addition::combine(2.0, 3.0), 5.0);
        assert_eq!(Addition::combine_n(2, 3), 6);
        assert_eq!(Addition::combine_all_option(vec![1, 2, 3]), Some(6));
        assert_eq!(Addition::combine_all_option::<Vec<i32>>(vec![]), None);
    }

    #[test]
    fn test_semigroup_self() {
        // Note: This forbids other semigroup such as `Multiplication` to be
        // implemented for `i32`.
        impl Magma for i32 {
            fn combine(x: i32, y: i32) -> i32 {
                x + y
            }
        }
        impl Associativity for i32 {}

        assert_eq!(i32::combine(1, 2), 3);
        assert_eq!(i32::combine_n(2, 3), 6);
        assert_eq!(i32::combine_all_option(vec![1, 2, 3]), Some(6));
    }
}
