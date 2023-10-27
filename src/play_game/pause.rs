use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //PAUSE用UIをspawnする
        .add_systems( OnExit ( MyState::InitApp ), spawn_ui_pause )

        //PAUSE処理
        .add_systems( Update, pause )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//PauseのText
counted_array!
(   const PAUSE_TEXT: [ MessageSect; _ ] =
    [   (   "P A U S E",
            ASSETS_FONT_ORBITRON_BLACK,
            PIXELS_PER_GRID * 4.0,
            Color::SILVER,
        ),
    ]
);

//UIのComponent
#[derive( Component )] struct UiPause;

////////////////////////////////////////////////////////////////////////////////

//PAUSE用UIをspawnする
fn spawn_ui_pause
(   qry_hidden_frame: Query<Entity, With<HiddenFrameMiddle>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_frame ) = qry_hidden_frame.get_single() else { return };

    //PAUSEのメッセージの準備
    let mut ui = misc::text_ui( &PAUSE_TEXT, &asset_svr );
    ui.visibility = Visibility::Hidden; //初期状態

    //レイアウト用の隠しフレームの中に子要素を作成する
    let child_id = cmds.spawn( ( ui, UiPause ) ).id();
    cmds.entity( hidden_frame ).add_child( child_id );
}

////////////////////////////////////////////////////////////////////////////////

//PAUSE処理
fn pause
(   qry_text: Query<&mut Visibility, With<UiPause>>,
    opt_gamepad: Option<Res<ConnectedGamepad>>,
    mut state: ResMut<State<MyState>>,
    mut saved_in: Local<MyState>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   //キーの状態
    let mut is_pressed = inkey.just_pressed( PAUSE_KEY );

    //ゲームパッドのボタンの状態
    if ! is_pressed
    {   let Some ( gamepad ) = opt_gamepad else { return }; //Resource未登録
        let Some ( id ) = gamepad.id() else { return };     //gamepad未接続
        is_pressed = inbtn.just_pressed( GamepadButton::new( id, PAUSE_BUTTON ) );
    }

    //PAUSEのトグル処理
    if is_pressed
    {   if state.get().is_pause()
        {   misc::hide( qry_text );           //PAUSEメッセージを非表示
            *state = State::new( *saved_in ); //OnEnter／OnExitを実行せす遷移する
        }
        else
        {   misc::show( qry_text );                //PAUSEメッセージを表示
            *saved_in = *state.get();              //遷移元のStateをローカルに保存する
            *state = State::new( MyState::Pause ); //OnEnter／OnExitを実行せす遷移する
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.