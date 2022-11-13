use super::*;

//bevyのカメラを設置する
pub fn spawn_camera( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg( not( target_arch = "wasm32" ) )]
pub fn toggle_window_mode
(   inkey: Res<Input<KeyCode>>,
    mut window: ResMut<Windows>,
)
{   let is_alt_return = ( inkey.pressed( KeyCode::LAlt ) || inkey.pressed( KeyCode::RAlt ) )
                        && inkey.just_pressed( KeyCode::Return );

    if is_alt_return
    {   use bevy::window::WindowMode::*;
        if let Some( window ) = window.get_primary_mut()
        {   let mode = if window.mode() == Windowed { SizedFullscreen } else { Windowed };
            window.set_mode( mode );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ESCキーが入力さたら一時停止する
pub fn pause_with_esc_key
(   q: Query<&mut Visibility, With<TextUiPause>>,
    mut inkey: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
)
{   if ! inkey.just_pressed( KeyCode::Escape ) { return }

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

//End of cooe.