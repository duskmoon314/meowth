/// # Magma
///
/// A `Magma` is a set with a binary operation [`combine`](Magma::combine) that
/// must be closed.
///
/// See [Magma](https://en.wikipedia.org/wiki/Magma_(algebra)) for more
/// information.
pub trait Magma<T = Self> {
    fn combine(x: T, y: T) -> T;
}
