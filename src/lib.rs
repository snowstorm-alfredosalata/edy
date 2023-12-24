//! # Edy
//! `edy` is simple and ergonomic library for handling dynamic-typed values, featuring highly-tailorable features, no heap allocations for primitive types, and more.
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
//! fn main() {
//!     let dy_float = 2i32;
//! 
//!     if let Ok(Dynamic::Int(v)) = double_dynamic(dy_float.into()) {
//!         println!("Value is: {v}")
//!     }
//!     else {
//!         panic!("Value had the wrong type!")
//!     }
//!     
//! }
//! ```

pub mod dynamic;
pub mod dynamic_type;
pub mod map;
pub mod error;
pub mod deps;

#[cfg(feature = "serde")]
pub mod dynamic_serde;

pub mod prelude {
    //! One-`use`-statement access to all `edy` featues.

    pub use super::deps::*;

    pub use super::dynamic::*;
    pub use super::dynamic_type::*;
    pub use super::map::*;
    pub use super::error::*;
}

#[cfg(test)]
mod tests;