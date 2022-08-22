use std::sync::{Arc, Mutex};

use teloxide::{
  dptree,
  prelude::{AutoSend, Dispatcher},
  Bot as TeloxideBot,
};

use super::{
  db::{dao::ConsumersDao, ConnectionStore},
  handlers::{
    build_handler,
    types::{ApplicationConfig, ConsumersDaoType},
  },
  models::Config,
};

pub struct Bot;

impl Bot {
  pub async fn run(bot: AutoSend<TeloxideBot>, connection: Arc<ConnectionStore>) -> () {
    let config = Config::from_env();
    let consumers_dao: ConsumersDaoType = Arc::new(Mutex::new(ConsumersDao::new(connection)));
    let application_config: ApplicationConfig = Arc::new(config);

    let deps = dptree::deps![application_config.clone(), consumers_dao.clone()];
    let handler = build_handler();

    Dispatcher::builder(bot, handler)
      .dependencies(deps)
      .build()
      .setup_ctrlc_handler()
      .dispatch()
      .await;
  }
}
