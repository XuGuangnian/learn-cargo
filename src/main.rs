fn main() {
    let os = std::env::consts::OS;

    // Windows平台使用log库打印
    #[cfg(all(windows, feature = "windows-log"))]
    {
        env_logger::init();
        log::info!("current os is {}", os);
    }

    // Linux平台使用tracing库打印
    #[cfg(all(target_os = "linux", feature = "linux-tracing"))]
    {
        tracing_subscriber::fmt::init();
        tracing::info!("current os is {}", os);
    }

    // 通用输出
    println!("os: {}", os);
}
