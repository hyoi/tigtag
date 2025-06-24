use super::*;

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用の定数
pub const DEBUG: fn() -> bool = || cfg!(debug_assertions);
pub const WASM: fn() -> bool = || cfg!(target_arch = "wasm32");

////////////////////////////////////////////////////////////////////////////////

// Gridに関連する定数
pub const GRID_CUSTOM_SIZE: Vec2 = Vec2::new(PIXELS_PER_GRID, PIXELS_PER_GRID);
pub const GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;
pub const GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;

////////////////////////////////////////////////////////////////////////////////

//(i32,i32)型にメソッドを追加し、グリッド座標からピクセル座標への変換を容易にする
pub trait GridToVec2
{
    fn to_vec2_of_screen(&self) -> Vec2;
}

// スクリーン(第四象限)のピクセル座標(Vec2)へ変換する
// ※注意事項：1) Y軸は負方向。2) アンカーがグリッド中央なので補正(0.5)が必要
impl GridToVec2 for (i32, i32)
{
    fn to_vec2_of_screen(&self) -> Vec2
    {
        Vec2::new(self.0 as f32 + 0.5, -self.1 as f32 - 0.5) * PIXELS_PER_GRID
    }
}

////////////////////////////////////////////////////////////////////////////////

// 極座標の型
// #[derive(Default, Clone)]
// pub struct Orbit
// {
//     pub r: f32,     // 極座標のr（中心点から飛翔体までの距離）
//     pub theta: f32, // 極座標のΘ（中心点から見た飛翔体の仰角）
//     pub phi: f32,   // 極座標のφ（中心点から見た飛翔体の平面の回転角）
// }
// impl Orbit
// {
//     // 極座標から直交座標へ変換するメソッド
//     pub fn vec3(&self) -> Vec3
//     {
//         let x = self.r * self.theta.sin() * self.phi.sin();
//         let y = self.r * self.theta.cos() * -1.0;
//         let z = self.r * self.theta.sin() * self.phi.cos();

//         Vec3::new(x, y, z)
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
