use super::*;

////////////////////////////////////////////////////////////////////////////////

//テキスト明滅のトレイト
pub trait Blinking
{   fn alpha( &mut self, time_delta: f32 ) -> f32;
}

//テキストを明滅させる
pub fn blinking_text<T: Component + Blinking>
(   mut qry_text: Query<( &mut Text, &mut T )>,
    time: Res<Time>,
)
{   let Ok ( ( mut text, mut ui ) ) = qry_text.get_single_mut() else { return };

    //透明度を変化させる
    let alpha = ui.alpha( time.delta().as_secs_f32() );
    text.set_alpha( alpha );
}

////////////////////////////////////////////////////////////////////////////////

//Hit ANY Keyの処理で無視するキーとボタン
pub const HAK_IGNORE_KEYS: &[ KeyCode ] =
&[  KeyCode::AltLeft    , KeyCode::AltRight,
    KeyCode::ControlLeft, KeyCode::ControlRight,
    KeyCode::ShiftLeft  , KeyCode::ShiftRight,
    KeyCode::ArrowUp    , KeyCode::ArrowDown,
    KeyCode::ArrowRight , KeyCode::ArrowLeft,
    KeyCode::CapsLock   , pause::ESC_KEY,
    KeyCode::Fn,
    KeyCode::Unidentified ( NativeKeyCode::Windows ( 57443 ) ), //ThinkPad [Fn]
];
pub const HAK_IGNORE_BUTTONS: &[ GamepadButtonType ] =
&[  FULL_SCREEN_BUTTON,
    GamepadButtonType::DPadUp,    GamepadButtonType::DPadDown,
    GamepadButtonType::DPadRight, GamepadButtonType::DPadLeft,
    pause::PAUSE_BUTTON,
];

//入力があればStateを変更する
pub fn hit_any_key<T: Send + Sync + Default + ChangeMyState>
(   next: Local<T>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    mut next_state: ResMut<NextState<MyState>>,
    inkey: Res<ButtonInput<KeyCode>>,
    inbtn: Res<ButtonInput<GamepadButton>>,
)
{   //無視キー以外のキー入力はあるか
    if inkey.any_pressed     ( HAK_IGNORE_KEYS.iter().copied() ) { return }
    if inkey.any_just_pressed( HAK_IGNORE_KEYS.iter().copied() ) { return } //[Fn]対策
    let mut is_pressed = inkey.get_just_pressed().len();

    #[cfg(debug_assertions)]
    if is_pressed != 0
    {   inkey.get_just_pressed().for_each( |key| { dbg!( key ); } );
    }

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
    {   next_state.set( next.state() );
    }
}

////////////////////////////////////////////////////////////////////////////////

//カウントダウンUIのプレイスホルダー
pub const CDPH: &str = "__Placeholder_for_countdown__";

//カウントダウンのトレイト
pub trait CountDown
{   fn count_down( &mut self ) -> &mut i32;
    fn next_state( &self ) -> MyState;
    fn timer( &mut self ) -> &mut Timer;
    fn gen_message( &self, n: i32 ) -> String;
    fn placeholder( &self ) -> Option<usize>;
    fn initialize( &mut self );
}

//カウントダウンを初期化する
pub fn init_count<T: Component + CountDown>
(   mut qrt_ui: Query<&mut T>,
)
{   let Ok ( mut ui ) = qrt_ui.get_single_mut() else { return };

    ui.initialize();
}

//カウントダウンを表示しゼロになったらStateを変更する
pub fn count_down<T: Component + CountDown>
(   mut qry_text_ui: Query<( &mut Text, &mut T )>,
    mut next_state: ResMut<NextState<MyState>>,
    time: Res<Time>,
)
{   let Ok ( ( mut text, mut ui ) ) = qry_text_ui.get_single_mut() else { return };
    let Some ( placeholder ) = ui.placeholder() else { return };

    //1秒経過したら
    if ui.timer().tick( time.delta() ).finished()
    {   *ui.count_down() -= 1;  //カウントダウン
        ui.timer().reset(); //1秒タイマーリセット
    }

    //カウントダウンが続いているなら
    let count_down = *ui.count_down();
    if count_down >= 0
    {   //カウントダウンの表示を更新する
        let message = ui.gen_message( count_down );
        text.sections[ placeholder ].value = message;
    }
    else
    {   //そうでないならStateを変更する
        next_state.set( ui.next_state() );
    }
}

////////////////////////////////////////////////////////////////////////////////

//テキスト拡縮のトレイト
pub trait Scaling
{   fn scale( &mut self, time_delta: f32 ) -> f32;
}

//テキストを拡縮させる
pub fn repeat_scaling_text<T: Component + Scaling>
(   mut qry_transform: Query<( &mut Transform, &mut T )>,
    time: Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32();

    for ( mut transform, mut ui ) in qry_transform.iter_mut()
    {   transform.scale = Vec3::ONE * ui.scale( time_delta );
    }
}

////////////////////////////////////////////////////////////////////////////////

//Text型にトレイトを追加（オーファンルール対策）
pub trait AddOnTraitForText
{   fn set_color( &mut self, color: Color );
    fn set_alpha( &mut self, alpha: f32 );
}
impl AddOnTraitForText for Text
{   //Textのsectionsの色を一括で変更する
    fn set_color( &mut self, color: Color )
    {   self.sections.iter_mut().for_each( |x| { x.style.color = color; } );
    }
    //Textのsectionsの透明度を一括で変更する
    fn set_alpha( &mut self, alpha: f32 )
    {   self.sections.iter_mut().for_each( |x| { x.style.color.set_alpha( alpha ); } );
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.