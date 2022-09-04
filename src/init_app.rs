use super::*;

//プラグインの設定
pub struct InitApp;
impl Plugin for InitApp
{   fn build( &self, app: &mut App )
    {   //メインウィンドウ、背景色、アンチエイリアシング、プラグイン
        let main_window = WindowDescriptor
        {   title    : APP_TITLE.to_string(),
            width    : SCREEN_PIXELS_WIDTH,
            height   : SCREEN_PIXELS_HEIGHT,
            resizable: false,
            fit_canvas_to_parent: true,
            ..default()
        };
        app
        .insert_resource( main_window )
        .insert_resource( ClearColor( SCREEN_BACKGROUND_COLOR ) )
        .insert_resource( Msaa { samples: 4 } )
        .add_plugins( DefaultPlugins )  // bevy default
        .add_plugin( AudioPlugin )      // bevy_kira_audio
        ;
    
        //ResourceとEvent
        app
        .add_state( GameState::Init )           //Stateの初期化
        .init_resource::<Record>()              //スコア等
        .init_resource::<CountDown>()           //カウントダウンタイマー
        .init_resource::<Map>()                 //迷路の情報
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

        //GameState::Init
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::Init )          //<ENTER>
            .with_system( start_fetching_assets )           //Assetのロード開始
            .with_system( spawn_sprite_now_loading )        //アニメ用スプライトの生成
        )
        .add_system_set
        (   SystemSet::on_update( GameState::Init )         //<UPDATE>
            .with_system( change_state_after_loading )      //ロード完了か判定しState変更
            .with_system( move_sprite_now_loading )         //ローディングアニメ
        )
        .add_system_set
        (   SystemSet::on_exit( GameState::Init )           //<EXIT>
            .with_system( despawn_entity::<SpriteTile> )    //アニメ用スプライトの削除
            .with_system( spawn_game_frame )                //枠の表示
            .with_system( spawn_text_ui )                   //text UIのspawn
        )
        ;

        //デバッグ用System
        #[cfg( debug_assertions )]
        app
        .add_system_set
        (   SystemSet::on_exit( GameState::Init )           //<EXIT>
            .with_system( spawn_debug_grid )                //方眼を表示
        )
        ;
        //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetsのロードを開始する
fn start_fetching_assets
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //Assetsのロードを開始
    let mut preload = Vec::new();
    FETCH_ASSETS.iter().for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

    //リソースに登録して解放しないようにする
    cmds.insert_resource( LoadedAssets { preload } );
}

//ローディングアニメ用スプライトを生成する
fn spawn_sprite_now_loading
(   mut cmds: Commands,
)
{   let mut rng = rand::thread_rng();
    let color = COLOR_SPRITE_TILE;
    let custom_size = Some ( Pixel::new( PIXELS_PER_GRID * 0.75, PIXELS_PER_GRID ) );
 

    for ( goal_y, line ) in DESIGN_NOWLOADING_MESSAGE.iter().enumerate()
    {   for ( goal_x, chara ) in line.chars().enumerate()
        {   if chara == ' ' { continue }    //空白文字は無視

            //スプライトの初期座標と最終座標
            let rnd_x = rng.gen_range( SCREEN_GRIDS_RANGE_X );
            let rnd_y = rng.gen_range( SCREEN_GRIDS_RANGE_Y );
            let start = Grid::new( rnd_x, rnd_y ).into_pixel_screen();
            let goal  = Grid::new( goal_x as i32, goal_y as i32 );

            //スプライトを作成する
            cmds
            .spawn_bundle( SpriteBundle::default() )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( start.extend( DEPTH_SPRITE_TILE ) ) )
            .insert( SpriteTile ( goal ) )
            ;
        } 
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetsのロードが完了したら、Stateを変更する
fn change_state_after_loading
(   assets   : Res<LoadedAssets>,
    mut state: ResMut<State<GameState>>,
    asset_svr: Res<AssetServer>,
    o_marker : Option<Res<MarkAfterFetchAssets>>,
)
{   //プリロードが完了したか？
    for handle in assets.preload.iter()
    {   use bevy::asset::LoadState::*;
        match asset_svr.get_load_state( handle )
        {   Loaded => {}        //完了
            Failed => panic!(), //ロード失敗⇒パニック
            _      => return,   //on_update()の中なので関数は繰り返し呼び出される
        }
    }

    //次のStateへ遷移する
    if let Some ( x ) = o_marker
    {   let _ = state.overwrite_set( x.0 );
    }
}

//スプライトを動かしてローディングアニメを見せる
fn move_sprite_now_loading
(   mut q: Query<( &mut Transform, &SpriteTile )>,
    time : Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32() * 5.0;

    let half_screen_w = SCREEN_PIXELS_WIDTH / 2.0;
    let mess_width = DESIGN_NOWLOADING_MESSAGE[ 0 ].len() as f32 * PIXELS_PER_GRID;
    let scale = SCREEN_PIXELS_WIDTH / mess_width;

    q.for_each_mut
    (   | ( mut transform, goal_xy ) |
        {   let mut goal = goal_xy.0.into_pixel_screen();
            goal.x = ( goal.x + half_screen_w ) * scale - half_screen_w;    //横幅の調整

            let position = &mut transform.translation;
            position.x += ( goal.x - position.x ) * time_delta;
            position.y += ( goal.y - position.y ) * time_delta;
        }
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//画面枠を表示する
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
                .spawn_bundle( SpriteBundle::default() )
                .insert( Sprite { custom_size, ..default() } )
                .insert( Transform::from_translation( pixel_xy.extend( DEPTH_SPRITE_GAME_FRAME ) ) )
                .insert( asset_svr.load( sprite_file ) as Handle<Image> )
                ;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//text UIのレイアウト用に隠しフレームを作る
fn hidden_frame
(   style: Style,
) -> NodeBundle
{   let color = UiColor ( Color::NONE );
    NodeBundle { style, color, ..default() }
}

//text UI用にTextBundleを作る
fn text_ui
(   message: &[ MessageSect ],
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
    let alignment = TextAlignment
    {   vertical  : VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let position_type = PositionType::Absolute;

    let text  = Text { sections, alignment };
    let style = Style { position_type, ..default() };
    TextBundle { text, style, ..default() }
}

//text UIを配置する
fn spawn_text_ui
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //レイアウト用の隠しフレームを作る
    let per100 = Val::Percent( 100.0 );
    let center_frame = hidden_frame( Style
    {   size           : Size::new( per100, per100 ),
        position_type  : PositionType::Absolute,
        justify_content: JustifyContent::Center,
        align_items    : AlignItems::Center,
        ..default()
    } );
    let header_frame = hidden_frame( Style
    {   size           : Size::new( Val::Px( SCREEN_PIXELS_WIDTH ), Val::Px( SCREEN_PIXELS_HEIGHT ) ),
        position_type  : PositionType::Absolute,
        flex_direction : FlexDirection::Column,
        justify_content: JustifyContent::FlexEnd, //画面の上端
        ..default()
    } );
    let footer_frame = hidden_frame( Style
    {   size           : Size::new( Val::Px( SCREEN_PIXELS_WIDTH ), Val::Px( SCREEN_PIXELS_HEIGHT ) ),
        position_type  : PositionType::Absolute,
        flex_direction : FlexDirection::Column,
        justify_content: JustifyContent::FlexStart, //画面の下端
        ..default()
    } );

    //中央
    let mut ui_title = text_ui( &CENTER_TITLE_TEXT, &asset_svr );
    let mut ui_start = text_ui( &CENTER_START_TEXT, &asset_svr );
    let mut ui_over  = text_ui( &CENTER_OVER_TEXT , &asset_svr );
    let mut ui_clear = text_ui( &CENTER_CLEAR_TEXT, &asset_svr );
    let mut ui_pause = text_ui( &CENTER_PAUSE_TEXT, &asset_svr );

    ui_title.visibility.is_visible = false;
    ui_start.visibility.is_visible = false;
    ui_over.visibility.is_visible  = false;
    ui_clear.visibility.is_visible = false;
    ui_pause.visibility.is_visible = false;

    //ヘッダー
    let mut ui_header_left   = text_ui( &HEADER_LEFT_TEXT  , &asset_svr );
    let mut ui_header_center = text_ui( &HEADER_CENTER_TEXT, &asset_svr );
    let mut ui_header_right  = text_ui( &HEADER_RIGHT_TEXT , &asset_svr );

    ui_header_left.style.align_self = AlignSelf::FlexStart;
    ui_header_left.text.alignment.horizontal = HorizontalAlign::Left;

    ui_header_center.style.align_self = AlignSelf::Center;
    ui_header_center.text.alignment.horizontal = HorizontalAlign::Center;

    ui_header_right.style.align_self = AlignSelf::FlexEnd;
    ui_header_right.text.alignment.horizontal = HorizontalAlign::Right;

    //フッター
    let mut ui_footer_left   = text_ui( &FOOTER_LEFT_TEXT  , &asset_svr );
    let mut ui_footer_center = text_ui( &FOOTER_CENTER_TEXT, &asset_svr );
    let mut ui_footer_right  = text_ui( &FOOTER_RIGHT_TEXT , &asset_svr );

    ui_footer_left.style.align_self = AlignSelf::FlexStart;
    ui_footer_left.text.alignment.horizontal = HorizontalAlign::Left;

    ui_footer_center.style.align_self = AlignSelf::Center;
    ui_footer_center.text.alignment.horizontal = HorizontalAlign::Center;

    ui_footer_right.style.align_self = AlignSelf::FlexEnd;
    ui_footer_right.text.alignment.horizontal = HorizontalAlign::Right;

    //隠しフレームの上に子要素を作成する
    cmds.spawn_bundle( center_frame ).with_children
    (   | cmds |
        {   //中央
            cmds.spawn_bundle( ui_title ).insert( TEXT_UI_TITLE );
            cmds.spawn_bundle( ui_start ).insert( TEXT_UI_START );
            cmds.spawn_bundle( ui_over  ).insert( TEXT_UI_OVER  );
            cmds.spawn_bundle( ui_clear ).insert( TEXT_UI_CLEAR );
            cmds.spawn_bundle( ui_pause ).insert( TextUiPause   );

            //ヘッダー
            cmds.spawn_bundle( header_frame ).with_children
            (   | cmds |
                {   cmds.spawn_bundle( ui_header_left   ).insert( HeaderLeft   );
                    cmds.spawn_bundle( ui_header_center ).insert( HeaderCenter );
                    cmds.spawn_bundle( ui_header_right  ).insert( HeaderRight  );
                }
            );

            //フッター
            cmds.spawn_bundle( footer_frame ).with_children
            (   | cmds |
                {   cmds.spawn_bundle( ui_footer_left   ).insert( FooterLeft   );
                    cmds.spawn_bundle( ui_footer_center ).insert( FooterCenter );
                    cmds.spawn_bundle( ui_footer_right  ).insert( FooterRight  );
                }
            );
        }
    );

    //おまけ
    let pixel = Grid::new( SCREEN_GRIDS_WIDTH - 4, SCREEN_GRIDS_HEIGHT - 1 ).into_pixel_screen();
    let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) * MAGNIFY_SPRITE_KANI );
    cmds
    .spawn_bundle( SpriteBundle::default() )
    .insert( Sprite { custom_size, ..default() } )
    .insert( asset_svr.load( ASSETS_SPRITE_KANI_DOTOWN ) as Handle<Image> )
    .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_KANI_DOTOWN ) ) )
    ;
}

//End of code.