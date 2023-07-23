use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //メッセージをspawn
        .add_systems( OnExit ( MyState::InitApp ), spawn_text_ui_pause )

        //Pause処理
        .add_systems( Update, pause_with_specified_key_button )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//PauseのText UI
counted_array!
(   const PAUSE_TEXT: [ MessageSect; _ ] =
    [   (   "P A U S E",
            ASSETS_FONT_ORBITRON_BLACK,
            PIXELS_PER_GRID * 4.0,
            Color::SILVER,
        ),
    ]
);

//Text UIのComponent
#[derive( Component )]
struct TextUiPause;

////////////////////////////////////////////////////////////////////////////////

//Text UIをspawnする
fn spawn_text_ui_pause
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //レイアウト用の隠しフレームを作る
    let per100 = Val::Percent( 100.0 );
    let background_color = BackgroundColor ( Color::NONE );
    let hidden_frame = NodeBundle
    {   style: Style
        {   width          : per100,
            height         : per100,
            position_type  : PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items    : AlignItems::Center,
            ..default()
        },
        background_color,
        ..default()
    };

    //Text UIを作る
    let mut ui = misc::text_ui( &PAUSE_TEXT, TextAlignment::Center, &asset_svr );
    ui.visibility = Visibility::Hidden;
    ui.z_index    = ZINDEX_TEXTUI_PAUSE;

    //隠しフレームの中に子要素を作成する
    cmds
    .spawn( hidden_frame )
    .with_children( | cmds | { cmds.spawn( ( ui, TextUiPause ) ); } )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//一時停止する
fn pause_with_specified_key_button
(   q_text: Query<&mut Visibility, With<TextUiPause>>,
    mut state: ResMut<State<MyState>>,
    mut old_state: Local<MyState>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    o_now_gamepad: Option<Res<NowGamepad>>,
)
{   //キーの状態
    let is_key_pressed = inkey.just_pressed( PAUSE_KEY );

    //パッドのボタンの状態
    let mut is_btn_pressed = false;
    if let Some ( now_gamepad ) = o_now_gamepad
    {   if let Some ( gamepad ) = now_gamepad.0
        {   if inbtn.just_pressed( GamepadButton::new( gamepad, PAUSE_BUTTON ) )
            {   is_btn_pressed = true;
            }
        }
    }

    //入力がないなら
    if ! is_key_pressed && ! is_btn_pressed { return }

    //PAUSEのトグル処理
    if state.get().is_pause()
    {   misc::hide_component( q_text );
        *state = State::new( *old_state ); //遷移先のOnEnterを実行しない
    }
    else
    {   misc::show_component( q_text );
        *old_state = *state.get();
        *state = State::new( MyState::Pause ); //遷移元のOnExitを実行しない
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.