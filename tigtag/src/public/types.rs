use super::*;

////////////////////////////////////////////////////////////////////////////////

//ゲームの状態
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States, MyState )]
pub enum MyState
{   #[default] LoadAssets,
    InitApp,
    InitGame,
    TitleDemo, DemoLoop,
    StageStart, MainLoop, StageClear,
    GameOver,
    Pause,
}

impl MyState
{   pub fn is_pause   ( &self ) -> bool { *self == MyState::Pause }
    pub fn is_demoplay( &self ) -> bool { *self == MyState::TitleDemo || *self == MyState::DemoLoop }
}

////////////////////////////////////////////////////////////////////////////////

//操作を受付けるgamepadのIDを保存するResource
#[derive( Resource, Default )]
pub struct TargetGamepad ( Option<Gamepad> );

impl TargetGamepad
{   pub fn id    ( &    self ) ->      Option<Gamepad> {      self.0 }
    pub fn id_mut( &mut self ) -> &mut Option<Gamepad> { &mut self.0 }
}

////////////////////////////////////////////////////////////////////////////////

//glamの型にメソッドを追加する準備
pub trait AddOnTraitForIVec2
{   fn to_vec2_on_screen( &self ) -> Vec2;
}

//glamの型にメソッドを追加する
impl AddOnTraitForIVec2 for IVec2
{   //スプライト等のアンカーが左上ではなく中央にあるため「+0.5」する
    fn to_vec2_on_screen( &self ) -> Vec2
    {   ( self.as_vec2() + 0.5 ) * PIXELS_PER_GRID * Vec2::new( 1.0, -1.0 )
    }
}

////////////////////////////////////////////////////////////////////////////////

//極座標の型
#[derive( Clone, Copy )]
pub struct Orbit
{   pub r    : f32, //極座標のr（注目点からカメラまでの距離）
    pub theta: f32, //極座標のΘ（注目点から見たカメラの垂直角度）
    pub phi  : f32, //極座標のφ（注目点から見たカメラの水平角度）
}

impl Default for Orbit
{   fn default() -> Self
    {   Self
        {   r    : CAMERA_ORBIT_INIT_R,
            theta: CAMERA_ORBIT_INIT_THETA,
            phi  : CAMERA_ORBIT_INIT_PHI,
        }
    }
}

impl Orbit
{   //極座標から直交座標へ変換する
    #[allow( clippy::wrong_self_convention )]
    pub fn to_vec3( &self ) -> Vec3
    {   let x = self.r * self.theta.sin() * self.phi.sin();
        let y = self.r * self.theta.cos() * -1.0;
        let z = self.r * self.theta.sin() * self.phi.cos();

        Vec3::new( x, y, z )
    }
}

////////////////////////////////////////////////////////////////////////////////

//極座標カメラのResource
#[derive( Resource, Clone, Copy )]
pub struct OrbitCamera
{   pub orbit: Orbit,    //カメラ自身の極座標
    pub look_at: Vec3,   //カメラの注視点の直交座標
    pub is_active: bool, //カメラがアクティブか否か
}

impl Default for OrbitCamera
{   fn default() -> Self
    {   Self
        {   orbit    : Orbit::default(),
            look_at  : Vec3::ZERO,
            is_active: false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//text UIのメッセージセクションの型
pub type MessageSect =
(   &'static str, //表示文字列
    &'static str, //フォントのAssets
    f32,   //フォントのサイズ
    Color, //フォントの色
);

////////////////////////////////////////////////////////////////////////////////

//基本図形に対する自前の糖衣構文（Bevy 0.13.0 Primitive Shapesへの対応）
pub struct Cube; //立方体
impl Cube
{   pub fn from_size( size: f32 ) -> Cuboid
    {   Cuboid::from_size( Vec3::splat( size ) )
    }
}

pub struct SquarePlane; //3D座標上の平面（正方形）
impl SquarePlane
{   pub fn from_size( size: f32 ) -> Mesh
    {   Plane3d::default().mesh().size( size, size ).build()
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.