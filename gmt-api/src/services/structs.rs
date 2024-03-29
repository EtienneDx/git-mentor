use std::fmt::{Debug, Display};

use poem::IntoResponse;
use poem_openapi::{payload::PlainText, registry::MetaMediaType, ResponseContent};

#[derive(PartialEq, Eq)]
pub struct StringResponse(PlainText<String>);

impl From<String> for StringResponse {
  fn from(s: String) -> Self {
    Self(PlainText(s))
  }
}

impl<'a> From<&'a str> for StringResponse {
  fn from(s: &'a str) -> Self {
    Self(PlainText(s.to_string()))
  }
}

impl IntoResponse for StringResponse {
  fn into_response(self) -> poem::Response {
    self.0.into_response()
  }
}

impl ResponseContent for StringResponse {
  fn media_types() -> Vec<MetaMediaType> {
    PlainText::<String>::media_types()
  }
}

impl Debug for StringResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.0 .0)
  }
}

impl Display for StringResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0 .0)
  }
}
