//! Algebraic structures and properties.

pub mod group;
pub mod magma;
pub mod monoid;
pub mod property;
pub mod semigroup;

pub use group::Group;
pub use magma::{Magma, MagmaK};
pub use monoid::{Monoid, MonoidK};
pub use property::{Associativity, Commutativity, Identity, Inverse, Totality};
pub use semigroup::{Semigroup, SemigroupK};
