use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, ToSchema)]
#[sqlx(type_name = "campaign_status")]
pub enum CampaignStatus {
    Pending,
    Active,
    Funded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Campaign {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub description: String,
    pub goal_amount: i64,
    pub current_amount: i64,
    pub status: CampaignStatus,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
