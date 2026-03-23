//! 本文件的核心职责是挂载所有的 Swagger UI 纯壳函数定义

#![allow(dead_code)]

pub mod admin_api_doc;
pub mod auth_api_doc;
pub mod campaign_api_doc;
pub mod order_api_doc;
pub mod user_api_doc;

pub use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

pub use crate::{
    dto::{
        request::{
            CreateCampaignReq, CreateOrderReq, LoginReq, RefreshReq, RegisterReq,
            UpdateCampaignReq, UpdatePasswordReq, UpdateUserReq,
        },
        response::{ApiResponse, AuthTokensRes, CampaignRes, LoginRes, NoData, OrderRes, UserRes},
    },
    model::Gender,
    util::pagination::PagedRes,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_api_doc::register_user,
        auth_api_doc::login_user,
        auth_api_doc::refresh_token,
        user_api_doc::retrieve_current_user,
        user_api_doc::update_current_user,
        user_api_doc::update_current_user_password,
        user_api_doc::deactivate_current_user,
        admin_api_doc::ban_user,
        admin_api_doc::cancel_campaign,
        campaign_api_doc::create_campaign,
        campaign_api_doc::list_campaigns,
        campaign_api_doc::retrieve_campaign,
        campaign_api_doc::update_campaign,
        campaign_api_doc::cancel_campaign,
        order_api_doc::create_order,
    ),
    components(
        schemas(
            ApiResponse<NoData>,
            ApiResponse<LoginRes>,
            ApiResponse<UserRes>,
            ApiResponse<CampaignRes>,
            ApiResponse<PagedRes<CampaignRes>>,
            ApiResponse<OrderRes>,
            PagedRes<CampaignRes>,
            AuthTokensRes,
            Gender,
            LoginReq,
            LoginRes,
            NoData,
            RefreshReq,
            RegisterReq,
            UpdatePasswordReq,
            UpdateUserReq,
            CreateCampaignReq,
            UpdateCampaignReq,
            CreateOrderReq,
            CampaignRes,
            OrderRes,
            UserRes,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "认证模块接口"),
        (name = "User", description = "用户管理接口（普通用户）"),
        (name = "Admin", description = "管理员专属接口"),
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
