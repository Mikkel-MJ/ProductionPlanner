use crate::model::current_user::CurrentUser;
use crate::model::task::{CreateTaskTemplate, TaskTemplate, UpdateTaskTemplate};
use crate::AppState;
use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_task_templates_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<TaskTemplate>>, StatusCode> {
    let order_result = sqlx::query_as!(
        TaskTemplate,
        r#"SELECT id,tenant_id,title FROM task_templates WHERE tenant_id =  $1"#,
        current_user.tenant_id
    )
    .fetch_all(&state.db)
    .await;

    match order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_task_template_by_id_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(template_id): Path<i64>,
) -> Result<Json<TaskTemplate>, StatusCode> {
    let order_result = sqlx::query_as!(
        TaskTemplate,
        r#"SELECT id,tenant_id,title FROM task_templates WHERE tenant_id = $1 AND id = $2"#,
        current_user.tenant_id,
        template_id
    )
    .fetch_one(&state.db)
    .await;

    match order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_task_template_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<CreateTaskTemplate>,
) -> Result<Json<TaskTemplate>, StatusCode> {
    let create_order_result = sqlx::query_as!(
        TaskTemplate,
        r#"INSERT INTO public.task_templates(tenant_id,title) VALUES ($1,$2) RETURNING id,tenant_id,title"#,
        current_user.tenant_id,
        payload.title,
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

// deletes a task template by id from the databas
pub async fn delete_task_template_by_id_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(template_id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let order_result = sqlx::query!(
        r#"DELETE FROM task_templates WHERE tenant_id = $1 AND id = $2"#,
        current_user.tenant_id,
        template_id
    )
    .execute(&state.db)
    .await;

    match order_result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_task_template_by_id_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(template_id): Path<i64>,
    Json(payload): Json<UpdateTaskTemplate>,
) -> Result<Json<TaskTemplate>, StatusCode> {
    let order_result = sqlx::query_as!(
        TaskTemplate,
        r#"UPDATE task_templates SET title = $1 WHERE tenant_id = $2 AND id = $3 RETURNING id,tenant_id,title"#,
        payload.title,
        current_user.tenant_id,
        template_id
    )
    .fetch_one(&state.db)
    .await;

    match order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
