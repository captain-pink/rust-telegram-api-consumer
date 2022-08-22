use crate::shared::{enums::env::Variables, utils::env_var};

use super::Credentials;

#[derive(Clone, Debug)]
pub struct Config {
  pub credentials: Credentials,
}

impl Config {
  pub fn from_env() -> Self {
    let username = Variables::BOT_ADMIN_USER.into();
    let username = env_var(username);

    let password = Variables::BOT_ADMIN_TOKEN.into();
    let password = env_var(password);

    Self {
      credentials: Credentials::new(username, password),
    }
  }
}
