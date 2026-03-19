use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::{LoginReq, RefreshReq, RegisterReq};
use crate::error::AppError;
use crate::extractor::ValidatedJson;
use crate::service::AuthService;

/// 用户注册接口
pub async fn register(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<RegisterReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = AuthService::register(&state.pool, req).await?;
    Ok(ApiResponse::success(user))
}

/// 用户登录接口
pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    let res = AuthService::login(&state.pool, &state.config, req).await?;
    Ok(ApiResponse::success(res))
}

/// 刷新 Token 接口
///
/// 只要提供有效的 refresh_token，就重新签发一对全新的 access_token 和 refresh_token。
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<RefreshReq>,
) -> Result<impl IntoResponse, AppError> {
    let res = AuthService::refresh(&state.pool, &state.config, &req.refresh_token).await?;
    Ok(ApiResponse::success(res))
}
