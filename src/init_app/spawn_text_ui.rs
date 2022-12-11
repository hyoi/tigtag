use super::*;

//プラグインの設定
pub struct SpawnTextUi;
impl Plugin for SpawnTextUi
{   fn build( &self, app: &mut App )
    {   //GameState::Init
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_exit( GameState::InitApp )        //<EXIT>
            .with_system( spawn_text_ui )                   //text UIのspawn
        )
        ;
        //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//text UIを配置する
fn spawn_text_ui
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //レイアウト用の隠しフレームを作る
    let per100 = Val::Percent( 100.0 );
    let width  = Val::Px( SCREEN_PIXELS_WIDTH  );
    let height = Val::Px( SCREEN_PIXELS_HEIGHT );
    let center_frame = hidden_frame
    (   Style
        {   size           : Size::new( per100, per100 ),
            position_type  : PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items    : AlignItems::Center,
            ..default()
        }
    );
    let title_frame = hidden_frame
    (   Style
        {   flex_direction: FlexDirection::Column,
            align_items   : AlignItems::Center,
            ..default()
        }
    );

    let header_frame = hidden_frame
    (   Style
        {   size           : Size::new( width, height ),
            position_type  : PositionType::Absolute,
            flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::FlexStart, //画面の上端
            ..default()
        }
    );
    let footer_frame = hidden_frame
    (   Style
        {   size           : Size::new( width, height ),
            position_type  : PositionType::Absolute,
            flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd, //画面の下端
            ..default()
        }
    );

    //中央
    let mut ui_title = text_ui( &CENTER_TITLE_TEXT, HorizontalAlign::Right , &asset_svr );
    let mut ui_demo  = text_ui( &CENTER_DEMO_TEXT , HorizontalAlign::Center, &asset_svr );
    let mut ui_start = text_ui( &CENTER_START_TEXT, HorizontalAlign::Center, &asset_svr );
    let mut ui_over  = text_ui( &CENTER_OVER_TEXT , HorizontalAlign::Center, &asset_svr );
    let mut ui_clear = text_ui( &CENTER_CLEAR_TEXT, HorizontalAlign::Center, &asset_svr );
    let mut ui_pause = text_ui( &CENTER_PAUSE_TEXT, HorizontalAlign::Center, &asset_svr );

    ui_title.style.position_type = PositionType::Relative;
    ui_demo.style.position_type  = PositionType::Relative;

    ui_title.visibility.is_visible = true; //親のvisibility.is_visibleで表示を制御する
    ui_demo.visibility.is_visible  = true; //親のvisibility.is_visibleで表示を制御する
    ui_start.visibility.is_visible = false;
    ui_over.visibility.is_visible  = false;
    ui_clear.visibility.is_visible = false;
    ui_pause.visibility.is_visible = false;

    //ヘッダー
    let mut ui_header_left   = text_ui( &HEADER_LEFT_TEXT  , HorizontalAlign::Center, &asset_svr );
    let mut ui_header_center = text_ui( &HEADER_CENTER_TEXT, HorizontalAlign::Center, &asset_svr );
    let mut ui_header_right  = text_ui( &HEADER_RIGHT_TEXT , HorizontalAlign::Center, &asset_svr );

    ui_header_left.style.align_self = AlignSelf::FlexStart;
    ui_header_left.text.alignment.horizontal = HorizontalAlign::Left;

    ui_header_center.style.align_self = AlignSelf::Center;
    ui_header_center.text.alignment.horizontal = HorizontalAlign::Center;

    ui_header_right.style.align_self = AlignSelf::FlexEnd;
    ui_header_right.text.alignment.horizontal = HorizontalAlign::Right;

    //フッター
    let mut ui_footer_left   = text_ui( &FOOTER_LEFT_TEXT  , HorizontalAlign::Center, &asset_svr );
    let mut ui_footer_center = text_ui( &FOOTER_CENTER_TEXT, HorizontalAlign::Center, &asset_svr );
    let mut ui_footer_right  = text_ui( &FOOTER_RIGHT_TEXT , HorizontalAlign::Center, &asset_svr );

    ui_footer_left.style.align_self = AlignSelf::FlexStart;
    ui_footer_left.text.alignment.horizontal = HorizontalAlign::Left;

    ui_footer_center.style.align_self = AlignSelf::Center;
    ui_footer_center.text.alignment.horizontal = HorizontalAlign::Center;

    ui_footer_right.style.align_self = AlignSelf::FlexEnd;
    ui_footer_right.text.alignment.horizontal = HorizontalAlign::Right;

    //隠しフレームの上に子要素を作成する
    cmds.spawn( center_frame ).with_children
    (   | cmds |
        {   //中央
            cmds.spawn( ( title_frame, TEXT_UI_TITLE ) ).with_children
            (   | cmds |
                {   cmds.spawn( ui_title );
                    cmds.spawn( ui_demo  );
                }
            );
            cmds.spawn( ( ui_start, TEXT_UI_START ) );
            cmds.spawn( ( ui_over , TEXT_UI_OVER  ) );
            cmds.spawn( ( ui_clear, TEXT_UI_CLEAR ) );
            cmds.spawn( ( ui_pause, TextUiPause   ) );

            //ヘッダー
            cmds.spawn( header_frame ).with_children
            (   | cmds |
                {   cmds.spawn( ( ui_header_left  , HeaderLeft   ) );
                    cmds.spawn( ( ui_header_center, HeaderCenter ) );
                    cmds.spawn( ( ui_header_right , HeaderRight  ) );
                }
            );

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

    //おまけ
    let pixel = Grid::new( SCREEN_GRIDS_WIDTH - 4, SCREEN_GRIDS_HEIGHT - 1 ).into_pixel_screen();
    let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) * MAGNIFY_SPRITE_KANI );
    cmds
    .spawn( SpriteBundle::default() )
    .insert( Sprite { custom_size, ..default() } )
    .insert( asset_svr.load( ASSETS_SPRITE_KANI_DOTOWN ) as Handle<Image> )
    .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_KANI_DOTOWN ) ) )
    ;
}

//text UIのレイアウト用に隠しフレームを作る
fn hidden_frame
(   style: Style,
) -> NodeBundle
{   let background_color = BackgroundColor ( Color::NONE );
    NodeBundle { style, background_color, ..default() }
}

//text UI用にTextBundleを作る
fn text_ui
(   message: &[ MessageSect ],
    horizontal: HorizontalAlign,
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
        horizontal,
    };
    let position_type = PositionType::Absolute;

    let text  = Text { sections, alignment };
    let style = Style { position_type, ..default() };
    TextBundle { text, style, ..default() }
}

//End of code.