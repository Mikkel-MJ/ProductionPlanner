use crate::model::current_user::CurrentUser;
use crate::model::task::Task;
use crate::AppState;
use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
pub async fn get_tasks_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<Task>>, StatusCode> {
    let task_result = sqlx::query_as!(
        Task,
        r#"SELECT id,order_id,tenant_id,note,title,status as "status:_" FROM tasks WHERE tenant_id =  $1"#,
        current_user.tenant_id
    )
    .fetch_all(&state.db)
    .await;

    match task_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_task_by_id_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(task_id): Path<i64>,
) -> Result<Json<Task>, StatusCode> {
    let order_result = sqlx::query_as!(
        Task,
        r#"SELECT id,order_id,tenant_id,note,title,status as "status:_" FROM tasks WHERE tenant_id =  $1 AND id = $2"#,
        current_user.tenant_id,
        task_id
    )
    .fetch_one(&state.db)
    .await;

    match order_result {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
