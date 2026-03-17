use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::{ChangePasswordReq, LoginReq, RegisterReq, UpdateUserReq};
use crate::error::AppError;
use crate::extractor::ValidatedJson;
use crate::service::UserService;

/// 用户注册接口
pub async fn register(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<RegisterReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::register(&state.pool, req).await?;
    Ok(ApiResponse::success(user))
}

/// 用户登录接口
pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::login(&state.pool, req).await?;
    Ok(ApiResponse::success(user))
}

/// 更新用户信息接口
///
/// 注意：目前尚未对接 JWT 中间件，暂时无法获取当前登录用户 ID。
/// 这里的 user_id 逻辑留待下一阶段对接权限时完善。
pub async fn update(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<UpdateUserReq>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 从 Auth 中间件获取真实的 UserID
    let mock_id = uuid::Uuid::nil();
    let user = UserService::update(&state.pool, mock_id, req).await?;
    Ok(ApiResponse::success(user))
}

/// 修改密码接口
pub async fn update_password(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<ChangePasswordReq>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 从 Auth 中间件获取真实的 UserID
    let mock_id = uuid::Uuid::nil();
    UserService::update_password(&state.pool, mock_id, req).await?;
    Ok(ApiResponse::success_without_data())
}

/// 软删除账号接口
pub async fn soft_delete(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 从 Auth 中间件获取真实的 UserID
    let mock_id = uuid::Uuid::nil();
    UserService::soft_delete(&state.pool, mock_id).await?;
    Ok(ApiResponse::success_without_data())
}
