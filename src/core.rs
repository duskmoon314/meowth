//! Core traits and types of meowth

pub mod applicative;
pub mod either;
pub mod foldable;
pub mod functor;
pub mod hkt;
pub mod id;
pub mod magma;
pub mod monad;
pub mod monoid;
pub mod semigroup;
pub mod state;

#[doc(inline)]
pub use applicative::Applicative;
#[doc(inline)]
pub use either::{Either, Left, Right};
#[doc(inline)]
pub use foldable::Foldable;
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
#[doc(inline)]
pub use state::State;
