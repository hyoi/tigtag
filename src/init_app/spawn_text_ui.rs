use super::*;

//text UIを配置する
pub fn spawn_text_ui
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
    let header_frame = hidden_frame
    (   Style
        {   size           : Size::new( width, height ),
            position_type  : PositionType::Absolute,
            flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd, //画面の上端
            ..default()
        }
    );
    let footer_frame = hidden_frame
    (   Style
        {   size           : Size::new( width, height ),
            position_type  : PositionType::Absolute,
            flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::FlexStart, //画面の下端
            ..default()
        }
    );

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

//End of code.