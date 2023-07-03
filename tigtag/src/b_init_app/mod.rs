use super::*;

//submodules
mod fetch_assets;
mod text_ui;

use fetch_assets::*;

//プラグインの設定
pub struct InitApp;
impl Plugin for InitApp
{   fn build( &self, app: &mut App )
    {   //メインウィンドウ、背景色、アンチエイリアシング、プラグイン
        let window = Window
        {   title     : APP_TITLE.to_string(),
            resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
            resizable : false,
            cursor    : bevy::window::Cursor { visible: false, ..default() },
            fit_canvas_to_parent: true, //Android Chromeで不具合が発生する場合コメントアウトする
            ..default()
        };
        let primary_window = Some( window );

        app
        .insert_resource( ClearColor( SCREEN_BACKGROUND_COLOR ) )
        .insert_resource( Msaa::Sample4 )
        .add_plugins
        (   DefaultPlugins
            .set( WindowPlugin { primary_window, ..default() } )
            .set( ImagePlugin::default_nearest() ) //pixel perfect style
        );

        //ResourceとEvent
        app
        .add_state::<MyState>()       //Stateの初期化
        .init_resource::<Record>()    //スコア等の初期化
        .init_resource::<CountDown>() //カウントダウンタイマーの初期化
        .init_resource::<Map>()       //迷路情報の初期化
        .add_event::<EventClear>()    //ステージクリアイベント
        .add_event::<EventOver>()     //ゲームオーバーイベント
        ;

        //Systemの登録
        app
        .add_systems( Startup, spawn_camera       ) //bevyのカメラ
        .add_systems( Update , pause_with_esc_key ) //[Esc]でPause
        .add_plugins( FetchAssets ) //Assets(Fonts、Sprites等)のロード
        .add_systems
        (   OnExit( MyState::InitApp ),
            (   spawn_game_frame,
                text_ui::spawn,
                debug::spawn_info.run_if( DEBUG ),
            )
        )
        ;

        //Not WASM用System
        #[cfg( not( target_arch = "wasm32" ) )]
        app
        .add_systems( Update, toggle_window_mode ) //[Alt]+[Enter]でフルスクリーン
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
    let sprite_file = if DEBUG() { ASSETS_SPRITE_DEBUG_GRID } else { ASSETS_SPRITE_BRICK_WALL };

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