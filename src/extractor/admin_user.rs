use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, State},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::model::user_model::Role;
use crate::{AppState, error::AppError, util::jwt::verify_token};

/// 管理员鉴权提取器 (Extractor)
///
/// 只有 JWT 解析出的角色为 `Admin` 的用户才能通过。
pub struct AdminUser(pub Uuid);

impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
    Arc<AppState>: axum::extract::FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized("缺少认证 Token 或格式错误".to_string()))?;

        let State(app_state) = parts
            .extract_with_state::<State<Arc<AppState>>, _>(state)
            .await
            .map_err(|_| AppError::Internal("无法获取应用配置状态".into()))?;

        let secret = &app_state.config.jwt_access_secret;

        let claims = verify_token(bearer.token(), secret)?;

        if claims.role != Role::Admin {
            return Err(AppError::Forbidden(
                "权限不足：仅管理员可执行该操作".to_string(),
            ));
        }

        Ok(Self(claims.sub))
    }
}
