use super::*;

//bevyのカメラの設置
pub fn spawn_camera( mut cmds: Commands )
{	cmds.spawn_bundle( Camera2dBundle::default() );
}

// ComponentでQueryしたEnityを削除する
pub fn despawn_entity<T: Component>( q: Query<Entity, With<T>>, mut cmds: Commands )
{	q.for_each( | id | { cmds.entity( id ).despawn_recursive() } );
}

// UI Textを表示する
pub fn show_ui<T: Component>( mut q: Query<&mut Visibility, With<T>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
}

// UI Textを隠す
pub fn hide_ui<T: Component>( mut q: Query<&mut Visibility, With<T>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
}

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_window_mode
(	inkey: Res<Input<KeyCode>>,
	mut window: ResMut<Windows>,
)
{	let is_pressed_alt    = inkey.pressed( KeyCode::LAlt ) || inkey.pressed( KeyCode::RAlt );
	let is_pressed_return = inkey.just_pressed( KeyCode::Return );

	if is_pressed_alt && is_pressed_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let x = if window.mode() == Windowed { SizedFullscreen } else { Windowed };
			window.set_mode( x );
		}
	}
}

//ESCキーが入力さたら一時停止する
pub fn handle_esc_key_for_pause
(	mut q: Query<&mut Visibility, With<MessagePause>>,
	mut inkey: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
)
{	if q.get_single_mut().is_err() { return }

	if inkey.just_pressed( KeyCode::Escape ) 
	{	match state.current()
		{	GameState::Pause => { hide_ui( q ); state.pop().unwrap() },
			_                => { show_ui( q ); state.push( GameState::Pause ).unwrap() },
		};
		//https://bevy-cheatbook.github.io/programming/states.html#with-input
		inkey.reset( KeyCode::Escape );
	}
}

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: usize, y: usize ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * y as f32 - PIXEL_PER_GRID;
	( x, y )
}

//End of code.