//! 自定义 Axum 提取器：反序列化 JSON + 自动校验
//!
//! 在 Axum 中，"提取器（Extractor）" 就是 Handler 函数的参数。
//! 比如 `Json<T>` 就是一个内置提取器，它负责从请求体中反序列化 JSON。
//!
//! 但 `Json<T>` 只做反序列化，不做数据校验。
//! 我们封装的 `ValidatedJson<T>` 在 `Json<T>` 的基础上，
//! 额外调用了 `validator::Validate::validate()` 来检查标注在结构体字段上的校验规则。
//!
//! 如果校验不通过，请求不会进入 Handler，直接返回 `AppError::BadRequest`。

use axum::extract::FromRequest;
use axum::extract::rejection::JsonRejection;
use axum::http::Request;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

/// 自定义提取器：JSON 反序列化 + 数据校验
///
/// 用法示例（在 Handler 的参数中使用）：
/// ```rust
/// async fn register(
///     ValidatedJson(req): ValidatedJson<RegisterReq>,
/// ) -> Result<impl IntoResponse, AppError> {
///     // 走到这里时，req 已经通过了所有校验规则
///     // 如果校验失败，根本不会进到这个函数体
/// }
/// ```
///
/// 其中 `RegisterReq` 需要同时 derive `Deserialize`（用于 JSON 解析）
/// 和 `Validate`（用于规则校验）：
/// ```rust
/// #[derive(Deserialize, Validate)]
/// pub struct RegisterReq {
///     #[validate(email(message = "邮箱格式不正确"))]
///     pub email: String,
///
///     #[validate(length(min = 6, message = "密码不能少于6位"))]
///     pub password: String,
/// }
/// ```
pub struct ValidatedJson<T>(pub T);

/// 为 `ValidatedJson<T>` 实现 Axum 的 `FromRequest` trait。
///
/// 这个 trait 告诉 Axum："当 Handler 参数类型是 `ValidatedJson<T>` 时，
/// 该怎么从 HTTP 请求中提取并构造出这个类型"。
///
/// 执行顺序：
/// 1. 先用 Axum 内置的 `Json<T>` 反序列化请求体 → 失败则返回 BadRequest
/// 2. 再调用 `validate()` 校验结构体上的规则 → 失败则返回 BadRequest
/// 3. 两步都通过，才把数据交给 Handler
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    // T 必须能被反序列化（Deserialize）且能被校验（Validate）
    T: DeserializeOwned + Validate,
    // S 是 Axum 的 State 类型，Send + Sync 是多线程安全约束
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 第一步：尝试从请求体中反序列化 JSON
        let json_result: Result<axum::Json<T>, JsonRejection> =
            axum::Json::from_request(req, state).await;

        match json_result {
            Ok(axum::Json(data)) => {
                // 第二步：对反序列化后的数据执行校验规则
                // 这里的 `?` 会自动触发 From<ValidationErrors> -> AppError::BadRequest
                data.validate()?;

                // 两步都通过，返回提取成功的数据
                Ok(Self(data))
            }
            Err(rejection) => {
                // JSON 解析失败（如格式错误、字段缺失、类型不匹配等）
                Err(AppError::BadRequest(format!(
                    "请求体解析失败: {}",
                    rejection
                )))
            }
        }
    }
}
