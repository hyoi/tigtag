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
    pub base_speed: f32,  // 基準の移動速度
    pub speedup: f32,     // スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub is_stop: bool,    // 移動停止フラグ
    pub px_start: Vec2,   // 1フレーム時間に移動した微小区間の始点
    pub px_end: Vec2,     // 1フレーム時間に移動した微小区間の終点
    pub opt_fn_autochase: Option<FnAutoChase>, // 敵キャラの移動方向を決める関数
    pub color: Srgba,     // 敵キャラの表示色
    pub anime: SpriteAnimationParams, // スプライトアニメーションの情報
}

// チェイサーの初期状態
impl Default for Chaser
{
    fn default() -> Self
    {
        const CHASER_TIME_PER_GRID: f32 = 0.20; // 0.13; //１グリッド進むために必要な時間

        Self {
            grid: IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer: Timer::from_seconds(CHASER_TIME_PER_GRID, TimerMode::Once),
            base_speed: PIXELS_PER_GRID / CHASER_TIME_PER_GRID,
            speedup: 1.0,
            is_stop: true,
            px_start: Vec2::default(),
            px_end: Vec2::default(),
            opt_fn_autochase: None,
            color: Srgba::NONE,
            anime: SpriteAnimationParams {
                timer: Timer::from_seconds(ANIME_TIMER_CHASER, TimerMode::Repeating),
                ..default()
            },
        }
    }
}

// スプライトアニメーションの切替間隔
const ANIME_TIMER_CHASER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

// チェイサーの移動方向を決める関数（関数ポインタ）
pub type FnAutoChase = fn(
    &mut Chaser,     // チェイサーのComponent
    &player::Player, // プレイヤーのComponent
    &[News],         //
) -> News;

////////////////////////////////////////////////////////////////////////////////

// チェイサーのスタート座標
pub const CHASER_START_POSITION: &[IVec2] = &[
    IVec2::new(1, 1),
    IVec2::new(1, MAX_Y),
    IVec2::new(MAX_X, 1),
    IVec2::new(MAX_X, MAX_Y),
];
const MAX_X: i32 = MAP_GRIDS_WIDTH - 2;
const MAX_Y: i32 = MAP_GRIDS_HEIGHT - 2;

// チェイサーの各色ごとの情報
pub const CHASERS_SPRITE_INFO: &[(&str, Srgba, Option<FnAutoChase>)] = &[
    (
        ASSETS_SPRITE_SHEET_CHASER_RED, // assetファイル名
        basic::RED,                     // 色
        chaser::SELECT_PATH_RED,        // 移動方向の決定関数
    ),
    (
        ASSETS_SPRITE_SHEET_CHASER_GREEN,
        basic::GREEN,
        chaser::SELECT_PATH_GREEN,
    ),
    (
        ASSETS_SPRITE_SHEET_CHASER_PINK,
        basic::FUCHSIA,
        chaser::SELECT_PATH_PINK,
    ),
    (
        ASSETS_SPRITE_SHEET_CHASER_BLUE,
        basic::BLUE,
        chaser::SELECT_PATH_BLUE,
    ),
];

//スピードアップの割増
pub const CHASER_ACCEL: f32 = 0.4;

////////////////////////////////////////////////////////////////////////////////

// 正方形メッシュの情報
pub const CHASER_SPRITE_RADIUS: f32 = PIXELS_PER_GRID * 0.35; // 半径

////////////////////////////////////////////////////////////////////////////////

// End of code.
