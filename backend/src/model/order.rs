use serde::{Serialize, Deserialize};

#[derive(Serialize,Debug,sqlx::FromRow)]
pub struct Order {
    pub id: i64,
    pub title: String,
    pub status: OrderStatus
}
#[derive(Debug,sqlx::Type,Serialize)]
#[sqlx(type_name = "order_status_enum",rename_all = "lowercase")]
pub enum OrderStatus {
    Open,
    Completed,
    Blocked
}

#[derive(Deserialize)]
pub struct CreateOrder {
    pub title: String
}