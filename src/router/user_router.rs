use axum::{
    Router,
    routing::{delete, get, patch, put},
};
use std::sync::Arc;

use crate::{AppState, handler::user_handler};

/// 用户模块子路由
///
/// 挂载路径: `/api/v1/users`
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(user_handler::retrieve))
        .route("/", put(user_handler::update))
        .route("/password", patch(user_handler::update_password))
        .route("/", delete(user_handler::deactivate))
}
