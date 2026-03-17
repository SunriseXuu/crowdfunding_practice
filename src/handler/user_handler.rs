use axum::{extract::State, response::IntoResponse};
use chrono::Duration;
use std::sync::Arc;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::{ChangePasswordReq, LoginReq, RegisterReq, UpdateUserReq};
use crate::dto::response::user_res::{AuthTokens, LoginRes};
use crate::error::AppError;
use crate::extractor::{AuthUser, ValidatedJson};
use crate::service::UserService;
use crate::util::jwt;

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

    // 签发双 Token
    let access_token = jwt::sign_token(
        user.id,
        &state.config.jwt_access_secret,
        Duration::minutes(15),
    )?;
    let refresh_token =
        jwt::sign_token(user.id, &state.config.jwt_refresh_secret, Duration::days(7))?;

    Ok(ApiResponse::success(LoginRes {
        user,
        tokens: AuthTokens {
            access_token,
            refresh_token,
        },
    }))
}

/// 更新用户信息接口
pub async fn update(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    ValidatedJson(req): ValidatedJson<UpdateUserReq>,
) -> Result<impl IntoResponse, AppError> {
    let user_res = UserService::update(&state.pool, user.user_id, req).await?;
    Ok(ApiResponse::success(user_res))
}

/// 修改密码接口
pub async fn update_password(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    ValidatedJson(req): ValidatedJson<ChangePasswordReq>,
) -> Result<impl IntoResponse, AppError> {
    UserService::update_password(&state.pool, user.user_id, req).await?;
    Ok(ApiResponse::success_without_data())
}

/// 软删除账号接口
pub async fn soft_delete(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    UserService::soft_delete(&state.pool, user.user_id).await?;
    Ok(ApiResponse::success_without_data())
}
