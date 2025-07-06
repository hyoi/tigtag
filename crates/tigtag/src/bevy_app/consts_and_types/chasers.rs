use super::*;

////////////////////////////////////////////////////////////////////////////////

// チェイサーのComponent
#[derive(Component)]
pub struct Chaser
{
    pub grid: IVec2,      // 移動中は移動元の座標、停止中はその場の座標
    pub next_grid: IVec2, // 移動中は移動先の座標、停止中はその場の座標
    pub direction: News,  // 移動の向き
    pub timer: Timer,     // 移動のタイマー
    pub is_stop: bool,    // 移動停止フラグ
    pub speedup: f32,     // スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub px_start: Vec2,   // 1フレーム時間に移動した微小区間の始点
    pub px_end: Vec2,     // 1フレーム時間に移動した微小区間の終点
    pub opt_fn_autochase: Option<FnAutoChase>, // 敵キャラの移動方向を決める関数
    pub color: Color,     // 敵キャラの表示色
    pub anime_timer: Timer, // アニメーションのタイマー
    pub sprite_sheet_frame: u32, // アニメーションのフレーム数
    pub sprite_sheet_indexes: FxHashMap<News, u32>, /* アニメーションの先頭位置(offset値) */
}

// 関数ポインタ型(敵キャラの移動方向を決める関数)
pub type FnAutoChase = fn(&mut Chaser, &player::Player, &[News]) -> News;

impl Default for Chaser
{
    fn default() -> Self
    {
        Self {
            grid: IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer: Timer::from_seconds(CHASER_TIME_PER_GRID, TimerMode::Once),
            is_stop: true,
            speedup: 1.0,
            px_start: Vec2::default(),
            px_end: Vec2::default(),
            opt_fn_autochase: None,
            color: Color::NONE,
            anime_timer: Timer::from_seconds(
                ANIME_TIMER_CHASER,
                TimerMode::Repeating,
            ),
            sprite_sheet_frame: SPRITE_SHEET_COLS_CHASER,
            sprite_sheet_indexes: (*SPRITE_SHEET_IDXS_CHASER).clone(),
        }
    }
}

// 敵キャラの設定値
pub const CHASER_START_POSITION: &[IVec2] = // スタート座標
    &[
        IVec2::new(1, 1),
        IVec2::new(1, MAX_Y),
        IVec2::new(MAX_X, 1),
        IVec2::new(MAX_X, MAX_Y),
    ];
const MAX_X: i32 = MAP_GRIDS_WIDTH - 2;
const MAX_Y: i32 = MAP_GRIDS_HEIGHT - 2;

// 各色ごとの情報（色と移動方向の決定関数とassetファイル名）
use crate::bevy_app::chasers::*;
pub const CHASERS_SPRITE_INFO: &[(Color, Option<FnAutoChase>, &str)] = &[
    (
        Color::Srgba(css::RED),
        chasers::CHOICE_WAY_RED,
        ASSETS_SPRITE_SHEET_CHASER_RED,
    ),
    (
        Color::Srgba(css::GREEN),
        chasers::CHOICE_WAY_GREEN,
        ASSETS_SPRITE_SHEET_CHASER_GREEN,
    ),
    (
        Color::Srgba(css::PINK),
        chasers::CHOICE_WAY_PINK,
        ASSETS_SPRITE_SHEET_CHASER_PINK,
    ),
    (
        Color::Srgba(css::BLUE),
        chasers::CHOICE_WAY_BLUE,
        ASSETS_SPRITE_SHEET_CHASER_BLUE,
    ),
];

pub const CHASER_TIME_PER_GRID: f32 = 0.20; // 0.13; //１グリッド進むために必要な時間
                                            // const CHASER_SPEED: f32 = PIXELS_PER_GRID / CHASER_TIME_PER_GRID; //速度
pub const CHASER_SPRITE_SCALING: f32 = 0.35; // primitive shape表示時の縮小係数
                                             // const CHASER_ACCEL: f32 = 0.4; //スピードアップの割増

// スプライトシートを使ったアニメーションの情報
pub const SPRITE_SHEET_SIZE_CHASER: UVec2 = UVec2::new(8, 8);
pub const SPRITE_SHEET_COLS_CHASER: u32 = 4;
pub const SPRITE_SHEET_ROWS_CHASER: u32 = 4;
static SPRITE_SHEET_IDXS_CHASER: LazyLock<FxHashMap<News, u32>> =
    LazyLock::new(|| {
        FxHashMap::from_iter([
            (News::North, 0),
            (News::East, 4),
            (News::West, 8),
            (News::South, 12),
        ])
    });
const ANIME_TIMER_CHASER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

// End of code.
