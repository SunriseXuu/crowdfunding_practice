use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::dto::request::order_req::CreateOrderReq;
use crate::error::AppError;
use crate::model::Order;

pub struct OrderRepo;

impl OrderRepo {
    /// 在事务中创建订单记录
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        req: &CreateOrderReq,
    ) -> Result<Order, AppError> {
        let order = sqlx::query_as!(
            Order,
            r#"
                INSERT INTO orders (user_id, campaign_id, amount)
                VALUES ($1, $2, $3)
                RETURNING id, user_id, campaign_id, amount, status as "status: _", created_at, updated_at
            "#,
            user_id,
            req.campaign_id,
            req.amount
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(order)
    }
}
