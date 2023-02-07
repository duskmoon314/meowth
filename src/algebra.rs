//! Algebraic structures and properties.
//!
//! This module contains algebraic structures and properties.
//! Mostly typeclasses and their instances.

pub mod applicative;
pub mod flatmap; // TODO: separate flatmap to a more appropriate mod
pub mod functor;
pub mod group;
pub mod magma;
pub mod monad;
pub mod monoid;
pub mod property;
pub mod semigroup;

pub use applicative::{Applicative, Apply};
pub use flatmap::FlatMap;
pub use functor::Functor;
pub use group::{Group, GroupK};
pub use magma::{Magma, MagmaK, Magmaal};
pub use monad::Monad;
pub use monoid::{Monoid, MonoidK, Monoidal};
pub use property::{Associativity, Commutativity, Identity, Inverse, Totality};
pub use semigroup::{Semigroup, SemigroupK, Semigroupal};
