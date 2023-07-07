use super::*;

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用
pub const DEBUG: fn() -> bool = || cfg!( debug_assertions );
pub const WASM : fn() -> bool = || cfg!( target_arch = "wasm32" );

////////////////////////////////////////////////////////////////////////////////

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() )
    // .insert( Camera { order: CAMERA2D_ORDER, ..default() } )
    // .insert( Camera2d { clear_color: CAMERA2D_BGCOLOR } )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut q_window: Query<&mut Window>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    mut button_events: EventReader<GamepadButtonChangedEvent>,
)
{   //ウィンドウが見つからないなら
    let Ok( mut window ) = q_window.get_single_mut() else { return };

    //[Alt]＋[Enter]の状態
    let is_key_pressed =
        ( inkey.pressed( KeyCode::AltRight ) || inkey.pressed( KeyCode::AltLeft ) )
            && inkey.just_pressed( KeyCode::Return );

    //パッドのボタンの状態
    let mut is_btn_pressed = false;
    let btn = GamepadButtonType::Select; //ps4[SHARE]
    for button_event in button_events.iter()
    {   if button_event.button_type == btn
        {   let btn = GamepadButton::new( button_event.gamepad, btn );
            is_btn_pressed = inbtn.just_pressed( btn );
            if is_btn_pressed { break }
        }
    }

    //入力がないなら
    if ! is_key_pressed && ! is_btn_pressed { return }

    //ウィンドウとフルスクリーンを切り替える
    window.mode = match window.mode
    {   Windowed => SizedFullscreen,
        _        => Windowed,
    };
}

////////////////////////////////////////////////////////////////////////////////

//一時停止する
pub fn pause_with_esc_key
(   q: Query<&mut Visibility, With<TextUiPause>>,
    mut state: ResMut<State<MyState>>,
    mut inkey: ResMut<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    mut old_state: Local<MyState>,
)
{   //パッドのボタン
    let btn_pause = GamepadButton::new( GAMEPAD, BUTTON_PAUSE );

    //入力がないなら関数脱出
    if ! inkey.just_pressed( KEY_PAUSE ) && ! inbtn.just_pressed( btn_pause ) { return }

    //PAUSEのトグル処理
    if state.get().is_pause()
    {   hide_component( q );
        *state = State::new( *old_state ); //遷移先のOnEnterを実行しない
    }
    else
    {   show_component( q );
        *old_state = *state.get();
        *state = State::new( MyState::Pause ); //遷移元のOnExitを実行しない
    }

    //NOTE: https://bevy-cheatbook.github.io/programming/states.html#with-input
    inkey.reset( KeyCode::Escape );
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたEnityを再帰的に削除する
pub fn despawn_entity<T: Component>
(   q: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   q.for_each( | ent | cmds.entity( ent ).despawn_recursive() );
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたComponentを見せる
pub fn show_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{// q.for_each_mut( | mut vis | *vis = Visibility::Inherited );
    q.for_each_mut( | mut vis | *vis = Visibility::Visible );
}

//QueryしたComponentを隠す
pub fn hide_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{   q.for_each_mut( | mut vis | *vis = Visibility::Hidden );
}

////////////////////////////////////////////////////////////////////////////////

//End of cooe.