use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Order {
    pub id: i64,
    pub tenant_id: i32,
    pub title: String,
    pub order_nr: Option<String>,
    pub note: Option<String>,
    pub status: OrderStatus,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum OrderStatus {
    Open = 1,
    Completed = 2,
    Blocked = 3,
}

#[derive(Deserialize)]
pub struct CreateOrder {
    pub title: String,
    pub order_nr: Option<String>,
    pub note: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct UpdateOrder {
    pub title: Option<String>,
    pub order_nr: Option<String>,
    pub note: Option<String>,
    pub status: Option<OrderStatus>,
}

#[derive(Deserialize)]
pub struct CreateOrderTemplate {
    pub title: String,
    pub task_templates: Vec<i64>,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct OrderTemplate {
    pub id: i64,
    pub tenant_id: i32,
    pub title: String,
}
