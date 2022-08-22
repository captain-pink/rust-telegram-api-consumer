use crate::app::api::{enums::Route, types::Routes};

#[derive(Clone, Debug)]
pub struct ApiConfig {
  base_url: String,
  routes: Routes,
}

impl ApiConfig {
  pub fn new(protocol: String, host: String, port: String, routes: Routes) -> Self {
    let base_url = ApiConfig::compose_base_url(protocol, host, port);

    Self { base_url, routes }
  }

  pub fn compose_base_url(protocol: String, host: String, port: String) -> String {
    format!("{}://{}:{}", protocol, host, port)
  }

  pub fn routes(&self) -> Routes {
    self.routes.clone()
  }

  pub fn base_url(&self) -> String {
    self.base_url.clone()
  }

  pub fn route(&self, route: &Route) -> String {
    let route = self
      .routes()
      .get(route)
      .expect("No route was found.")
      .clone();

    format!("{}{}", self.base_url(), route)
  }
}
