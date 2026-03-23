use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::request::order_req::CreateOrderReq;
use crate::dto::response::order_res::OrderRes;
use crate::error::AppError;
use crate::model::CampaignStatus;
use crate::repository::{CampaignRepo, OrderRepo};

pub struct OrderService;

impl OrderService {
    /// 创建订单业务逻辑（含严谨事务与行级锁）
    pub async fn create_order(
        pool: &PgPool,
        user_id: Uuid,
        req: CreateOrderReq,
    ) -> Result<OrderRes, AppError> {
        // 1. 开启事务
        let mut tx = pool.begin().await?;

        // 2. 锁定并查询众筹项目情况 (SELECT FOR UPDATE)
        let campaign = CampaignRepo::find_by_id_for_update(&mut tx, req.campaign_id)
            .await?
            .ok_or_else(|| AppError::NotFound("目标众筹项目不存在".to_string()))?;

        // 3. 业务校验
        // 3.1 检查项目状态是否为 Active
        if campaign.status != CampaignStatus::Active {
            return Err(AppError::BadRequest(
                "当前项目不在众筹期，无法平摊投资".to_string(),
            ));
        }

        // 3.2 检查是否已经结束
        if campaign.end_at < chrono::Utc::now() {
            return Err(AppError::BadRequest("众筹活动已经结束".to_string()));
        }

        // 4. 执行更新
        // 4.1 更新众筹项目累积金额
        // 注意：这里由于上面已经加了 FOR UPDATE 锁，我们可以安全地在 Rust 中计算后写回，
        // 或者直接用 SQL 叠加。为了严谨，我们直接用 SQL 叠加。
        sqlx::query!(
            r#"
                UPDATE campaigns
                SET current_amount = current_amount + $1,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $2
            "#,
            req.amount,
            req.campaign_id
        )
        .execute(&mut *tx)
        .await?;

        // 4.2 创建订单流水
        let order = OrderRepo::create(&mut tx, user_id, &req).await?;

        // 5. 提交事务
        tx.commit().await?;

        Ok(OrderRes::from(order))
    }
}
