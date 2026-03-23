use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

/// 创建订单请求
#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateOrderReq {
    /// 众筹项目 ID
    pub campaign_id: Uuid,

    /// 投资金额（单位：分）
    #[validate(range(min = 1, message = "投资金额必须大于 0"))]
    pub amount: i64,
}
