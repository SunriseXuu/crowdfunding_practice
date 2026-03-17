//! 全局统一错误处理枚举
//!
//! 所有 Service / Repository 层抛出的错误都必须收敛到 `AppError`。
//! 通过为 `AppError` 实现 Axum 的 `IntoResponse` trait，
//! 我们可以在 Handler 中直接用 `?` 操作符向上传播错误，
//! Axum 会自动调用 `IntoResponse` 将错误转化成 HTTP 响应返回给前端。
//!
//! 这等价于前端里在 Axios 拦截器中统一处理 Error 并弹 Toast 的思路，
//! 只不过这里是在后端统一兜底、统一输出格式。

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::dto::ApiResponse;

/// `AppError` 是整个应用唯一的错误类型。
///
/// 使用 `#[derive(Error)]`（来自 thiserror 库）后，
/// Rust 编译器会自动帮我们实现 `std::fmt::Display` 和 `std::error::Error` 这两个 trait。
/// 每一个 `#[error("...")]` 注解就是这个错误变体被打印时的文案模板。
///
/// 你可以把每个变体理解为前端里 `throw new BadRequestError("xxx")` 的不同子类。
#[derive(Debug, Error)]
pub enum AppError {
    // ── 客户端错误 (4xx) ──────────────────────────────────────────────────
    /// 请求参数不合法（如字段缺失、格式错误、校验不通过）
    /// 对应 HTTP 400
    #[error("Bad Request: {0}")]
    BadRequest(String),

    /// 未登录或 Token 无效/过期
    /// 对应 HTTP 401
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 已登录但没有权限访问该资源（如普通用户试图访问管理员接口）
    /// 对应 HTTP 403
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 请求的资源不存在（如查询一个不存在的众筹项目 ID）
    /// 对应 HTTP 404
    #[error("Not Found: {0}")]
    NotFound(String),

    /// 业务冲突（如邮箱已被注册、众筹金额已满、重复投资等）
    /// 对应 HTTP 409
    #[error("Conflict: {0}")]
    Conflict(String),

    // ── 服务端错误 (5xx) ──────────────────────────────────────────────────
    /// 服务器内部意外错误（兜底用，不应该暴露具体细节给前端）
    /// 对应 HTTP 500
    #[error("Internal Server Error: {0}")]
    Internal(String),
}

/// 为 `AppError` 实现 Axum 的 `IntoResponse` trait。
///
/// 这是整个错误处理机制的核心粘合剂：
/// 当你在 Handler 里写 `some_function().await?` 时，如果出错，
/// Axum 会调用这个方法，把 `AppError` 自动转化成一个合法的 HTTP 响应。
///
/// 返回格式统一为：
/// ```json
/// {
///     "code": 40001,
///     "success": false,
///     "data": null,
///     "msg": "Bad Request: 邮箱格式不正确"
/// }
/// ```
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 根据错误变体，映射到对应的 HTTP 状态码和业务 code
        let (http_status, business_code) = match &self {
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, 40000),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, 40100),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, 40300),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, 40400),
            AppError::Conflict(_) => (StatusCode::CONFLICT, 40900),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50000),
        };

        // 在服务端日志中记录错误详情（前端看不到，方便我们排查问题）
        tracing::error!("AppError occurred: {}", &self);

        // 使用 ApiResponse 结构体构造错误响应
        let body = ApiResponse::error(business_code, self.to_string());

        (http_status, axum::Json(body)).into_response()
    }
}

// ── 自动转换：让其他错误类型能自动变成 AppError ──────────────────────────

/// 数据库错误 → AppError::Internal
/// 当 Repository 层的 sqlx 查询出错时，`?` 操作符会自动触发这个转换。
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        // 这里故意不把 sqlx 的原始错误信息返回给前端（防止 SQL 泄漏）
        // 但在服务端日志里完整记录，方便排查
        tracing::error!("Database error: {:?}", err);
        AppError::Internal("数据库操作异常，请稍后重试".to_string())
    }
}

/// 参数校验错误 → AppError::BadRequest
/// 当 ValidatedJson 提取器校验失败时，`?` 会自动触发这个转换。
impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        // 将校验错误转成可读的字符串返回给前端，方便前端定位哪个字段不合法
        AppError::BadRequest(format!("参数校验失败: {}", err))
    }
}

/// JWT 错误 → AppError::Unauthorized
/// 当鉴权中间件解析 Token 失败时，`?` 会自动触发这个转换。
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        tracing::error!("JWT error: {:?}", err);
        AppError::Unauthorized("Token 无效或已过期，请重新登录".to_string())
    }
}
