pub mod user_router;

use axum::Router;
use std::sync::Arc;

use crate::AppState;

/// 组装所有路由
pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        // 合并用户相关路由
        .nest("/api/v1", user_router::routes())
        .with_state(state)
}
