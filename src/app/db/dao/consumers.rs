use std::sync::Arc;

use diesel::prelude::*;

use diesel::insert_into;
use log::info;

use crate::app::db::dto::{Consumer, CreateConsumer};
use crate::app::db::ConnectionStore;

pub struct ConsumersDao {
  connection_store: Arc<ConnectionStore>,
}

impl ConsumersDao {
  pub fn new(connection_store: Arc<ConnectionStore>) -> Self {
    Self { connection_store }
  }

  pub fn insert(&mut self, consumer: &CreateConsumer) -> usize {
    use crate::schema::consumers::dsl::*;

    let conn = &mut self.connection_store.get();
    let fail_msg = format!("Failed to insert the Consumer: {:?}", &consumer);

    insert_into(consumers)
      .values(consumer)
      .execute(conn)
      .expect(&fail_msg)
  }

  pub fn find_all(&mut self) -> Vec<Consumer> {
    use crate::schema::consumers::dsl::*;

    let conn = &mut self.connection_store.get();

    consumers
      .load::<Consumer>(conn)
      .expect("Failed to load all consumers")
  }

  pub fn find_by_tel_id(&mut self, tel_id: String) -> Option<Consumer> {
    use crate::schema::consumers::dsl::*;

    let conn = &mut self.connection_store.get();

    consumers
      .filter(tel_user_id.eq(tel_id))
      .first::<Consumer>(conn)
      .optional()
      .expect("Failed to load all consumers")
  }

  pub fn exists_by_tel_id(&mut self, tel_id: String) -> bool {
    let consumer = self.find_by_tel_id(tel_id);
    info!("consumer: {:?}", consumer);

    if let Some(_c) = consumer {
      return true;
    }

    false
  }
}
