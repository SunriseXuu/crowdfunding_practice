use axum::{
    Router,
    routing::{delete, patch, post, put},
};
use std::sync::Arc;

use crate::AppState;

pub mod user_handler;

/// 组装所有路由
pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        // 合并用户相关路由
        .nest("/api/v1", user_routes())
        .with_state(state)
}

/// 用户模块子路由
fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(user_handler::register))
        .route("/auth/login", post(user_handler::login))
        .route("/users", put(user_handler::update))
        .route("/users/password", patch(user_handler::update_password))
        .route("/users", delete(user_handler::soft_delete))
}
