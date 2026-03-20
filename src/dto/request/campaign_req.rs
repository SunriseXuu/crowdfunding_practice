use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCampaignReq {
    #[validate(length(min = 2, max = 100, message = "标题长度需在 2-100 之间"))]
    pub title: String,

    #[validate(length(min = 10, message = "描述内容不得少于 10 个字符"))]
    pub description: String,

    #[validate(range(min = 100, message = "目标金额不能低于 100 分 (即 1 元)"))]
    pub goal_amount: i64,

    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCampaignReq {
    #[validate(length(min = 2, max = 100, message = "标题长度需在 2-100 之间"))]
    pub title: Option<String>,

    pub description: Option<String>,

    #[validate(range(min = 100, message = "目标金额不能低于 100 分"))]
    pub goal_amount: Option<i64>,

    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
}
