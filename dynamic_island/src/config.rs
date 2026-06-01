// 配置路径占位模块
//
// config 的实际实现在主应用中。
// 灵动岛扩展使用此占位模块以保持编译独立性。

pub fn get_app_config_dir() -> std::path::PathBuf {
    dirs::config_dir()
        .map(|d| d.join("ai-gateway"))
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}
