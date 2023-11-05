use super::*;

////////////////////////////////////////////////////////////////////////////////

//カウントダウン用のResource
#[derive( Resource )]
pub struct CountDownTimer
{   pub counter: i32,   //カウンター
    pub timer  : Timer, //１秒タイマー
}
impl Default for CountDownTimer
{   fn default() -> Self
    {   Self
        {   counter: 0,
            timer  : Timer::from_seconds( 1.0, TimerMode::Once ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//Center UIの基本トレイト
pub trait TextUI
{   fn message( &self ) -> &[ MessageSect ];
}

//カウントダウンのトレイト
pub trait CountDown
{   fn initial_count( &self ) -> i32;
    fn next_state( &self ) -> MyState;
    fn to_string( &self, n: i32 ) -> String;
    fn placeholder( &self ) -> Option<usize>;
}

//Hit ANY Key! のトレイト
pub trait HitAnyKey
{   fn shortcut( &self ) -> MyState;
}

////////////////////////////////////////////////////////////////////////////////

//ゲームスタートメッセージのComponent
#[derive( Component, Clone, Copy )]
pub struct Start<'a>
{   count     : i32,
    next_state: MyState,
    message   : &'a [ MessageSect ],
    string    : fn ( i32 ) -> String,
}

impl<'a> TextUI for Start<'a>
{   fn message( &self ) -> &[ MessageSect ] { self.message }
}

impl<'a> CountDown for Start<'a>
{   fn initial_count( &self ) -> i32 { self.count + 1 }
    fn next_state( &self ) -> MyState { self.next_state }
    fn to_string( &self, n: i32 ) -> String { ( self.string )( n ) }
    fn placeholder( &self ) -> Option<usize> { self.message.iter().position( |x| x.0 == CDPH ) }
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

//ステージクリアメッセージのComponent
#[derive( Component, Clone, Copy )]
pub struct Clear<'a>
{   count     : i32,
    next_state: MyState,
    message   : &'a [ MessageSect ],
    string    : fn ( i32 ) -> String,
}

impl<'a> TextUI for Clear<'a>
{   fn message( &self ) -> &[ MessageSect ] { self.message }
}

impl<'a> CountDown for Clear<'a>
{   fn initial_count( &self ) -> i32 { self.count + 1 }
    fn next_state( &self ) -> MyState { self.next_state }
    fn to_string( &self, n: i32 ) -> String { ( self.string )( n ) }
    fn placeholder( &self ) -> Option<usize> { self.message.iter().position( |x| x.0 == CDPH ) }
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

//ゲームオーバーメッセージのComponent
#[derive( Component, Clone, Copy )]
pub struct Over<'a>
{   count     : i32,
    next_state: MyState,
    message   : &'a [ MessageSect ],
    string    : fn ( i32 ) -> String,
    shortcut  : MyState,
}

impl<'a> TextUI for Over<'a>
{   fn message( &self ) -> &[ MessageSect ] { self.message }
}

impl<'a> CountDown for Over<'a>
{   fn initial_count( &self ) -> i32 { self.count + 1 }
    fn next_state( &self ) -> MyState { self.next_state }
    fn to_string( &self, n: i32 ) -> String { ( self.string )( n ) }
    fn placeholder( &self ) -> Option<usize> { self.message.iter().position( |x| x.0 == CDPH ) }
}

impl<'a> HitAnyKey for Over<'a>
{   fn shortcut( &self ) -> MyState { self.shortcut }
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

////////////////////////////////////////////////////////////////////////////////

//タイトルのComponent
#[derive( Component, Clone, Copy )]
pub struct Title<'a>
{   title: &'a [ MessageSect ],
    demo : &'a [ MessageSect ],
    shortcut: MyState,
}

trait TitleUI
{   fn title( &self ) -> &[ MessageSect ];
    fn demo ( &self ) -> &[ MessageSect ];
}

impl<'a> TitleUI for Title<'a>
{   fn title( &self ) -> &[ MessageSect ] { self.title }
    fn demo ( &self ) -> &[ MessageSect ] { self.demo  }
}

impl<'a> HitAnyKey for Title<'a>
{   fn shortcut( &self ) -> MyState { self.shortcut }
}

impl<'a> Default for Title<'a>
{   fn default() -> Self
    {   Self
        {   title: UI_TITLE,
            demo : UI_DEMO,
            shortcut: MyState::StageStart,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

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
    let title_frame = NodeBundle
    {   style: Style
        {   flex_direction: FlexDirection::Column,
            align_items   : AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor( Color::NONE ),
        ..default()
    };
    let child_id = cmds.spawn( ( title_frame, component ) ).with_children
    (   | cmds |
        {   cmds.spawn( ui_title );
            cmds.spawn( ui_demo  );
        }
    ).id();
    cmds.entity( hidden_frame ).add_child( child_id );
}

//UIをspawnする
pub fn spawn_in_hidden_frame<T: Component + Default + Copy + TextUI>
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

////////////////////////////////////////////////////////////////////////////////

//カウントダウンを初期化する
pub fn init_countdown<T: Component + CountDown>
(   qrt_ui: Query<&T>,
    opt_countdown: Option<ResMut<CountDownTimer>>,
)
{   let Ok ( ui ) = qrt_ui.get_single() else { return };
    let Some ( mut countdown ) = opt_countdown else { return };

    countdown.counter = ui.initial_count();
    countdown.timer.reset();
}

////////////////////////////////////////////////////////////////////////////////

//カウントダウンを表示しゼロになったらStateを変更する
pub fn counting_down<T: Component + CountDown>
(   mut qry_text_ui: Query<(&mut Text, &T)>,
    opt_countdown: Option<ResMut<CountDownTimer>>,
    mut next_state: ResMut<NextState<MyState>>,
    time: Res<Time>,
)
{   let Ok ( ( mut text, ui ) ) = qry_text_ui.get_single_mut() else { return };
    let Some ( placeholder ) = ui.placeholder() else { return };
    let Some ( mut countdown ) = opt_countdown else { return };

    //1秒経過したら
    if countdown.timer.tick( time.delta() ).finished()
    {   countdown.counter -= 1;  //カウントダウン
        countdown.timer.reset(); //1秒タイマーリセット
    }

    //カウントダウンが続いているなら
    if countdown.counter > 0
    {   //カウントダウンの表示を更新する
        let message = ui.to_string( countdown.counter - 1 );
        text.sections[ placeholder ].value = message;
    }
    else
    {   //そうでないならStateを変更する
        next_state.set( ui.next_state() );
    }
}

////////////////////////////////////////////////////////////////////////////////

//キー入力さたらStateを変更する
pub fn hit_any_key<T: Component + HitAnyKey>
(   qry_ui: Query<&T>,
    opt_gamepad: Option<Res<ConnectedGamepad>>,
    mut next_state: ResMut<NextState<MyState>>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   let Ok ( ui ) = qry_ui.get_single() else { return };

    //無視キー以外のキー入力はあるか
    for key in HAK_IGNORE_KEYS { if inkey.pressed( *key ) { return } }
    let mut is_pressed = inkey.get_just_pressed().len();

    //無視ボタン以外のボタン入力はあるか
    if is_pressed == 0
    {   let Some ( gamepad ) = opt_gamepad else { return };
        let Some ( id ) = gamepad.id() else { return };
        for buton in HAK_IGNORE_BUTTONS
        {   if inbtn.pressed( GamepadButton::new( id, *buton ) ) { return }
        }
        is_pressed = inbtn.get_just_pressed().filter( |x| x.gamepad == id ).count();
    }

    //Stateを遷移させる
    if is_pressed > 0
    {   next_state.set( ui.shortcut() );
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.