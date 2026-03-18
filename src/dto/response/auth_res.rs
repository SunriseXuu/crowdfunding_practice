use serde::Serialize;

use crate::dto::response::user_res::UserRes;

/// 登录成功附带的 Token 凭证
#[derive(Debug, Serialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

/// 登录成功响应体
#[derive(Debug, Serialize)]
pub struct LoginRes {
    pub user: UserRes,
    pub tokens: AuthTokens,
}
