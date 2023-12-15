//! # Arakne Value Types
//! This crate contains structs and helpers to handle dynamic-typed values that serve as an interface between 
//! the Device struct and its Components, as well as between the Device struct and the scripting engine.
//! 
//! This approach is needed, as the type (and function or constructor signatures) of components cannot be known at compile-time,
//! and possibly might be wholly unknown to the core library (as defining custom component structs in application code is a core feature of this library).

mod dynamic;
mod dynamic_type;
mod map;
mod error;
mod deps;

#[cfg(feature = "serde")]
mod dynamic_serde;

pub use self::dynamic::*;
pub use self::dynamic_type::*;
pub use self::map::*;

#[cfg(test)]
mod tests;