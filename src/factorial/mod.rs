//! This module is a near-copy of the special functions ln_factorial and binomial
//! as implemented in the crate statrs, to avoid unnecessary dependencies.

mod factorial;
mod gamma;
mod consts;

pub use factorial::{factorial, binomial};
