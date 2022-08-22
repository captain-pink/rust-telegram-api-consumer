use log::info;
use teloxide::prelude::AutoSend;
use teloxide::{dispatching::HandlerExt, dptree, types::Message};
use teloxide::{Bot, RequestError};

use crate::app::handlers::types::{ApplicationConfig, ConsumersDaoType, MessageHandler};

use super::{CommonCommands, CommonService};

async fn handle(
  msg: Message,
  bot: AutoSend<Bot>,
  cmd: CommonCommands,
  dao: ConsumersDaoType,
  config: ApplicationConfig,
) -> Result<(), RequestError> {
  match cmd {
    CommonCommands::Start => CommonService::start(msg, bot).await,
    CommonCommands::LetMeIn { username, token } => {
      let user_creds = (username.as_str(), token.as_str());
    
      info!(
        "Creds: {}:{}\nfrom: {:?}",
        username.as_str(),
        token.as_str(),
        msg.from()
      );

      CommonService::let_me_in(msg, bot, user_creds, dao, config).await
    }
  }
}

pub fn common() -> MessageHandler {
  dptree::entry()
    .filter_command::<CommonCommands>()
    .endpoint(handle)
}
