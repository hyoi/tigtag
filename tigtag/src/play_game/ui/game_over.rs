use super::*;

////////////////////////////////////////////////////////////////////////////////

//メッセージの設定
const UI_GAME_OVER: &[ MessageSect ] =
&[  ( "Game Over\n", ASSETS_FONT_REGGAEONE_REGULAR   , PIXELS_PER_GRID * 5.5, Color::RED  ),
    ( " \n \n"     , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.6, Color::NONE ),
];

const UI_REPLAY: &[ MessageSect ] =
&[  ( "REPLAY?", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::GOLD ),
];

const UI_HIT_ANY_KEY: &[ MessageSect ] =
&[  ( "\n"            , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::NONE ),
    ( "Hit ANY key!\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
    ( "or\n"          , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, Color::CYAN ),
    ( "ANY button!\n" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
    ( effect::CDPH    , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 4.0, Color::GOLD ),
];

////////////////////////////////////////////////////////////////////////////////

//可視化制御用のComponent
#[derive( Component )]
pub struct Message;

//カウントダウンを適用するためのComponent
#[derive( Component )]
pub struct CountDown<'a>
{   count_down: i32,
    count_text: &'a [ MessageSect ],
    next_state: MyState,
    timer     : Timer,
}

impl<'a> Default for CountDown<'a>
{   fn default() -> Self
    {   Self
        {   count_down: 10, //カウントダウンの最大値
            count_text: UI_HIT_ANY_KEY,
            next_state: MyState::TitleDemo,
            timer     : Timer::from_seconds( 1.0, TimerMode::Once ),
        }
    }
}

//カウントダウンのトレイトの実装
impl<'a> effect::CountDown for CountDown<'a>
{   fn count_down( &mut self ) -> &mut i32 { &mut self.count_down }
    fn next_state( &self ) -> MyState { self.next_state }
    fn timer( &mut self ) -> &mut Timer { &mut self.timer }
    fn gen_message( &self, n: i32 ) -> String { n.to_string() }
    fn placeholder( &self ) -> Option<usize> { self.count_text.iter().position( |x| x.0 == effect::CDPH ) }
    fn initialize( &mut self ) { *self = Self::default(); }
}

//明滅効果を適用するためのComponent
#[derive( Component, Default )]
pub struct TextREPLAY { blink_cycle: f32 }

//明滅させるためのトレイトの実装
impl effect::Blinking for TextREPLAY
{   fn alpha( &mut self, time_delta: f32 ) -> f32
    {   let radian = &mut self.blink_cycle;
        *radian += TAU * time_delta;
        *radian -= if *radian > TAU { TAU } else { 0.0 };

        ( *radian ).sin() * 0.5 + 0.5 //0.0 ～ 1.0
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームオーバーをspawnする
pub fn spawn_text
(   qry_hidden_node: Query<Entity, With<init_app::HiddenNode>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_node ) = qry_hidden_node.get_single() else { return };

    //メッセージの準備
    let ui_game_over = misc::text_ui( UI_GAME_OVER  , &asset_svr );
    let ui_replay    = misc::text_ui( UI_REPLAY     , &asset_svr );
    let mut ui_hakey = misc::text_ui( UI_HIT_ANY_KEY, &asset_svr );

    ui_hakey.text.justify = JustifyText::Center; //センタリング

    let children =
    &[  cmds.spawn(   ui_game_over                       ).id(),
        cmds.spawn( ( ui_replay, TextREPLAY::default() ) ).id(),
        cmds.spawn( ( ui_hakey , CountDown::default()  ) ).id(),
    ];

    //レイアウト用の隠しノードの中に子要素を作成する
    let mut game_over_node = NodeBundle
    {   style: Style
        {   flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items    : AlignItems::Center,
            grid_column    : GridPlacement::start_end( 1, 4 ), //３列連結
            grid_row       : GridPlacement::start_end( 2, 3 ), //２行目
            ..default()
        },
        background_color: Color::NONE.into(),
        visibility: Visibility::Hidden, //初期非表示
        ..default()
    };

    if DEBUG()
    {   //debug時にborderを可視化
        game_over_node.style.border = UiRect::all( Val::Px( 1.0 ) );
        game_over_node.border_color = Color::GREEN.into();
    }

    //レイアウト用の隠しフレームの中に子要素を作成する
    let child = cmds.spawn( ( game_over_node, Message ) ).push_children( children ).id();
    cmds.entity( hidden_node ).add_child( child );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.