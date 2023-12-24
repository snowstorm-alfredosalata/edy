#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;

use crate::{dynamic_type::DynamicType, map::Map, error::TypeError};

/// The Dynamic enum serves as the main exchange value within the internal api.
/// It wraps any primitive type, excluding u64, i128 at present implementation, exposing intuitive APIs for type comparison.
#[derive(Clone, Debug)]
pub enum Dynamic {
    Str(String),
    Int(i64),
    Bool(bool),
    Float(f64),
    Limits((f64, f64)),
    
    #[cfg(feature = "chrono")]
    Timestamp(NaiveDateTime),

    Array(Vec<Dynamic>),
    Map(Map),
    Null,
}

impl Dynamic {
    pub fn get_type(&self) -> DynamicType {
        match self {
            Dynamic::Str(_) => DynamicType::Str,
            Dynamic::Int(_) => DynamicType::Int,
            Dynamic::Bool(_) => DynamicType::Bool,
            Dynamic::Float(_) => DynamicType::Float,
            Dynamic::Limits(_) => DynamicType::Limits,
            
            #[cfg(feature = "chrono")]
            Dynamic::Timestamp(_) => DynamicType::Timestamp,
            
            Dynamic::Array(_) => DynamicType::Array,
            Dynamic::Map(_) => DynamicType::Map,
            Dynamic::Null => DynamicType::Null,
        }
    }
}

impl Default for Dynamic {
    fn default() -> Self {
        Self::Null
    }
}

/// This macro serves as a clean way to map primitives to Dynamic values.
/// It's simply used as `<Primitive> => <DynamicEnumKey>`.
/// It implements `TryFrom<Dynamic> for <Primitive>` and `From<Primitive> for Dynamic`
macro_rules! map_dynamic {
    ($($from:ident => $to:ident),+) => {
        $(
            impl TryFrom<Dynamic> for $from {
                type Error = TypeError;

                fn try_from(value: Dynamic) -> Result<Self, Self::Error> {
                    match value {
                        Dynamic::$to(value) => Ok(value as $from),
                        _ => Err(
                            TypeError {
                            expected_type: DynamicType::$to,
                            found_type: value.get_type(),
                        }
                        .into()),
                    }
                }
            }

            impl From<$from> for Dynamic {
                fn from(value: $from) -> Self {
                    Dynamic::$to(value.into())
                }
            }
        )*
    }
}

map_dynamic! { 

    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,

    u8 => Int,
    u16 => Int,
    u32 => Int,

    f32 => Float,
    f64 => Float,

    bool => Bool,

    String => Str
}

#[cfg(feature = "chrono")]
map_dynamic! { NaiveDateTime => Timestamp }