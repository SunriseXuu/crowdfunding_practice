//! 统一 API 响应格式
//!
//! 所有接口的返回值都必须经过 `ApiResponse<T>` 包装后再返回给前端。
//! 这样前端只需要约定一种数据结构，不管请求成功还是失败，
//! 拿到的 JSON 骨架永远是：
//!
//! ```json
//! {
//!     "code": 0,
//!     "success": true,
//!     "data": { ... },
//!     "msg": "ok"
//! }
//! ```

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

/// 空数据实体（完美映射 data: null）
#[derive(Debug, Serialize, ToSchema)]
pub struct NoData;

/// 统一 API 响应结构体
///
/// 泛型参数 `T` 表示 `data` 字段的类型。
/// `T: Serialize` 约束确保 data 能被序列化成 JSON（类似 TS 里的 `T extends Serializable`）。
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    /// 业务状态码（200 = 成功，其他由 AppError 决定）
    pub code: i32,
    /// 请求是否成功
    pub success: bool,
    /// 返回的业务数据（失败时为 null）
    pub data: Option<T>,
    /// 提示信息
    pub msg: String,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应的工厂方法
    ///
    /// 用法示例：
    /// ```rust
    /// ApiResponse::success(user)  // data = Some(user)
    /// ```
    pub fn success(data: T) -> Self {
        Self {
            code: 20000,
            success: true,
            data: Some(data),
            msg: "ok".to_string(),
        }
    }
}

impl ApiResponse<NoData> {
    /// 成功但不需要返回数据的工厂方法
    ///
    /// 用法示例（如删除操作）：
    /// ```rust
    /// ApiResponse::success_without_data()  // data = null
    /// ```
    pub fn success_without_data() -> Self {
        Self {
            code: 20000,
            success: true,
            data: None,
            msg: "ok".to_string(),
        }
    }

    /// 错误响应的工厂方法（供 AppError 的 IntoResponse 调用）
    ///
    /// 用法示例：
    /// ```rust
    /// ApiResponse::error(40000, "参数校验失败")
    /// ```
    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            success: false,
            data: None,
            msg: msg.into(),
        }
    }
}

/// 让 Axum 知道如何把 `ApiResponse<T>` 转成 HTTP 响应
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, axum::Json(self)).into_response()
    }
}
