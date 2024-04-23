use std::sync::OnceLock;

static POOL: OnceLock<sqlx::SqlitePool> = OnceLock::new();

pub async fn init() {
    let url = std::option_env!("DATABASE_URL").expect("database url");
    // The sqlx::sqlite driver sets `PRAGMA foreign_keys = ON` by default
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("connected to database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrated database");
    _ = POOL.set(pool);
}

pub fn get() -> &'static sqlx::SqlitePool {
    POOL.get().expect("database initialised")
}
