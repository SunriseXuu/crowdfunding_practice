use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 订单状态枚举
#[derive(
    Debug, Serialize, Deserialize, sqlx::Type, utoipa::ToSchema, PartialEq, Eq, Clone, Copy,
)]
#[sqlx(type_name = "order_status")]
pub enum OrderStatus {
    Paid,
    Refunded,
}

/// 订单模型
#[derive(Debug, Serialize, Deserialize, FromRow, utoipa::ToSchema)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub campaign_id: Uuid,
    pub amount: i64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
