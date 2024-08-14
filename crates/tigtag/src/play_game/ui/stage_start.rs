use super::*;

////////////////////////////////////////////////////////////////////////////////

//メッセージの設定
const UI_STAGE_START: &[ MessageSect ] =
&[  ( "Start\n"   , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::CYAN ),
    ( "\n"        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
    ( "Ready...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 2.5, Color::CYAN ),
    ( "\n"        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
    ( effect::CDPH, ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::GOLD ),
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
        {   count_down: 5, //カウントダウンの最大値
            count_text: UI_STAGE_START,
            next_state: MyState::MainLoop,
            timer     : Timer::from_seconds( 1.0, TimerMode::Once ),
        }
    }
}

//カウントダウンのトレイトの実装
impl<'a> effect::CountDown for CountDown<'a>
{   fn count_down( &mut self ) -> &mut i32 { &mut self.count_down }
    fn next_state( &self ) -> MyState { self.next_state }
    fn timer( &mut self ) -> &mut Timer { &mut self.timer }
    fn gen_message( &self, n: i32 ) -> String { if n == 0 { "Go!!".to_string() } else { n.to_string() } }
    fn placeholder( &self ) -> Option<usize> { self.count_text.iter().position( |x| x.0 == effect::CDPH ) }
    fn initialize( &mut self ) { *self = Self::default(); }
}

////////////////////////////////////////////////////////////////////////////////

//ステージスタートをspawnする
pub fn spawn_text
(   qry_hidden_node: Query<Entity, With<HiddenNode>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_node ) = qry_hidden_node.get_single() else { return };

    //メッセージの準備
    let mut ui = misc::text_ui( UI_STAGE_START, &asset_svr );
    ui.style.grid_column  = GridPlacement::start_end( 1, 4 ); //３列連結
    ui.style.grid_row     = GridPlacement::start_end( 2, 3 ); //２行目
    ui.style.align_self   = AlignSelf::Center;
    ui.style.justify_self = JustifySelf::Center;
    ui.text.justify       = JustifyText::Center;
    ui.visibility         = Visibility::Hidden; //初期状態

    //レイアウト用の隠しフレームの中に子要素を作成する
    let child_id = cmds.spawn( ( ui, Message, CountDown::default() ) ).id();
    cmds.entity( hidden_node ).add_child( child_id );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.