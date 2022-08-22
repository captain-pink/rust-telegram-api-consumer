use teloxide::prelude::AutoSend;
use teloxide::{dispatching::HandlerExt, dptree, types::Message};
use teloxide::{Bot, RequestError};

use crate::app::handlers::types::{ConsumersDaoType, MessageHandler};

use super::service::AdminService;
use super::AdminCommands;

fn filter(msg: Message, dao: ConsumersDaoType) -> bool {
  msg
    .from()
    .map(|_user| {
      let mut dao = dao.lock().unwrap();

      dao.exists_by_tel_id(msg.chat.id.to_string())
    })
    .unwrap_or_default()
}

async fn handle(
  msg: Message,
  bot: AutoSend<Bot>,
  cmd: AdminCommands,
  dao: ConsumersDaoType,
) -> Result<(), RequestError> {
  match cmd {
    AdminCommands::Consumers => AdminService::consumers(msg, bot, dao.clone()).await,
  }
}

pub fn admin() -> MessageHandler {
  dptree::filter(filter)
    .filter_command::<AdminCommands>()
    .endpoint(handle)
}
