use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

/// JWT 载荷 (Payload)
///
/// 其中包含了我们需要在无状态请求中传递的信息，
/// 这里只存放了用户 ID 以及 Token 的过期时间。
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户的唯一标识符
    pub sub: Uuid,
    /// 过期时间 (Expiration Time)
    pub exp: usize,
    /// 签发时间 (Issued At)
    pub iat: usize,
}

/// 签发 Token (Access 或 Refresh)
///
/// 传入用户 ID, 密钥以及有效期，返回生成的 JWT 字符串
pub fn sign_token(user_id: Uuid, secret: &str, expiration: Duration) -> Result<String, AppError> {
    let now = Utc::now();
    let expire_at = now + expiration;

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expire_at.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("生成 Token 失败: {}", e)))?;

    Ok(token)
}

/// 验证并解码 Token (Access 或 Refresh)
///
/// 传入 Token 字符串和对应的密钥，如果通过验证，返回解析后的载荷 (Claims)
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    // 如果 token 过期、被篡改，decode 会返回 Err
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("无效或已过期的 Token".to_string()))?;

    Ok(token_data.claims)
}
