//! AppConfig 负责加载和校验整个应用启动时必须的环境变量

use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl AppConfig {
    /// 从当前环境或 `.env` 文件初始化应用配置
    pub fn init() -> Self {
        // 尝试从 .env 文件加载环境变量。
        // 使用 .ok() 是因为在真正的生产容器里（不用 .env 文件），找不到它也不该报错。
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("🚨 FATAL: DATABASE_URL must be set in .env or system environment");

        // 默认监听在 8080 端口，除非环境变量显式修改
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("🚨 FATAL: PORT must be a valid number");

        // 暂定一个 default 的鉴权密钥。后续做到 User 服务时我们再强制迁移到 .env
        let jwt_secret =
            env::var("JWT_SECRET").unwrap_or_else(|_| "super-secret-key-for-dev".to_string());

        Self {
            database_url,
            port,
            jwt_secret,
        }
    }
}
