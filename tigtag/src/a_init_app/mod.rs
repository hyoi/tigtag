use super::*;

//submodules
mod fetch_assets;
mod text_ui;

use fetch_assets::*;

//プラグインの設定
pub struct InitApp;
impl Plugin for InitApp
{   fn build( &self, app: &mut App )
    {   //Systemの登録
        app
        .add_plugins( FetchAssets ) //Assets(Fonts、Sprites等)のロード
        .add_systems
        (   OnExit( MyState::InitApp ),
            (   spawn_game_frame,
                text_ui::spawn,
                debug::spawn_info.run_if( misc::DEBUG ),
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_game_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );
    let sprite_file = if misc::DEBUG() { ASSETS_SPRITE_DEBUG_GRID } else { ASSETS_SPRITE_BRICK_WALL };

    for ( y, line ) in DESIGN_GAME_FRAME.iter().enumerate()
    {   for ( x, char ) in line.chars().enumerate()
        {   if char == '#'
            {   let pixel_xy = Grid::new( x as i32, y as i32 ).into_pixel_screen();
                cmds
                .spawn( SpriteBundle::default() )
                .insert( Sprite { custom_size, ..default() } )
                .insert( Transform::from_translation( pixel_xy.extend( DEPTH_SPRITE_GAME_FRAME ) ) )
                .insert( asset_svr.load( sprite_file ) as Handle<Image> )
                ;
            }
        }
    }
}

//End of code.