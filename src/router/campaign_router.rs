use crate::AppState;
use crate::handler::campaign_handler;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use std::sync::Arc;

/// 众筹模块子路由
///
/// 挂载路径: `/api/v1/campaigns`
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(campaign_handler::create))
        .route("/", get(campaign_handler::list_active))
        .route("/{id}", get(campaign_handler::retrieve))
        .route("/{id}", put(campaign_handler::update))
        .route("/{id}", delete(campaign_handler::offline))
}
