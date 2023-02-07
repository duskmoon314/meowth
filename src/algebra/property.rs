/// # Totality
///
/// Totality is a property of binary operations, which means that for every pair
/// of elements `x` and `y` of a set, the operation `combine(x, y)` is defined.
///
/// See [Total function](https://en.wikipedia.org/wiki/Total_function) for more
/// information.
pub trait Totality<T = Self> {
    fn is_closed() -> bool {
        true
    }
}

macro_rules! impl_totality {
    ($($t:ty),*) => {
        $(
            #[cfg(feature = "instance")]
            impl Totality for $t {}
        )*
    };
}

// In `instance`, we impl AddGroup for all numeric types.
impl_totality!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

#[cfg(feature = "instance")]
impl<T> Totality for Option<T> {}

#[cfg(feature = "instance")]
impl<T> Totality for Vec<T> {}

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

macro_rules! impl_associativity {
    ($($t:ty),*) => {
        $(
            #[cfg(feature = "instance")]
            impl Associativity for $t {}
        )*
    };
}

// In `instance`, we impl AddGroup for all numeric types.
impl_associativity!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

#[cfg(feature = "instance")]
impl<T> Associativity for Option<T> {}

#[cfg(feature = "instance")]
impl<T> Associativity for Vec<T> {}

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

macro_rules! impl_identity {
    ($($t:ty),*) => {
        $(
            #[cfg(feature = "instance")]
            impl Identity for $t {
                const IDENTITY: Self = 0 as Self;
            }
        )*
    };
}

// In `instance`, we impl AddGroup for all numeric types.
impl_identity!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl<T> Identity for Option<T> {
    const IDENTITY: Self = None;
}

#[cfg(feature = "instance")]
impl<T> Identity for Vec<T> {
    const IDENTITY: Self = vec![];
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

#[cfg(feature = "instance")]
impl<T: core::ops::Neg<Output = T>> Inverse<T> for T {
    fn inverse(x: T) -> T {
        -x
    }
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

macro_rules! impl_commutativity {
    ($($t:ty),*) => {
        $(
            #[cfg(feature = "instance")]
            impl Commutativity for $t {}
        )*
    };
}

// In `instance`, we impl AddGroup for all numeric types.
impl_commutativity!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
