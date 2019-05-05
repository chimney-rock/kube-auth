use jsonwebtoken::{decode as jwt_decode, Header, Algorithm, Validation};
use serde::{Deserialize, Serialize};
use failure::Fallible;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Claims {
  pub sub: String,
  pub aud: String,
  pub iat: i64,
  pub exp: i64,
  pub nbf: i64
}

pub fn decode<S>(token: S) -> Fallible<Claims>
  where S: Into<String> {

  jwt_decode::<Claims>(&token.into(), get_secret().as_ref(), &Validation::default())
    .map(|data| Ok(data.claims.into()))?
}

pub fn get_secret() -> String {
  std::env::var("HEIMDALLR_JWT_SECRET").unwrap_or_else(|_| "supercalifragilisticexpialidocious".into())
}
