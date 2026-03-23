use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::model::OrderStatus;

/// 创建订单请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderReq {
    /// 众筹项目 ID
    pub campaign_id: Uuid,

    /// 投资金额（单位：分）
    #[validate(range(min = 1, message = "投资金额必须大于 0"))]
    pub amount: i64,
}

/// 订单分页查询参数
#[derive(Debug, Deserialize, IntoParams)]
pub struct OrderQueryReq {
    /// 过滤特定状态(Paid, Refunded等)
    pub status: Option<OrderStatus>,

    /// 按特定项目过滤
    pub campaign_id: Option<Uuid>,
}
