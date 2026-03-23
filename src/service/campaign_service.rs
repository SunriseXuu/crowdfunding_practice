use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::{
        request::campaign_req::{CampaignQueryReq, CreateCampaignReq, UpdateCampaignReq},
        response::campaign_res::CampaignRes,
    },
    error::AppError,
    model::CampaignStatus,
    repository::CampaignRepo,
    util::{PageParams, PagedRes},
};

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

    /// 获取众筹项目列表业务
    pub async fn list(
        pool: &PgPool,
        query: CampaignQueryReq,
        page_params: PageParams,
    ) -> Result<PagedRes<CampaignRes>, AppError> {
        let paged_data = CampaignRepo::list(pool, &query, &page_params).await?;

        let res_items: Vec<CampaignRes> = paged_data
            .items
            .into_iter()
            .map(CampaignRes::from)
            .collect();

        Ok(PagedRes {
            items: res_items,
            total: paged_data.total,
            page: paged_data.page,
            size: paged_data.size,
            has_next: paged_data.has_next,
        })
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

    /// 取消一个众筹项目业务
    pub async fn cancel(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<CampaignRes, AppError> {
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
