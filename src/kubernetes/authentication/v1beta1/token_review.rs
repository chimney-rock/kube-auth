use serde::{de::{self, Deserialize, Deserializer, Visitor}, ser::SerializeStruct};

use crate::kubernetes::Resource;

/// TokenReview attempts to authenticate a token to a known user.
/// Note: TokenReview requests may be cached by the webhook token authenticator plugin in the kube-apiserver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenReview {
  pub spec:   super::TokenReviewSpec,
  pub status: Option<super::TokenReviewStatus>
}

impl Resource for TokenReview {
  fn api_version() -> &'static str {
    "authentication.k8s.io/v1beta1"
  }

  fn kind() -> &'static str {
    "TokenReview"
  }

  fn version() -> &'static str {
    "v1beta1"
  }
}

impl<'de> Deserialize<'de> for TokenReview {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
    // Field names (Used for error messages)
    const FIELDS: &'static [&'static str] = &["apiVersion", "kind", "spec", "status"];

    // Fields that are permitted for TokenReview objects
    enum Field {
      ApiVersion,
      Kind,
      Spec,
      Status
    }

    impl<'de> Deserialize<'de> for Field {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
          type Value = Field;

          fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Field Identifier")
          }

          /// Check each key & ensure nothing wacky gets passed
          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: de::Error {
            match value {
              "apiVersion" => Ok(Field::ApiVersion),
              "kind"       => Ok(Field::Kind),
              "spec"       => Ok(Field::Spec),
              "status"     => Ok(Field::Status),
              _            => Err(de::Error::unknown_field(value, FIELDS))
            }
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct TokenReviewVisitor;
    impl<'de> Visitor<'de> for TokenReviewVisitor {
      type Value = TokenReview;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("TokenReview")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de> {
        let mut value_spec:    Option<super::TokenReviewSpec>   = None;
        let mut value_status:  Option<super::TokenReviewStatus> = None;
        let mut value_version: Option<&str> = None;
        let mut value_kind:    Option<&str> = None;

        while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
          match key {
            Field::ApiVersion => {
              // Ensures that the api version matches
              value_version = Some(de::MapAccess::next_value(&mut map)?);
              if value_version != Some(<Self::Value as Resource>::api_version()) {
                return Err(de::Error::invalid_value(de::Unexpected::Str(&value_version.unwrap()), &<Self::Value as Resource>::api_version()));
              }
            },
            Field::Kind => {
              // Ensures this is a TokenReview object
              value_kind = Some(de::MapAccess::next_value(&mut map)?);
              if value_kind != Some(<Self::Value as Resource>::kind()) {
                return Err(de::Error::invalid_value(de::Unexpected::Str(&value_kind.unwrap()), &<Self::Value as Resource>::kind()));
              }
            },
            Field::Spec   => value_spec   = Some(de::MapAccess::next_value(&mut map)?),
            Field::Status => value_status = de::MapAccess::next_value(&mut map)?
          }
        }

        // TODO: Find better way to ensure these fields exist
        let _ = value_version.ok_or_else(|| de::Error::missing_field("apiVersion"))?;
        let _ = value_kind.ok_or_else(|| de::Error::missing_field("apiVersion"))?;

        Ok(TokenReview {
          spec: value_spec.ok_or_else(|| de::Error::missing_field("spec"))?,
          status: value_status
        })
      }
    }

    deserializer.deserialize_struct("TokenReview", FIELDS, TokenReviewVisitor)
  }
}

impl serde::Serialize for TokenReview {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
    let mut state = serializer.serialize_struct("TokenReview", 3 + self.status.as_ref().map_or(0, |_| 1))?;
    
    SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as Resource>::api_version())?;
    SerializeStruct::serialize_field(&mut state, "kind", <Self as Resource>::kind())?;
    SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
    if let Some(value) = &self.status {
      SerializeStruct::serialize_field(&mut state, "status", value)?;
    }
    SerializeStruct::end(state)
  }
}


#[cfg(test)]
mod tests {
  use speculate::speculate;
  use super::*;

  speculate! {
    it "deserializes a valid TokenRequest" {
      let json = r#"
        {
          "apiVersion": "authentication.k8s.io/v1beta1",
          "kind": "TokenReview",
          "spec": {
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxc3Qtc2hpYmUiLCJuYW1lIjoiVGFrYXJhIiwiaWF0IjoxNTE2MjM5MDIyfQ.jvzLvx1iDgLotO5-tClI5ZuUW8yEUKR_YKh-FYdlvaM"
          }
        }"#;
      let review: TokenReview = serde_json::from_str(&json).unwrap();
      assert_eq!(review.spec.token, Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxc3Qtc2hpYmUiLCJuYW1lIjoiVGFrYXJhIiwiaWF0IjoxNTE2MjM5MDIyfQ.jvzLvx1iDgLotO5-tClI5ZuUW8yEUKR_YKh-FYdlvaM".to_owned()));
    }

    #[should_panic]
    it "panics when `apiVersion` is missing" {
      let json = r#"
        {
          "kind": "TokenReview",
          "spec": {
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxc3Qtc2hpYmUiLCJuYW1lIjoiVGFrYXJhIiwiaWF0IjoxNTE2MjM5MDIyfQ.jvzLvx1iDgLotO5-tClI5ZuUW8yEUKR_YKh-FYdlvaM"
          }
        }"#;
      let _: TokenReview = serde_json::from_str(&json).unwrap();
    }

    #[should_panic]
    it "panics when `kind` is missing" {
      let json = r#"
        {
          "apiVersion": "authentication.k8s.io/v1beta1",
          "spec": {
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxc3Qtc2hpYmUiLCJuYW1lIjoiVGFrYXJhIiwiaWF0IjoxNTE2MjM5MDIyfQ.jvzLvx1iDgLotO5-tClI5ZuUW8yEUKR_YKh-FYdlvaM"
          }
        }"#;
      let _: TokenReview = serde_json::from_str(&json).unwrap();
    }
  }
}
