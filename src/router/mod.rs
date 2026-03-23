pub mod admin_router;
pub mod auth_router;
pub mod campaign_router;
pub mod order_router;
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
        // 挂载众筹模块路由
        .nest("/api/v1/campaigns", campaign_router::routes())
        // 挂载订单模块路由
        .nest("/api/v1/orders", order_router::routes())
        // 挂载管理员模块路由
        .nest("/api/v1/admin", admin_router::routes())
        // 挂载状态
        .with_state(state)
}
