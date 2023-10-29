use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //assetsの事前ロード後にInitAppへ遷移させる
        .insert_resource( AfterLoadAssetsTo ( MyState::InitApp ) )

        //ゲーム枠とフッターを表示する
        .add_systems
        (   OnEnter ( MyState::InitApp ),
            (   spawn_screen_frame, //ゲーム枠を表示
                spawn_text_ui,      //TEXT UIを表示
                misc::change_state_with_res::<AfterInitAppTo<MyState>>, //無条件遷移
            )
        )

        //footerにFPSを表示する
        .add_plugins( FrameTimeDiagnosticsPlugin ) //FPSプラグイン
        .add_systems( Update, update_fps )         //FPS表示更新
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//FPS表示のComponent
#[derive( Component )] struct UiFps;

//text UIの初期値
const NA2  : &str = "##";
const NA5  : &str = "#####";
const NA2_5: &str = "##-#####";
const NA3_2: &str = "###.##";

//ヘッダーの設定
counted_array!
(   const TEXT_HEADER_LEFT: [ MessageSect; _ ] =
    [   ( " STAGE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
        ( NA2      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
    ]
);
counted_array!
(   const TEXT_HEADER_CENTER: [ MessageSect; _ ] =
    [   ( " SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD   ),
        ( NA5      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE  ),
    ]
);
counted_array!
(   const TEXT_HEADER_RIGHT: [ MessageSect; _ ] =
    [   ( " Hi-SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
        ( NA5         , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
    ]
);

//フッターの設定
counted_array!
(   const TEXT_FOOTER_LEFT: [ MessageSect; _ ] =
    [   ( "  FPS ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.60, Color::TEAL   ),
        ( NA3_2   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.40, Color::SILVER ),
        ( " demo ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID       * 0.45, Color::TEAL   ),
        ( NA2_5   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.25, Color::SILVER ),
]
);
counted_array!
(   const TEXT_FOOTER_CENTER: [ MessageSect; _ ] =
    [   ( "hyoi 2021 - 2023", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL ),
    ]
);
counted_array!
(   const TEXT_FOOTER_RIGHT: [ MessageSect; _ ] =
    [   ( "Powered by ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
        ( "RUST"       , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
        ( " & "        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
        ( "BEVY  "     , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
    ]
);

//おまけ(蟹)
const GRID_X_KANI: i32 = SCREEN_GRIDS_WIDTH  - 4;
const GRID_Y_KANI: i32 = SCREEN_GRIDS_HEIGHT - 1;
const MAGNIFY_SPRITE_KANI: f32 = 0.9;
const COLOR_SPRITE_KANI: Color = Color::rgba( 1.0, 1.0, 1.0, 0.6 );

////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_screen_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( SIZE_GRID );
    let alpha = if DEBUG() { 0.5 } else { 1.0 }; //DEBUG時に透過させる
    let color = Color::rgba( 1.0, 1.0, 1.0, alpha );
    let regex = Regex::new( SCREEN_FRAME_LABEL_REGEX ).unwrap();
    let adjust = Vec2::X * PIXELS_PER_GRID / 2.0;

    for ( y, line ) in SCREEN_FRAME.design.iter().enumerate()
    {   //レンガのスプライトを敷き詰める
        for ( x, char ) in line.chars().enumerate()
        {   if char == SCREEN_FRAME_SPACE_CHAR { continue }

            let vec2 = IVec2::new( x as i32, y as i32 ).to_sprite_pixels();
            let vec3 = vec2.extend( DEPTH_SPRITE_GAME_FRAME );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
            ;
        }

        //ラベル文字列があるなら
        for m in regex.find_iter( line )
        {   let value = m.as_str().to_string();
            let style = TextStyle
            {   font     : asset_svr.load( ASSETS_FONT_PRESSSTART2P_REGULAR ),
                font_size: PIXELS_PER_GRID,
                color    : Color::SILVER,
            };
            let sections = vec![ TextSection { value, style } ];
            let vec2 = IVec2::new( m.start() as i32, y as i32 ).to_sprite_pixels() - adjust;
            let vec3 = vec2.extend( DEPTH_SPRITE_GAME_FRAME + 1.0 );

            cmds.spawn( Text2dBundle::default() )
            .insert( Text { sections, ..default() } )
            .insert( Anchor::CenterLeft )
            .insert( Transform::from_translation( vec3 ) )
            ;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//TEXT UIを配置する
fn spawn_text_ui
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //レイアウト用の隠しフレームの準備
    let width  = Val::Px( SCREEN_PIXELS_WIDTH  );
    let height = Val::Px( SCREEN_PIXELS_HEIGHT );
    let background_color = BackgroundColor ( Color::NONE );

    let hidden_frame_header = NodeBundle
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

    let hidden_frame_middle = NodeBundle
    {   style: Style
        {   width,
            height,
            position_type  : PositionType::Absolute,
            flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::Center, //画面の中央
            align_items    : AlignItems::Center,     //中央揃え
            ..default()
        },
        background_color,
        ..default()
    };

    let hidden_frame_footer = NodeBundle
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

    //ヘッダーの準備
    let mut header_left   = misc::text_ui( &TEXT_HEADER_LEFT  , &asset_svr );
    let mut header_center = misc::text_ui( &TEXT_HEADER_CENTER, &asset_svr );
    let mut header_right  = misc::text_ui( &TEXT_HEADER_RIGHT , &asset_svr );
    header_left.style.align_self   = AlignSelf::FlexStart;
    header_center.style.align_self = AlignSelf::Center;
    header_right.style.align_self  = AlignSelf::FlexEnd;

    //フッターの準備
    let mut footer_left   = misc::text_ui( &TEXT_FOOTER_LEFT  , &asset_svr );
    let mut footer_center = misc::text_ui( &TEXT_FOOTER_CENTER, &asset_svr );
    let mut footer_right  = misc::text_ui( &TEXT_FOOTER_RIGHT , &asset_svr );
    footer_left.style.align_self   = AlignSelf::FlexStart;
    footer_center.style.align_self = AlignSelf::Center;
    footer_right.style.align_self  = AlignSelf::FlexEnd;

    //隠しフレームと子要素を作成する
    cmds.spawn( hidden_frame_header ).with_children
    (   | cmds |
        {   cmds.spawn( ( header_left  , UiStage   ) );
            cmds.spawn( ( header_center, UiScore   ) );
            cmds.spawn( ( header_right , UiHiScore ) );
        }
    );
    cmds.spawn( ( hidden_frame_middle, HiddenFrameMiddle ) );
    cmds.spawn( hidden_frame_footer ).with_children
    (   | cmds |
        {   cmds.spawn( ( footer_left, UiFps ) );
            cmds.spawn(   footer_center        );
            cmds.spawn(   footer_right         );
        }
    );

    //おまけ(蟹)
    let custom_size = Some ( SIZE_GRID * MAGNIFY_SPRITE_KANI );
    let color = COLOR_SPRITE_KANI;
    let vec2 = IVec2::new( GRID_X_KANI, GRID_Y_KANI ).to_sprite_pixels();
    let vec3 = vec2.extend( DEPTH_SPRITE_KANI_DOTOWN );

    cmds.spawn( SpriteBundle::default() )
    .insert( Sprite { custom_size, color, ..default() } )
    .insert( Transform::from_translation( vec3 ) )
    .insert( asset_svr.load( ASSETS_SPRITE_KANI_DOTOWN ) as Handle<Image> )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//フッターを更新する(FPS)
fn update_fps
(   mut qry_text: Query<&mut Text, With<UiFps>>,
    diag_store: Res<DiagnosticsStore>,
)
{   let Ok( mut text ) = qry_text.get_single_mut() else { return };

    let fps_avr = diag_store
    .get( FrameTimeDiagnosticsPlugin::FPS )
    .map_or
    (   NA3_2.to_string(),
        | fps |
        fps.average()
        .map_or
        (   NA3_2.to_string(),
            | avg |
            format!( "{avg:06.2}" )
        )
    );

    text.sections[ 1 ].value = fps_avr;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.