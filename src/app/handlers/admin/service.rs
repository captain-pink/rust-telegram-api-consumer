use std::sync::{Arc, Mutex};

use teloxide::{
  prelude::AutoSend, requests::Requester, respond, types::Message, Bot, RequestError,
};

use crate::app::db::dao::ConsumersDao;

pub struct AdminService;

impl AdminService {
  pub async fn consumers(
    msg: Message,
    bot: AutoSend<Bot>,
    dao: Arc<Mutex<ConsumersDao>>,
  ) -> Result<(), RequestError> {
    let message = AdminService::reply_message(dao.clone());

    bot.send_message(msg.chat.id, message).await?;

    respond(())
  }

  fn reply_message(dao: Arc<Mutex<ConsumersDao>>) -> String {
    let consumers = dao.lock().unwrap().find_all();

    format!("Active consumers:\n{:?}", consumers)
  }
}
