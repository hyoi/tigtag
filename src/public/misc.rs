use super::*;

//bevyのカメラを設置する
pub fn spawn_camera( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/*
//ウィンドウとフルスクリーンを切り替える
#[cfg( not( target_arch = "wasm32" ) )]
pub fn toggle_window_mode
(   inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    mut window: ResMut<Windows>,
)
{   //パッドのボタンの状態
    let btn_fullscreen = GamepadButton::new( GAMEPAD, _BUTTON_FULLSCREEN );
    let is_btn_fullscreen = inbtn.just_pressed( btn_fullscreen );

    //Alt＋Enterキーの状態
    let is_key_fullscreen =
    ( inkey.pressed( _KEY_ALT_RIGHT ) || inkey.pressed( _KEY_ALT_LEFT ) )
    && inkey.just_pressed( _KEY_FULLSCREEN );

    //入力がないなら関数脱出
    if ! is_key_fullscreen && ! is_btn_fullscreen { return }

    use bevy::window::WindowMode::*;
    if let Some( window ) = window.get_primary_mut()
    {   let mode = if window.mode() == Windowed { SizedFullscreen } else { Windowed };
        window.set_mode( mode );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//一時停止する
pub fn pause_with_esc_key
(   q: Query<&mut Visibility, With<TextUiPause>>,
    mut state: ResMut<State<GameState>>,
    mut inkey: ResMut<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   //パッドのボタン
    let btn_pause = GamepadButton::new( GAMEPAD, BUTTON_PAUSE );

    //入力がないなら関数脱出
    if ! inkey.just_pressed( KEY_PAUSE ) && ! inbtn.just_pressed( btn_pause ) { return }

    //PAUSEのトグル処理
    if state.current().is_pause()
    {   hide_component( q );
        let _ = state.pop();
    }
    else
    {   show_component( q );
        let _ = state.push( GameState::Pause );
    }

    //NOTE: https://bevy-cheatbook.github.io/programming/states.html#with-input
    inkey.reset( KeyCode::Escape );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Componentを見せる
pub fn show_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{   let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
}

//Componentを隠す
pub fn hide_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{   let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ComponentでQueryしたEnityを再帰的に削除する
pub fn despawn_entity<T: Component>
(   q: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   q.for_each( | id | cmds.entity( id ).despawn_recursive() );
}
*/
//End of cooe.