use actix_web::{Error, HttpResponse, web};
use futures::future::{Future, result};

use crate::db::Database;
use crate::kubernetes::authentication::v1beta1::{TokenReview, TokenReviewStatus};

/// HTTP handler for health checks.
pub fn handler(token_review: web::Json<TokenReview>, _db: web::Data<Database>) -> impl Future<Item = HttpResponse, Error = Error> {
  let mut response = token_review.to_owned();
  response.status  = Some(TokenReviewStatus::denied());

  result(Ok(HttpResponse::Ok().json(response)))
}
