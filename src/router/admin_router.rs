use axum::{Router, routing::delete};
use std::sync::Arc;

use crate::{AppState, handler::admin_handler};

/// 管理员模块子路由
///
/// 挂载路径: `/api/v1/admin`
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users/{id}", delete(admin_handler::ban_user))
        .route("/campaigns/{id}", delete(admin_handler::cancel_campaign))
}
