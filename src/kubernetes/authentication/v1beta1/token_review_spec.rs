#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReviewSpec {
  pub token: Option<String>
}

impl<'de> serde::Deserialize<'de> for TokenReviewSpec {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = TokenReviewSpec;
      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct TokenReviewSpec")
      }
    }
  }
}
