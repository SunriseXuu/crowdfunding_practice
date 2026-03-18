use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Type, PartialEq, Eq)]
#[sqlx(type_name = "gender_enum")] // 绑定 Postgres 中对应的枚举名称
pub enum Gender {
    M,
    F,
    O,
}

/// User 数据库模型
///
/// 对应数据库中的 `users` 表。
/// 使用 `sqlx::FromRow` 宏可以让我们直接将 SQL 查询结果映射到这个结构体。
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    /// 用户唯一标识 (UUID v4)
    pub id: Uuid,
    /// 邮箱（登录账号，唯一）
    pub email: String,
    /// 经过 Argon2 哈希加密后的密码
    pub password_hash: String,
    /// 用户昵称
    pub username: String,
    /// 年龄
    pub age: Option<i32>,
    /// 性别
    pub gender: Option<Gender>,
    /// 是否有效（用于软删除，true 为有效，false 为已注销）
    pub is_active: bool,
    /// 注册时间
    pub created_at: DateTime<Utc>,
    /// 最后一次更新时间
    pub updated_at: DateTime<Utc>,
}
