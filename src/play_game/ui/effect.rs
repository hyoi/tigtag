use super::*;

////////////////////////////////////////////////////////////////////////////////

//Center UIの基本トレイト
pub trait TextUI
{   fn message( &self ) -> &[ MessageSect ];
}

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

//カウントダウンのトレイト
pub trait CountDown
{   fn initial_count( &self ) -> i32;
    fn next_state( &self ) -> MyState;
    fn to_string( &self, n: i32 ) -> String;
    fn placeholder( &self ) -> Option<usize>;
}

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

//Hit ANY Key! のトレイト
pub trait HitAnyKey
{   fn next_state( &self ) -> MyState;
}

//キー入力さたらStateを変更する
pub fn hit_any_key<T: Component + HitAnyKey>
(   qry_ui: Query<&T>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    mut next_state: ResMut<NextState<MyState>>,
    inkey: Res<ButtonInput<KeyCode>>,
    inbtn: Res<ButtonInput<GamepadButton>>,
)
{   let Ok ( ui ) = qry_ui.get_single() else { return };

    //無視キー以外のキー入力はあるか
    if inkey.any_pressed( HAK_IGNORE_KEYS.iter().copied() ) { return }
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
    {   next_state.set( ui.next_state() );
    }
}

////////////////////////////////////////////////////////////////////////////////

//テキスト明滅のトレイト
pub trait BlinkingText
{   fn alpha( &mut self, time_delta: f32 ) -> f32;
}

//テキストを明滅させる
pub fn blinking_text<T: Component + BlinkingText>
(   mut qry_text: Query<( &mut Text, &mut T )>,
    time: Res<Time>,
)
{   let Ok ( ( mut text, mut ui ) ) = qry_text.get_single_mut() else { return };

    //text.sectionsをイテレーターで回して色を変化させる
    let alpha = ui.alpha( time.delta().as_secs_f32() );
    text.sections.iter_mut().for_each( |x| { x.style.color.set_a( alpha ); } );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.