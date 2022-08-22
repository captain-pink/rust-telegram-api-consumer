use self::types::MessageHandler;
use teloxide::{dispatching::UpdateFilterExt, types::Update};

mod admin;
use admin::admin;

mod common;
use common::common;

mod default;
use default::default;

pub mod types;

pub fn build_handler() -> MessageHandler {
    Update::filter_message()
        .branch(common())
        .branch(admin())
        .branch(default())
}
