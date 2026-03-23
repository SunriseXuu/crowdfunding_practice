use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    dto::{
        request::{CreateOrderReq, OrderQueryReq},
        response::MyOrderRes,
    },
    error::AppError,
    model::Order,
    util::{PageParams, PagedRes},
};

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

    /// 获取个人订单列表（连表查询 campaign 标题和状态）
    pub async fn list_me(
        pool: &sqlx::PgPool,
        user_id: Uuid,
        query: &OrderQueryReq,
        page_params: &PageParams,
    ) -> Result<PagedRes<MyOrderRes>, AppError> {
        // 构建带有联合查询的 SQL
        let mut qb = sqlx::QueryBuilder::new(
            r#"
                SELECT 
                    o.id, 
                    o.campaign_id, 
                    c.title as campaign_title, 
                    c.status as "campaign_status: crate::model::CampaignStatus",
                    o.amount, 
                    o.status as "status: crate::model::OrderStatus", 
                    o.created_at
                FROM orders o
                INNER JOIN campaigns c ON o.campaign_id = c.id
                WHERE o.user_id = 
            "#,
        );
        qb.push_bind(user_id);

        let mut count_qb =
            sqlx::QueryBuilder::new("SELECT COUNT(o.id) FROM orders o WHERE o.user_id = ");
        count_qb.push_bind(user_id);

        // 处理查询参数
        if let Some(status) = &query.status {
            qb.push(" AND o.status = ");
            qb.push_bind(status);
            count_qb.push(" AND o.status = ");
            count_qb.push_bind(status);
        }

        if let Some(campaign_id) = &query.campaign_id {
            qb.push(" AND o.campaign_id = ");
            qb.push_bind(campaign_id);
            count_qb.push(" AND o.campaign_id = ");
            count_qb.push_bind(campaign_id);
        }

        // 查总量
        let total: i64 = count_qb.build_query_scalar().fetch_one(pool).await?;

        // 分页
        qb.push(" ORDER BY o.created_at DESC LIMIT ");
        qb.push_bind(page_params.limit());
        qb.push(" OFFSET ");
        qb.push_bind(page_params.offset());

        // 使用 as! 宏强转类型
        let items = qb.build_query_as::<MyOrderRes>().fetch_all(pool).await?;

        Ok(PagedRes::new(
            items,
            total,
            page_params.page,
            page_params.size,
        ))
    }

    /// 将某个众筹项目的所有已支付订单批量设为退款状态
    pub async fn refund_by_campaign(
        pool: &sqlx::PgPool,
        campaign_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
                UPDATE orders 
                SET status = 'Refunded' 
                WHERE campaign_id = $1 AND status = 'Paid'
            "#,
            campaign_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
