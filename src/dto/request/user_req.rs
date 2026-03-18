use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::model::Gender;

/// 更新用户信息请求体
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserReq {
    /// 可选的用户名更新
    #[validate(length(min = 2, max = 50, message = "用户名长度需在2-50位之间"))]
    pub username: Option<String>,

    /// 年龄（可选）
    #[validate(range(min = 1, max = 150, message = "年龄必须在 1 到 150 之间"))]
    pub age: Option<i32>,

    /// 性别（可选）
    pub gender: Option<Gender>,
}

/// 修改密码请求体
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePasswordReq {
    /// 旧密码（不需校验长度，直接传给后端比对）
    pub old_password: String,

    /// 新密码
    #[validate(length(min = 6, message = "新密码不能少于6位"))]
    pub new_password: String,
}
