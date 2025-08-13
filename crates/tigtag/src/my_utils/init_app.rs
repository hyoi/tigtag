use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        // アプリの準備
        let window_plugin = WindowPlugin {
            primary_window: MAIN_WINDOW.clone(),
            ..default()
        };
        let log_plugin = LogPlugin {
            filter: (if DEBUG() { LOG_LV_DEV } else { LOG_LV_REL }).into(),
            ..default()
        };

        application
            // first party plugins
            .add_plugins((
                DefaultPlugins
                    .set(window_plugin) // 主ウィンドウ
                    .set(log_plugin) // ログレベル
                    .set(ImagePlugin::default_nearest()), // ピクセルパーフェクト
                FrameTimeDiagnosticsPlugin::default(), // FPS Plugin
            ))
            // Stateの初期化
            .init_state::<MyState>()
            // Updateスケジュール（without State）
            .add_systems(
                Update,
                (
                    // 特別なキー入力
                    misc::app_close_on_key.run_if(not(WASM)), // アプリ終了
                    misc::toggle_window_mode.run_if(not(WASM)), // 全画面切換
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
