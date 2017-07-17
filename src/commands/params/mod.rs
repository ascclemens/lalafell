pub mod error;
pub mod helpers;
#[cfg(test)]
mod test;

use self::error::Error;

use std::result::Result as StdResult;

use serde::de::{self, Deserialize, IntoDeserializer};
use serde::de::value::SeqDeserializer;

macro_rules! forward_parsed_values {
  ($($ty:ident => $method:ident,)*) => {
    $(
      fn $method<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor<'de>
      {
        if self.parts.is_empty() {
          return Err(Error::MissingParams);
        }
        match self.parts.remove(0).parse::<$ty>() {
          Ok(val) => val.into_deserializer().$method(visitor),
          Err(e) => Err(de::Error::custom(e))
        }
      }
    )*
  }
}

pub type Result<T> = StdResult<T, Error>;

struct Deserializer<'de> {
  parts: Vec<&'de str>,
  count: usize
}

/// Parse a command-line parameter string into a Rust type, most often a struct.
///
/// The order that fields are declared in the struct is the order in which they are parsed.
///
/// Note that this handles certain types in radically different ways than what you would expect.
///
/// `Option<T>` will only ever result in `None` if it is at the end of a string of parameters.
/// `Options`s can be chained at the end of a struct, but as soon as a `None` is hit, the rest are
/// guaranteed to be `None`, as well.
///
/// `Vec<T>` will consume every parameter except the ones before it. `Vec`s must contain one element
/// or the parser will return the `MissingParams` error. `Option<Vec<T>>` can be used in lieu of
/// empty `Vec`s.
///
/// If you want a list of a specific number of parts, use a fixed-length slice, e.g. `[String; 2]`.
pub fn from_str<'a, T>(input: &'a str) -> Result<T>
  where T: Deserialize<'a>
{
  let parts: Vec<&str> = input.split_whitespace().collect();
  let mut deserializer = Deserializer { parts: parts, count: 0 };
  let t = T::deserialize(&mut deserializer)?;
  if deserializer.parts.is_empty() {
    Ok(t)
  } else {
    Err(Error::Custom("trailing characters".to_owned()))
  }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
  type Error = Error;

  fn deserialize_struct<V>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where V: de::Visitor<'de>
  {
    self.deserialize_tuple(fields.len(), visitor)
  }

  fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where V: de::Visitor<'de>
  {
    visitor.visit_seq(self)
  }

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where V: de::Visitor<'de>
  {
    if self.parts.is_empty() {
      return Err(Error::MissingParams);
    }
    let first = self.parts.remove(0);
    first.into_deserializer().deserialize_any(visitor)
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where V: de::Visitor<'de>
  {
    if self.parts.is_empty() {
      visitor.visit_none()
    } else {
      visitor.visit_some(self)
    }
  }

  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where V: de::Visitor<'de>
  {
    if self.parts.is_empty() {
      return Err(Error::MissingParams);
    }
    SeqDeserializer::new(self.parts.drain(..)).deserialize_seq(visitor)
  }

  forward_parsed_values! {
    bool => deserialize_bool,
    u8 => deserialize_u8,
    u16 => deserialize_u16,
    u32 => deserialize_u32,
    u64 => deserialize_u64,
    i8 => deserialize_i8,
    i16 => deserialize_i16,
    i32 => deserialize_i32,
    i64 => deserialize_i64,
    f32 => deserialize_f32,
    f64 => deserialize_f64,
  }

  forward_to_deserialize_any! {
    char str string unit
    bytes byte_buf map unit_struct tuple_struct
    identifier ignored_any newtype_struct enum
  }
}

impl<'de> de::SeqAccess<'de> for Deserializer<'de> {
  type Error = Error;

  fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>>
    where V: de::DeserializeSeed<'de>
  {
    self.count += 1;
    seed.deserialize(self).map(Some)
  }

  fn size_hint(&self) -> Option<usize> {
    match self.parts.iter().size_hint() {
      (lower, Some(upper)) if lower == upper => Some(upper),
      _ => None,
    }
  }
}
