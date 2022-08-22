#[macro_use]
extern crate diesel;

#[macro_use]
extern crate strum_macros;

extern crate pretty_env_logger;
extern crate log;
extern crate r2d2;
extern crate r2d2_sqlite;

mod app;
mod shared;
mod schema;

use dotenv::dotenv;
use crate::app::App;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();

  dotenv().ok();

  let app: App = App::new();
  app.run().await.unwrap();
}
