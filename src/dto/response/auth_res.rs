use serde::Serialize;
use utoipa::ToSchema;

use crate::dto::response::user_res::UserRes;

/// 登录成功附带的 Token
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthTokensRes {
    pub access_token: String,
    pub refresh_token: String,
}

/// 登录成功响应体
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginRes {
    pub user: UserRes,
    pub tokens: AuthTokensRes,
}
