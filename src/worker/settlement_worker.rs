use std::sync::Arc;
use tokio::time::{Duration, interval};
use tracing::{error, info};

use crate::{AppState, service::SettlementService};

/// 启动结算后台任务
/// 它会每隔一定时间检查是否有已过期的众筹项目并进行状态切换和退款处理
pub async fn start(state: Arc<AppState>) {
    info!("⚙️ [Worker] Settlement worker started");

    // 设置检查间隔，例如每 60 秒执行一次
    let mut interval = interval(Duration::from_secs(60));

    loop {
        // 等待下一个周期
        interval.tick().await;

        info!("⏰ [Worker] Running scheduled settlement check...");

        // 执行结算业务逻辑
        if let Err(e) = SettlementService::settle_expired_campaigns(&state.pool).await {
            error!("🚨 [Worker] Settlement check failed: {:?}", e);
        }
    }
}
