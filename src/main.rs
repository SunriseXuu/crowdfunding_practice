mod config;

fn main() {
    // 1. 加载全局配置
    let app_config = config::AppConfig::init();
    println!("✅ App Config Loaded!");
    println!("🚀 Server will start on port: {}", app_config.port);
}
