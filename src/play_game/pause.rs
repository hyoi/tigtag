use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   OnExit ( MyState::InitGame ),
            (   //PAUSE用UIをspawnする
                spawn_text,
            )
        )
        .add_systems( Update, pause ) //PAUSE処理
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//メッセージの設定
const UI_PAUSE: &[ MessageSect ] =
&[  ( "P A U S E", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::SILVER ),
];

//PAUSEのキー／ボタン
pub const PAUSE_KEY: KeyCode = KeyCode::Escape;
pub const PAUSE_BUTTON: GamepadButtonType = GamepadButtonType::Select; //PS4[SHARE]

////////////////////////////////////////////////////////////////////////////////

//可視化制御用のComponent
#[derive( Component )]
pub struct Pause;

////////////////////////////////////////////////////////////////////////////////

//PAUSEをspawnする
fn spawn_text
(   qry_hidden_node: Query<Entity, With<init_app::HiddenNode>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_node ) = qry_hidden_node.get_single() else { return };

    //メッセージの準備
    let mut ui = misc::text_ui( UI_PAUSE, &asset_svr );
    ui.style.grid_column  = GridPlacement::start_end( 1, 4 ); //３列連結
    ui.style.grid_row     = GridPlacement::start_end( 2, 3 ); //２行目
    ui.style.align_self   = AlignSelf::Center;
    ui.style.justify_self = JustifySelf::Center;
    ui.text.justify       = JustifyText::Center;
    ui.visibility         = Visibility::Hidden; //初期状態

    //レイアウト用の隠しフレームの中に子要素を作成する
    let child_id = cmds.spawn( ( ui, Pause ) ).id();
    cmds.entity( hidden_node ).add_child( child_id );
}

////////////////////////////////////////////////////////////////////////////////

//PAUSE処理
fn pause
(   qry_text: Query<&mut Visibility, With<Pause>>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
    input_gamepad: Res<ButtonInput<GamepadButton>>,
    mut state: ResMut<State<MyState>>,
    mut saved_in: Local<MyState>,
)
{   //キーの状態
    let mut is_pressed = input_keyboard.just_pressed( PAUSE_KEY );

    //ゲームパッドのボタンの状態
    if ! is_pressed
    {   let Some ( gamepad ) = opt_gamepad else { return }; //Resource未登録
        let Some ( id ) = gamepad.id() else { return };     //gamepad未接続
        is_pressed = input_gamepad.just_pressed( GamepadButton::new( id, PAUSE_BUTTON ) );
    }

    //PAUSEのトグル処理
    if is_pressed
    {   if state.get().is_pause()
        {   misc::hide_component( qry_text ); //PAUSEメッセージを非表示
            *state = State::new( *saved_in ); //OnEnter／OnExitを実行せす遷移する
        }
        else
        {   misc::show_component( qry_text );      //PAUSEメッセージを表示
            *saved_in = *state.get();              //遷移元のStateをローカルに保存する
            *state = State::new( MyState::Pause ); //OnEnter／OnExitを実行せす遷移する
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.