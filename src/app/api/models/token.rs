use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Token {
  value: String,
  valid_till: DateTime<Utc>,
}

impl Token {
  pub fn new(value: String, valid_till: DateTime<Utc>) -> Self {
    Self { value, valid_till }
  }

  pub fn value(&self) -> String {
    self.value.clone()
  }

  pub fn is_valid(&self) -> bool {
    self.valid_till > Utc::now()
  }
}
