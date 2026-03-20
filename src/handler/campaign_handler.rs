use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::dto::request::campaign_req::{CreateCampaignReq, UpdateCampaignReq};
use crate::error::AppError;
use crate::extractor::{AuthenticatedUser, ValidatedJson};
use crate::service::CampaignService;

/// 创建众筹项目接口
pub async fn create(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    ValidatedJson(req): ValidatedJson<CreateCampaignReq>,
) -> Result<impl IntoResponse, AppError> {
    let res = CampaignService::create(&state.pool, user_id, req).await?;
    Ok(ApiResponse::success(res))
}

/// 获取活跃的众筹列表接口
pub async fn list_active(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let res = CampaignService::list_active(&state.pool).await?;
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
