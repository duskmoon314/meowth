//! Identity

/// `Id` (or Identity) provides a function [`id`](Id::id) that maps any value to
/// itself.
pub trait Id<T> {
    /// # `id` function
    ///
    /// The identity function maps any value to itself.
    fn id(self) -> T;
}

/// Impl of [`Id`]
///
/// There should only be one implementation
impl<T> Id<T> for T {
    #[inline(always)]
    fn id(self) -> T {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let x = 1;
        let y = x.id();
        assert_eq!(x, y);
    }
}
