use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::request::{CampaignQueryReq, CreateCampaignReq, UpdateCampaignReq},
    error::AppError,
    model::{Campaign, CampaignStatus},
    util::{PageParams, PagedRes},
};

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

    /// 获取众筹项目列表数据库操作, 支持分页、动态条件查询
    pub async fn list(
        pool: &PgPool,
        query: &CampaignQueryReq,
        page_params: &PageParams,
    ) -> Result<PagedRes<Campaign>, AppError> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"
                SELECT id, creator_id, title, description, goal_amount, current_amount, 
                    status as "status: CampaignStatus", start_at, end_at, created_at, updated_at 
                FROM campaigns WHERE 1=1 
            "#,
        );

        let mut count_qb = sqlx::QueryBuilder::new(
            r#"
                SELECT COUNT(id) FROM campaigns WHERE 1=1 
            "#,
        );

        // 处理动态查询参数 (过滤状态)
        if let Some(status) = &query.status {
            qb.push(" AND status = ");
            qb.push_bind(status);
            count_qb.push(" AND status = ");
            count_qb.push_bind(status);
        }

        // 处理模糊查询 (LIKE)
        if let Some(title) = &query.title {
            qb.push(" AND title ILIKE "); // ILIKE = 大小写不敏感的 LIKE
            qb.push_bind(format!("%{}%", title));
            count_qb.push(" AND title ILIKE ");
            count_qb.push_bind(format!("%{}%", title));
        }

        // 总量查询
        let total: i64 = count_qb.build_query_scalar().fetch_one(pool).await?;

        // 排序与分页
        qb.push(" ORDER BY created_at DESC LIMIT ");
        qb.push_bind(page_params.limit());
        qb.push(" OFFSET ");
        qb.push_bind(page_params.offset());

        // 执行查询（返回强类型）
        let campaigns = qb.build_query_as::<Campaign>().fetch_all(pool).await?;

        Ok(PagedRes::new(
            campaigns,
            total,
            page_params.page,
            page_params.size,
        ))
    }

    /// 查找所有已过期且仍处于 Active 状态的众筹项目
    pub async fn list_expired_active(pool: &PgPool) -> Result<Vec<Campaign>, AppError> {
        let campaigns = sqlx::query_as!(
            Campaign,
            r#"
                SELECT id, creator_id, title, description, goal_amount, current_amount, 
                    status as "status: CampaignStatus", start_at, end_at, created_at, updated_at
                FROM campaigns
                WHERE status = 'Active' AND end_at <= CURRENT_TIMESTAMP
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

    /// 原子的增加众筹金额（防超卖，要求在外部已通过 SELECT FOR UPDATE 锁行）
    pub async fn add_fund(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        campaign_id: Uuid,
        amount: i64,
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
                UPDATE campaigns
                SET current_amount = current_amount + $1,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $2
            "#,
            amount,
            campaign_id
        )
        .execute(&mut **tx)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("未找到对应的众筹项目".to_string()));
        }

        Ok(())
    }
}
