use super::*;

#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    responses(
        (status = 200, description = "成功获取个人信息", body = ApiResponse<UserRes>),
        (status = 401, description = "未授权或 Token 已过期", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "User"
)]
pub fn get_me() {}

#[utoipa::path(
    put,
    path = "/api/v1/users/profile",
    request_body = UpdateUserReq,
    responses(
        (status = 200, description = "更新成功", body = ApiResponse<UserRes>)
    ),
    security(("jwt" = [])),
    tag = "User"
)]
pub fn update() {}

#[utoipa::path(
    patch,
    path = "/api/v1/users/password",
    request_body = UpdatePasswordReq,
    responses(
        (status = 200, description = "密码修改成功", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "User"
)]
pub fn update_password() {}

#[utoipa::path(
    delete,
    path = "/api/v1/users",
    responses(
        (status = 200, description = "注销成功", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "User"
)]
pub fn soft_delete() {}
