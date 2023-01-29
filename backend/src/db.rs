use sqlx::{Pool,Postgres};

pub async fn new_db_pool(max_con: u32,con_str:&str) -> Pool<Postgres> {
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_con)
        .connect(con_str)
        .await
        .unwrap();
    db
}

pub async fn migrate_db(pool:&Pool<Postgres>) {
    sqlx::migrate!("./migrations")
    .run(pool)
    .await
    .expect("Failed to run migrations")
}