//! Higher Kinded Types

/// `Hkt1` represents the HKT `F<_>`, such as `Option<_>`.
///
/// For example, `MyF<T>` can be implemented as:
///
/// ```
/// use cats_core::Hkt1;
///
/// struct MyF<T>(T);
///
/// impl<A> Hkt1 for MyF<A> {
///     type Unwrapped = A;
///     type Wrapped<T> = MyF<T>;
/// }
/// ```
pub trait Hkt1 {
    /// The type of the inner value
    type Unwrapped;
    /// The type of the outer value
    type Wrapped<T>;
}

impl<A> Hkt1 for Option<A> {
    type Unwrapped = A;
    type Wrapped<T> = Option<T>;
}

impl<A> Hkt1 for Vec<A> {
    type Unwrapped = A;
    type Wrapped<T> = Vec<T>;
}
