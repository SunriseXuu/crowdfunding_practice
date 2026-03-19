use super::*;

#[utoipa::path(
    delete,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = Uuid, Path, description = "目标用户的 UUID")
    ),
    responses(
        (status = 200, description = "管理员封禁账号成功", body = ApiResponse<NoData>),
        (status = 403, description = "权限不足（仅管理员可执行）", body = ApiResponse<NoData>),
        (status = 404, description = "目标用户不存在或已被封禁", body = ApiResponse<NoData>)
    ),
    security(("jwt" = [])),
    tag = "Admin"
)]
pub fn ban_user() {}
