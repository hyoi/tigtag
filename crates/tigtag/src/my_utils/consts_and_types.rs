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

// グリッド座標からピクセル座標へ変換するメソッドの追加
pub trait GridToVec2
{
    fn to_vec2_of_screen(&self) -> Vec2;
}

// スクリーン(第四象限)のピクセル座標(Vec2)へ変換する
// Note: 1.Y軸は負方向。2.アンカーがグリッド中央なので補正(0.5)が必要
impl GridToVec2 for (i32, i32)
{
    fn to_vec2_of_screen(&self) -> Vec2
    {
        Vec2::new(self.0 as f32 + 0.5, -self.1 as f32 - 0.5) * PIXELS_PER_GRID
    }
}
impl GridToVec2 for IVec2
{
    fn to_vec2_of_screen(&self) -> Vec2
    {
        Vec2::new(self.x as f32 + 0.5, -self.y as f32 - 0.5) * PIXELS_PER_GRID
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
