use crate::middlewares::auth::{self, AuthConfig};
use crate::{handlers::order_handlers, handlers::system_handlers, AppState};
use axum::{middleware, routing::get, Router};
pub async fn create_app(state: AppState, auth: AuthConfig) -> Router {
    Router::new()
        .route(
            "/order",
            get(order_handlers::get_orders_handler).post(order_handlers::create_order_handler),
        )
        .route(
            "/order/:order_id",
            get(order_handlers::get_order_by_id_handler),
        )
        .route_layer(middleware::from_fn_with_state(auth, auth::authentication))
        .route("/health", get(system_handlers::health_handler))
        .with_state(state)
}
