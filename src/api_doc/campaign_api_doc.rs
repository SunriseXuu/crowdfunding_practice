use crate::{
    dto::request::{CampaignQueryReq, CreateCampaignReq, UpdateCampaignReq},
    dto::response::{ApiResponse, CampaignRes, NoData},
    util::{PageParams, PagedRes},
};

#[utoipa::path(
    post,
    path = "/api/v1/campaigns",
    request_body = CreateCampaignReq,
    responses(
        (status = 200, description = "众筹项目创建成功", body = ApiResponse<CampaignRes>)
    ),
    security(("jwt" = [])),
    tag = "Campaign"
)]
pub fn create_campaign() {}

#[utoipa::path(
    get,
    path = "/api/v1/campaigns",
    params(
        PageParams,
        CampaignQueryReq
    ),
    responses(
        (status = 200, description = "获取众筹项目列表成功", body = ApiResponse<PagedRes<CampaignRes>>)
    ),
    tag = "Campaign"
)]
pub fn list_campaigns() {}

#[utoipa::path(
    get,
    path = "/api/v1/campaigns/{id}",
    params(
        ("id" = Uuid, Path, description = "众筹项目的 UUID")
    ),
    responses(
        (status = 200, description = "获取众筹项目详情成功", body = ApiResponse<CampaignRes>),
        (status = 404, description = "众筹项目未找到", body = ApiResponse<NoData>)
    ),
    tag = "Campaign"
)]
pub fn retrieve_campaign() {}

#[utoipa::path(
    put,
    path = "/api/v1/campaigns/{id}",
    params(
        ("id" = Uuid, Path, description = "众筹项目的 UUID")
    ),
    request_body = UpdateCampaignReq,
    responses(
        (status = 200, description = "众筹项目更新成功", body = ApiResponse<CampaignRes>),
        (status = 403, description = "无权修改他人众筹项目", body = ApiResponse<NoData>),
        (status = 400, description = "众筹项目状态不允许修改", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "Campaign"
)]
pub fn update_campaign() {}

#[utoipa::path(
    delete,
    path = "/api/v1/campaigns/{id}",
    params(
        ("id" = Uuid, Path, description = "众筹项目的 UUID")
    ),
    responses(
        (status = 200, description = "众筹项目取消成功", body = ApiResponse<CampaignRes>),
        (status = 403, description = "无权操作他人众筹项目", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "Campaign"
)]
pub fn cancel_campaign() {}
