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

use crate::{AppState, error::AppError, util::jwt::verify_token};

/// JWT 鉴权提取器 (Extractor)
///
/// 任何需要登录才能访问的接口，只需要在参数列表中加上 `AuthenticatedUser(user_id)` 即可。
/// Axum 会在进入业务函数之前，自动执行这里的 `from_request_parts` 逻辑。
pub struct AuthenticatedUser(pub Uuid);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    Arc<AppState>: axum::extract::FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 1. 从请求头中提取 Authorization: Bearer <token>
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized("缺少认证 Token 或格式错误".to_string()))?;

        // 2. 正确地从 Axum 状态中提取 AppState
        let State(app_state) = parts
            .extract_with_state::<State<Arc<AppState>>, _>(state)
            .await
            .map_err(|_| AppError::Internal("无法获取应用配置状态".into()))?;

        let secret = &app_state.config.jwt_access_secret;

        // 3. 验证 Token (这步调用了我们第一步写的纯函数)
        let claims = verify_token(bearer.token(), secret)?;

        // 4. 验证通过，把解析出的用户 ID 包裹在 AuthenticatedUser 中传递给下游 Handler
        Ok(Self(claims.sub))
    }
}
