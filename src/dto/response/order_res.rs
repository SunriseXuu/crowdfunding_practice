use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::model::Order;
use crate::model::order_model::OrderStatus;

/// 订单响应结构
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OrderRes {
    pub id: Uuid,
    pub user_id: Uuid,
    pub campaign_id: Uuid,
    pub amount: i64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

impl From<Order> for OrderRes {
    fn from(order: Order) -> Self {
        Self {
            id: order.id,
            user_id: order.user_id,
            campaign_id: order.campaign_id,
            amount: order.amount,
            status: order.status,
            created_at: order.created_at,
        }
    }
}
