#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

// TODO: Try adding specialization (default impls)
// #![cfg_attr(feature = "specialization", feature(specialization))]

pub mod algebra;
pub mod hkt;
pub mod utils;
// pub mod types;

pub use utils::EqT;
