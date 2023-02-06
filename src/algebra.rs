//! Algebraic structures and properties.
//!
//! This module contains algebraic structures and properties.
//! Mostly typeclasses and their instances.

pub mod functor;
pub mod group;
pub mod magma;
pub mod monoid;
pub mod property;
pub mod semigroup;

pub use functor::Functor;
pub use group::{Group, GroupK};
pub use magma::{Magma, MagmaK};
pub use monoid::{Monoid, MonoidK};
pub use property::{Associativity, Commutativity, Identity, Inverse, Totality};
pub use semigroup::{Semigroup, SemigroupK};
