//! `Serde` implementation for Dynamic

use std::error::Error;

use crate::prelude::Dynamic;
use serde::{
    ser::{SerializeMap, SerializeSeq, SerializeTuple},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<'de> Deserialize<'de> for Dynamic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArakneValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ArakneValueVisitor {
            type Value = Dynamic;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid Dynamic value.")
            }

            fn visit_str<E>(self, value: &str) -> Result<Dynamic, E>
            where
                E: Error,
            {
                Ok(Dynamic::Str(value.to_string()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Dynamic, E>
            where
                E: Error,
            {
                Ok(Dynamic::Int(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Dynamic, E>
            where
                E: Error,
            {
                Ok(Dynamic::Int(value as i64))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Dynamic, E>
            where
                E: Error,
            {
                Ok(Dynamic::Bool(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Dynamic, E>
            where
                E: Error,
            {
                Ok(Dynamic::Float(value))
            }

            fn visit_unit<E>(self) -> Result<Dynamic, E>
            where
                E: Error,
            {
                Ok(Dynamic::Null)
            }
        }

        deserializer.deserialize_any(ArakneValueVisitor)
    }
}

impl Serialize for Dynamic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Dynamic::Str(s) => serializer.serialize_str(s),
            Dynamic::Int(i) => serializer.serialize_i64(*i),
            Dynamic::Bool(b) => serializer.serialize_bool(*b),
            Dynamic::Float(f) => serializer.serialize_f64(*f),
            Dynamic::Limits(tp) => {
                let mut tup = serializer.serialize_tuple(2)?;
                tup.serialize_element(&tp.0)?;
                tup.serialize_element(&tp.1)?;
                tup.end()
            }

            #[cfg(feature = "chrono")]
            Dynamic::Timestamp(ts) => serializer.serialize_str(&ts.to_string()),

            Dynamic::Array(sq) => {
                let mut seq = serializer.serialize_seq(Some(sq.len()))?;
                for element in sq {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
            Dynamic::Map(ts) => {
                let mut map = serializer.serialize_map(None)?;
                for (k, v) in ts.iter() {
                    map.serialize_entry(k, v)?;
                }

                map.end()
            }
            Dynamic::Null => serializer.serialize_unit(),
        }
    }
}
