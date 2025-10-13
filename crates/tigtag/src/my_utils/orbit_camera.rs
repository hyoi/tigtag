#![allow(dead_code)]
use super::*;

////////////////////////////////////////////////////////////////////////////////

// 球体座標の型
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Spherical
{
    pub r: f32,     // 中心点から飛翔体までの距離
    pub theta: f32, // 真下(-Y)を0.0、XZ平面上をπ×0.5、天頂(+Y)をπ×1.0とする仰角
    pub phi: f32,   // XZ平面中心からZ軸方向を起点とし反時計回りの回転角
}

// 球体座標から直交座標への変換
impl From<Spherical> for Vec3
{
    fn from(s: Spherical) -> Self
    {
        // [0, π] を[-π/2, π/2]へ
        let theta = s.theta - PI * 0.5;

        // rのXZ平面への射影ベクトルの長さ
        let r_xz = s.r * theta.cos();

        // 直交座標へ変換
        let x = r_xz * s.phi.sin();
        let y = s.r * theta.sin();
        let z = r_xz * s.phi.cos();

        Vec3::new(x, y, z)
    }
}

// 直交座標から球体座標への変換
impl From<Vec3> for Spherical
{
    fn from(vec3: Vec3) -> Self
    {
        // r
        let r = vec3.length();
        const ZERO_TOLERANCE: f32 = 1e-2;
        if r < ZERO_TOLERANCE
        {
            return Spherical::default(); // ALL Zero
        }

        // theta
        let theta = (vec3.y / r).asin() + PI * 0.5;

        // phi
        let mut phi = vec3.x.atan2(vec3.z); // -π～π
        if phi < 0.0
        {
            phi += TAU; // 0～2πに正規化
        }

        Spherical { r, theta, phi }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 球体座標カメラのResource
#[derive(Resource, Default, Clone, Copy)]
pub struct OrbitCamera
{
    pub spherical: Spherical,    // 球体座標上のカメラの位置
    pub look_at: (Vec3, Vec3),   // 注視点、視線軸上のロール
    pub clamp_r: (f32, f32),     // 球体座標のRのminとmax
    pub clamp_theta: (f32, f32), // 球体座標のθのminとmax
}

// 球体座標を範囲内に収める
impl OrbitCamera
{
    fn clamp(&mut self) -> &mut Self
    {
        // 準備
        let spherical = &mut self.spherical;
        let (min_r, nax_r) = self.clamp_r;
        let (min_theta, max_theta) = self.clamp_theta;

        // 範囲外なら書き換える
        spherical.r = spherical.r.clamp(min_r, nax_r);
        spherical.theta = spherical.theta.clamp(min_theta, max_theta);
        spherical.phi %= TAU; //(-TAU..TAU)

        self
    }
}

////////////////////////////////////////////////////////////////////////////////

// 球座標に従って3D Cameraを移動する
#[allow(clippy::type_complexity)]
pub fn move_orbit_camera<T: Component>(
    mut query_camera: Query<(&mut Transform, &Camera), (With<Camera3d>, With<T>)>,
    option_orbit_camera: Option<ResMut<OrbitCamera>>,
    time: Res<Time>,
    mut cmds: Commands,
    mut message_reader: MessageReader<handle_input::MessUserAction>,
) -> Result
{
    // 準備
    let (mut transform, camera) = query_camera.single_mut()?;

    // OrbitCameraのResourceが登録済なら
    let Some(mut orbit_camera) = option_orbit_camera
    else
    {
        // 未登録なら初期化・登録する
        let orbit_camera = OrbitCamera {
            spherical: Spherical::from(transform.translation),
            look_at: (Vec3::ZERO, Vec3::Y),
            clamp_r: (1.0, 10.0),
            clamp_theta: (PI * 0.5, PI),
        };
        cmds.insert_resource(orbit_camera);

        // 一旦returnする
        return Ok(());
    };

    // カメラが無効化されているなら
    if !camera.is_active
    {
        return Ok(());
    }

    // 入力（UserAction,value）をフラットなVecにまとめる
    let mut flat_messages = Vec::<(handle_input::UserAction, f32)>::new();
    message_reader
        .read()
        .for_each(|x| flat_messages.extend(x.0.clone()));

    // 最終的な入力にまとめる
    let mut input_value = Vec3::ZERO;
    flat_messages.iter().for_each(|&(action, value)| {
        use handle_input::UserAction::*;
        #[rustfmt::skip]
        let xyz_v = match action
        {
            // デジタル入力
            MoveUp    => (&mut input_value.y,  value),
            MoveDown  => (&mut input_value.y, -value),
            MoveLeft  => (&mut input_value.x, -value),
            MoveRight => (&mut input_value.x,  value),
            ZoomIn    => (&mut input_value.z, -value),
            ZoomOut   => (&mut input_value.z,  value),
            // アナログ入力
            AxisVertNormal   (x) => (&mut input_value.y,  value * x),
            AxisVertReverse  (x) => (&mut input_value.y, -value * x),
            AxisHorizNormal  (x) => (&mut input_value.x,  value * x),
            AxisHorizReverse (x) => (&mut input_value.x, -value * x),
            AxisZoomNormal   (x) => (&mut input_value.z,  value * x),
            AxisZoomReverse  (x) => (&mut input_value.z, -value * x),
            // エラー
            _ => unreachable!("Invalid value(UserAction) in the message buffer."),
        };
        *xyz_v.0 = fold_rule(*xyz_v.0, xyz_v.1);
    });

    // 球体座標を更新する
    let delta_time = time.delta_secs();
    let delta_camera_move = input_value * delta_time;
    orbit_camera.spherical.r += delta_camera_move.z;
    orbit_camera.spherical.theta += delta_camera_move.y;
    orbit_camera.spherical.phi += delta_camera_move.x;

    // カメラを移動する
    orbit_camera.clamp(); // 球座標を範囲内に収める
    let origin = orbit_camera.look_at.0; // 注視点（直交座標）
    let roll = orbit_camera.look_at.1; // カメラの視線を軸にしたロール
    let translation = origin + Vec3::from(orbit_camera.spherical);
    *transform = Transform::from_translation(translation).looking_at(origin, roll);

    Ok(())
}

// 各デバイスからの入力値を畳み込んで1つにまとめる際のルール
// 　１）aとbの符号が異なる ➡ 合計を返す
// 　２）aとbの符号が一致する ➡ 絶対値が大きい方を返す
#[rustfmt::skip]
fn fold_rule(a: f32, b: f32) -> f32
{
    // aとbの符号が違うなら
    if a * b <= 0.0
        { a + b }
    else if a.abs() > b.abs()
        { a } else { b }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
