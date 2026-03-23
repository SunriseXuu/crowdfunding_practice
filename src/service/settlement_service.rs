use sqlx::PgPool;
use tracing::{info, warn};

use crate::{
    error::AppError,
    model::CampaignStatus,
    repository::{CampaignRepo, OrderRepo},
};

pub struct SettlementService;

impl SettlementService {
    /// 结算所有已到期的项目
    pub async fn settle_expired_campaigns(pool: &PgPool) -> Result<(), AppError> {
        // 1. 找出所有过期项目
        let expired_campaigns = CampaignRepo::list_expired_active(pool).await?;

        if expired_campaigns.is_empty() {
            return Ok(());
        }

        info!(
            "🔍 [Settlement] Found {} expired campaigns to process",
            expired_campaigns.len()
        );

        for campaign in expired_campaigns {
            let id = campaign.id;
            // 判定是否成功
            let is_success = campaign.current_amount >= campaign.goal_amount;
            let target_status = if is_success {
                CampaignStatus::Funded
            } else {
                CampaignStatus::Failed
            };

            info!(
                "⚖️ [Settlement] Campaign '{}' ({}) - Goal: {}, Current: {} -> Result: {:?}",
                campaign.title, id, campaign.goal_amount, campaign.current_amount, target_status
            );

            // 开启事务进行状态流转
            let mut tx = pool.begin().await?;

            // A. 更新项目状态 (这里我们直接用 sqlx 宏以支持事务)
            sqlx::query!(
                "UPDATE campaigns SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                target_status as CampaignStatus,
                id
            )
            .execute(&mut *tx)
            .await?;

            // B. 如果失败，将订单设为退款
            if !is_success {
                OrderRepo::refund_by_campaign(&mut tx, id).await?;
                warn!(
                    "💸 [Settlement] Campaign failed. Orders from '{}' mark as Refunded",
                    id
                );
            }

            tx.commit().await?;
            info!("✅ [Settlement] Campaign '{}' processed successfully", id);
        }

        Ok(())
    }
}
