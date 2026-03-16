mod config;
mod error;

use config::AppConfig;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// AppState 是整个应用的"全局共享上下文"。
///
/// 类比前端的 React Context 或 Pinia Store：
/// 任何一个 Handler 都可以通过 Axum 的依赖注入拿到这个对象。
/// 我们用 Arc 包裹它，是因为 Axum 会在多个异步线程间复用这个状态，
/// Arc（原子引用计数）可以安全地让多个所有者共享同一份数据，而不用复制它。
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<AppConfig>,
}

#[tokio::main]
async fn main() {
    // ── 步骤一：初始化全局日志追踪 (tracing) ────────────────────────────────
    // EnvFilter 允许你通过 RUST_LOG 环境变量来控制日志级别，
    // 例如 RUST_LOG=debug cargo run。如果没有设置，默认输出 INFO 级别。
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── 步骤二：加载应用配置 ─────────────────────────────────────────────────
    let config = AppConfig::init();
    info!("✅ App Config Loaded!");

    // ── 步骤三：建立 SQLx 数据库连接池 ──────────────────────────────────────
    // PgPool 是一个连接池（Connection Pool）。
    // 类比前端的 HTTP 客户端池：与其每个请求都新建/销毁一个 TCP 连接，
    // 连接池会预先建立若干条数据库连接并复用它们，大幅降低连接开销。
    let pool = PgPoolOptions::new()
        .max_connections(20)     // 最多同时维持 20 条物理连接
        .connect(&config.database_url)
        .await
        .expect("🚨 FATAL: Cannot connect to the database. Is Docker Postgres running?");

    info!("✅ Database connected!");

    // ── 步骤四：组装全局 AppState ────────────────────────────────────────────
    let port = config.port;
    let state = AppState {
        db: pool,
        config: Arc::new(config),
    };

    // ── 步骤五：配置 Axum 路由并启动服务器 ──────────────────────────────────
    // 暂时用一个健康检查路由占位，后续会替换为 api_router 模块
    let app = axum::Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("🚨 FATAL: Cannot bind to the port");

    axum::serve(listener, app)
        .await
        .expect("🚨 FATAL: Server crashed unexpectedly");
}
