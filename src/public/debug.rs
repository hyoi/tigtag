#![allow( dead_code )]

use super::*;

////////////////////////////////////////////////////////////////////////////////

//スプライトの設定
const COLOR_SPRITE_DEBUG_GRID: Color = Color::rgba( 1.0, 1.0, 1.0, 0.01 );

//マス目状にスプライトを敷き詰める
pub fn spawn_2d_sprites
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let color = COLOR_SPRITE_DEBUG_GRID;
    let custom_size = Some ( SIZE_GRID );

    for x in SCREEN_GRIDS_X_RANGE
    {   for y in SCREEN_GRIDS_Y_RANGE
        {   let vec2 = IVec2::new( x, y ).to_sprite_pixels();
            let vec3 = vec2.extend( DEPTH_SPRITE_DEBUG_GRID );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            .insert( asset_svr.load( ASSETS_SPRITE_DEBUG_GRID ) as Handle<Image> )
            .with_children
            (   | cmds |
                {   let value = format!( "{:02}\n{:02}", x, y ).to_string();
                    let style = TextStyle
                    {   font     : asset_svr.load( ASSETS_FONT_PRESSSTART2P_REGULAR ),
                        font_size: PIXELS_PER_GRID * 0.25,
                        color    : Color::DARK_GREEN,
                    };
                    let sections  = vec![ TextSection { value, style } ];
                    let alignment = TextAlignment::Center;

                    cmds.spawn( Text2dBundle::default() )
                    .insert( Text { sections, alignment, ..default() } )
                    .insert( Transform::from_translation( Vec3::Z ) )
                    ;
                }
            );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.