use super::*;

////////////////////////////////////////////////////////////////////////////////

// 全画面メッセージの設定リスト
impl Default for core_logic::overlay_ui::messages::MessageSettings
{
    fn default() -> Self
    {
        Self(vec![
            Box::new(OverlayTitleDemo::default()),
            Box::new(OverlayStageStart::default()),
            Box::new(OverlayStageClear::default()),
            Box::new(OverlayGameOver::default()),
        ])
    }
}

////////////////////////////////////////////////////////////////////////////////

// タイトルで使うカラー
const TITLE_COLOR1: Color = Color::srgba(0.6, 1.0, 0.4, 0.75);
const TITLE_COLOR2: Color = Color::srgba(0.0, 0.7, 0.5, 0.75);

// 全画面表示のカントダウン用プレイスホルダー
pub const _CDPH_: &str = "_#_";

////////////////////////////////////////////////////////////////////////////////

// 全画面メッセージ（タイトルデモ）のComponent
#[derive(Component, Clone, OverlayMessage, Blinking)]
pub struct OverlayTitleDemo
{
    text_spans: &'static [core_logic::overlay_ui::messages::TextUiSpan],
    blinking: core_logic::overlay_ui::effect::BlinkingParams,
}

// Componentの初期化
impl Default for OverlayTitleDemo
{
    fn default() -> Self
    {
        Self {
            #[rustfmt::skip]
            text_spans: &[
                ( APP_TITLE     , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 3.5, TITLE_COLOR1 ),
                ( "\nv"         , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, TITLE_COLOR2 ),
                ( APP_VER       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, TITLE_COLOR2 ),
                ( "\n \n"       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, COLOR_NONE   ),
                ( "D E M O"     , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, COLOR_GOLD   ),
                ( "\n \n"       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, COLOR_NONE   ),
                ( "Hit ANY key!", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN   ),
                ( "\nor\n"      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, COLOR_CYAN   ),
                ( "ANY button!" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN   ),
            ],
            blinking: core_logic::overlay_ui::effect::BlinkingParams {
                cycle: 0.0,
                spans_index: 4,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メッセージ（ステージ開始）のComponent
#[derive(Component, Clone, OverlayMessage, CountDown)]
pub struct OverlayStageStart
{
    text_spans: &'static [core_logic::overlay_ui::messages::TextUiSpan],
    countdown: core_logic::overlay_ui::effect::CountDownParams,
}

// Componentの初期化
impl Default for OverlayStageStart
{
    fn default() -> Self
    {
        Self {
            #[rustfmt::skip]
            text_spans: &[
                ( "START\n"   , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, COLOR_CYAN ),
                ( "ready...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 2.0, COLOR_CYAN ),
                ( _CDPH_      , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.5, COLOR_GOLD ),
            ],
            countdown: core_logic::overlay_ui::effect::CountDownParams {
                start_value: 5,
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                counter: 0,
                spans_index: 2,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メッセージ（ステージクリア）のComponent
#[derive(Component, Clone, OverlayMessage, CountDown)]
pub struct OverlayStageClear
{
    text_spans: &'static [core_logic::overlay_ui::messages::TextUiSpan],
    countdown: core_logic::overlay_ui::effect::CountDownParams,
}

// Componentの初期化
impl Default for OverlayStageClear
{
    fn default() -> Self
    {
        Self {
            #[rustfmt::skip]
            text_spans: &[
                ( "CLEAR!!!\n"  , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.7, COLOR_CYAN ),
                ( "next stage\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 2.0, COLOR_GOLD ),
                ( "ready...\n"  , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 2.0, COLOR_CYAN ),
                ( _CDPH_        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.5, COLOR_GOLD ),
            ],
            countdown: core_logic::overlay_ui::effect::CountDownParams {
                start_value: 10,
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                counter: 0,
                spans_index: 3,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メッセージ（ゲームオーバー）のComponent
#[derive(Component, Clone, OverlayMessage, Blinking, CountDown)]
pub struct OverlayGameOver
{
    text_spans: &'static [core_logic::overlay_ui::messages::TextUiSpan],
    blinking: core_logic::overlay_ui::effect::BlinkingParams,
    countdown: core_logic::overlay_ui::effect::CountDownParams,
}

// Componentの初期化
impl Default for OverlayGameOver
{
    fn default() -> Self
    {
        Self {
            #[rustfmt::skip]
            text_spans: &[
                ( "Game Over\n"   , ASSETS_FONT_REGGAEONE_REGULAR   , PIXELS_PER_GRID * 4.0, COLOR_RED  ),
                ( " \n"           , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.6, COLOR_NONE ),
                ( "REPLAY?"       , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 2.0, COLOR_GOLD ),
                ( "\n"            , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, COLOR_NONE ),
                ( "Hit ANY key!\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN ),
                ( "or\n"          , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, COLOR_CYAN ),
                ( "ANY button!\n" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN ),
                ( _CDPH_          , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 3.5, COLOR_GOLD ),
            ],
            blinking: core_logic::overlay_ui::effect::BlinkingParams {
                cycle: 0.0,
                spans_index: 2,
            },
            countdown: core_logic::overlay_ui::effect::CountDownParams {
                start_value: 10,
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                counter: 0,
                spans_index: 7,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メニュー（Pause）のComponent
#[derive(Component, Clone, OverlayMenu, ScalingItem)]
pub struct OverlayPauseMenu
{
    overlay_menu: core_logic::overlay_ui::pause_menu::OverlayMenuParams,
    scaling_item: core_logic::overlay_ui::pause_menu::ScalingItemParams,
}

// Pauseメニューの設定に使う定数
pub const PAUSE_MENU_BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);
pub const PAUSE_TITLE_SIZE: f32 = PIXELS_PER_GRID * 3.5;
pub const PAUSE_SELECTED_COLOR: Color = COLOR_CYAN;
pub const PAUSE_NORMAL_COLOR: Color = COLOR_SILVER;
pub const PAUSE_BASE_SIZE: f32 = PIXELS_PER_GRID * 2.0;

// Componentの初期化
impl Default for OverlayPauseMenu
{
    fn default() -> Self
    {
        use core_logic::overlay_ui::pause_menu::PauseMenuItem::*;
        Self {
            #[rustfmt::skip]
            overlay_menu: core_logic::overlay_ui::pause_menu::OverlayMenuParams
            {
                settings: Vec::from_iter(&[
                    ( Label , ( "PAUSE" , ASSETS_FONT_ORBITRON_BLACK, PAUSE_TITLE_SIZE     , COLOR_GOLD         ) ),
                    ( Label , ( " "     , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 1.2, COLOR_NONE         ) ),
                    ( Exit  , ( "EXIT"  , ASSETS_FONT_ORBITRON_BLACK, PAUSE_BASE_SIZE      , PAUSE_NORMAL_COLOR ) ),
                    // ( Label , ( " "     , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.8, COLOR_NONE         ) ),
                    // ( Config, ( "CONFIG", ASSETS_FONT_ORBITRON_BLACK, PAUSE_BASE_SIZE      , PAUSE_NORMAL_COLOR ) ),
                ]),
                selected_index: 2,
            },
            scaling_item:
                core_logic::overlay_ui::pause_menu::ScalingItemParams::default(),
        }
    }
}

// Pauseメニューのキーとボタンの設定
impl Default for core_logic::overlay_ui::pause_menu::PauseMenuOpen
{
    fn default() -> Self
    {
        Self {
            keys: vec![(KeyCode::Escape, None)],
            buttons: vec![GamepadButton::Mode], // Mode: ps4[PSボタン]
        }
    }
}
impl Default for core_logic::overlay_ui::pause_menu::PauseMenuUp
{
    fn default() -> Self
    {
        Self {
            keys: vec![(KeyCode::ArrowUp, None), (KeyCode::KeyW, None)],
            buttons: vec![GamepadButton::DPadUp],
        }
    }
}
impl Default for core_logic::overlay_ui::pause_menu::PauseMenuDown
{
    fn default() -> Self
    {
        Self {
            keys: vec![(KeyCode::ArrowDown, None), (KeyCode::KeyS, None)],
            buttons: vec![GamepadButton::DPadDown],
        }
    }
}
impl Default for core_logic::overlay_ui::pause_menu::PauseMenuApply
{
    fn default() -> Self
    {
        Self {
            keys: vec![
                (KeyCode::Enter, None),
                (KeyCode::Space, None),
                (KeyCode::KeyE, None),
            ],
            buttons: vec![GamepadButton::East],
        }
    }
}
impl Default for core_logic::overlay_ui::pause_menu::PauseMenuCancel
{
    fn default() -> Self
    {
        Self {
            keys: vec![(KeyCode::Escape, None)],
            buttons: vec![GamepadButton::South],
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
