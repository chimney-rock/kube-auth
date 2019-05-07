use jsonwebtoken::{decode as jwt_decode, Header, Algorithm, Validation};
use diesel::{RunQueryDsl, ExpressionMethods, pg::Pg, sql_types::Jsonb};
use chrono::{Local, NaiveDateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::tokens;
use crate::server::HttpError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Identifiable, Queryable, Insertable, AsChangeset)]
#[table_name="tokens"]
pub struct Token {
  pub id: Uuid,
  pub user_id: Uuid,
  pub claims: Claims,
  pub expires_at: NaiveDateTime
}

#[derive(Clone, Debug, Insertable, AsChangeset)]
#[table_name="tokens"]
struct NewToken {
  id: Uuid,
  user_id: Uuid,
  claims: Claims,
  expires_at: NaiveDateTime
}

impl Token {
  fn new(user_id: Uuid, scopes: Vec<String>, expires_at: NaiveDateTime, conn: &diesel::pg::PgConnection) -> Result<Token, HttpError> {
    use crate::db::tokens::dsl::tokens;
    let generated_id = Uuid::new_v4();

    let new_token = NewToken {
      user_id,
      expires_at,
      id: generated_id,
      claims: Claims {
        user_id: Some(user_id),
        exp: expires_at.timestamp(),
        jti: generated_id,
        ..Default::default()
      }
    };

    Ok(diesel::insert_into(tokens)
      .values(&new_token)
      .get_result(conn)?)
  }

  // fn upsert(self, conn: &diesel::pg::PgConnection) {
  //   use crate::db::tokens::dsl::*;
  //   diesel::insert_into(tokens)
  //     .values(&self)
  //     .on_conflict(id)
  //     .do_update()
  //     .set(&self)
  //     .execute(conn);
  // }
}

#[derive(Clone, Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[sql_type = "Jsonb"]
pub struct Claims {
  pub sub: String,
  pub iss: String,
  pub iat: i64,
  pub exp: i64,
  pub nbf: i64,
  pub jti: Uuid,
  pub user_id: Option<Uuid>,
  pub scopes: Vec<String>
}

impl Default for Claims {
  fn default() -> Claims {
    Claims {
      sub: "auth".into(),
      iss: "heimdallr".into(),
      iat: Utc::now().timestamp(),
      exp: (Utc::now() + Duration::hours(2)).timestamp(),
      nbf: Utc::now().timestamp(),
      jti: Uuid::new_v4(),
      user_id: None,
      scopes: vec![]
    }
  }
}

impl diesel::deserialize::FromSql<Jsonb, Pg> for Claims {
  fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
    let value = <serde_json::Value as diesel::deserialize::FromSql<Jsonb, Pg>>::from_sql(bytes)?;
    Ok(serde_json::from_value(value)?)
  }
}

impl diesel::serialize::ToSql<Jsonb, Pg> for Claims {
  fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, Pg>) -> diesel::serialize::Result {
    let value = serde_json::to_value(self)?;
    <serde_json::Value as diesel::serialize::ToSql<Jsonb, Pg>>::to_sql(&value, out)
  }
}
