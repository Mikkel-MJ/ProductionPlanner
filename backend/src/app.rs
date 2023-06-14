use crate::handlers::{task_handlers, task_template_handlers};
use crate::middlewares::auth::{self, AuthConfig};
use crate::{handlers::order_handlers, handlers::system_handlers, AppState};
use axum::{middleware, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
pub async fn create_app(state: AppState, auth: AuthConfig) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    Router::new()
        .route(
            "/order",
            get(order_handlers::get_orders_handler).post(order_handlers::create_order_handler),
        )
        .route(
            "/order/:order_id",
            get(order_handlers::get_order_by_id_handler)
                .patch(order_handlers::update_order_by_id_handler)
                .delete(order_handlers::delete_order_by_id_handler),
        )
        .route("/task", get(task_handlers::get_tasks_handler))
        .route("/task/:task_id", get(task_handlers::get_task_by_id_handler))
        // .route(
        //     "/template/task",
        //     get(task_template_handlers::get_task_templates_handler)
        //         .post(task_template_handlers::create_task_template_handler),
        // )
        // .route(
        //     "/template/task/:template_id",
        //     get(task_template_handlers::get_task_template_by_id_handler)
        //         .put(task_template_handlers::update_task_template_by_id_handler)
        //         .delete(task_template_handlers::delete_task_template_by_id_handler),
        // )
        .route_layer(middleware::from_fn_with_state(auth, auth::authentication))
        .route("/health", get(system_handlers::health_handler))
        .layer(cors)
        .with_state(state)
}
