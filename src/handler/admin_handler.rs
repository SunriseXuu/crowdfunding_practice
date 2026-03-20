use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use crate::dto::ApiResponse;
use crate::error::AppError;
use crate::extractor::AdminUser;
use crate::service::AdminService;

/// 管理员强制封禁一个账号接口
pub async fn ban_user(
    State(state): State<Arc<AppState>>,
    AdminUser(_admin_id): AdminUser,
    Path(target_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    AdminService::ban_user(&state.pool, target_id).await?;
    Ok(ApiResponse::success_without_data())
}

/// 管理员强制取消一个众筹项目接口
pub async fn cancel_campaign(
    State(state): State<Arc<AppState>>,
    AdminUser(_admin_id): AdminUser,
    Path(campaign_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    AdminService::cancel_campaign(&state.pool, campaign_id).await?;
    Ok(ApiResponse::success_without_data())
}
