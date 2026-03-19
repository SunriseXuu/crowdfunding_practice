use axum::{extract::State, response::IntoResponse};
use chrono::Duration;
use std::sync::Arc;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::{LoginReq, RegisterReq};
use crate::dto::response::{AuthTokens, LoginRes};
use crate::error::AppError;
use crate::extractor::ValidatedJson;
use crate::service::AuthService;
use crate::util::jwt;

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
    let user = AuthService::login(&state.pool, req).await?;

    // 签发双 Token
    let access_token = jwt::sign_token(
        user.id,
        user.role.clone(),
        &state.config.jwt_access_secret,
        Duration::minutes(15),
    )?;
    let refresh_token = jwt::sign_token(
        user.id,
        user.role.clone(),
        &state.config.jwt_refresh_secret,
        Duration::days(7),
    )?;

    Ok(ApiResponse::success(LoginRes {
        user,
        tokens: AuthTokens {
            access_token,
            refresh_token,
        },
    }))
}
