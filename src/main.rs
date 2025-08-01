fn main() {
    let os = std::env::consts::OS;

    // Windows平台使用log库打印
    #[cfg(target_os = "windows")]
    {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
        log::info!("log: current os is {}", os);
    }

    // Linux平台使用tracing库打印
    #[cfg(target_os = "linux")]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
        tracing::info!("traceing: current os is {}", os);
    }

    // 通用输出
    println!("os: {}", os);
}
