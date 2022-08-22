#[derive(Clone, Debug)]
pub struct Credentials {
    username: String,
    token: String,
}

impl Credentials {
    pub fn new(username: String, token: String) -> Self {
        Self { username, token }
    }

    pub fn username(&self) -> String {
      self.username.clone()
    }

    pub fn token(&self) -> String {
      self.token.clone()
    }
}
