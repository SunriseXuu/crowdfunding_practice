use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::model::Gender;

/// 注册请求体
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterReq {
    /// 邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    /// 密码
    #[validate(length(min = 6, message = "密码不能少于6位"))]
    pub password: String,

    /// 用户名
    #[validate(length(min = 2, max = 50, message = "用户名长度需在2-50位之间"))]
    pub username: String,

    /// 年龄（可选）
    #[validate(range(min = 1, max = 150, message = "年龄必须在 1 到 150 之间"))]
    pub age: Option<i32>,

    /// 性别（可选），由 Serde 自动处理枚举映射
    pub gender: Option<Gender>,
}

/// 登录请求体
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginReq {
    /// 邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    /// 密码（不需校验长度，直接传给后端比对）
    pub password: String,
}
