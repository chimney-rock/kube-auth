use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Success {
  pub status: String
}

impl Default for Success {
  fn default() -> Success {
    Success { status: "OK".to_string() }
  }
}

impl Success {
  pub fn create() -> Success {
    Default::default()
  }
}
