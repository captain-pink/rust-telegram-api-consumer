use std::sync::{Arc, Mutex};

use teloxide::{
  dispatching::DpHandlerDescription,
  prelude::{DependencyMap, Handler},
  RequestError,
};

use crate::app::{db::dao::ConsumersDao, models::Config};

pub type MessageHandler =
  Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription>;

pub type ApplicationConfig = Arc<Config>;

pub type ConsumersDaoType = Arc<Mutex<ConsumersDao>>;
