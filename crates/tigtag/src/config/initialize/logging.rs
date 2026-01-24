use super::*;

////////////////////////////////////////////////////////////////////////////////

// ログフィルター
const LOG_FILTER_DEVELOP: &str = formatcp!("warn,wgpu_hal=error,{APP_TITLE}=info");
const LOG_FILTER_RELEASE: &str = "error";

// LogPluginの初期化
pub trait InitLogPlugin
{
    fn initialize() -> Self;
}

impl InitLogPlugin for LogPlugin
{
    fn initialize() -> Self
    {
        // ログ出力の制御
        let (level, filter) = match cfg!(debug_assertions)
        {
            // deverop
            true => (Level::INFO, LOG_FILTER_DEVELOP.into()),
            // release
            false => (Level::ERROR, LOG_FILTER_RELEASE.into()),
        };
        Self {
            level,
            filter,
            ..default()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
