use teloxide::prelude::AutoSend;
use teloxide::requests::Requester;
use teloxide::{dispatching::HandlerExt, dptree, types::Message};
use teloxide::{respond, Bot, RequestError};

use crate::app::api::VehicleMessageController;
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
  controller: VehicleMessageController,
) -> Result<(), RequestError> {
  match cmd {
    AdminCommands::Consumers => AdminService::consumers(msg, bot, dao.clone()).await,
    AdminCommands::Vehicles { limit, offset } => {
      let vehicles = controller.vehicles(limit, offset, None).await;
      let message = format!("{:?}", vehicles);

      bot.send_message(msg.chat.id, message).await?;

      respond(())
    }
  }
}

pub fn admin() -> MessageHandler {
  dptree::filter(filter)
    .filter_command::<AdminCommands>()
    .endpoint(handle)
}
