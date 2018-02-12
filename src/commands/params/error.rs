use std::fmt;
use std::error::Error as StdError;
use serde::de::Error as SerdeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
  MissingParams,
  TrailingCharacters,
  MissingValue(&'static str),
  Custom(String),
}

impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::MissingParams => "missing params",
      Error::TrailingCharacters => "trailing characters",
      Error::MissingValue(_) => "missing value",
      Error::Custom(_) => "custom error",
    }
  }

  fn cause(&self) -> Option<&StdError> {
    None
  }
}

impl fmt::Display for Error {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::MissingParams => write!(fmt, "missing params"),
      Error::TrailingCharacters => write!(fmt, "trailing characters"),
      Error::MissingValue(field) => write!(fmt, "missing value for field {}", field),
      Error::Custom(ref msg) => write!(fmt, "{}", msg),
    }
  }
}

impl SerdeError for Error {
  fn custom<T: fmt::Display>(msg: T) -> Self {
    Error::Custom(format!("{}", msg))
  }

  fn missing_field(field: &'static str) -> Error {
    Error::MissingValue(field)
  }
}
