use axum::{Router, routing::post};
use std::sync::Arc;

use crate::{AppState, handler::auth_handler};

/// 鉴权模块子路由
///
/// 挂载路径: `/api/v1/auth`
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .route("/refresh", post(auth_handler::refresh))
}
