use teloxide::{
  dptree, prelude::AutoSend, requests::Requester, respond, types::Message, Bot, RequestError,
};

use crate::app::handlers::types::MessageHandler;

async fn handle(message: Message, bot: AutoSend<Bot>) -> Result<(), RequestError> {
  bot
    .send_message(message.chat.id, "Don't touch me stinky, I'm working!!")
    .await?;
  bot.send_message(message.chat.id, "🥲").await?;

  respond(())
}

pub fn default() -> MessageHandler {
  dptree::endpoint(handle)
}
