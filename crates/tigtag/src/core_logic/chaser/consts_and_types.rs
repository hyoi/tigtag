use super::*;

////////////////////////////////////////////////////////////////////////////////

// チェイサーのComponent
#[derive(Component)]
pub struct Chaser
{
    pub cell: IVec2,      // 移動中は移動元の座標、停止中はその場の座標
    pub next_cell: IVec2, // 移動中は移動先の座標、停止中はその場の座標
    pub px_start: Vec3,   // 1フレーム時間に移動した微小区間の始点
    pub px_end: Vec3,     // 1フレーム時間に移動した微小区間の終点
    pub direction: News,  // 移動の向き
    pub speedup: f32,     // スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub timer: Timer,     // 移動のタイマー
    pub is_stop: bool,    // 移動停止フラグ
    pub base_speed: f32,  // 基準の移動速度
    pub anime: SpriteAnimationParams, // スプライトアニメーションの情報
    pub option_fn_autochase: Option<FnAutoChase>, // 敵キャラの移動方向を決める関数
    pub color: Srgba,     // 敵キャラの表示色
}

// チェイサーの移動方向を決める関数（関数ポインタ）
pub type FnAutoChase = fn(
    &mut Chaser,     // チェイサーのComponent
    &player::Player, // プレイヤーのComponent
    &[News],         //
) -> News;

// チェイサーの初期状態
impl Default for Chaser
{
    fn default() -> Self
    {
        const CHASER_TIME_PER_GRID: f32 = 0.20; // 0.13; //１グリッド進むために必要な時間

        Self {
            cell: IVec2::default(),
            next_cell: IVec2::default(),
            px_start: Vec3::default(),
            px_end: Vec3::default(),
            direction: News::South,
            speedup: 1.0,
            timer: Timer::from_seconds(CHASER_TIME_PER_GRID, TimerMode::Once),
            is_stop: true,
            base_speed: PIXELS_PER_GRID / CHASER_TIME_PER_GRID,
            anime: SpriteAnimationParams {
                timer: Timer::from_seconds(ANIME_TIMER_CHASER, TimerMode::Repeating),
                ..default()
            },
            option_fn_autochase: None,
            color: Srgba::NONE,
        }
    }
}

// スプライトアニメーションの切替間隔
const ANIME_TIMER_CHASER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

// チェイサーのスタート座標
pub const CHASER_START_POSITION: &[IVec2] = &[
    IVec2::new(1, 1),
    IVec2::new(1, MAX_Y),
    IVec2::new(MAX_X, 1),
    IVec2::new(MAX_X, MAX_Y),
];
const MAX_X: i32 = map::MAP_WIDTH_IN_CELLS - 2;
const MAX_Y: i32 = map::MAP_HEIGHT_IN_CELLS - 2;

////////////////////////////////////////////////////////////////////////////////

// チェイサーの各色ごとの情報
pub const CHASERS_SPRITE_INFO: &[(&str, Srgba, Option<FnAutoChase>)] = &[
    (
        ASSETS_SPRITESHEET_CHASER_RED, // assetファイル名
        basic::RED,                    // 色
        chaser::SELECT_PATH_RED,       // 移動方向の決定関数
    ),
    (
        ASSETS_SPRITESHEET_CHASER_GREEN,
        basic::GREEN,
        chaser::SELECT_PATH_GREEN,
    ),
    (
        ASSETS_SPRITESHEET_CHASER_PINK,
        basic::FUCHSIA,
        chaser::SELECT_PATH_PINK,
    ),
    (
        ASSETS_SPRITESHEET_CHASER_BLUE,
        basic::BLUE,
        chaser::SELECT_PATH_BLUE,
    ),
];

////////////////////////////////////////////////////////////////////////////////

// 正方形メッシュの情報
pub const CHASER_SPRITE_SCALING: f32 = 0.6;

////////////////////////////////////////////////////////////////////////////////

// スピードアップの割増
pub const CHASER_ACCEL: f32 = 0.4;

////////////////////////////////////////////////////////////////////////////////

// End of code.
