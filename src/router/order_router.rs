use axum::{Router, routing::post};
use std::sync::Arc;

use crate::{AppState, handler::order_handler};

/// 订单模块子路由
///
/// 挂载路径: `/api/v1/orders`
pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", post(order_handler::create))
}
