use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone)]
pub struct Consumer {
  pub id: i32,
  pub tel_user_id: String,
  pub username: Option<String>,
  pub first_name: String,
  pub last_name: Option<String>,
  pub is_bot: bool,
  pub is_admin: bool,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
