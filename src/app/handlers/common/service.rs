use teloxide::{
  prelude::AutoSend, requests::Requester, respond, types::Message, Bot, RequestError,
};

use crate::app::{
  auth::helpers::check_login,
  db::dto::CreateConsumer,
  handlers::types::{ApplicationConfig, ConsumersDaoType},
};

pub struct CommonService;

impl CommonService {
  pub async fn start(msg: Message, bot: AutoSend<Bot>) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, "Are you crazy?").await?;
    respond(())
  }

  pub async fn let_me_in<'a>(
    msg: Message,
    bot: AutoSend<Bot>,
    (username, token): (&'a str, &'a str),
    dao: ConsumersDaoType,
    config: ApplicationConfig,
  ) -> Result<(), RequestError> {
    let should_save_consumer =
      should_save_consumer(dao.clone(), msg.clone(), (username, token), config);

    if should_save_consumer {
      append_consumer(dao.clone(), msg.clone());

      bot
        .send_message(msg.clone().chat.id, "okay, jump on")
        .await?;
    } else {
      bot
        .send_message(msg.clone().chat.id, "It's you - 🤥")
        .await?;
    }

    respond(())
  }
}

fn append_consumer<'a>(dao: ConsumersDaoType, msg: Message) -> () {
  let user = msg.from().unwrap();

  let consumer = CreateConsumer::new(
    user.id.to_string(),
    user.username.clone(),
    user.first_name.clone(),
    user.last_name.clone(),
    user.is_bot,
    true,
  );

  dao.lock().unwrap().insert(&consumer);
}

fn should_save_consumer<'a>(
  dao: ConsumersDaoType,
  msg: Message,
  (username, token): (&'a str, &'a str),
  config: ApplicationConfig,
) -> bool {
  let is_logging_passed = check_login(&config.credentials, (username, token));
  let consumer_exists = dao
    .lock()
    .unwrap()
    .exists_by_tel_id(msg.chat.id.to_string());

  consumer_exists && is_logging_passed
}
