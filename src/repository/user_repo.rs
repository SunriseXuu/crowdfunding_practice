use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::model::{Gender, User};

/// 用户仓库层 (Repository)
///
/// 负责最纯粹的数据库增删改查。这里不包含任何业务逻辑，
/// 只负责执行 SQL 并返回 Model。
pub struct UserRepo;

impl UserRepo {
    /// 插入一个新用户数据库操作
    pub async fn create(
        pool: &PgPool,
        email: &str,
        password_hash: &str,
        username: &str,
        age: Option<i32>,
        gender: Option<Gender>,
    ) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (email, password_hash, username, age, gender)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, email, password_hash, username, age, gender as "gender: _", role as "role: _", is_deactivated, is_banned, created_at, updated_at
            "#,
            email,
            password_hash,
            username,
            age,
            gender as _
        )
        .fetch_one(pool)
        .await
    }

    /// 通过 ID 查找一个活跃用户数据库操作
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
                SELECT id, email, password_hash, username, age, gender as "gender: _", role as "role: _", is_deactivated, is_banned, created_at, updated_at
                FROM users
                WHERE id = $1 AND is_deactivated = false AND is_banned = false
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 通过邮箱查找一个活跃用户数据库操作
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
                SELECT id, email, password_hash, username, age, gender as "gender: _", role as "role: _", is_deactivated, is_banned, created_at, updated_at
                FROM users
                WHERE email = $1 AND is_deactivated = false AND is_banned = false
            "#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    /// 更新一个用户的信息（支持局部更新）数据库操作
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        username: Option<String>,
        age: Option<i32>,
        gender: Option<Gender>,
    ) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
                UPDATE users
                SET 
                    username = COALESCE($2::VARCHAR, username),
                    age = COALESCE($3::INT, age),
                    gender = COALESCE($4::gender_enum, gender)
                WHERE id = $1 AND is_deactivated = false AND is_banned = false
                RETURNING id, email, password_hash, username, age, gender as "gender: _", role as "role: _", is_deactivated, is_banned, created_at, updated_at
            "#,
            id,
            username,
            age,
            gender as _
        )
        .fetch_one(pool)
        .await
    }

    /// 更新一个用户的密码哈希数据库操作
    pub async fn update_password(pool: &PgPool, id: Uuid, password_hash: &str) -> Result<u64> {
        let result = sqlx::query!(
            r#"
                UPDATE users
                SET password_hash = $2
                WHERE id = $1 AND is_deactivated = false AND is_banned = false
            "#,
            id,
            password_hash
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// 用户自行注销账号数据库操作：将 is_deactivated 设为 true
    pub async fn deactivate(pool: &PgPool, id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            r#"
                UPDATE users
                SET is_deactivated = true
                WHERE id = $1 AND is_deactivated = false AND is_banned = false
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// 管理员封禁账号数据库操作：将 is_banned 设为 true
    pub async fn ban(pool: &PgPool, id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            r#"
                UPDATE users
                SET is_banned = true
                WHERE id = $1 AND is_deactivated = false AND is_banned = false
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}
