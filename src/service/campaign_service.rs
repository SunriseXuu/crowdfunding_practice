use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::request::campaign_req::{CreateCampaignReq, UpdateCampaignReq};
use crate::dto::response::campaign_res::CampaignRes;
use crate::error::AppError;
use crate::model::CampaignStatus;
use crate::repository::CampaignRepo;

pub struct CampaignService;

impl CampaignService {
    /// 创建众筹项目业务
    pub async fn create(
        pool: &PgPool,
        creator_id: Uuid,
        req: CreateCampaignReq,
    ) -> Result<CampaignRes, AppError> {
        // 业务校验：结束时间必须晚于开始时间
        if req.end_at <= req.start_at {
            return Err(AppError::BadRequest("结束时间必须晚于开始时间".to_string()));
        }

        let campaign = CampaignRepo::create(pool, creator_id, req).await?;
        Ok(CampaignRes::from(campaign))
    }

    /// 获取活跃的众筹项目列表业务
    pub async fn list_active(pool: &PgPool) -> Result<Vec<CampaignRes>, AppError> {
        let campaigns = CampaignRepo::list_active(pool).await?;
        Ok(campaigns.into_iter().map(CampaignRes::from).collect())
    }

    /// 获取一个众筹项目业务
    pub async fn retrieve(pool: &PgPool, id: Uuid) -> Result<CampaignRes, AppError> {
        let campaign = CampaignRepo::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;

        Ok(CampaignRes::from(campaign))
    }

    /// 更新一个众筹项目业务
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        req: UpdateCampaignReq,
    ) -> Result<CampaignRes, AppError> {
        // 1. 获取原项目并校验权限
        let original = CampaignRepo::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;
        if original.creator_id != user_id {
            return Err(AppError::Forbidden("无权修改他人的项目".to_string()));
        }

        // 2. 状态校验：只有 Pending 状态可以修改核心信息（此处可根据需求调整）
        if original.status != CampaignStatus::Pending {
            return Err(AppError::BadRequest("只有待发布项目可以修改".to_string()));
        }

        // 3. 执行更新
        let updated = CampaignRepo::update(pool, id, req).await?;
        Ok(CampaignRes::from(updated))
    }

    /// 下架一个众筹项目业务
    pub async fn offline(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<CampaignRes, AppError> {
        // 1. 获取原项目并校验权限
        let original = CampaignRepo::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;
        if original.creator_id != user_id {
            return Err(AppError::Forbidden("无权操作他人的项目".to_string()));
        }

        // 2. 状态校验：只有 Active 状态可以取消
        if original.status != CampaignStatus::Active {
            return Err(AppError::BadRequest("只有活跃的项目可以取消".to_string()));
        }

        // 3. 执行取消
        let cancelled = CampaignRepo::update_status(pool, id, CampaignStatus::Cancelled).await?;
        Ok(CampaignRes::from(cancelled))
    }
}
