use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, appl: &mut App)
    {
        // 主ウィンドウの準備
        let window_plugin = WindowPlugin {
            primary_window: MAIN_WINDOW.clone(),
            ..default()
        };
        let log_plugin = LogPlugin {
            filter: (if DEBUG() { LOG_LV_DEV } else { LOG_LV_REL }).into(),
            ..default()
        };
        appl.add_plugins(
            DefaultPlugins
                .set(window_plugin) // 主ウィンドウ
                .set(log_plugin) // ログレベル
                .set(ImagePlugin::default_nearest()), // ピクセルパーフェクト
        );

        // キー入力でアプリ終了
        appl.add_systems(Update, misc::app_close_on_key.run_if(not(WASM)));

        // フルスクリーン切換
        appl.add_systems(Update, misc::toggle_window_mode.run_if(not(WASM)));

        // Stateの初期化
        // ※前提条件：1) DefaultPluginsの後に記述すること。2) enumのDefaltを設定すること
        appl.init_state::<MyState>();
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
