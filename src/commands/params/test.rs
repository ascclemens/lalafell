#[derive(Deserialize, Debug, PartialEq)]
struct Params {
  who: String,
  number: Option<u8>
}

#[test]
fn parse_params() {
  let params: Params = super::from_str("someone 5").unwrap();
  let expected = Params { who: "someone".to_owned(), number: Some(5) };
  assert_eq!(expected, params);
}

#[test]
fn parse_optional_params() {
  let params: Params = super::from_str("someone").unwrap();
  let expected = Params { who: "someone".to_owned(), number: None };
  assert_eq!(expected, params);
}

#[test]
fn parse_missing_params() {
  match super::from_str::<Params>("") {
    Err(super::error::Error::MissingParams) => {},
    _ => panic!()
  }
}

#[test]
#[should_panic]
fn parse_invalid_params() {
  super::from_str::<Params>("someone bad").unwrap();
}

#[test]
fn parse_vec() {
  #[derive(Debug, Deserialize, PartialEq)]
  struct VecParams {
    first: String,
    others: Vec<String>
  }
  let params: VecParams = super::from_str("first second third").unwrap();
  let expected = VecParams { first: "first".to_owned(), others: vec!["second".to_owned(), "third".to_owned()] };
  assert_eq!(expected, params);
}

#[test]
fn parse_slice() {
  #[derive(Debug, Deserialize, PartialEq)]
  struct SliceParams {
    first: String,
    name: [String; 2],
    last: u8
  }
  let params: SliceParams = super::from_str("first second third 4").unwrap();
  let expected = SliceParams {
    first: "first".to_owned(),
    name: ["second".to_owned(), "third".to_owned()],
    last: 4
  };
  assert_eq!(expected, params);
}

#[test]
fn parse_empty_vec() {
  #[derive(Debug, Deserialize, PartialEq)]
  struct VecParams {
    first: String,
    empty: Option<Vec<String>>
  }
  let params: VecParams = super::from_str("first").unwrap();
  let expected = VecParams { first: "first".to_string(), empty: None };
  assert_eq!(expected, params);
}

#[test]
fn parse_optional_vec() {
  #[derive(Debug, Deserialize, PartialEq)]
  struct VecParams {
    first: String,
    empty: Option<Vec<String>>
  }
  let params: VecParams = super::from_str("first second third").unwrap();
  let expected = VecParams {
    first: "first".to_string(),
    empty: Some(vec!["second".to_owned(), "third".to_owned()])
  };
  assert_eq!(expected, params);
}

#[test]
fn parse_vec_u8s() {
  #[derive(Debug, Deserialize, PartialEq)]
  struct VecParams {
    first: u8,
    rest: Vec<u8>
  }
  let params: VecParams = super::from_str("1 2 3").unwrap();
  let expected = VecParams {
    first: 1,
    rest: vec![2, 3]
  };
  assert_eq!(expected, params);
}

#[test]
#[should_panic]
fn parse_missing_vec() {
  #[derive(Debug, Deserialize, PartialEq)]
  struct VecParams {
    first: u8,
    rest: Vec<u8>
  }
  super::from_str::<VecParams>("1").unwrap();
}
