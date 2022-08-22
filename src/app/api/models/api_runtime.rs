use super::Token;

#[derive(Clone, Debug)]
pub struct ApiRuntimeConfig {
  token: Option<Token>,
}

impl ApiRuntimeConfig {
  pub fn new(token: Option<Token>) -> Self {
    Self { token }
  }

  pub fn default() -> Self {
    Self { token: None }
  }

  pub fn token(&self) -> Option<Token> {
    self.token.clone()
  }

  pub fn set_token(&mut self, token: Token) -> () {
    self.token = Some(token);
  }
}
