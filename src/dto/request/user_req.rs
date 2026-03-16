use serde::Deserialize;
use validator::Validate;

/// 注册用户请求体
#[derive(Debug, Deserialize, Validate)]
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
}

/// 用户登录请求体
#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    /// 邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    /// 密码（不需校验长度，直接传给后端比对）
    pub password: String,
}

/// 更新用户信息请求体
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserReq {
    /// 可选的用户名更新
    #[validate(length(min = 2, max = 50, message = "用户名长度需在2-50位之间"))]
    pub username: Option<String>,
}

/// 修改密码请求体
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordReq {
    /// 旧密码（不需校验长度，直接传给后端比对）
    pub old_password: String,

    /// 新密码
    #[validate(length(min = 6, message = "新密码不能少于6位"))]
    pub new_password: String,
}
