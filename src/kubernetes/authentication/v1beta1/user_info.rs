use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserInfo {
  /// Any additional information provided by the authenticator.
  pub extra: Option<std::collections::BTreeMap<String, Vec<String>>>,

  /// The names of groups this user is a part of.
  pub groups: Option<Vec<String>>,

  /// A unique value that identifies this user across time.
  /// If this user is deleted and another user by the same name is added, they will have different UIDs.
  pub uid: Option<String>,

  /// The name that uniquely identifies this user among all active users.
  pub username: Option<String>
}
