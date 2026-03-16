use crate::model::User;
use sqlx::{PgPool, Result};
use uuid::Uuid;

/// 用户仓库层 (Repository)
///
/// 负责最纯粹的数据库增删改查。这里不包含任何业务逻辑，
/// 只负责执行 SQL 并返回 Model。
pub struct UserRepo;

impl UserRepo {
    /// 插入一个新用户
    pub async fn create(
        pool: &PgPool,
        email: &str,
        password_hash: &str,
        username: &str,
    ) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (email, password_hash, username)
                VALUES ($1, $2, $3)
                RETURNING id, email, password_hash, username, is_active, created_at, updated_at
            "#,
            email,
            password_hash,
            username
        )
        .fetch_one(pool)
        .await
    }

    /// 通过 ID 查找活跃用户
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
                SELECT id, email, password_hash, username, is_active, created_at, updated_at
                FROM users
                WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 通过邮箱查找活跃用户
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
                SELECT id, email, password_hash, username, is_active, created_at, updated_at
                FROM users
                WHERE email = $1 AND is_active = true
            "#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    /// 更新用户名
    pub async fn update(pool: &PgPool, id: Uuid, username: &str) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
                UPDATE users
                SET username = $2
                WHERE id = $1 AND is_active = true
                RETURNING id, email, password_hash, username, is_active, created_at, updated_at
            "#,
            id,
            username
        )
        .fetch_one(pool)
        .await
    }

    /// 更新密码哈希
    pub async fn update_password(pool: &PgPool, id: Uuid, password_hash: &str) -> Result<User> {
        sqlx::query_as!(
            User,
            r#"
                UPDATE users
                SET password_hash = $2
                WHERE id = $1 AND is_active = true
                RETURNING id, email, password_hash, username, is_active, created_at, updated_at
            "#,
            id,
            password_hash
        )
        .fetch_one(pool)
        .await
    }

    /// 软删除：将 is_active 设为 false
    pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            r#"
                UPDATE users
                SET is_active = false
                WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}
