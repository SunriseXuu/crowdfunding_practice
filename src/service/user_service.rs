use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::request::{ChangePasswordReq, LoginReq, RegisterReq, UpdateUserReq};
use crate::dto::response::UserRes;
use crate::error::AppError;
use crate::repository::UserRepo;

/// 用户业务逻辑层 (Service)
///
/// 负责处理业务逻辑，如：
/// - 密码的哈希处理与校验
/// - 邮箱冲突检查
/// - 数据的 DTO 转换
pub struct UserService;

impl UserService {
    /// 注册用户
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
        let user = UserRepo::create(pool, &req.email, &password_hash, &req.username).await?;

        Ok(UserRes::from(user))
    }

    /// 登录用户
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

    /// 更新用户信息
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        req: UpdateUserReq,
    ) -> Result<UserRes, AppError> {
        // 如果传了用户名，更新用户名
        if let Some(username) = req.username {
            let user = UserRepo::update(pool, user_id, &username).await?;
            Ok(UserRes::from(user))
        }
        // 如果没传数据，查询当前信息返回
        else {
            let user = UserRepo::find_by_id(pool, user_id)
                .await?
                .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
            Ok(UserRes::from(user))
        }
    }

    /// 修改用户密码
    pub async fn update_password(
        pool: &PgPool,
        user_id: Uuid,
        req: ChangePasswordReq,
    ) -> Result<(), AppError> {
        // 1. 获取用户信息
        let user = UserRepo::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        // 2. 校验旧密码
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::Internal(format!("密码格式解析异常: {}", e)))?;
        Argon2::default()
            .verify_password(req.old_password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::BadRequest("旧密码错误".to_string()))?;

        // 3. 加密新密码
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let new_hash = argon2
            .hash_password(req.new_password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("新密码加密异常: {}", e)))?
            .to_string();

        // 4. 存入数据库
        UserRepo::update_password(pool, user_id, &new_hash).await?;

        Ok(())
    }

    /// 软删除用户
    pub async fn soft_delete(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        let affected = UserRepo::soft_delete(pool, user_id).await?;
        if affected == 0 {
            return Err(AppError::NotFound("用户不存在或已被注销".to_string()));
        }

        Ok(())
    }
}
