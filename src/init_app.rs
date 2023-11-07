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
                spawn_ui_footer,    //フッターを表示
                misc::change_state::<GameStart>, //無条件遷移
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
#[derive( Component )] pub struct UiFps;

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

//フッターをspawnする
fn spawn_ui_footer
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //隠しフレームを作成する(画面の下端 寄せ)
    let hidden_frame = misc::hidden_ui_frame( JustifyContent::FlexEnd );

    //フッターの準備
    let mut footer_left   = misc::text_ui( TEXT_FOOTER_LEFT  , &asset_svr );
    let mut footer_center = misc::text_ui( TEXT_FOOTER_CENTER, &asset_svr );
    let mut footer_right  = misc::text_ui( TEXT_FOOTER_RIGHT , &asset_svr );
    footer_left.style.align_self   = AlignSelf::FlexStart;
    footer_center.style.align_self = AlignSelf::Center;
    footer_right.style.align_self  = AlignSelf::FlexEnd;

    //隠しフレームの中に子要素を作成する
    cmds.spawn( hidden_frame ).with_children
    (   |cmds|
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