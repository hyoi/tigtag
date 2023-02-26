use super::*;

//bevyのカメラを設置する
pub fn spawn_camera( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンを切り替える
#[cfg( not( target_arch = "wasm32" ) )]
pub fn toggle_window_mode
(   mut windows: Query<&mut Window>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   //ウィンドウが見つからなければ関数脱出
    let Ok( mut window ) = windows.get_single_mut() else { return };

   //パッドのボタンの状態
    let btn_fullscreen = GamepadButton::new( GAMEPAD, _BUTTON_FULLSCREEN );
    let is_btn_fullscreen = inbtn.just_pressed( btn_fullscreen );

    //Alt＋Enterキーの状態
    let is_key_fullscreen =
    ( inkey.pressed( _KEY_ALT_RIGHT ) || inkey.pressed( _KEY_ALT_LEFT ) )
    && inkey.just_pressed( _KEY_FULLSCREEN );

    //入力がないなら関数脱出
    if ! is_key_fullscreen && ! is_btn_fullscreen { return }

    //ウィンドウとフルスクリーンを切り替える
    use bevy::window::WindowMode::*;
    let mode = if window.mode == Windowed { SizedFullscreen } else { Windowed };
    window.mode = mode;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//一時停止する
pub fn pause_with_esc_key
(   q: Query<&mut Visibility, With<TextUiPause>>,
    mut state: ResMut<State<GameState>>,
    mut inkey: ResMut<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    mut old_state: Local<GameState>,
)
{   //パッドのボタン
    let btn_pause = GamepadButton::new( GAMEPAD, BUTTON_PAUSE );

    //入力がないなら関数脱出
    if ! inkey.just_pressed( KEY_PAUSE ) && ! inbtn.just_pressed( btn_pause ) { return }

    //PAUSEのトグル処理
    if state.0.is_pause()
    {   hide_component( q );
        state.0 = *old_state;
    }
    else
    {   show_component( q );
        *old_state = state.0;
        state.0 = GameState::Pause;
    }

    //NOTE: https://bevy-cheatbook.github.io/programming/states.html#with-input
    inkey.reset( KeyCode::Escape );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Componentを見せる
pub fn show_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{   let _ = q.get_single_mut().map( | mut ui | *ui = Visibility::Inherited );
}

//Componentを隠す
pub fn hide_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{   let _ = q.get_single_mut().map( | mut ui | *ui = Visibility::Hidden );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ComponentでQueryしたEnityを再帰的に削除する
pub fn despawn_entity<T: Component>
(   q: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   q.for_each( | id | cmds.entity( id ).despawn_recursive() );
}

//End of cooe.