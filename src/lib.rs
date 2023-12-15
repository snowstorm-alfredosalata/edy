//! # Edy
//! `edy` is simple and ergonomic library for handling dynamic-typed values, featuring highly tailorable features, no heap allocations for primitive types, and more.
//!
//! ## Edy at a glance
//! ```rust
//! use edy::prelude::*;
//!
//! fn double_dynamic(value: Dynamic) -> Result<Dynamic, Error> {
//!     match value {
//!         Dynamic::Str(v) => Ok(format!("{v}{v}").into()),
//!         Dynamic::Int(v) =>  Ok((v*2).into()),
//!         Dynamic::Float(v) => Ok((v*2f64).into()),
//!         _ => Err(UnsupportedTypeError {
//!                 expected_types: vec![DynamicType::Str, DynamicType::Int, DynamicType::Float],
//!                 found_type: value.get_type()
//!             }.into())
//!     }
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let dy: Dynamic = 2i32.into();
//!     let v: i32 = double_dynamic(dy)?
//!                  .try_into()?;
//!
//!     assert_eq!(4, v);
//!
//!     let dy: Dynamic = "2".into();
//!     let v: String = double_dynamic(dy)?
//!                     .try_into()?;
//!
//!     assert_eq!("22", &v);
//!
//!     Ok(())
//! }
//! ```

pub mod deps;
pub mod dynamic;
pub mod dynamic_type;
pub mod error;
pub mod map;

#[cfg(feature = "serde")]
pub mod dynamic_serde;

pub mod prelude {
    //! One-`use`-statement access to all `edy` features.

    pub use super::deps::*;

    pub use super::dynamic::*;
    pub use super::dynamic_type::*;
    pub use super::error::*;
    pub use super::map::*;
}

#[cfg(test)]
mod tests;
