use super::*;

//bevyのカメラを設置する
pub fn spawn_camera( mut cmds: Commands )
{   cmds.spawn_bundle( Camera2dBundle::default() );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//デバッグ用に方眼を表示する
#[cfg( debug_assertions )]
pub fn spawn_debug_grid
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );
    let color = COLOR_SPRITE_DEBUG_GRID;

    for x in SCREEN_GRIDS_RANGE_X
    {   for y in SCREEN_GRIDS_RANGE_Y
        {   let pixel_xy = Grid::new( x, y ).into_pixel_screen();
            cmds
            .spawn_bundle( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( pixel_xy.extend( DEPTH_SPRITE_DEBUG_GRID ) ) )
            .insert( asset_svr.load( ASSETS_SPRITE_DEBUG_GRID ) as Handle<Image> )
            ;
        }
    }
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
{	q.for_each( | id | cmds.entity( id ).despawn_recursive() );
}

//End of cooe.