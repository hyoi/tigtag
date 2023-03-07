use super::*;

//submodules
mod fetch_assets;
mod spawn_text_ui;

use fetch_assets::*;
use spawn_text_ui::*;

//プラグインの設定
pub struct InitApp;
impl Plugin for InitApp
{   fn build( &self, app: &mut App )
    {   //メインウィンドウ、背景色、アンチエイリアシング、プラグイン
        let window = Window
        {   title     : APP_TITLE.to_string(),
            resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
            resizable : false,
            fit_canvas_to_parent: true, //Android Chromeで不具合が発生する場合コメントアウトする
            ..default()
        };
        let primary_window = Some( window );
        app
        .insert_resource( ClearColor( SCREEN_BACKGROUND_COLOR ) )
        .insert_resource( Msaa::Sample4 )
        .add_plugins( DefaultPlugins.set( WindowPlugin { primary_window, ..default() } ) )
        ;

        //ResourceとEvent
        app
        .add_state::<MyState>()                 //Stateの初期化
        .init_resource::<Record>()              //スコア等の初期化
        .init_resource::<CountDown>()           //カウントダウンタイマーの初期化
        .init_resource::<Map>()                 //迷路情報の初期化
        .add_event::<EventClear>()              //ステージクリアイベント
        .add_event::<EventOver>()               //ゲームオーバーイベント
        ;

        //共通のSystem
        app
        .add_startup_system( spawn_camera )     //bevyのカメラ
        .add_system( pause_with_esc_key )       //[Esc]でPause
        ;

        //Not WASM用System
        #[cfg( not( target_arch = "wasm32" ) )]
        app
        .add_system( toggle_window_mode )       //[Alt]+[Enter]でフルスクリーン
        ;

        //MyState::Init
        //------------------------------------------------------------------------------------------
        app
        .add_plugin( FetchAssets )              //Fonts、Sprites等のプリロード
        .add_plugin( SpawnTextUi )              //Text UIのspawn
        ;
        //デバッグ用System
        #[cfg( debug_assertions )]
        app
        .add_system
        (   spawn_debug_info                    //debug用の情報を表示
            .in_schedule( OnExit( MyState::InitApp ) )
        )
        ;
        // //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_game_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );
    let sprite_file = if cfg!( debug_assertions )
    {   ASSETS_SPRITE_DEBUG_GRID
    }
    else
    {   ASSETS_SPRITE_BRICK_WALL
    };

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

////////////////////////////////////////////////////////////////////////////////////////////////////

//デバッグ用の情報を表示
#[cfg( debug_assertions )]
pub fn spawn_debug_info
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
#[cfg( debug_assertions )]
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
        position: UiRect { left, top, ..default() },
        size: Size { width, height },
        ..default()
    };
    TextBundle { text, style, ..default() }
}

//End of code.