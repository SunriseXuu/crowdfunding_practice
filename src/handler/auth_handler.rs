use axum::{extract::State, response::IntoResponse};
use chrono::Duration;
use std::sync::Arc;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::{LoginReq, RefreshReq, RegisterReq};
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

/// 刷新 Token 接口
///
/// 只要提供有效的 refresh_token，就重新签发一对全新的 access_token 和 refresh_token。
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    ValidatedJson(req): ValidatedJson<RefreshReq>,
) -> Result<impl IntoResponse, AppError> {
    // 1. 校验证明：用 refresh_secret 解密
    let claims = jwt::verify_token(&req.refresh_token, &state.config.jwt_refresh_secret)?;

    // 2. 检查业务态：即便 Token 有效，账号还在吗？被封禁了吗？
    let user = AuthService::refresh(&state.pool, claims.sub).await?;

    // 3. 业务态合法，重新签发全新的两个 Token
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

    // 直接复用 LoginRes 给前端吐数据
    Ok(ApiResponse::success(LoginRes {
        user,
        tokens: AuthTokens {
            access_token,
            refresh_token,
        },
    }))
}
