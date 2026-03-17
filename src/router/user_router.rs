use axum::{
    Router,
    routing::{delete, patch, post, put},
};
use std::sync::Arc;

use crate::{AppState, handler::user_handler};

/// 用户模块子路由
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(user_handler::register))
        .route("/auth/login", post(user_handler::login))
        .route("/users", put(user_handler::update))
        .route("/users/password", patch(user_handler::update_password))
        .route("/users", delete(user_handler::soft_delete))
}
