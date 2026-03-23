use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::request::campaign_req::{CreateCampaignReq, UpdateCampaignReq};
use crate::error::AppError;
use crate::model::{Campaign, CampaignStatus};

pub struct CampaignRepo;

impl CampaignRepo {
    /// 创建众筹项目数据库操作
    pub async fn create(
        pool: &PgPool,
        creator_id: Uuid,
        req: CreateCampaignReq,
    ) -> Result<Campaign, AppError> {
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
                INSERT INTO campaigns (creator_id, title, description, goal_amount, start_at, end_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, creator_id, title, description, goal_amount, current_amount, 
                        status as "status: CampaignStatus", start_at, end_at, created_at, updated_at
            "#,
            creator_id,
            req.title,
            req.description,
            req.goal_amount,
            req.start_at,
            req.end_at
        )
        .fetch_one(pool)
        .await?;

        Ok(campaign)
    }

    /// 获取活跃的众筹项目列表数据库操作
    pub async fn list_active(pool: &PgPool) -> Result<Vec<Campaign>, AppError> {
        let campaigns = sqlx::query_as!(
            Campaign,
            r#"
                SELECT id, creator_id, title, description, goal_amount, current_amount, 
                    status as "status: CampaignStatus", start_at, end_at, created_at, updated_at
                FROM campaigns
                WHERE status = 'Active'
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(campaigns)
    }

    /// 根据ID获取众筹项目数据库操作
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Campaign>, AppError> {
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
                SELECT id, creator_id, title, description, goal_amount, current_amount, 
                    status as "status: CampaignStatus", start_at, end_at, created_at, updated_at
                FROM campaigns
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(campaign)
    }

    /// 更新众筹项目数据库操作
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        req: UpdateCampaignReq,
    ) -> Result<Campaign, AppError> {
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
                UPDATE campaigns
                SET title = COALESCE($1, title),
                    description = COALESCE($2, description),
                    goal_amount = COALESCE($3, goal_amount),
                    start_at = COALESCE($4, start_at),
                    end_at = COALESCE($5, end_at),
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $6
                RETURNING id, creator_id, title, description, goal_amount, current_amount, 
                        status as "status: CampaignStatus", start_at, end_at, created_at, updated_at
            "#,
            req.title,
            req.description,
            req.goal_amount,
            req.start_at,
            req.end_at,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(campaign)
    }

    /// 根据ID获取众筹项目并加行级锁（用于事务中的并发控制）
    pub async fn find_by_id_for_update(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        id: Uuid,
    ) -> Result<Option<Campaign>, AppError> {
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
                SELECT id, creator_id, title, description, goal_amount, current_amount, 
                    status as "status: CampaignStatus", start_at, end_at, created_at, updated_at
                FROM campaigns
                WHERE id = $1
                FOR UPDATE
            "#,
            id
        )
        .fetch_optional(&mut **tx)
        .await?;

        Ok(campaign)
    }

    /// 更新众筹项目状态数据库操作
    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        status: CampaignStatus,
    ) -> Result<Campaign, AppError> {
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
                UPDATE campaigns
                SET status = $1,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $2
                RETURNING id, creator_id, title, description, goal_amount, current_amount, 
                        status as "status: CampaignStatus", start_at, end_at, created_at, updated_at
            "#,
            status as CampaignStatus,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(campaign)
    }
}
