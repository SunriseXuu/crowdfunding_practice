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

        // 数据库连接字符串，必须在 .env 中显式设定，否则无法启动
        let database_url = env::var("DATABASE_URL")
            .expect("🚨 FATAL: DATABASE_URL must be set in .env or system environment");

        // 默认监听在 8080 端口，除非环境变量显式修改
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("🚨 FATAL: PORT must be a valid number");

        // JWT_SECRET 在 .env 中必须显式设定，绝不允许使用 default 值跑生产环境
        let jwt_secret = env::var("JWT_SECRET")
            .expect("🚨 FATAL: JWT_SECRET must be set in .env or system environment");

        Self {
            database_url,
            port,
            jwt_secret,
        }
    }
}
