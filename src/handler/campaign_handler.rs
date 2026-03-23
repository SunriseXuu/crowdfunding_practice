use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    dto::{
        ApiResponse,
        request::campaign_req::{CampaignQueryReq, CreateCampaignReq, UpdateCampaignReq},
    },
    error::AppError,
    extractor::{AuthenticatedUser, ValidatedJson},
    service::CampaignService,
    util::pagination::PageParams,
};

/// 创建众筹项目接口
pub async fn create(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    ValidatedJson(req): ValidatedJson<CreateCampaignReq>,
) -> Result<impl IntoResponse, AppError> {
    let res = CampaignService::create(&state.pool, user_id, req).await?;
    Ok(ApiResponse::success(res))
}

/// 获取众筹项目列表接口
pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CampaignQueryReq>,
    Query(page_params): Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let res = CampaignService::list(&state.pool, query, page_params).await?;
    Ok(ApiResponse::success(res))
}

/// 获取一个众筹项目接口
pub async fn retrieve(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let res = CampaignService::retrieve(&state.pool, id).await?;
    Ok(ApiResponse::success(res))
}

/// 更新一个众筹项目接口
pub async fn update(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(id): Path<Uuid>,
    ValidatedJson(req): ValidatedJson<UpdateCampaignReq>,
) -> Result<impl IntoResponse, AppError> {
    let res = CampaignService::update(&state.pool, id, user_id, req).await?;
    Ok(ApiResponse::success(res))
}

/// 取消一个众筹项目接口
pub async fn cancel(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let res = CampaignService::cancel(&state.pool, id, user_id).await?;
    Ok(ApiResponse::success(res))
}
