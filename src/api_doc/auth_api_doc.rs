use super::*;

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterReq,
    responses(
        (status = 200, description = "注册成功", body = ApiResponse<NoData>)
    ),
    tag = "Auth"
)]
pub fn register() {}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginReq,
    responses(
        (status = 200, description = "登录成功并下发Token", body = ApiResponse<LoginRes>),
        (status = 401, description = "邮箱或密码错误", body = ApiResponse<NoData>)
    ),
    tag = "Auth"
)]
pub fn login() {}
