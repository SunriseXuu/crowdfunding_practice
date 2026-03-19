use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::repository::UserRepo;

/// 管理员业务逻辑层（Admin Service）
pub struct AdminService;

impl AdminService {
    /// 封禁用户业务
    pub async fn ban_user(pool: &PgPool, target_id: Uuid) -> Result<(), AppError> {
        let affected = UserRepo::ban(pool, target_id).await?;
        if affected == 0 {
            return Err(AppError::NotFound("目标用户不存在或已被封禁".to_string()));
        }

        Ok(())
    }
}
