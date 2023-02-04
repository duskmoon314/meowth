/// # Associativity
///
/// Associativity is a property of binary operations, which means that
/// rearranging the parentheses in an expression will not change the result.
///
/// See [Associative property](https://en.wikipedia.org/wiki/Associative_property)
/// for more information.
pub trait Associativity<T = Self> {
    fn is_associative() -> bool {
        true
    }
}

/// # Identity
///
/// Identity is a property means that there is an element `IDENTITY` of a binary
/// operation on a set that leaves unchanged every element of the set when
/// the operation is applied.
///
/// See [Identity element](https://en.wikipedia.org/wiki/Identity_element) for
/// more information.
pub trait Identity<T = Self> {
    const IDENTITY: T;

    fn identity() -> T {
        Self::IDENTITY
    }

    fn is_identity(x: T) -> bool
    where
        T: PartialEq + Sized,
    {
        x == Self::IDENTITY
    }
}

/// # Inverse
///
/// Inverse is a property means that for every element `x` of a set there is an
/// element `y` such that `combine(x, y) = IDENTITY` or `combine(y, x) =
/// IDENTITY`.
///
/// See [Inverse properties](https://en.wikipedia.org/wiki/Quasigroup#Inverse_properties)
/// for more information.
pub trait Inverse<T = Self> {
    fn inverse(x: T) -> T;
}

/// # Commutativity
///
/// Commutativity is a property of binary operations, which means that the order
/// of the operands does not change the result.
///
/// See [Commutative property](https://en.wikipedia.org/wiki/Commutative_property)
/// for more information.
pub trait Commutativity<T = Self> {
    fn is_commutative() -> bool {
        true
    }
}
