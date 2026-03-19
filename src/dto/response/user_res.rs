use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::{Gender, User, user_model::Role};

/// 用户信息响应体
#[derive(Debug, Serialize, ToSchema)]
pub struct UserRes {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub age: Option<i32>,
    pub gender: Option<Gender>,
    pub role: Role,
    pub is_deactivated: bool,
    pub is_banned: bool,
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
            age: user.age,
            gender: user.gender,
            role: user.role,
            is_deactivated: user.is_deactivated,
            is_banned: user.is_banned,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
