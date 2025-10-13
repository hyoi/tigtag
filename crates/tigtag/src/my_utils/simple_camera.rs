use super::*;

////////////////////////////////////////////////////////////////////////////////

// カメラをspawnするために必要な情報
pub struct Setting(
    isize,               // カメラのレンダリング優先度
    Color,               // レンダリング時の背景色
    Box<dyn BoxedTrait>, // Component×2（マーカーとカメラの種類）
    Transform,           // カメラの位置
);

// 型がバラバラなComponetをリストに収納する為のトレイト
pub trait BoxedTrait: Send + Sync + 'static
{
    // カメラのspawnメソッド
    fn spawn_camera(
        self: Box<Self>,
        cmds: &mut Commands,
        order: isize,
        color: Color,
        transform: Transform,
    );

    // SettingのCloneを実装する場合に必要になるメソッド
    // fn clone_box(&self) -> Box<dyn BoxedTrait>;
}

//------------------------------------------------------------------------------

// タプル(isize, Color, C1, C2, Transform)からSettingへの変換トレイト（From）の実装
impl<C1, C2> From<(isize, Color, C1, C2, Transform)> for Setting
where
    C1: Clone + 'static + Component,
    C2: Clone + 'static + Component + IsCamera,
{
    fn from(tuple: (isize, Color, C1, C2, Transform)) -> Self
    {
        Setting(tuple.0, tuple.1, Box::new((tuple.2, tuple.3)), tuple.4)
    }
}

// Camera2dとCamera3dを同列に扱うためのトレイト
trait IsCamera {}
impl IsCamera for Camera2d {}
impl IsCamera for Camera3d {}

// タプル(Component1, Component2)のBoxedTraitの実装
impl<C1, C2> BoxedTrait for (C1, C2)
where
    C1: Clone + 'static + Component,
    C2: Clone + 'static + Component + IsCamera,
{
    fn spawn_camera(
        self: Box<Self>,
        cmds: &mut Commands,
        order: isize,
        color: Color,
        transform: Transform,
    )
    {
        cmds.spawn((
            *self, // (Component1, Component2)
            Camera {
                order,
                clear_color: color.into(),
                viewport: gen_viewport(),
                ..default()
            },
            transform,
            Msaa::Sample4,
        ));
    }

    // SettingのCloneを実装する場合に必要になるメソッド
    // fn clone_box(&self) -> Box<dyn BoxedTrait> { Box::new(self.clone()) }
}

// SettingのCloneを実装する場合に必要になるメソッド
// impl Clone for Setting
// {
//     fn clone(&self) -> Self { Setting(self.0, self.1, self.2.clone_box(), self.3) }
// }

////////////////////////////////////////////////////////////////////////////////

// タイトルバーWクリックや最大化ボタンによるウィンドウ最大化、および
// WASMでCanvasへのfit(最大化)を設定した場合に表示が崩れることがある。
// それを緩和するためカメラにviewportを設定する場合に使う
fn gen_viewport() -> Option<Viewport>
{
    match ATTACH_VIEWPORT()
    {
        true => Some(Viewport {
            physical_position: UVec2::ZERO,
            physical_size: SCREEN_PIXELS_RESO,
            ..default()
        }),
        _ => None,
    }
}

////////////////////////////////////////////////////////////////////////////////

// リストを基にカメラをspawnするSystem
pub fn spawn<T: Resource + Deref<Target = Vec<Setting>> + DerefMut>(
    mut settings: ResMut<T>,
    mut cmds: Commands,
)
{
    settings.drain(..).for_each(
        |Setting(order, bg_color, boxed_trait, transform)| {
            boxed_trait.spawn_camera(&mut cmds, order, bg_color, transform)
        },
    );
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
