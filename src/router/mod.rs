pub mod auth_router;
pub mod user_router;

use axum::Router;
use std::sync::Arc;

use crate::AppState;

/// 组装所有路由
pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        // 挂载认证模块路由
        .nest("/api/v1/auth", auth_router::routes())
        // 挂载用户模块路由
        .nest("/api/v1/users", user_router::routes())
        // 挂载状态
        .with_state(state)
}
