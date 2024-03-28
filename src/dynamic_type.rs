//! DynamicType enum. Used for type-comparison of Dynamic values.

use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The DynamicType enum is mainly used for ergonomic type comparison of Dynamic values.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DynamicType {
    Str,
    Int,
    Bool,
    Float,
    Array,
    Limits,

    #[cfg(feature = "chrono")]
    Timestamp,

    Map,
    Null,
}

impl Default for DynamicType {
    fn default() -> Self {
        Self::Null
    }
}

impl Display for DynamicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicType::Str => write!(f, "Str"),
            DynamicType::Int => write!(f, "Int"),
            DynamicType::Bool => write!(f, "Bool"),
            DynamicType::Float => write!(f, "Float"),
            DynamicType::Array => write!(f, "Array"),
            DynamicType::Limits => write!(f, "Limits"),

            #[cfg(feature = "chrono")]
            DynamicType::Timestamp => write!(f, "Timestamp"),

            DynamicType::Map => write!(f, "Map"),
            DynamicType::Null => write!(f, "Null"),
        }
    }
}
