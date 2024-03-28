//! The `Dynamic` enum, the core block of `edy`.

#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;

use crate::{dynamic_type::DynamicType, error::TypeError, map::Map};

/// The `Dynamic` enum wraps any primitive type, excluding `u64`, `i128` at present implementation, exposing intuitive APIs for type comparison.
/// It can also wrap more complex and recursive types, such as `Vec<Dynamic>` and `Map`, a wrapper over `HashMap<String, Dynamic>`.
/// 
/// With the `serde` feature, the Dynamic enum also has everything needed to map any valid JSON object.
#[cfg_attr(feature = "serde", doc = r##"
See this example:

```
use edy::prelude::*;

fn main() {
    let to_ser = r#"
    {
        "u8": 8, 
        "i64": -64
    }
    "#;

    let map = serde_json::from_str::<Map>(to_ser).unwrap();

    assert!(map.try_get_casted::<u8>("u8").unwrap() == 8);
    assert!(map.try_get_casted::<i64>("i64").unwrap() == -64);
}
```
"##)]
#[derive(Clone, PartialEq, Debug)]
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

impl From<&str> for Dynamic {
    fn from(value: &str) -> Self {
        Dynamic::Str(value.into())
    }
}

#[cfg(feature = "chrono")]
map_dynamic! { NaiveDateTime => Timestamp }

impl<T: Into<Dynamic>> From<Vec<T>> for Dynamic {
    fn from(mut val: Vec<T>) -> Self {
        let vec: Vec<Dynamic> = val.drain(..).map(|el| Into::<Dynamic>::into(el)).collect();

        Dynamic::Array(vec)
    }
}

impl<T: TryFrom<Dynamic, Error = TypeError>> TryFrom<Dynamic> for Vec<T> {
    type Error = TypeError;

    fn try_from(value: Dynamic) -> Result<Self, Self::Error> {
        match value {
            Dynamic::Array(mut value) => {
                let mut out: Vec<T> = Vec::default();
                for v in value.drain(..) {
                    out.push(v.try_into()?);
                }

                Ok(out)
            }
            _ => Err(TypeError {
                expected_type: DynamicType::Array,
                found_type: value.get_type(),
            }),
        }
    }
}
