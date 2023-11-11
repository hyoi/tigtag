use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   OnEnter ( MyState::GameStart ),
            (   spawn_ui_header, //ヘッダーを表示
            )
        )
        .add_systems
        (   Update,
            (   update_stage,    //ステージ表示更新
                update_score,    //スコア表示更新
                update_hi_score, //ハイスコア表示更新
            )
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

//ヘッダーのComponent
#[derive( Component )] struct Stage;
#[derive( Component )] struct Score;
#[derive( Component )] struct HiScore;

////////////////////////////////////////////////////////////////////////////////

//フッターをspawnする
fn spawn_ui_header
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //隠しフレームを作成する(画面の上端 寄せ)
    let hidden_frame = misc::hidden_ui_frame( JustifyContent::FlexStart );

    //ヘッダーの準備
    let mut header_left   = misc::text_ui( TEXT_HEADER_LEFT  , &asset_svr );
    let mut header_center = misc::text_ui( TEXT_HEADER_CENTER, &asset_svr );
    let mut header_right  = misc::text_ui( TEXT_HEADER_RIGHT , &asset_svr );
    header_left.style.align_self   = AlignSelf::FlexStart;
    header_center.style.align_self = AlignSelf::Center;
    header_right.style.align_self  = AlignSelf::FlexEnd;

    //隠しフレームの中に子要素を作成する
    cmds.spawn( hidden_frame ).with_children
    (   |cmds|
        {   cmds.spawn( ( header_left  , Stage   ) );
            cmds.spawn( ( header_center, Score   ) );
            cmds.spawn( ( header_right , HiScore ) );
        }
    );
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