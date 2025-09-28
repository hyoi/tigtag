use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーのComponent
#[derive(Component)]
pub struct Player
{
    pub cell: IVec2,      // 移動元の座標（停止中はその場の座標）
    pub next_cell: IVec2, // 移動先の座標（停止中はその場の座標）
    pub px_start: Vec3,   // 1フレームに移動した微小区間の始点
    pub px_end: Vec3,     // 1フレームに移動した微小区間の終点
    pub direction: News,  // 移動の向き
    pub speedup: f32,     // スピードアップの係数
    pub timer: Timer,     // 移動のタイマー
    pub is_stop: bool,    // 移動停止フラグ
    pub base_speed: f32,  // 基準の移動速度
    pub anime: SpriteAnimationParams, // スプライトアニメーションの情報
}

// プレイヤーの初期状態
impl Default for Player
{
    fn default() -> Self
    {
        const PLAYER_TIME_PER_GRID: f32 = 0.15; // 0.09; //１グリッド進むために必要な時間

        Self {
            cell: IVec2::default(),
            next_cell: IVec2::default(),
            px_start: Vec3::default(),
            px_end: Vec3::default(),
            direction: News::South,
            speedup: 1.0,
            timer: Timer::from_seconds(PLAYER_TIME_PER_GRID, TimerMode::Once),
            is_stop: true,
            base_speed: PIXELS_PER_GRID / PLAYER_TIME_PER_GRID,
            anime: SpriteAnimationParams {
                timer: Timer::from_seconds(ANIME_TIMER_PLAYER, TimerMode::Repeating),
                ..default()
            },
        }
    }
}

// スプライトアニメーションの切替間隔
const ANIME_TIMER_PLAYER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

// 三角形メッシュの情報
pub const PLAYER_SPRITE_RADIUS: f32 = PIXELS_PER_GRID * 0.4; // 半径
pub const PLAYER_SPRITE_COLOR: Color = Color::Srgba(css::YELLOW); // 色

////////////////////////////////////////////////////////////////////////////////

// マップのドット配置情報（demo用）
// #[derive(Resource, Default)]
// pub struct DemoMapParams
// {
//     dots_sum_y: [i32; map::MAP_HEIGHT_IN_CELLS as usize], // 行に残っているdotsを数えた配列
//     dots_sum_x: [i32; map::MAP_WIDTH_IN_CELLS as usize], // 列に残っているdotsを数えた配列
//     dots_rect: IVec2Rect,                                // 全dotを内包する最小の矩形
// }

// #[derive(Default)]
// struct IVec2Rect
// {
//     min: IVec2,
//     max: IVec2,
// }

// impl DemoMapParams
// {
//     pub fn dots_sum_y(&self, y: i32) -> i32 { self.dots_sum_y[y as usize] }
//     pub fn dots_sum_y_mut(&mut self, y: i32) -> &mut i32
//     {
//         &mut self.dots_sum_y[y as usize]
//     }
//     pub fn dots_sum_x(&self, x: i32) -> i32 { self.dots_sum_x[x as usize] }
//     pub fn dots_sum_x_mut(&mut self, x: i32) -> &mut i32
//     {
//         &mut self.dots_sum_x[x as usize]
//     }

//     pub fn dots_rect_min(&self) -> IVec2 { self.dots_rect.min }
//     pub fn dots_rect_min_mut(&mut self) -> &mut IVec2 { &mut self.dots_rect.min }
//     pub fn dots_rect_max(&self) -> IVec2 { self.dots_rect.max }
//     pub fn dots_rect_max_mut(&mut self) -> &mut IVec2 { &mut self.dots_rect.max }
// }

////////////////////////////////////////////////////////////////////////////////

// 自走プレイヤー(デモ時)の移動方向を決める関数（関数ポインタ）
// #[derive(Resource)]
// #[allow(clippy::type_complexity)]
// pub struct DemoAutoDriveFn(
//     pub  fn(
//         &Player,                // プレイヤーのComponent
//         Query<&chaser::Chaser>, // チェイサーのComponent
//         Res<map::Map>,          // マップ
//         Res<DemoMapParams>,     // デモ用情報
//         &[News],                // プレイヤーがいるセルの四方の道のリスト
//     ) -> News,
// );

////////////////////////////////////////////////////////////////////////////////

// End of code.
