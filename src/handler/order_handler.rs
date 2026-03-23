use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{
    AppState,
    dto::ApiResponse,
    dto::request::order_req::{CreateOrderReq, OrderQueryReq},
    error::AppError,
    extractor::{AuthenticatedUser, ValidatedJson},
    service::OrderService,
    util::PageParams,
};

/// 投资一个众筹项目创建订单接口
pub async fn create(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    ValidatedJson(req): ValidatedJson<CreateOrderReq>,
) -> Result<impl IntoResponse, AppError> {
    let res = OrderService::create_order(&state.pool, user_id, req).await?;
    Ok(ApiResponse::success(res))
}

/// 获取个人订单列表接口
pub async fn list_me(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<OrderQueryReq>,
    Query(page): Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let res = OrderService::list_me(&state.pool, user_id, query, page).await?;
    Ok(ApiResponse::success(res))
}
