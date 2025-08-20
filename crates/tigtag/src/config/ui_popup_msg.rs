use super::*;

////////////////////////////////////////////////////////////////////////////////

//マーカーComponent
pub mod popup
{
    use super::*;
    use popup_text_ui::effect::{
        CountDownParams, BlinkingParams, HitAnyKeyParams, ScalingParams,
    };

    //--------------------------------------------------------------------------

    #[derive(Component, Clone, CountDown)]
    pub struct StageSatrt
    {
        countdown: CountDownParams,
    }
    #[derive(Component, Clone, CountDown)]
    pub struct StageClear
    {
        countdown: CountDownParams,
    }
    #[derive(Component, Clone, CountDown, Blinking, HitAnyKey)]
    pub struct GameOver
    {
        countdown: CountDownParams,
        blinking: BlinkingParams,
        hit_any_key: HitAnyKeyParams,
    }
    #[derive(Component, Clone, Blinking, HitAnyKey)]
    pub struct TitleDemo
    {
        blinking: BlinkingParams,
        hit_any_key: HitAnyKeyParams,
    }
    #[derive(Component, Clone)]
    pub struct Pause
    {
        scaling: ScalingParams,
    }
    impl popup_text_ui::effect::PopupMenu for Pause
    {
        fn init(&mut self) { *self = Self::default(); }
        fn resize_font(&mut self, time_delta: f32) -> f32
        {
            let radian = &mut self.scaling.cycle;
            *radian += TAU * time_delta;
            *radian -= if *radian > TAU { TAU } else { 0.0 };

            let size = self.scaling.selected_base_size
                + self.scaling.max_scaling * ((*radian).sin() * 0.5 + 0.5); //0.0 ～ 1.0

            (size as i32) as f32 //小数点未満を切り捨て
        }
        fn selected_menuitem_index(&self) -> usize { self.scaling.spans_index }
        fn selected_menuitem_index_mut(&mut self) -> &mut usize
        {
            &mut self.scaling.spans_index
        }
        fn menuitem_len(&self) -> usize { self.scaling.spans_len }
        fn unselected_size(&self) -> f32 { self.scaling.unselected_size }
        fn selected_base_size(&self) -> f32 { self.scaling.selected_base_size }
    }

    //--------------------------------------------------------------------------

    impl Default for StageSatrt
    {
        fn default() -> Self
        {
            Self {
                countdown: CountDownParams {
                    start_value: 5,
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                    counter: 0,
                    spans_index: 2,
                },
            }
        }
    }
    impl Default for StageClear
    {
        fn default() -> Self
        {
            Self {
                countdown: CountDownParams {
                    start_value: 10,
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                    counter: 0,
                    spans_index: 2,
                },
            }
        }
    }
    impl Default for GameOver
    {
        fn default() -> Self
        {
            Self {
                countdown: CountDownParams {
                    start_value: 10,
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                    counter: 0,
                    spans_index: 7,
                },
                blinking: BlinkingParams {
                    cycle: 0.0,
                    spans_index: 2,
                },
                hit_any_key: HitAnyKeyParams {
                    ignore_keys: IGNORE_KEYS,
                },
            }
        }
    }
    impl Default for TitleDemo
    {
        fn default() -> Self
        {
            Self {
                blinking: BlinkingParams {
                    cycle: 0.0,
                    spans_index: 5,
                },
                hit_any_key: HitAnyKeyParams {
                    ignore_keys: IGNORE_KEYS,
                },
            }
        }
    }
    impl Default for Pause
    {
        fn default() -> Self
        {
            Self {
                scaling: ScalingParams {
                    unselected_size: PIXELS_PER_GRID * 1.5,
                    selected_base_size: PIXELS_PER_GRID * 2.5,
                    max_scaling: PIXELS_PER_GRID * 1.0,
                    cycle: 0.0,
                    spans_index: 0,
                    spans_len: POPUP_PAUSE.len(),
                },
            }
        }
    }
}

// ポップアップメッセージをspawnするために必要な情報のリスト（Resource）
#[derive(Resource, Deref, DerefMut)]
pub struct PopupMessages(pub Vec<popup_text_ui::TextBlock>);

impl Default for PopupMessages
{
    fn default() -> Self
    {
        PopupMessages(vec![
            (
                Box::new(popup::StageSatrt::default()),
                Vec::from(POPUP_STAGE_START),
            ),
            (
                Box::new(popup::StageClear::default()),
                Vec::from(POPUP_STAGE_CLEAR),
            ),
            (
                Box::new(popup::GameOver::default()),
                Vec::from(POPUP_GAME_OVER),
            ),
            (
                Box::new(popup::TitleDemo::default()),
                Vec::from(POPUP_TITLE_DEMO),
            ),
            (Box::new(popup::Pause::default()), Vec::from(POPUP_PAUSE)),
        ])
    }
}

pub const _CDPH_: &str = "__countdown_placeholder__";

#[rustfmt::skip]
const POPUP_STAGE_START: &[ popup_text_ui::TextUiSpanSettings ] = &[
    ( "START\n"   , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, COLOR_CYAN ),
    ( "ready...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 2.0, COLOR_CYAN ),
    ( _CDPH_      , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.5, COLOR_GOLD ),
];

#[rustfmt::skip]
const POPUP_STAGE_CLEAR: &[ popup_text_ui::TextUiSpanSettings ] = &[
    ( "C L E A R !!!\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.7, COLOR_CYAN ),
    ( "next stage...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 2.0, COLOR_CYAN ),
    ( _CDPH_           , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.5, COLOR_GOLD ),
];

#[rustfmt::skip]
const POPUP_GAME_OVER: &[ popup_text_ui::TextUiSpanSettings ] = &[
    ( "Game Over\n"   , ASSETS_FONT_REGGAEONE_REGULAR   , PIXELS_PER_GRID * 4.0, COLOR_RED  ),
    ( " \n"           , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.6, COLOR_NONE ),
    ( "REPLAY?"       , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 2.0, COLOR_GOLD ),
    ( " \n"           , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, COLOR_NONE ),
    ( "Hit ANY key!\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN ),
    ( "or\n"          , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, COLOR_CYAN ),
    ( "ANY button!\n" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN ),
    ( _CDPH_          , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 3.5, COLOR_GOLD ),
];

const TITLE_COLOR1: Color = Color::srgba(0.6, 1.0, 0.4, 0.75);
const TITLE_COLOR2: Color = Color::srgba(0.0, 0.7, 0.5, 0.75);

#[rustfmt::skip]
const POPUP_TITLE_DEMO: &[ popup_text_ui::TextUiSpanSettings ] = &[
    ( APP_TITLE       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 3.5, TITLE_COLOR1 ),
    ( "\nv"           , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ( APP_VER         , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ( "\n"            , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, COLOR_NONE   ),
    ( " \n"           , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 1.0, COLOR_NONE   ),
    ( "D E M O\n"     , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, COLOR_GOLD   ),
    ( " \n"           , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.0, COLOR_NONE   ),
    ( "Hit ANY key!\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN   ),
    ( "or\n"          , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, COLOR_CYAN   ),
    ( "ANY button!"   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, COLOR_CYAN   ),
];

//Hit ANY Keyの処理で無視するキーとボタン
#[rustfmt::skip]
const IGNORE_KEYS: &[KeyCode] = &[
    KeyCode::AltLeft    , KeyCode::AltRight,
    KeyCode::ControlLeft, KeyCode::ControlRight,
    KeyCode::ShiftLeft  , KeyCode::ShiftRight,
    KeyCode::SuperLeft  , KeyCode::SuperRight,
    KeyCode::ArrowUp    , KeyCode::ArrowDown,
    KeyCode::ArrowRight , KeyCode::ArrowLeft,
    KeyCode::CapsLock   , KeyCode::Fn,
    KeyCode::Tab        , EXIT_APP_KEY,
    KeyCode::Unidentified(NativeKeyCode::Windows(57443)), //ThinkPad [Fn]
];
// const IGNORE_BUTTONS: &[ GamepadButtonType ] =
// &[
//     GamepadButtonType::DPadUp,    GamepadButtonType::DPadDown,
//     GamepadButtonType::DPadRight, GamepadButtonType::DPadLeft,
// ];

//メニューアイテムの色
pub const MENU_ITEM_COLOR_SELECTED: Color = COLOR_YELLOW;
pub const MENU_ITEM_COLOR_NORMAL: Color = COLOR_SILVER;

//メニューアイテムの設定
#[rustfmt::skip]
const POPUP_PAUSE: &[ popup_text_ui::TextUiSpanSettings ] = &[
    ( "PAUSE\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 1.5, MENU_ITEM_COLOR_NORMAL ),
    ( "EXIT\n" , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 1.5, MENU_ITEM_COLOR_NORMAL ),
    ( "CONFIG" , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 1.5, MENU_ITEM_COLOR_NORMAL ),
];

//
// #[rustfmt::skip]
// const MENU_SELECT_KEYS: &[KeyCode] = &[
//     (KeyCode::ArrowUp  , popup_text_ui::effect::MenuItemSelect::Above ),
//     (KeyCode::KeyW     , popup_text_ui::effect::MenuItemSelect::Above ),
//     (KeyCode::ArrowDown, popup_text_ui::effect::MenuItemSelect::Below ),
//     (KeyCode::KeyS     , popup_text_ui::effect::MenuItemSelect::Below ),
// ];

////////////////////////////////////////////////////////////////////////////////

// End of code.
