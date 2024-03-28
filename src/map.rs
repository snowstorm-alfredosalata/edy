//! The Map struct, wrapping HashMap<String, Dynamic>.

use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::deps::HashMap;
use crate::prelude::{Dynamic, MissingKeyError};

/// The Map struct wraps an HashMap<String, Dynamic>, exposing ergonomic api to access the underlying data
/// with hard types.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub struct Map(HashMap<String, Dynamic>);

impl Map {
    pub fn new() -> Self {
        Map(HashMap::default())
    }

    pub fn try_get_casted<T: TryFrom<Dynamic, Error = crate::error::TypeError>>(
        &self,
        key: &str,
    ) -> Result<T, crate::error::Error> {
        let av = (**self)
            .get(key)
            .ok_or_else(|| MissingKeyError {
                payload_key: key.into(),
            })?
            .clone();

        T::try_from(av).map_err(|err| err.into())
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Dynamic> {
        self.0.iter()
    }
}

impl Deref for Map {
    type Target = HashMap<String, Dynamic>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<HashMap<String, Dynamic>> for Map {
    fn from(value: HashMap<String, Dynamic>) -> Self {
        Self(value)
    }
}
