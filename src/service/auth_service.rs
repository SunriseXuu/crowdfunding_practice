use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::Duration;
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::dto::request::{LoginReq, RegisterReq};
use crate::dto::response::{AuthTokensRes, LoginRes, UserRes};
use crate::error::AppError;
use crate::repository::UserRepo;
use crate::util::jwt;

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
    pub async fn login(
        pool: &PgPool,
        config: &AppConfig,
        req: LoginReq,
    ) -> Result<LoginRes, AppError> {
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

        // 3. 签发双 Token
        let access_token = jwt::sign_token(
            user.id,
            user.role.clone(),
            &config.jwt_access_secret,
            Duration::minutes(15),
        )?;
        let refresh_token = jwt::sign_token(
            user.id,
            user.role.clone(),
            &config.jwt_refresh_secret,
            Duration::days(7),
        )?;

        Ok(LoginRes {
            user: UserRes::from(user),
            tokens: AuthTokensRes {
                access_token,
                refresh_token,
            },
        })
    }

    /// 刷新 Token 验证业务
    /// 用于确保持有合法 Refresh Token 的用户依然处于正常允许登录的状态
    pub async fn refresh(
        pool: &PgPool,
        config: &AppConfig,
        refresh_token: &str,
    ) -> Result<AuthTokensRes, AppError> {
        // 1. 校验证明：拿着这把专属的长效钥匙（用 refresh_secret 解密）
        let claims = jwt::verify_token(refresh_token, &config.jwt_refresh_secret)?;

        // 2. 检查业务态：即便钥匙有效，账号还在吗？被封禁了吗？
        let user = UserRepo::find_by_id(pool, claims.sub)
            .await?
            .ok_or_else(|| {
                AppError::Unauthorized("该账号已被注销或封禁，Token 失效".to_string())
            })?;

        // 3. 签发新的双 Token
        let access_token = jwt::sign_token(
            user.id,
            user.role.clone(),
            &config.jwt_access_secret,
            Duration::minutes(15),
        )?;
        let new_refresh_token = jwt::sign_token(
            user.id,
            user.role.clone(),
            &config.jwt_refresh_secret,
            Duration::days(7),
        )?;

        Ok(AuthTokensRes {
            access_token,
            refresh_token: new_refresh_token,
        })
    }
}
