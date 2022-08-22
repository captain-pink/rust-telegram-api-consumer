use crate::app::api::{enums::Route, types::Routes};

use super::AuthCredentials;

#[derive(Clone, Debug)]
pub struct ApiConfig {
  host: String,
  routes: Routes,
  credentials: AuthCredentials,
}

impl ApiConfig {
  pub fn new(host: String, routes: Routes, credentials: AuthCredentials) -> Self {
    Self {
      host,
      routes,
      credentials,
    }
  }

  pub fn routes(&self) -> Routes {
    self.routes.clone()
  }

  pub fn host(&self) -> String {
    self.host.to_string()
  }

  pub fn credentials(&self) -> AuthCredentials {
    self.credentials.clone()
  }

  pub fn route(&self, route: &Route) -> String {
    self
      .routes()
      .get(route)
      .expect("No route was found.")
      .clone()
  }
}
