use super::*;

////////////////////////////////////////////////////////////////////////////////

//UIレイアウト用隠しフレーム(中央)のComponent
#[derive( Component )] pub struct HiddenFrameCenter;

////////////////////////////////////////////////////////////////////////////////

//ゲームスタートメッセージのComponent
#[derive( Component, Clone, Copy )]
pub struct Start<'a>
{   count     : i32,
    next_state: MyState,
    message   : &'a [ MessageSect ],
    string    : fn ( i32 ) -> String,
}

impl<'a> Default for Start<'a>
{   fn default() -> Self
    {   Self
        {   count     : 5,
            next_state: MyState::MainLoop,
            message   : UI_START,
            string    : |n| if n == 0 { "Go!!".to_string() } else { n.to_string() },
        }
    }
}

impl<'a> effect::TextUI for Start<'a>
{   fn message( &self ) -> &[ MessageSect ] { self.message }
}

impl<'a> effect::CountDown for Start<'a>
{   fn initial_count( &self ) -> i32 { self.count + 1 }
    fn next_state( &self ) -> MyState { self.next_state }
    fn to_string( &self, n: i32 ) -> String { ( self.string )( n ) }
    fn placeholder( &self ) -> Option<usize> { self.message.iter().position( |x| x.0 == CDPH ) }
}

////////////////////////////////////////////////////////////////////////////////

//ステージクリアメッセージのComponent
#[derive( Component, Clone, Copy )]
pub struct Clear<'a>
{   count     : i32,
    next_state: MyState,
    message   : &'a [ MessageSect ],
    string    : fn ( i32 ) -> String,
}

impl<'a> Default for Clear<'a>
{   fn default() -> Self
    {   Self
        {   count     : 4,
            next_state: MyState::StageStart,
            message   : UI_CLEAR,
            string    : |n| ( n + 6 ).to_string(),
        }
    }
}

impl<'a> effect::TextUI for Clear<'a>
{   fn message( &self ) -> &[ MessageSect ] { self.message }
}

impl<'a> effect::CountDown for Clear<'a>
{   fn initial_count( &self ) -> i32 { self.count + 1 }
    fn next_state( &self ) -> MyState { self.next_state }
    fn to_string( &self, n: i32 ) -> String { ( self.string )( n ) }
    fn placeholder( &self ) -> Option<usize> { self.message.iter().position( |x| x.0 == CDPH ) }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームオーバーメッセージのComponent
#[derive( Component, Clone, Copy )]
pub struct Over<'a>
{   count     : i32,
    next_state: MyState,
    message   : &'a [ MessageSect ],
    string    : fn ( i32 ) -> String,
    shortcut  : MyState,
}

impl<'a> Default for Over<'a>
{   fn default() -> Self
    {   Self
        {   count     : 10,
            next_state: MyState::TitleDemo,
            message   : UI_OVER,
            string    : |n| n.to_string(),
            shortcut  : MyState::StageStart,
        }
    }
}

impl<'a> effect::TextUI for Over<'a>
{   fn message( &self ) -> &[ MessageSect ] { self.message }
}

impl<'a> effect::CountDown for Over<'a>
{   fn initial_count( &self ) -> i32 { self.count + 1 }
    fn next_state( &self ) -> MyState { self.next_state }
    fn to_string( &self, n: i32 ) -> String { ( self.string )( n ) }
    fn placeholder( &self ) -> Option<usize> { self.message.iter().position( |x| x.0 == CDPH ) }
}

impl<'a> effect::HitAnyKey for Over<'a>
{   #[allow(clippy::misnamed_getters)]
    fn next_state( &self ) -> MyState { self.shortcut }
}

////////////////////////////////////////////////////////////////////////////////

//タイトルのComponent
#[derive( Component, Clone, Copy )]
pub struct Title<'a>
{   title: &'a [ MessageSect ],
    demo : &'a [ MessageSect ],
    next_state: MyState,
}

impl<'a> Default for Title<'a>
{   fn default() -> Self
    {   Self
        {   title: UI_TITLE,
            demo : UI_DEMO,
            next_state: MyState::StageStart,
        }
    }
}

trait TitleUI //effect.rsに移動できない理由がわからない
{   fn title( &self ) -> &[ MessageSect ];
    fn demo ( &self ) -> &[ MessageSect ];
}

impl<'a> TitleUI for Title<'a>
{   fn title( &self ) -> &[ MessageSect ] { self.title }
    fn demo ( &self ) -> &[ MessageSect ] { self.demo  }
}

impl<'a> effect::HitAnyKey for Title<'a>
{   fn next_state( &self ) -> MyState { self.next_state }
}

//タイトル下のDEMOの表示
#[derive( Component, Default )]
pub struct Demo ( f32 );

impl effect::BlinkingText for Demo
{   fn alpha( &mut self, time_delta: f32 ) -> f32
    {   let angle = &mut self.0;
        *angle += 360.0 * time_delta;
        *angle -= if *angle > 360.0 { 360.0 } else { 0.0 };

        ( *angle ).to_radians().sin() //sin波
    }
}

////////////////////////////////////////////////////////////////////////////////

//UI用の隠しフレームをspawnする
pub fn spawn_hidden_frame
(   mut cmds: Commands,
)
{   //隠しフレームを作成する(画面の中央 寄せ)
    let hidden_frame_middle = misc::hidden_ui_frame( JustifyContent::Center );

    //隠しフレームを作成する
    cmds.spawn( ( hidden_frame_middle, HiddenFrameCenter ) );
}

////////////////////////////////////////////////////////////////////////////////

//UIをspawnする
pub fn spawn_in_hidden_frame<T: Component + Default + Copy + effect::TextUI>
(   component: Local<T>,
    qry_hidden_frame: Query<Entity, With<HiddenFrameCenter>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_frame ) = qry_hidden_frame.get_single() else { return };

    //メッセージの準備
    let mut ui = misc::text_ui( component.message(), &asset_svr );
    ui.visibility = Visibility::Hidden; //初期状態

    //レイアウト用の隠しフレームの中に子要素を作成する
    let child_id = cmds.spawn( ( ui, *component ) ).id();
    cmds.entity( hidden_frame ).add_child( child_id );
}

//タイトルをspawnする
pub fn spawn_title
(   qry_hidden_frame: Query<Entity, With<HiddenFrameCenter>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_frame ) = qry_hidden_frame.get_single() else { return };

    //メッセージの準備
    let component = Title::default();
    let mut ui_title = misc::text_ui( component.title(), &asset_svr );
    let mut ui_demo  = misc::text_ui( component.demo (), &asset_svr );
    ui_title.text.alignment = TextAlignment::Right;  //右寄せ
    ui_demo.text.alignment  = TextAlignment::Center; //センタリング
    ui_title.style.position_type = PositionType::Relative;
    ui_demo.style.position_type  = PositionType::Relative;
    ui_title.visibility = Visibility::Inherited; //親のvisibility.is_visibleで表示を制御する
    ui_demo.visibility  = Visibility::Inherited; //親のvisibility.is_visibleで表示を制御する

    //レイアウト用の隠しフレームの中に子要素を作成する
    let border = UiRect::all( Val::Px( 1.0 ) );
    let title_frame = NodeBundle
    {   style: Style
        {   flex_direction: FlexDirection::Column,
            align_items   : AlignItems::Center,
            border,
            ..default()
        },
        background_color: Color::NONE.into(),
        // border_color: Color::RED.into(),
        ..default()
    };
    let child_id = cmds.spawn( ( title_frame, component ) ).with_children
    (   | cmds |
        {   cmds.spawn( ui_title );
            cmds.spawn( ( ui_demo, Demo::default() ) );
        }
    ).id();
    cmds.entity( hidden_frame ).add_child( child_id );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.