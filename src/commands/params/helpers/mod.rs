macro_rules! helper {
  ($name: ident, $kind: ident, $starts: expr, $message: expr) => {
    use discord::model::$kind;

    use serde::de::{self, Deserialize, Deserializer};

    use std::ops::Deref;

    #[derive(Debug, Deserialize)]
    pub struct $name {
      #[serde(deserialize_with = "parse")]
      id: $kind
    }

    fn parse<'de, D>(deserializer: D) -> Result<$kind, D::Error>
      where D: Deserializer<'de>
    {
      let s = String::deserialize(deserializer)?;
      let mut data = None;
      for start in $starts {
        if s.starts_with(start) && s.ends_with('>') {
          data = Some(&s[start.len()..s.len() - 1]);
          break;
        }
      }
      let s = data.unwrap_or(&s);
      s.parse::<u64>().map($kind).map_err(|e| de::Error::custom(&format!($message, e)))
    }

    impl Deref for $name {
      type Target = $kind;

      fn deref(&self) -> &Self::Target {
        &self.id
      }
    }
  }
}

pub mod mention;
pub mod channel;
