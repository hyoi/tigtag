use super::*;

////////////////////////////////////////////////////////////////////////////////

// (i32, i32)とIVec2を「スクリーン第四象限のピクセル座標(Vec2)」へ変換する拡張トレイト
pub trait I32x2TypeExt
{
    const UNIT: Vec2 = Vec2::new(PIXELS_PER_CELL, -PIXELS_PER_CELL); // Y軸は負方向
    const ADJUSTOR: f32 = 0.5; // アンカーがグリッド中央なので補正(0.5)が必要
    fn to_screen_pixels(&self) -> Vec2;
}

impl I32x2TypeExt for (i32, i32)
{
    fn to_screen_pixels(&self) -> Vec2
    {
        (IVec2::from(*self).as_vec2() + Self::ADJUSTOR) * Self::UNIT
    }
}
impl I32x2TypeExt for IVec2
{
    fn to_screen_pixels(&self) -> Vec2
    {
        (self.as_vec2() + Self::ADJUSTOR) * Self::UNIT
    }
}

////////////////////////////////////////////////////////////////////////////////

// 指定されたMyStateへ遷移するSystem
pub fn set_next_state(my_state: MyState) -> impl FnMut(ResMut<NextState<MyState>>)
{
    move |mut next_state: ResMut<NextState<MyState>>| {
        next_state.set(my_state);
        info!("into {:?} state", my_state);
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
