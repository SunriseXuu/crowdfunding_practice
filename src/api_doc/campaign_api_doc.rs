use super::*;

#[utoipa::path(
    post,
    path = "/api/v1/campaigns",
    request_body = CreateCampaignReq,
    responses(
        (status = 200, description = "项目创建成功", body = ApiResponse<CampaignRes>)
    ),
    security(("jwt" = [])),
    tag = "Campaign"
)]
pub fn create_campaign() {}

#[utoipa::path(
    get,
    path = "/api/v1/campaigns",
    responses(
        (status = 200, description = "获取活跃项目列表成功", body = ApiResponse<Vec<CampaignRes>>)
    ),
    tag = "Campaign"
)]
pub fn list_active_campaigns() {}

#[utoipa::path(
    get,
    path = "/api/v1/campaigns/{id}",
    params(
        ("id" = Uuid, Path, description = "项目的 UUID")
    ),
    responses(
        (status = 200, description = "获取项目详情成功", body = ApiResponse<CampaignRes>),
        (status = 404, description = "项目未找到", body = ApiResponse<NoData>)
    ),
    tag = "Campaign"
)]
pub fn retrieve_campaign() {}

#[utoipa::path(
    put,
    path = "/api/v1/campaigns/{id}",
    params(
        ("id" = Uuid, Path, description = "项目的 UUID")
    ),
    request_body = UpdateCampaignReq,
    responses(
        (status = 200, description = "项目更新成功", body = ApiResponse<CampaignRes>),
        (status = 403, description = "无权修改他人项目", body = ApiResponse<NoData>),
        (status = 400, description = "项目状态不允许修改", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "Campaign"
)]
pub fn update_campaign() {}

#[utoipa::path(
    delete,
    path = "/api/v1/campaigns/{id}",
    params(
        ("id" = Uuid, Path, description = "项目的 UUID")
    ),
    responses(
        (status = 200, description = "项目取消成功", body = ApiResponse<CampaignRes>),
        (status = 403, description = "无权操作他人项目", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "Campaign"
)]
pub fn cancel_campaign() {}
