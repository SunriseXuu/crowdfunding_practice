use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::{Campaign, CampaignStatus};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CampaignRes {
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

impl From<Campaign> for CampaignRes {
    fn from(c: Campaign) -> Self {
        Self {
            id: c.id,
            creator_id: c.creator_id,
            title: c.title,
            description: c.description,
            goal_amount: c.goal_amount,
            current_amount: c.current_amount,
            status: c.status,
            start_at: c.start_at,
            end_at: c.end_at,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}
