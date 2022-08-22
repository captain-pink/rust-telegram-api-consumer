use std::collections::HashMap;

use log::info;
use reqwest::Client;
use serde_json::Value;

use crate::shared::{enums::env::Variables, utils::env_var};

use super::{enums::Route, models::ApiConfig, types::Routes};

#[derive(Clone, Debug)]
pub struct VehicleMessageController {
  config: ApiConfig,
  client: Client,
}

impl VehicleMessageController {
  pub fn default() -> Self {
    let host = Variables::VEHICLES_HOST.into();
    let host = env_var(host);

    let port = Variables::VEHICLES_PORT.into();
    let port = env_var(port);

    let protocol = Variables::VEHICLES_PROTOCOL.into();
    let protocol = env_var(protocol);

    let routes: Routes = HashMap::from([(Route::Vehicles, "/vehicles".to_string())]);
    let config = ApiConfig::new(protocol, host, port, routes);

    Self {
      config,
      client: reqwest::Client::new(),
    }
  }

  pub async fn vehicles(&self, limit: String, offset: String, _sort: Option<String>) -> Value {
    let url = self.config.route(&Route::Vehicles);

    let response = self
      .client()
      .get(url)
      .query(&[("limit", limit.as_str()), ("offset", offset.as_str())])
      .send()
      .await
      .expect("Failed to request vehicles");

    let response = response
      .json::<Value>()
      .await
      .expect("Failed to parse response");

    info!("{:?}", &response);

    response
  }

  fn client(&self) -> reqwest::Client {
    self.client.clone()
  }
}
