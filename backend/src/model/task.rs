use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub tenant_id: i32,
    pub order_id: i64,
    pub title: String,
    pub note: Option<String>,
    pub status: TaskStatus,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum TaskStatus {
    Unstarted = 1,
    Started = 2,
    Completed = 3,
    Issues = 4,
    Blocked = 5,
}
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct TaskTemplate {
    pub id: i64,
    pub tenant_id: i32,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateTaskTemplate {
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTaskTemplate {
    pub title: String,
}
