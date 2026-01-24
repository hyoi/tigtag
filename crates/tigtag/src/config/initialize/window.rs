use super::*;

////////////////////////////////////////////////////////////////////////////////

// ウィンドウの縦横のCell数
pub const WINDOW_CELLS_WIDTH: u32 = 25;
pub const WINDOW_CELLS_HEIGHT: u32 = 19;

// 基底Cellの縦横Pixel数
pub const PIXELS_PER_CELL: f32 = BASE_CELL_PIXELS as f32 * BASE_CELL_SCALING;
const BASE_CELL_PIXELS: u32 = 8;
const BASE_CELL_SCALING: f32 = 4.0;

// ウィンドウの縦横のPixel数
pub const WINDOW_PIXELS_WIDTH: f32 = PIXELS_PER_CELL * WINDOW_CELLS_WIDTH as f32;
pub const WINDOW_PIXELS_HEIGHT: f32 = PIXELS_PER_CELL * WINDOW_CELLS_HEIGHT as f32;

// ウィンドウの解像度（UVec2）
pub const WINDOW_BASE_RESOLUTION: UVec2 =
    UVec2::new(WINDOW_PIXELS_WIDTH as u32, WINDOW_PIXELS_HEIGHT as u32);

// WindowPluginの初期化
pub trait InitWindowPlugin
{
    fn initialize() -> Self;
}

impl InitWindowPlugin for WindowPlugin
{
    fn initialize() -> Self
    {
        Self {
            // 主ウィンドウの初期設定
            primary_window: Some(Window {
                resolution: WINDOW_BASE_RESOLUTION.into(), // ウィンドウ解像度
                mode: WindowMode::Windowed, // resolutionを確実に反映するために必要
                resizable: false,           // リサイズ不可
                decorations: true,          // タイトルバーを表示する
                title: format!("{APP_TITLE} v{APP_VERSION} {APP_COPYRIGHT}"), // タイトル文字列
                enabled_buttons: EnabledButtons {
                    minimize: false, // 最小化ボタン非表示
                    maximize: false, // 最大化ボタン非表示
                    close: true,     // クローズボタン表示
                },
                // fit_canvas_to_parent: true, // v0.13で廃止(#11057)、v0.14で復活(#11278)
                ..default()
            }),
            ..default()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
