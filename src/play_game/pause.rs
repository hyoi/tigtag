use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //PAUSE用UIをspawnする
        .add_systems
        (   OnExit ( MyState::GameStart ),
            ui::spawn_in_middle_frame::<UiPause>
        )

        //PAUSE処理
        .add_systems( Update, pause )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//TextUIのComponent
#[derive( Component, Clone, Copy )]
pub struct UiPause<'a> ( &'a [ MessageSect<'a> ] );

const UI_PAUSE: &[ MessageSect ] =
&[  ( "P A U S E", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::SILVER ),
];

impl<'a> Default for UiPause<'a>
{   fn default() -> Self { Self ( UI_PAUSE ) }
}

impl<'a> TextUI for UiPause<'a>
{   fn message( &self ) -> & [ MessageSect ] { self.0 }
}

////////////////////////////////////////////////////////////////////////////////

//PAUSE処理
fn pause
(   qry_text: Query<&mut Visibility, With<UiPause<'static>>>,
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