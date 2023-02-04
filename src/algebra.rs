//! Algebraic structures and properties.

pub mod group;
pub mod magma;
pub mod monoid;
pub mod property;
pub mod semigroup;

pub use group::Group;
pub use magma::Magma;
pub use monoid::Monoid;
pub use property::{Associativity, Commutativity, Identity, Inverse};
pub use semigroup::Semigroup;
