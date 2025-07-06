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
    pub is_stop: bool,    // 移動停止フラグ
    pub speedup: f32,     // スピードアップ係数
    pub px_start: Vec2,   // 1フレームに移動した微小区間の始点
    pub px_end: Vec2,     // 1フレームに移動した微小区間の終点
    pub fn_autodrive: Option<FnAutoDrive>, // 自走プレイヤー(デモ時)の移動方向を決める関数
    pub anime: SpriteAnimationParams,      // スプライトアニメーションの情報
}

// 関数のポインタ型（自走プレイヤー(デモ時)の移動方向を決める関数）
type FnAutoDrive = fn(
    &Player,            //プレイヤーのComponent
    Query<&Chaser>,     //チェイサーのComponent
    Res<Map>,           //マップ
    Res<DemoMapParams>, //デモ用情報
    &[News],            //
) -> News;

pub struct SpriteAnimationParams
{
    pub timer: Timer,            // スプライトアニメーションのタイマー
    pub sprite_sheet_frame: u32, // スプライトアニメーションのフレーム数
    pub sprite_sheet_indexes: FxHashMap<News, u32>, // スプライトアニメーションの先頭位置(offset値)
}

impl Default for Player
{
    fn default() -> Self
    {
        Self {
            grid: IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer: Timer::from_seconds(PLAYER_TIME_PER_GRID, TimerMode::Once),
            is_stop: true,
            speedup: 1.0,
            px_start: Vec2::default(),
            px_end: Vec2::default(),
            fn_autodrive: None,
            anime: SpriteAnimationParams::default(),
        }
    }
}

impl Default for SpriteAnimationParams
{
    fn default() -> Self
    {
        Self {
            timer: Timer::from_seconds(ANIME_TIMER_PLAYER, TimerMode::Repeating),
            sprite_sheet_frame: SPRITE_SHEET_COLS_PLAYER,
            sprite_sheet_indexes: (*SPRITE_SHEET_IDXS_PLAYER).clone(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの設定値
pub const PLAYER_TIME_PER_GRID: f32 = 0.15; // 0.09; //１グリッド進むために必要な時間
pub const PLAYER_SPEED: f32 = PIXELS_PER_GRID / PLAYER_TIME_PER_GRID; // 速度
pub const PLAYER_SPRITE_SCALING: f32 = 0.4; // primitive shape表示時の縮小係数
pub const PLAYER_SPRITE_COLOR: Color = Color::Srgba(css::YELLOW);

// スプライトシートを使ったアニメーションの情報
pub const SPRITE_SHEET_SIZE_PLAYER: UVec2 = UVec2::new(8, 8);
pub const SPRITE_SHEET_COLS_PLAYER: u32 = 4;
pub const SPRITE_SHEET_ROWS_PLAYER: u32 = 4;
pub static SPRITE_SHEET_IDXS_PLAYER: LazyLock<FxHashMap<News, u32>> =
    LazyLock::new(|| {
        FxHashMap::from_iter([
            (News::North, 0),
            (News::East, 4),
            (News::West, 8),
            (News::South, 12),
        ])
    });
pub const ANIME_TIMER_PLAYER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

// スプライトシートでアニメーションするためのトレイト実装
impl CharacterAnimation for Player
{
    fn anime_timer_mut(&mut self) -> &mut Timer { &mut self.anime.timer }
    fn sprite_sheet_frame(&self) -> u32 { self.anime.sprite_sheet_frame }
    fn sprite_sheet_offset(&self, news: News) -> u32
    {
        *self.anime.sprite_sheet_indexes.get(&news).unwrap()
    }
    fn direction(&self) -> News { self.direction }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
