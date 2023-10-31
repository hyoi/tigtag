use super::*;

////////////////////////////////////////////////////////////////////////////////

//UIをspawnする
pub fn spawn_in_middle_frame<T: Component + Default + Copy + TextUI>
(   component: Local<T>,
    qry_hidden_frame: Query<Entity, With<HiddenFrameMiddle>>,
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
pub fn show_countdown<T: Component + CountDown>
(   mut qry_text_ui: Query<(&mut Text, &T)>,
    opt_countdown: Option<ResMut<CountDownTimer>>,
    mut next_state: ResMut<NextState<MyState>>,
    time: Res<Time>,
)
{   let Ok ( ( mut text, ui ) ) = qry_text_ui.get_single_mut() else { return };
    let Some ( mut countdown ) = opt_countdown else { return };

    let finished = countdown.timer.tick( time.delta() ).finished();
    
    //1秒経過したら
    if finished
    {   countdown.counter -= 1;  //カウントダウン
        countdown.timer.reset(); //1秒タイマーリセット
    }

    //カウントダウンが終わったら、次のStateへ遷移する
    if countdown.counter <= 0
    {   next_state.set( ui.next_state() );
        return;
    }

    //カウントダウンの表示を更新
    let message = ui.to_string( countdown.counter - 1 );
    text.sections[ ui.placeholder() ].value = message;
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
