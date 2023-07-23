use super::*;

//internal submodules
mod load_assets;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //ResourceとEvent
        .add_state::<MyState>()         //State登録
        .init_resource::<Record>()      //スコア等の初期化
        .init_resource::<Map>()         //迷路情報の初期化
        .init_resource::<CountDown>()   //カウントダウンタイマーの初期化
        .add_event::<EventClear>()      //ステージクリアイベントの登録
        .add_event::<EventOver>()       //ゲームオーバーイベントの登録
        .init_resource::<NowGamepad>()  //操作を受け付けるgamepad
        .init_resource::<CrossButton>() //十字ボタンの入力状態保存用

        //ゲームの準備
        .add_plugins( load_assets::Schedule ) //assetsの事前読込

        //Assetsの読込後
        .add_systems
        (   OnExit ( MyState::InitApp ),
            (   spawn_screen_frame, //ゲーム枠をspawn
                spawn_text_ui,      //Text UIをspawn
            )
        )

        //FPSの表示
        .add_plugins( FrameTimeDiagnosticsPlugin ) //FPSプラグイン
        .add_systems( Update, update_footer_fps ) //表示更新
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_screen_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   for ( y, line ) in DESIGN_GAME_FRAME.iter().enumerate()
    {   for ( x, char ) in line.chars().enumerate()
        {   if char != ' '
            {   let px2d = Grid::new( x as i32, y as i32 ).px2d_map();
                let px3d = px2d.extend( DEPTH_SPRITE_GAME_FRAME );
    
                cmds.spawn( SpriteBundle::default() )
                .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
                .insert( Sprite { custom_size: Some ( SIZE_GRID ), ..default() } )
                .insert( Transform::from_translation( px3d ) )
                ;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//text UIを配置する
fn spawn_text_ui
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //レイアウト用の隠しフレームを作る
    let per100 = Val::Percent( 100.0 );
    let width  = Val::Px( SCREEN_PIXELS_WIDTH  );
    let height = Val::Px( SCREEN_PIXELS_HEIGHT );
    let background_color = BackgroundColor ( Color::NONE );

    let center_frame = NodeBundle
    {   style: Style
        {   width          : per100,
            height         : per100,
            position_type  : PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items    : AlignItems::Center,
            ..default()
        },
        background_color,
        ..default()
    };

    let title_frame = NodeBundle
    {   style: Style
        {   flex_direction: FlexDirection::Column,
            align_items   : AlignItems::Center,
            ..default()
        },
        background_color,
        ..default()
    };

    let header_frame = NodeBundle
    {   style: Style
        {   width,
            height,
            position_type  : PositionType::Absolute,
            flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::FlexStart, //画面の上端
            ..default()
        },
        background_color,
        ..default()
    };

    let footer_frame = NodeBundle
    {   style: Style
        {   width,
            height,
            position_type  : PositionType::Absolute,
            flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd, //画面の下端
            ..default()
        },
        background_color,
        ..default()
    };

    //ヘッダー
    let mut ui_header_left   = misc::text_ui( &HEADER_LEFT_TEXT  , TextAlignment::Center, &asset_svr );
    let mut ui_header_center = misc::text_ui( &HEADER_CENTER_TEXT, TextAlignment::Center, &asset_svr );
    let mut ui_header_right  = misc::text_ui( &HEADER_RIGHT_TEXT , TextAlignment::Center, &asset_svr );
    ui_header_left.style.align_self = AlignSelf::FlexStart;
    ui_header_left.text.alignment = TextAlignment::Left;
    ui_header_center.style.align_self = AlignSelf::Center;
    ui_header_center.text.alignment = TextAlignment::Center;
    ui_header_right.style.align_self = AlignSelf::FlexEnd;
    ui_header_right.text.alignment = TextAlignment::Right;

    //中央
    let mut ui_title = misc::text_ui( &CENTER_TITLE_TEXT, TextAlignment::Right , &asset_svr );
    let mut ui_demo  = misc::text_ui( &CENTER_DEMO_TEXT , TextAlignment::Center, &asset_svr );
    let mut ui_start = misc::text_ui( &CENTER_START_TEXT, TextAlignment::Center, &asset_svr );
    let mut ui_over  = misc::text_ui( &CENTER_OVER_TEXT , TextAlignment::Center, &asset_svr );
    let mut ui_clear = misc::text_ui( &CENTER_CLEAR_TEXT, TextAlignment::Center, &asset_svr );
    ui_title.style.position_type = PositionType::Relative;
    ui_demo.style.position_type  = PositionType::Relative;
    ui_title.visibility = Visibility::Inherited; //親のvisibility.is_visibleで表示を制御する
    ui_demo.visibility  = Visibility::Inherited; //親のvisibility.is_visibleで表示を制御する
    ui_start.visibility = Visibility::Hidden;
    ui_over.visibility  = Visibility::Hidden;
    ui_clear.visibility = Visibility::Hidden;

    //フッター
    let mut ui_footer_left   = misc::text_ui( &FOOTER_LEFT_TEXT  , TextAlignment::Center, &asset_svr );
    let mut ui_footer_center = misc::text_ui( &FOOTER_CENTER_TEXT, TextAlignment::Center, &asset_svr );
    let mut ui_footer_right  = misc::text_ui( &FOOTER_RIGHT_TEXT , TextAlignment::Center, &asset_svr );
    ui_footer_left.style.align_self = AlignSelf::FlexStart;
    ui_footer_left.text.alignment = TextAlignment::Left;
    ui_footer_center.style.align_self = AlignSelf::Center;
    ui_footer_center.text.alignment = TextAlignment::Center;
    ui_footer_right.style.align_self = AlignSelf::FlexEnd;
    ui_footer_right.text.alignment = TextAlignment::Right;

    //隠しフレームの中に子要素を作成する
    cmds.spawn( center_frame ).with_children
    (   | cmds |
        {   //ヘッダー
            cmds.spawn( header_frame ).with_children
            (   | cmds |
                {   cmds.spawn( ( ui_header_left  , HeaderLeft   ) );
                    cmds.spawn( ( ui_header_center, HeaderCenter ) );
                    cmds.spawn( ( ui_header_right , HeaderRight  ) );
                }
            );

            //中央
            cmds.spawn( ( title_frame, TEXT_UI_TITLE ) ).with_children
            (   | cmds |
                {   cmds.spawn( ui_title );
                    cmds.spawn( ui_demo  );
                }
            );
            cmds.spawn( ( ui_start, TEXT_UI_START ) );
            cmds.spawn( ( ui_over , TEXT_UI_OVER  ) );
            cmds.spawn( ( ui_clear, TEXT_UI_CLEAR ) );

            //フッター
            cmds.spawn( footer_frame ).with_children
            (   | cmds |
                {   cmds.spawn( ( ui_footer_left  , FooterLeft   ) );
                    cmds.spawn( ( ui_footer_center, FooterCenter ) );
                    cmds.spawn( ( ui_footer_right , FooterRight  ) );
                }
            );
        }
    );

    //おまけ(蟹)
    let px2d = Grid::new( SCREEN_GRIDS_WIDTH - 4, SCREEN_GRIDS_HEIGHT - 1 ).px2d_screen();
    let px3d = px2d.extend( DEPTH_SPRITE_KANI_DOTOWN );
    let custom_size = Some ( SIZE_GRID * MAGNIFY_SPRITE_KANI );
    cmds
    .spawn( SpriteBundle::default() )
    .insert( asset_svr.load( ASSETS_SPRITE_KANI_DOTOWN ) as Handle<Image> )
    .insert( Sprite { custom_size, ..default() } )
    .insert( Transform::from_translation( px3d ) )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(FPS)
fn update_footer_fps
(   mut q_ui: Query<&mut Text, With<FooterLeft>>,
    diag: Res<DiagnosticsStore>,
)
{   let Ok( mut ui ) = q_ui.get_single_mut() else { return };

    let fps_avr = diag
    .get( FrameTimeDiagnosticsPlugin::FPS )
    .map_or
    (   NA3_2.to_string(),
        | fps |
        fps.average().map_or( NA3_2.to_string(), | avg | format!( "{avg:06.2}" ) )
    );
    ui.sections[ 1 ].value = fps_avr;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.