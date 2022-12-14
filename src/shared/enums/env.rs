#[derive(Clone, IntoStaticStr)]
pub enum Variables {
  DB_URL,
  DB_POOL_SIZE,
  BOT_TOKEN,
  BOT_ADMIN_USER,
  BOT_ADMIN_TOKEN,
  DATABASE_URL,
  DATABASE_POOL_SIZE,
  VEHICLES_HOST,
  VEHICLES_PORT,
  VEHICLES_WS_PORT,
  VEHICLES_PROTOCOL,
}
