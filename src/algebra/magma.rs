use crate::hkt::HKT1;

use super::*;

/// # Magma
///
/// A `Magma` is a set of `T` with a binary operation
/// [`combine`](Magma::combine) that must be closed.
///
/// See [Magma](https://en.wikipedia.org/wiki/Magma_(algebra)) for more
/// information.
pub trait Magma<T = Self>: Totality<T> {
    fn combine(x: T, y: T) -> T;
}

macro_rules! impl_magma_for_numberic {
    ($($t:ty),*) => {
        $(
            #[cfg(feature = "instance")]
            impl Magma for $t {
                fn combine(x: $t, y: $t) -> $t {
                    x + y
                }
            }
        )*
    };
}

impl_magma_for_numberic!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

#[cfg(feature = "instance")]
impl<T: Magma> Magma for Option<T> {
    fn combine(x: Option<T>, y: Option<T>) -> Option<T> {
        match (x, y) {
            (Some(x), Some(y)) => Some(T::combine(x, y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }
}

#[cfg(feature = "instance")]
impl<T> Magma for Vec<T> {
    fn combine(x: Vec<T>, y: Vec<T>) -> Vec<T> {
        let mut z = x;
        z.extend(y);
        z
    }
}

/// # MagmaK
///
/// A `MagmaK` is a set of `F<_>` with a binary operation
/// [`combine_k`](MagmaK::combine_k) that must be closed.
///
/// `F<_>` represents a higher-kinded type (HKT), such as `Option<_>`. Since
/// Rust does not have HKT, this is implemented via generic associated type
/// (GAT).
///
/// See [Magma](https://en.wikipedia.org/wiki/Magma_(algebra)) for more
/// information.
///
/// ## Example
///
/// ```
/// use cats::algebra::*;
///
/// // Using `Option<_>` as `MagmaK`
/// assert_eq!(Option::combine_k(Some(1), Some(2)), Some(1));
/// assert_eq!(Option::combine_k(None, Some(1)), Some(1));
///
/// // Notice the difference when using `Option<_>` as `Magma`
/// assert_eq!(Option::combine(Some(1), Some(2)), Some(3));
/// assert_eq!(Option::combine(Some(1), None), Some(1));
///
/// // But `Vec<_>` always concat
/// assert_eq!(Vec::combine_k(vec![1], vec![2]), vec![1, 2]);
/// assert_eq!(Vec::combine(vec![1], vec![2]), vec![1, 2]);
/// ```
///
/// Actually, `MagmaK` is not implemented for `F<_>` directly, but for
/// `F<T>`. With the bound of generic, Rust can infer the type of `T` from
/// the context. Thus, the usage is pretty like in other languages with HKT.
/// (Or maybe I made a mistake in understanding HKT.)
pub trait MagmaK: HKT1 + Totality + Sized {
    fn combine_k<T>(x: Self::Wrapped<T>, y: Self::Wrapped<T>) -> Self::Wrapped<T>
    where
        // I find this a trick to let Rust infer the type of `T` from the
        // context.
        Self: Totality<Self::Wrapped<T>>;
}

#[cfg(feature = "instance")]
impl<A> MagmaK for Option<A> {
    fn combine_k<T>(x: Option<T>, y: Option<T>) -> Option<T> {
        x.or(y)
    }
}

#[cfg(feature = "instance")]
impl<A> MagmaK for Vec<A> {
    fn combine_k<T>(x: Vec<T>, y: Vec<T>) -> Vec<T> {
        let mut z = x;
        z.extend(y);
        z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magma() {
        assert_eq!(i32::combine(1, 2), 3);
        assert_eq!(f64::combine(1.0, 2.0), 3.0);
        assert_eq!(Option::combine(Some(1), Some(2)), Some(3));
    }

    #[test]
    fn test_magmak() {
        assert_eq!(Option::combine_k(Some(1), Some(2)), Some(1));
        assert_eq!(Option::combine_k(Some(1), None), Some(1));
        assert_eq!(Option::combine_k(None, Some(2)), Some(2));
        assert_eq!(Option::combine_k::<i32>(None, None), None);

        assert_eq!(Vec::combine_k(vec![1], vec![2]), vec![1, 2]);
    }
}
