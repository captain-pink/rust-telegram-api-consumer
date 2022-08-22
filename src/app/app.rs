use log::info;

use std::error::Error;
use std::sync::Arc;

use teloxide::{requests::RequesterExt, Bot as TeloxideBot};

use crate::app::bot::Bot;
use crate::app::db::ConnectionStore;

use crate::shared::enums::env::Variables;
use crate::shared::utils::env_var;

pub struct App;

impl App {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn run(&self) -> Result<(), Box<dyn Error>> {
    info!("App: initialize bot");

    let connection = Arc::new(ConnectionStore::from_env());

    let token = Variables::BOT_TOKEN.into();
    let token = env_var(token);

    let bot = TeloxideBot::new(token).auto_send();

    Bot::run(bot, connection.clone()).await;

    log::info!("App: remove bot listener");

    Ok(())
  }
}
