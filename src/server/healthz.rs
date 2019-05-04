use actix_web::{Error, HttpResponse, web};
// use futures::future::{Future, ok};
use futures::future::{Future, result};
use serde::Serialize;

use crate::db::Database;

#[derive(Serialize)]
pub struct HealthResponse<'a> {
  pub healthy: bool,
  pub version: &'a str
}

impl<'a> Default for HealthResponse<'a> {
  fn default() -> HealthResponse<'a> {
    HealthResponse {
      healthy: true,
      version: env!("CARGO_PKG_VERSION")
    }
  }
}

impl<'a> HealthResponse<'a> {
  fn create() -> HealthResponse<'a> {
    HealthResponse { ..Default::default() }
  }
}

/// HTTP handler for health checks.
pub fn handler(_db: web::Data<Database>) -> impl Future<Item = HttpResponse, Error = Error> {
  result(Ok(HttpResponse::Ok().json(HealthResponse::create()))).responder()
}
