use std::sync::Arc;

use crate::config::AppConfig;

/// AppState 是整个应用的"全局共享上下文"。
///
/// 任何一个 Handler 都可以通过 Axum 的依赖注入拿到这个对象。
/// 我们用 Arc 包裹它，是因为 Axum 会在多个异步线程间复用这个状态，
/// Arc（原子引用计数）可以安全地让多个所有者共享同一份数据，而不用复制它。
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool, config: AppConfig) -> Self {
        Self {
            pool,
            config: Arc::new(config),
        }
    }
}
