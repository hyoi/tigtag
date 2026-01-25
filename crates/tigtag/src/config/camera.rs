use super::*;

////////////////////////////////////////////////////////////////////////////////

// カメラのレンダリング順序
pub const CAMERA_ORDER_OVERLAY: isize = 10; // 最前列
// pub const CAMERA_ORDER_UI: isize = 3;
// pub const CAMERA_ORDER_2D: isize = 2;
// pub const CAMERA_ORDER_3D: isize = 1;
// pub const CAMERA_ORDER_BG: isize = 0; // 最後列

// カメラの背景色
pub const CAMERA_BG_COLOR_OVERLAY: Color = Color::BLACK;
// pub const CAMERA_BG_COLOR_UI: Color = Color::NONE;
// pub const CAMERA_BG_COLOR_2D: Color = Color::NONE;
// pub const CAMERA_BG_COLOR_3D: Color = Color::NONE;
// pub const CAMERA_BG_COLOR_BG: Color = Color::BLACK; // 背景色

// レンダーレイヤの番号（CameraとEntityのバインド用）
// pub const RENDER_LAYER_OVERLAY: usize = CAMERA_ORDER_OVERLAY as usize;
// pub const RENDER_LAYER_UI: usize = CAMERA_ORDER_UI as usize;
// pub const RENDER_LAYER_2D: usize = CAMERA_ORDER_2D as usize;
// pub const RENDER_LAYER_3D: usize = CAMERA_ORDER_3D as usize;
// pub const RENDER_LAYER_BG: usize = CAMERA_ORDER_BG as usize;

// 2Dカメラの座標
// xy座標平面の第四象限を利用。左上隅が(0,0)、X軸はプラス方向、Y軸はマイナス方向へ伸びる
pub const POSITION_CAMERA_2D: Vec3 = Vec3::new(
    WINDOW_PIXELS_WIDTH * 0.5,
    WINDOW_PIXELS_HEIGHT * -0.5,
    999.0, // 2Dカメラなのでz軸は無視される？
);

// 3Dカメラの座標
// pub const POSITION_CAMERA_3D: Vec3 = Vec3::new(0.0, 7.0, 14.0);

// 3Dカメラの座標（球座標）
// pub const POSITION_CAMERA_3D_ORBIT: orbit_camera::Spherical =
//     orbit_camera::Spherical {
//         r: 8.0,          // 中心からの距離（球の半径）
//         theta: PI * 0.6, // 1.0:天頂、0.5:真横、0.0:真下
//         phi: TAU * 0.9,  // xz平面（時計の6時方向が0.0で反時計回り）
//     };

////////////////////////////////////////////////////////////////////////////////

// カメラのComponent
// #[derive(Component, Clone)]
// pub struct SimpleCamUi;
// #[derive(Component, Clone)]
// pub struct SimpleCam2d;
// #[derive(Component, Clone)]
// pub struct SimpleCam3dOrbit;

// カメラ生成の情報を格納するResource
// #[derive(Resource, Deref, DerefMut)]
// pub struct CameraSettings(pub Vec<simple_camera::Setting>);

// カメラ情報の初期化
// impl Default for CameraSettings
// {
//     fn default() -> Self
//     {
//         Self(vec![
//             simple_camera::Setting::from((
//                 CAMERA_ORDER_UI,    // カメラのレンダリング優先順（0が最後）
//                 CAMERA_BG_COLOR_UI, // レンダリング時の背景色（NONEは透明）
//                 SimpleCamUi,        // マーカー（Component）
//                 Camera2d,           // カメラ種類（Component）
//                 Transform::from_translation(POSITION_CAMERA_2D), // カメラの位置
//             )),
//             simple_camera::Setting::from((
//                 CAMERA_ORDER_2D,
//                 CAMERA_BG_COLOR_2D,
//                 SimpleCam2d,
//                 Camera2d,
//                 Transform::from_translation(POSITION_CAMERA_2D),
//             )),
//             simple_camera::Setting::from((
//                 CAMERA_ORDER_3D,
//                 CAMERA_BG_COLOR_3D,
//                 SimpleCam3dOrbit,
//                 Camera3d::default(),
//                 Transform::from_translation(POSITION_CAMERA_3D)
//                     .looking_at(Vec3::ZERO, Vec3::Y),
//             )),
//         ])
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
