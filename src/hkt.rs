//! Higher Kinded Types (HKT)

/// # HKT `F<_>`
///
/// `HKT1` represents the HKT `F<_>`, such as `Option<_>`. For example,
/// `MyF<T>` can be implemented as:
///
/// ```
/// use cats::hkt::*;
///
/// struct MyF<T>(T);
///
/// impl<A> HKT1 for MyF<A> {
///     type Unwrapped = A;
///     type Wrapped<T> = MyF<T>;
/// }
/// ```
pub trait HKT1 {
    type Unwrapped;
    type Wrapped<T>;
}

#[cfg(feature = "instance")]
impl<A> HKT1 for Option<A> {
    type Unwrapped = A;
    type Wrapped<T> = Option<T>;
}

#[cfg(feature = "instance")]
impl<A> HKT1 for Vec<A> {
    type Unwrapped = A;
    type Wrapped<T> = Vec<T>;
}

/// # HKT `F<_, _>`
pub trait HKT2 {
    type Unwrapped1;
    type Unwrapped2;
    type Wrapped<T1, T2>;
}
