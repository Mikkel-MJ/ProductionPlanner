use serde::Serialize;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Order {
    pub id: i64,
    pub title: String,
}
