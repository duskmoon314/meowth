#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod applicative;
pub mod functor;
pub mod hkt;
pub mod id;
pub mod magma;
pub mod monad;
pub mod monoid;
pub mod semigroup;

#[doc(inline)]
pub use applicative::Applicative;
#[doc(inline)]
pub use functor::Functor;
#[doc(inline)]
pub use hkt::Hkt1;
#[doc(inline)]
pub use id::Id;
#[doc(inline)]
pub use magma::{Magma, MagmaK, Magmoidal};
#[doc(inline)]
pub use monad::Monad;
#[doc(inline)]
pub use monoid::{Monoid, MonoidK, Monoidal};
#[doc(inline)]
pub use semigroup::{Semigroup, SemigroupK};
