use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;

use crate::dto::request::{LoginReq, RegisterReq};
use crate::dto::response::UserRes;
use crate::error::AppError;
use crate::repository::UserRepo;

/// 认证业务逻辑层 (Auth Service)
///
/// 负责处理身份认证相关的逻辑，如：
/// - 注册（防止邮箱冲突，哈希密码）
/// - 登录（验证密码，后续签发Token在Handler中完成或在此类中拓展）
pub struct AuthService;

impl AuthService {
    /// 注册用户业务
    pub async fn register(pool: &PgPool, req: RegisterReq) -> Result<UserRes, AppError> {
        // 1. 检查邮箱是否已存在 (业务逻辑判断，非技术错误)
        if UserRepo::find_by_email(pool, &req.email).await?.is_some() {
            return Err(AppError::Conflict("该邮箱已被注册".to_string()));
        }

        // 2. 密码哈希加密 (Argon2)
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("密码加密异常: {}", e)))?
            .to_string();

        // 3. 调用 Repository 存入数据库
        let user = UserRepo::create(
            pool,
            &req.email,
            &password_hash,
            &req.username,
            req.age,
            req.gender,
        )
        .await?;

        Ok(UserRes::from(user))
    }

    /// 登录用户业务
    pub async fn login(pool: &PgPool, req: LoginReq) -> Result<UserRes, AppError> {
        // 1. 按邮箱查找用户
        let user = UserRepo::find_by_email(pool, &req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("邮箱或密码不正确".to_string()))?;

        // 2. 校验密码哈希
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::Internal(format!("密码格式解析异常: {}", e)))?;
        Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized("邮箱或密码不正确".to_string()))?;

        Ok(UserRes::from(user))
    }
}
