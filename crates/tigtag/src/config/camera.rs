use super::*;

////////////////////////////////////////////////////////////////////////////////

// カメラのレンダリング順序
pub const CAMERA_ORDER_OVERLAY: isize = 10; // 最前列
pub const CAMERA_ORDER_2D: isize = 0; // 最後列

// カメラの背景色
pub const CAMERA_BG_COLOR_OVERLAY: Color = Color::BLACK;
pub const CAMERA_BG_COLOR_2D: Color = Color::BLACK; // 背景色

// 2Dカメラの座標
// xy座標平面の第四象限を利用。左上隅が(0,0)、X軸はプラス方向、Y軸はマイナス方向へ伸びる
pub const POSITION_CAMERA_2D: Vec3 = Vec3::new(
    WINDOW_PIXELS_WIDTH * 0.5,
    WINDOW_PIXELS_HEIGHT * -0.5,
    999.0, // 2Dカメラなのでz軸は無視される？
);

////////////////////////////////////////////////////////////////////////////////

// カメラのComponent
#[derive(Component, Clone)]
pub struct SimpleCam2d;

// カメラ生成の情報を格納するResource
#[derive(Resource, Deref, DerefMut)]
pub struct CameraSettings(pub Vec<simple_camera::Setting>);

// カメラ情報の初期化
impl Default for CameraSettings
{
    fn default() -> Self
    {
        Self(vec![
            simple_camera::Setting::from((
                CAMERA_ORDER_2D,
                CAMERA_BG_COLOR_2D,
                SimpleCam2d,
                Camera2d,
                Transform::from_translation(POSITION_CAMERA_2D),
            )),
        ])
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
