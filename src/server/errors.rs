use actix_web::{http::StatusCode, error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponseBody {
  pub error: bool,
  pub message: Option<String>
}

impl Default for ErrorResponseBody {
  fn default() -> ErrorResponseBody {
    ErrorResponseBody { error: true, message: None }
  }
}

impl ErrorResponseBody {
  fn create<S>(message: S) -> ErrorResponseBody
    where S: Into<String> {
    ErrorResponseBody { message: Some(message.into()), ..Default::default() }
  }
}

#[derive(Debug)]
pub enum HttpError {
  BadRequest(String),   // 400
  Unauthorized,         // 401
  PaymentRequired,      // 402
  Forbidden,            // 403
  NotFound,             // 404
  MethodNotAllowed,     // 405
  NotAcceptable,        // 406
  Conflict,             // 409
  PreconditionFailed,   // 412
  PayloadTooLarge,      // 413
  UnsupportedMediaType, // 415
  ImATeaPot,            // 418
  InternalServerError,  // 500
  NotImplemented        // 501
}

impl fmt::Display for HttpError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      HttpError::BadRequest(ref message) => write!(f, "Bad Request ({})", message),
      HttpError::Unauthorized         => write!(f, "Unauthorized"),
      HttpError::PaymentRequired      => write!(f, "Payment Required"),
      HttpError::Forbidden            => write!(f, "Forbidden"),
      HttpError::NotFound             => write!(f, "Not Found"),
      HttpError::MethodNotAllowed     => write!(f, "Method Not Allowed"),
      HttpError::NotAcceptable        => write!(f, "Not Acceptable"),
      HttpError::Conflict             => write!(f, "Conflict"),
      HttpError::PreconditionFailed   => write!(f, "Precondition Failed"),
      HttpError::PayloadTooLarge      => write!(f, "Payload Too Large"),
      HttpError::UnsupportedMediaType => write!(f, "Unsupported Media Type"),
      HttpError::ImATeaPot            => write!(f, "I'm a Teapot"),
      HttpError::InternalServerError  => write!(f, "Internal Server Error"),
      HttpError::NotImplemented       => write!(f, "Not Implemented")
    }
  }
}

impl ResponseError for HttpError {
  fn error_response(&self) -> HttpResponse {
    match *self {
      HttpError::BadRequest(ref message) => {
        HttpResponse::BadRequest().json(ErrorResponseBody::create(message.to_owned()))
      },
      HttpError::Unauthorized         => HttpResponse::Unauthorized().json(ErrorResponseBody::create("Unauthorized")),
      HttpError::PaymentRequired      => HttpResponse::PaymentRequired().json(ErrorResponseBody::create("Payment Required")),
      HttpError::Forbidden            => HttpResponse::Forbidden().json(ErrorResponseBody::create("Forbidden")),
      HttpError::NotFound             => HttpResponse::NotFound().json(ErrorResponseBody::create("Not Found")),
      HttpError::MethodNotAllowed     => HttpResponse::MethodNotAllowed().json(ErrorResponseBody::create("Method Not Allowed")),
      HttpError::NotAcceptable        => HttpResponse::NotAcceptable().json(ErrorResponseBody::create("Not Acceptable")),
      HttpError::Conflict             => HttpResponse::Conflict().json(ErrorResponseBody::create("Conflict")),
      HttpError::PreconditionFailed   => HttpResponse::PreconditionFailed().json(ErrorResponseBody::create("Precondition Failed")),
      HttpError::PayloadTooLarge      => HttpResponse::PayloadTooLarge().json(ErrorResponseBody::create("Payload Too Large")),
      HttpError::UnsupportedMediaType => HttpResponse::UnsupportedMediaType().json(ErrorResponseBody::create("Unsupported Media Type")),
      HttpError::ImATeaPot            => HttpResponse::build(StatusCode::IM_A_TEAPOT).json(ErrorResponseBody::create("I'm a Teapot")),
      HttpError::InternalServerError  => HttpResponse::InternalServerError().json(ErrorResponseBody::create("Internal Server Error")),
      HttpError::NotImplemented       => HttpResponse::NotImplemented().json(ErrorResponseBody::create("Not Implemented"))
    }
  }
}

impl From<DieselError> for HttpError {
  fn from(error: DieselError) -> HttpError {
    match error {
      DieselError::DatabaseError(kind, info) => {
        if let DatabaseErrorKind::UniqueViolation = kind {
          let message = info.details().unwrap_or_else(|| info.message()).to_string();
          return HttpError::BadRequest(message);
        }
        HttpError::InternalServerError
      }
      _ => HttpError::InternalServerError,
    }
  }
}
