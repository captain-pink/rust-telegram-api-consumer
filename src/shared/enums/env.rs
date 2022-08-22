#[derive(Clone, IntoStaticStr)]
pub enum Variables {
    DB_URL,
    DB_POOL_SIZE,
    BOT_TOKEN,
    BOT_ADMIN_USER,
    BOT_ADMIN_TOKEN,
    VFS_HOST,
    VFS_USERNAME,
    VFS_PASSWORD,
    DATABASE_URL,
    DATABASE_POOL_SIZE,
}
