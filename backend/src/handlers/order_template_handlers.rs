use crate::model::current_user::CurrentUser;
use crate::model::order::CreateOrderTemplate;
use crate::AppState;
use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

async fn create_order_template_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<CreateOrderTemplate>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.db.begin().await.unwrap();

    let create_template_result = sqlx::query!(
        r#"INSERT INTO public.order_templates(tenant_id,title) VALUES ($1,$2) RETURNING id"#,
        current_user.tenant_id,
        payload.title,
    )
    .fetch_one(&mut conn)
    .await;

    let template_id = match create_template_result {
        Ok(o) => o.id,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    let order_idVec: Vec<i64> = vec![template_id, payload.task_templates.len() as i64];

    let insert_result = sqlx::query!(
        r#"INSERT INTO public.order_task_template(order_template_id,task_template_id) SELECT * FROM UNNEST($1::bigint[],$2::bigint[])"#,
        &order_idVec,
        &payload.task_templates
    ).execute(&mut conn).await;

    match insert_result {
        Ok(_) => (),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    match conn.commit().await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
