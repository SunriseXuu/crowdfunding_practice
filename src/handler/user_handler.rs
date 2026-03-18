use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::{UpdatePasswordReq, UpdateUserReq};
use crate::error::AppError;
use crate::extractor::{AuthenticatedUser, ValidatedJson};
use crate::service::UserService;

/// 获取当前用户信息接口
pub async fn get_me(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let user_res = UserService::get_me(&state.pool, user_id).await?;
    Ok(ApiResponse::success(user_res))
}

/// 更新用户信息接口
pub async fn update(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    ValidatedJson(req): ValidatedJson<UpdateUserReq>,
) -> Result<impl IntoResponse, AppError> {
    let user_res = UserService::update(&state.pool, user_id, req).await?;
    Ok(ApiResponse::success(user_res))
}

/// 修改密码接口
pub async fn update_password(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    ValidatedJson(req): ValidatedJson<UpdatePasswordReq>,
) -> Result<impl IntoResponse, AppError> {
    UserService::update_password(&state.pool, user_id, req).await?;
    Ok(ApiResponse::success_without_data())
}

/// 软删除账号接口
pub async fn soft_delete(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    UserService::soft_delete(&state.pool, user_id).await?;
    Ok(ApiResponse::success_without_data())
}
