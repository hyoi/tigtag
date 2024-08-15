use super::*;

////////////////////////////////////////////////////////////////////////////////

//メッセージの設定
const TITLE_COLOR1: Color = Color::srgba( 0.6, 1.0, 0.4, 0.75 );
const TITLE_COLOR2: Color = Color::srgba( 0.0, 0.7, 0.5, 0.75 );

const UI_TITLE: &[ MessageSect ] =
&[  ( APP_TITLE, ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 3.5, TITLE_COLOR1 ),
    ( "\nv"    , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ( APP_VER  , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ( "     \n", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, Color::NONE  ),
];

const UI_DEMO: &[ MessageSect ] =
&[  ( "\nD E M O", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::GOLD ),
];

const UI_HIT_ANY_KEY: &[ MessageSect ] =
&[  ( "\n\n"          , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::NONE ),
    ( "Hit ANY key!\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
    ( "or\n"          , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, Color::CYAN ),
    ( "ANY button!"   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
];

////////////////////////////////////////////////////////////////////////////////

//可視化制御用のComponent
#[derive( Component )]
pub struct Message;

//タイトルロゴのComponent
#[derive( Component )]
pub struct TextTitleLogo;

//明滅効果を適用するためのComponent
#[derive( Component, Default )]
pub struct TextDEMO { blink_cycle: f32 }

//明滅させるためのトレイトの実装
impl effect::Blinking for TextDEMO
{   fn alpha( &mut self, time_delta: f32 ) -> f32
    {   let radian = &mut self.blink_cycle;
        *radian += TAU * time_delta;
        *radian -= if *radian > TAU { TAU } else { 0.0 };

        ( *radian ).sin() * 0.5 + 0.5 //0.0 ～ 1.0
    }
}

////////////////////////////////////////////////////////////////////////////////

//タイトルをspawnする
pub fn spawn_text
(   qry_hidden_node: Query<Entity, With<HiddenNode>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_node ) = qry_hidden_node.get_single() else { return };

    //メッセージの準備
    let mut ui_title = misc::text_ui( UI_TITLE      , &asset_svr );
    let     ui_demo  = misc::text_ui( UI_DEMO       , &asset_svr );
    let mut ui_hakey = misc::text_ui( UI_HIT_ANY_KEY, &asset_svr );

    ui_title.text.justify = JustifyText::Right;  //右寄せ
    ui_hakey.text.justify = JustifyText::Center; //センタリング

    let children =
    &[  cmds.spawn( ( ui_title, TextTitleLogo       ) ).id(),
        cmds.spawn( ( ui_demo , TextDEMO::default() ) ).id(),
        cmds.spawn(   ui_hakey                        ).id(),
    ];

    //レイアウト用の隠しノードの中に子要素を作成する
    let mut title_node = NodeBundle
    {   style: Style
        {   flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items    : AlignItems::Center,
            grid_column    : GridPlacement::start_end( 1, 4 ), //３列連結
            grid_row       : GridPlacement::start_end( 2, 3 ), //２行目
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    };

    if DEBUG()
    {   //debug時にborderを可視化
        title_node.style.border = UiRect::all( Val::Px( 1.0 ) );
        title_node.border_color = Color::GREEN.into();
    }

    //レイアウト用の隠しフレームの中に子要素を作成する
    let child = cmds.spawn( ( title_node, Message ) ).push_children( children ).id();
    cmds.entity( hidden_node ).add_child( child );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.