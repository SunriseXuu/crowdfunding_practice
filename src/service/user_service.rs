use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::request::{UpdatePasswordReq, UpdateUserReq};
use crate::dto::response::UserRes;
use crate::error::AppError;
use crate::repository::UserRepo;

/// 用户业务逻辑层 (User Service)
///
/// 负责处理业务逻辑，如：
/// - 查询信息
/// - 密码修改
/// - 更新信息
pub struct UserService;

impl UserService {
    /// 获取一个用户信息业务
    pub async fn retrieve(pool: &PgPool, user_id: Uuid) -> Result<UserRes, AppError> {
        let user = UserRepo::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在或已被删除".to_string()))?;
        Ok(UserRes::from(user))
    }

    /// 更新用户信息业务
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        req: UpdateUserReq,
    ) -> Result<UserRes, AppError> {
        // 如果没有传数据，查询当前信息返回
        if req.username.is_none() && req.age.is_none() && req.gender.is_none() {
            let user = UserRepo::find_by_id(pool, user_id)
                .await?
                .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
            Ok(UserRes::from(user))
        }
        // 如果有传数据，更新数据库
        else {
            let user = UserRepo::update(pool, user_id, req.username, req.age, req.gender).await?;
            Ok(UserRes::from(user))
        }
    }

    /// 修改用户密码业务
    pub async fn update_password(
        pool: &PgPool,
        user_id: Uuid,
        req: UpdatePasswordReq,
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

    /// 软删除用户业务
    pub async fn soft_delete(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        let affected = UserRepo::soft_delete(pool, user_id).await?;
        if affected == 0 {
            return Err(AppError::NotFound("用户不存在或已被注销".to_string()));
        }

        Ok(())
    }
}
