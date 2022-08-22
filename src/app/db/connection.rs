use diesel::{r2d2::ConnectionManager, sqlite::SqliteConnection};
use r2d2::Pool;

use crate::shared::{enums::env::Variables, utils::env_var};

use super::types::PooledStoreConnection;

#[derive(Clone)]
pub struct ConnectionStore {
  pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl ConnectionStore {
  pub fn from_env() -> Self {
    let db_url = Variables::DATABASE_URL.into();
    let db_url = env_var(db_url);

    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = Pool::builder()
      .build(manager)
      .expect("Failed to create pool.");

    Self { pool }
  }

  pub fn get(&self) -> PooledStoreConnection {
    self
      .pool
      .get()
      .expect("Failed to get connnection from pool.")
  }
}
