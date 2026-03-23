mod api_doc;
mod config;
mod dto;
mod error;
mod extractor;
mod handler;
mod model;
mod repository;
mod router;
mod service;
mod util;
mod worker;

use config::AppConfig;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use util::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
    tracing::info!("✅ App Config Loaded!");

    // ── 步骤三：建立 SQLx 数据库连接池 ──────────────────────────────────────
    // PgPool 是一个连接池（Connection Pool）。
    // 类比前端的 HTTP 客户端池：与其每个请求都新建/销毁一个 TCP 连接，
    // 连接池会预先建立若干条数据库连接并复用它们，大幅降低连接开销。
    let pool = PgPoolOptions::new()
        .max_connections(20) // 最多同时维持 20 条物理连接
        .connect(&config.database_url)
        .await
        .expect("🚨 FATAL: Cannot connect to the database. Is Docker Postgres running?");

    tracing::info!("✅ Database connected!");

    // ── 步骤四：组装全局 AppState ────────────────────────────────────────────
    let port = config.port;
    let state = Arc::new(AppState::new(pool, config));

    // ── 步骤五：启动后台异步任务 (Worker) ──────────────────────────────────
    let worker_state = Arc::clone(&state);
    tokio::spawn(async move {
        worker::settlement_worker::start(worker_state).await;
    });

    // ── 步骤六：配置 Axum 路由并启动服务器 ──────────────────────────────────
    let app = router::init_router(state)
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc::ApiDoc::openapi()),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("🚨 FATAL: Cannot bind to the port");

    axum::serve(listener, app)
        .await
        .expect("🚨 FATAL: Server crashed unexpectedly");
}
