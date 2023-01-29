use crate::model::order::Order;
use crate::{model::order::CreateOrder, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
pub async fn get_orders_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    let order_result =
        sqlx::query_as!(Order, r#"select id,title,status as "status:_" from orders"#)
            .fetch_all(&state.db)
            .await;

    match order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_order_by_id_handler(
    State(state): State<AppState>,
    Path(order_id): Path<i64>,
) -> Result<Json<Order>, StatusCode> {
    let order_result = sqlx::query_as!(
        Order,
        r#"select id,title,status as "status:_" from orders where id = $1"#,
        order_id
    )
    .fetch_one(&state.db)
    .await;

    match order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_order_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, StatusCode> {
    let create_order_result = sqlx::query_as!(
        Order,
        r#"INSERT INTO public.orders(title) VALUES ($1) RETURNING id,title,status as "status:_""#,
        payload.title
    )
    .fetch_one(&state.db)
    .await;

    match create_order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
