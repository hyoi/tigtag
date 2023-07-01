use super::*;

//debug用スプライトをspawnする
pub fn spawn_sprite
(   q: Query<Entity, With<DotsRect>>,
    map: ResMut<Map>,
    mut cmds: Commands,
)
{   //スプライトがあれば削除する
    q.for_each( | id | cmds.entity( id ).despawn_recursive() );

    let ( x, y, w, h ) = map.demo.debug_pixel_rect();
    let custom_size = Some ( Pixel::new( w, h ) );
    let pixel3 = Pixel::new( x, y ).extend( _DEPTH_SPRITE_DEBUG_RECT );
    let color = _COLOR_SPRITE_DEBUG_RECT;

    cmds
    .spawn( ( SpriteBundle::default(), DotsRect ) )
    .insert( Sprite { color, custom_size, ..default() } )
    .insert( Transform::from_translation( pixel3 ) )
    ;
}

//debug用スプライトの表示を更新する
pub fn update_sprite
(   mut q: Query<( &mut Transform, &mut Sprite ), With<DotsRect>>,
    map: Res<Map>,
)
{   let Ok ( ( mut transform, mut sprite ) ) = q.get_single_mut() else { return };

    let ( x, y, w, h ) = map.demo.debug_pixel_rect();
    let custom_size = Some ( Pixel::new( w, h ) );
    let pixel3 = Pixel::new( x, y ).extend( _DEPTH_SPRITE_DEBUG_RECT );

    transform.translation = pixel3;
    sprite.custom_size = custom_size;
}

//debug用スプライトのpixel座標を求める
impl DemoParams
{   pub fn debug_pixel_rect( &self ) -> ( f32, f32, f32, f32 )
    {   let px_min = self.dots_rect_min().into_pixel_map();
        let px_max = self.dots_rect_max().into_pixel_map();

        let px_w = px_max.x - px_min.x;
        let px_h = px_min.y - px_max.y; //pixelはY軸が逆向き
        let px_x = px_min.x + px_w / 2.0;
        let px_y = px_max.y + px_h / 2.0; //pixelはY軸が逆向き

        ( px_x, px_y, px_w + PIXELS_PER_GRID, px_h + PIXELS_PER_GRID )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//デバッグ用の情報を表示
pub fn spawn_info
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );
    let color = _COLOR_SPRITE_DEBUG_GRID;

    //方眼を表示する
    for x in SCREEN_GRIDS_RANGE_X
    {   for y in SCREEN_GRIDS_RANGE_Y
        {   let pixel_xy = Grid::new( x, y ).into_pixel_screen();
            cmds
            .spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( pixel_xy.extend( _DEPTH_SPRITE_DEBUG_GRID ) ) )
            .insert( asset_svr.load( ASSETS_SPRITE_DEBUG_GRID ) as Handle<Image> )
            ;
        }
    }

    //Map内に数値用のText UIを表示する
    for x in SCREEN_GRIDS_RANGE_X
    {   for y in SCREEN_GRIDS_RANGE_Y
        {   let grid = Grid::new( x, y );
            let pixel = grid.into_pixel_map();

            //UIのFLEX座標系に合せる
            let mut text_ui = Pixel::new( pixel.x, - pixel.y );
            text_ui.x += SCREEN_PIXELS_WIDTH  / 2.0 - PIXELS_PER_GRID / 2.0;
            text_ui.y += SCREEN_PIXELS_HEIGHT / 2.0 - PIXELS_PER_GRID;

            let mut txt = NUM_TILE_TEXT;
            let val = format!( "{x},{y}" );
            txt[ 0 ].0 = &val;

            cmds
            .spawn( ( text_ui_num_tile( text_ui, &txt, &asset_svr ), TextUiNumTile ( grid ) ) )
            ;
        }
    }
}

//デバッグ用のText UI
fn text_ui_num_tile
(   pixel: Pixel,
    message: &[ MessageSect ],
    asset_svr: &Res<AssetServer>,
) -> TextBundle
{   let mut sections = Vec::new();
    for ( line, file, size, color ) in message.iter()
    {   let value = line.to_string();
        let style = TextStyle
        {   font     : asset_svr.load( *file ),
            font_size: *size,
            color    : *color
        };
        sections.push( TextSection { value, style } );
    }
    let text = Text { sections, ..default() };
    let ( left, top, width, height ) =
    (   Val::Px( pixel.x ),
        Val::Px( pixel.y ),
        Val::Px( PIXELS_PER_GRID ),
        Val::Px( PIXELS_PER_GRID ),
    );
    let style = Style
    {   position_type: PositionType::Absolute,
        left,
        top,
        width,
        height,
        ..default()
    };
    TextBundle { text, style, ..default() }
}

//End of cooe.