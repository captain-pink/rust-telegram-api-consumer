use diesel::{r2d2::ConnectionManager, SqliteConnection};
use r2d2::PooledConnection;

pub type PooledStoreConnection = PooledConnection<ConnectionManager<SqliteConnection>>;
