use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーのComponent
#[derive(Component)]
pub struct Player
{
    pub grid: IVec2,      // 移動元の座標（停止中はその場の座標）
    pub next_grid: IVec2, // 移動先の座標（停止中はその場の座標）
    pub direction: News,  // 移動の向き
    pub timer: Timer,     // 移動のタイマー
    pub base_speed: f32,  // 基準の移動速度
    pub speedup: f32,     // スピードアップの係数
    pub is_stop: bool,    // 移動停止フラグ
    pub px_start: Vec2,   // 1フレームに移動した微小区間の始点
    pub px_end: Vec2,     // 1フレームに移動した微小区間の終点
    pub fn_autodrive: Option<FnAutoDrive>, // 自走プレイヤー(デモ時)の移動方向を決める関数
    pub anime: SpriteAnimationParams,      // スプライトアニメーションの情報
}

// プレイヤーの初期状態
impl Default for Player
{
    fn default() -> Self
    {
        const PLAYER_TIME_PER_GRID: f32 = 0.15; // 0.09; //１グリッド進むために必要な時間

        Self {
            grid: IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer: Timer::from_seconds(PLAYER_TIME_PER_GRID, TimerMode::Once),
            base_speed: PIXELS_PER_GRID / PLAYER_TIME_PER_GRID,
            speedup: 1.0,
            is_stop: true,
            px_start: Vec2::default(),
            px_end: Vec2::default(),
            fn_autodrive: None,
            anime: SpriteAnimationParams::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 自走プレイヤー(デモ時)の移動方向を決める関数（関数ポインタ）
type FnAutoDrive = fn(
    &Player,                //プレイヤーのComponent
    Query<&chaser::Chaser>, //チェイサーのComponent
    Res<Map>,               //マップ
    Res<DemoMapParams>,     //デモ用情報
    &[News],                //
) -> News;

////////////////////////////////////////////////////////////////////////////////

//スプライトアニメーションの情報
pub struct SpriteAnimationParams
{
    pub timer: Timer,                               // タイマー
    pub num_patterns: u32,                          // フレーム数
    pub sprite_sheet_offsets: FxHashMap<News, u32>, // 先頭位置(offset値)
}

// スプライトシートの情報１
impl Default for SpriteAnimationParams
{
    fn default() -> Self
    {
        const NUM_PATTERNS: u32 = 4; // パターン数(TextureAtlasLayoutのcolumns)
        Self {
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            num_patterns: NUM_PATTERNS,
            sprite_sheet_offsets: FxHashMap::from_iter([
                (News::North, 0),
                (News::East, NUM_PATTERNS),
                (News::West, NUM_PATTERNS * 2),
                (News::South, NUM_PATTERNS * 3),
            ]),
        }
    }
}

// スプライトシートの情報２（TextureAtlasLayout）
pub static PLAYER_SPRITESHEET_LAYOUT: LazyLock<TextureAtlasLayout> =
    LazyLock::new(|| {
        TextureAtlasLayout::from_grid(
            UVec2::new(8, 8), // １セルの縦横px
            4,                // columns（アニメのパターン数）
            4,                // rows（上下左右の向きで４つ）
            None,             // padding
            None,             // offset
        )
    });

////////////////////////////////////////////////////////////////////////////////

// 三角形メッシュの情報
pub const PLAYER_SPRITE_RADIUS: f32 = PIXELS_PER_GRID * 0.4; // 半径
pub const PLAYER_SPRITE_COLOR: Color = Color::Srgba(css::YELLOW); // 色

////////////////////////////////////////////////////////////////////////////////

// End of code.
