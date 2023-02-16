use crate::model::current_user::CurrentUser;
use crate::model::order::{Order, UpdateOrder};
use crate::{model::order::CreateOrder, AppState};
use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
pub async fn get_orders_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    let order_result = sqlx::query_as!(
        Order,
        r#"SELECT id,tenant_id,order_nr,note,title,status as "status:_" FROM orders WHERE tenant_id =  $1"#,
        current_user.tenant_id
    )
    .fetch_all(&state.db)
    .await;

    match order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_order_by_id_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(order_id): Path<i64>,
) -> Result<Json<Order>, StatusCode> {
    let order_result = sqlx::query_as!(
        Order,
        r#"SELECT id,tenant_id,order_nr,note,title,status as "status:_" FROM orders WHERE tenant_id = $1 AND id = $2"#,
        current_user.tenant_id,
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
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, StatusCode> {
    let create_order_result = sqlx::query_as!(
        Order,
        r#"INSERT INTO public.orders(tenant_id,title,order_nr,note) VALUES ($1,$2,$3,$4) RETURNING id,tenant_id,order_nr,note,title,status as "status:_""#,
        current_user.tenant_id,
        payload.title,
        payload.order_nr,
        payload.note
    )
    .fetch_one(&state.db)
    .await;

    match create_order_result {
        Ok(o) => Ok(Json(o)),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn update_order_by_id_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(order_id): Path<i64>,
    Json(payload): Json<UpdateOrder>,
) -> Result<Json<Order>, StatusCode> {
    let create_order_result = sqlx::query_as!(
        Order,
        r#"update "orders"
        set title = coalesce($1, "orders".title),
        order_nr = coalesce($2, "orders".order_nr),
        note = coalesce($3, "orders".note),
        status = coalesce($4, "orders".status)
        where tenant_id = $5 AND id = $6 RETURNING id,tenant_id,order_nr,note,title,status as "status:_""#,
        payload.title,
        payload.order_nr,
        payload.note,
        payload.status.map(|s| s as i32),
        current_user.tenant_id,
        order_id
    )
    .fetch_one(&state.db)
    .await;

    match create_order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_order_by_id_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(order_id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let order_result = sqlx::query!(
        r#"DELETE FROM orders WHERE tenant_id = $1 AND id = $2"#,
        current_user.tenant_id,
        order_id
    )
    .execute(&state.db)
    .await;

    match order_result {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}
