macro_rules! helper {
  ($name: ident, $kind: ident, $starts: expr, $message: expr) => {
    use serenity::model::id::$kind;

    use std::ops::Deref;
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct $name {
      id: $kind
    }

    impl FromStr for $name {
      type Err = ::std::num::ParseIntError;

      fn from_str(s: &str) -> Result<$name, ::std::num::ParseIntError> {
        let mut data = None;
        for start in $starts {
          if s.starts_with(start) && s.ends_with('>') {
            data = Some(&s[start.len()..s.len() - 1]);
            break;
          }
        }
        let s = data.unwrap_or(&s);
        s.parse::<u64>().map($kind).map(|id| $name { id })
      }
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
pub mod role;
