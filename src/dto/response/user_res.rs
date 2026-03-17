use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::model::User;

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

/// 用户信息响应体
#[derive(Debug, Serialize)]
pub struct UserRes {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 实现从 User Model 到 UserRes DTO 的转换
///
/// 我们在 Service 层查出 User 模型后，通过 `.into()` 即可轻松完成脱敏。
impl From<User> for UserRes {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
