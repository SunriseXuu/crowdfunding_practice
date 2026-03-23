use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::order_req::CreateOrderReq;
use crate::error::AppError;
use crate::extractor::{AuthenticatedUser, ValidatedJson};
use crate::service::OrderService;

/// 创建订单/发起投资接口
pub async fn create(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    ValidatedJson(req): ValidatedJson<CreateOrderReq>,
) -> Result<impl IntoResponse, AppError> {
    let res = OrderService::create_order(&state.pool, user_id, req).await?;
    Ok(ApiResponse::success(res))
}
