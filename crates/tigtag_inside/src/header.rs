use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   OnEnter ( MyState::InitGame ),
            (   spawn_ui_header, //ヘッダーのUIをspawn
            )
        )
        .add_systems
        (   Update,
            (   update_stage,    //ステージ表示更新
                update_score,    //スコア表示更新
                update_hi_score, //ハイスコア表示更新
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//ヘッダーのComponent
#[derive( Component )] struct Stage;
#[derive( Component )] struct Score;
#[derive( Component )] struct HiScore;

//ヘッダー＆フッターのプレイスホルダー
const NA2  : &str = "##";
const NA5  : &str = "#####";
const NA2_5: &str = "##-#####";
const NA3_2: &str = "###.##";
const PLACE_HOLDERS_HEAD_FOOT: &[ &str ] = &[ NA2, NA5, NA2_5, NA3_2 ];

//ヘッダーの設定
const TEXT_HEADER_LEFT: &[ MessageSect ] =
&[  ( " STAGE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
    ( NA2      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
];
static PLACE_HOLDER_HEADER_LEFT: Lazy<Option<usize>> = Lazy::new
(   || TEXT_HEADER_LEFT.iter().position( |x| PLACE_HOLDERS_HEAD_FOOT.contains( &x.0 ) )
);

const TEXT_HEADER_CENTER: &[ MessageSect ] =
&[  ( " SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
    ( NA5      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
];
static PLACE_HOLDER_HEADER_CENTER: Lazy<Option<usize>> = Lazy::new
(   || TEXT_HEADER_CENTER.iter().position( |x| PLACE_HOLDERS_HEAD_FOOT.contains( &x.0 ) )
);

const TEXT_HEADER_RIGHT: &[ MessageSect ] =
&[  ( " Hi-SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
    ( NA5         , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
];
static PLACE_HOLDER_HEADER_RIGHT: Lazy<Option<usize>> = Lazy::new
(   || TEXT_HEADER_RIGHT.iter().position( |x| PLACE_HOLDERS_HEAD_FOOT.contains( &x.0 ) )
);

////////////////////////////////////////////////////////////////////////////////

//削除するUIをQueryする準備
type RemoveTargets = Or
<(  With<UiHeaderLeft>,
    With<UiHeaderCenter>,
    With<UiHeaderRight>
)>;

//ヘッダーをspawnする
fn spawn_ui_header
(   qry_hidden_node: Query<Entity, With<HiddenNode>>,
    qry_remove_targets: Query<Entity, RemoveTargets>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //隠しフレームの取得
    let Ok ( hidden_frame_id ) = qry_hidden_node.get_single() else { return };

    //ヘッダーの準備
    let mut header_left   = misc::text_ui( TEXT_HEADER_LEFT  , &asset_svr );
    let mut header_center = misc::text_ui( TEXT_HEADER_CENTER, &asset_svr );
    let mut header_right  = misc::text_ui( TEXT_HEADER_RIGHT , &asset_svr );

    //グリッド１行目（ヘッダー）のレイアウト指定
    header_left.style.grid_row       = GridPlacement::start( 1 ); //ヘッダーに配置
    header_left.style.grid_column    = GridPlacement::start( 1 ); //左端のセル
    header_left.style.align_self     = AlignSelf::Start;          //上段寄せ
    header_left.style.justify_self   = JustifySelf::Start;        //左寄せ

    header_center.style.grid_row     = GridPlacement::start( 1 ); //ヘッダーに配置
    header_center.style.grid_column  = GridPlacement::start( 2 ); //右端のセル
    header_center.style.align_self   = AlignSelf::Start;          //上段寄せ
    header_center.style.justify_self = JustifySelf::Center;       //中央寄せ

    header_right.style.grid_row      = GridPlacement::start( 1 ); //ヘッダーに配置
    header_right.style.grid_column   = GridPlacement::start( 3 ); //右端のセル
    header_right.style.align_self    = AlignSelf::Start;          //上段寄せ
    header_right.style.justify_self  = JustifySelf::End;          //右寄せ

    //子要素をspawnして隠しフレームの養子にする
    let children = vec!
    [   cmds.spawn( ( header_left  , Stage   ) ).id(),
        cmds.spawn( ( header_center, Score   ) ).id(),
        cmds.spawn( ( header_right , HiScore ) ).id(),
    ];
    cmds.entity( hidden_frame_id ).push_children( &children );

    //要らないUIの削除(cmdがmoveするので呼び出し位置注意)
    misc::despawn_by_filter( qry_remove_targets, cmds );
}

////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(SCORE)
fn update_score
(   mut qry_text: Query<&mut Text, With<Score>>,
    opt_record: Option<Res<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( record ) = opt_record else { return };

    let Some ( index ) = *PLACE_HOLDER_HEADER_CENTER else { return };
    text.sections[ index ].value = format!( "{:05}", record.score() );
}

//UIの表示を更新する(HI-SCORE)
fn update_hi_score
(   mut qry_text: Query<&mut Text, With<HiScore>>,
    opt_record: Option<Res<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( record ) = opt_record else { return };

    let Some ( index ) = *PLACE_HOLDER_HEADER_RIGHT else { return };
    text.sections[ index ].value = format!( "{:05}", record.hi_score() );
}

//UIの表示を更新する(STAGE)
fn update_stage
(   mut qry_text: Query<&mut Text, With<Stage>>,
    opt_record: Option<Res<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( record ) = opt_record else { return };

    let Some ( index ) = *PLACE_HOLDER_HEADER_LEFT else { return };
    text.sections[ index ].value = format!( "{:02}", record.stage() );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.