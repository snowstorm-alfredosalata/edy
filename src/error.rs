//! `edy` error types.

use crate::dynamic_type::DynamicType;

#[derive(Debug)]
pub struct TypeError {
    pub expected_type: DynamicType,
    pub found_type: DynamicType,
}

impl std::error::Error for TypeError {}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unexpected value of type {0}: expected {1}.",
            self.found_type, self.expected_type
        )
    }
}

#[derive(Debug)]
pub struct UnsupportedTypeError {
    pub expected_types: Vec<DynamicType>,
    pub found_type: DynamicType,
}

impl std::error::Error for UnsupportedTypeError {}

impl std::fmt::Display for UnsupportedTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unexpected value of type {0}: expected any of types [",
            self.found_type
        )?;

        for ty in &self.expected_types {
            write!(f, "{ty}, ")?;
        }

        write!(f, "].")
    }
}

#[derive(Debug)]
pub struct MissingKeyError {
    pub payload_key: String,
}

impl std::error::Error for MissingKeyError {}

impl std::fmt::Display for MissingKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Key {0} missing in Map.", self.payload_key)
    }
}

#[derive(Debug)]
pub enum Error {
    TypeError(TypeError),
    UnsupportedTypeError(UnsupportedTypeError),
    MissingKeyError(MissingKeyError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::TypeError(e) => e.fmt(f),
            Error::UnsupportedTypeError(e) => e.fmt(f),
            Error::MissingKeyError(e) => e.fmt(f),
        }
    }
}

impl From<TypeError> for Error {
    fn from(value: TypeError) -> Self {
        Error::TypeError(value)
    }
}

impl From<MissingKeyError> for Error {
    fn from(value: MissingKeyError) -> Self {
        Error::MissingKeyError(value)
    }
}

impl From<UnsupportedTypeError> for Error {
    fn from(value: UnsupportedTypeError) -> Self {
        Error::UnsupportedTypeError(value)
    }
}
