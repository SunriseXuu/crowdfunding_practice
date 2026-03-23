use crate::{
    dto::{
        request::{CreateOrderReq, OrderQueryReq},
        response::{ApiResponse, MyOrderRes, NoData, OrderRes},
    },
    util::{PageParams, PagedRes},
};

#[utoipa::path(
    post,
    path = "/api/v1/orders",
    request_body = CreateOrderReq,
    responses(
        (status = 200, description = "投资成功", body = ApiResponse<OrderRes>),
        (status = 401, description = "未授权", body = ApiResponse<NoData>),
        (status = 400, description = "参数验证失败或项目状态不允许投资", body = ApiResponse<NoData>),
        (status = 404, description = "目标项目不存在", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "Order"
)]
pub fn create_order() {}

#[utoipa::path(
    get,
    path = "/api/v1/orders/me",
    params(
        PageParams,
        OrderQueryReq
    ),
    responses(
        (status = 200, description = "获取个人订单列表成功", body = ApiResponse<PagedRes<MyOrderRes>>)
    ),
    security(("jwt" = [])),
    tag = "Order"
)]
pub fn list_my_orders() {}
