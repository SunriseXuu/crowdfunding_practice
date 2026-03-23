use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    model::CampaignStatus,
    repository::{CampaignRepo, UserRepo},
};

/// 管理员业务逻辑层（Admin Service）
pub struct AdminService;

impl AdminService {
    /// 强制封禁一个用户业务
    pub async fn ban_user(pool: &PgPool, target_id: Uuid) -> Result<(), AppError> {
        let affected = UserRepo::ban(pool, target_id).await?;
        if affected == 0 {
            return Err(AppError::NotFound("目标用户不存在或已被封禁".to_string()));
        }

        Ok(())
    }

    /// 强制取消一个众筹项目业务
    pub async fn cancel_campaign(pool: &PgPool, campaign_id: Uuid) -> Result<(), AppError> {
        // 直接调用 Repo 强制切换状态为 Cancelled
        // 注意：这里后续可以增加记录取消原因的逻辑
        CampaignRepo::update_status(pool, campaign_id, CampaignStatus::Cancelled).await?;
        Ok(())
    }
}
