use jsonwebtoken::{decode as jwt_decode, Header, Algorithm, Validation};
use actix_web::{Error, HttpResponse, web};
use futures::future::{Future, IntoFuture, Either, err, result};
use serde::Deserialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub aud: String,
  pub iat: i64,
  pub exp: i64,
  pub nbf: i64
}

use crate::db::Database;
use crate::server::errors::HttpError;
use crate::kubernetes::authentication::v1beta1::{TokenReview, TokenReviewStatus};

/// HTTP handler token authentication.
pub fn handler(token_review: web::Json<TokenReview>, _db: web::Data<Database>) -> impl Future<Item = HttpResponse, Error = Error> {
  debug!("Parsing TokenReview request = {:?}", token_review);

  let mut response = token_review.to_owned();
  web::block(move || {
    jwt_decode::<Claims>(&token_review.spec.token, "supercalifragilisticexpialidocious".as_ref(), &Validation::default())
  })
  .then(move |res| match res {
    Ok(token) => {
      debug!("{:?}", token);
      response.status = TokenReviewStatus::denied();
      result(Ok(HttpResponse::Unauthorized().json(response)))
    },
    Err(e) => {
      debug!("ERROR = {:?}", e);
      response.status = TokenReviewStatus::denied();
      result(Ok(HttpResponse::Unauthorized().json(response)))
    }
  })
}
