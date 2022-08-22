use serde::Deserialize;

use crate::schema::consumers;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = consumers)]
pub struct CreateConsumer {
  tel_user_id: String,
  username: Option<String>,
  first_name: String,
  last_name: Option<String>,
  is_bot: bool,
  is_admin: bool,
}

impl CreateConsumer {
  pub fn new(
    tel_user_id: String,
    username: Option<String>,
    first_name: String,
    last_name: Option<String>,
    is_bot: bool,
    is_admin: bool,
  ) -> Self {
    Self {
      tel_user_id,
      username,
      first_name,
      last_name,
      is_bot,
      is_admin,
    }
  }
}
