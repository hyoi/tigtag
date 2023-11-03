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
#[derive( Component )] struct UiStage;
#[derive( Component )] struct UiScore;
#[derive( Component )] struct UiHiScore;

////////////////////////////////////////////////////////////////////////////////

//フッターをspawnする
fn spawn_ui_header
(   qry_hidden_frame: Query<Entity, With<HiddenFrameHeader>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_frame ) = qry_hidden_frame.get_single() else { return };

    //ヘッダーの準備
    let mut header_left   = misc::text_ui( TEXT_HEADER_LEFT  , &asset_svr );
    let mut header_center = misc::text_ui( TEXT_HEADER_CENTER, &asset_svr );
    let mut header_right  = misc::text_ui( TEXT_HEADER_RIGHT , &asset_svr );
    header_left.style.align_self   = AlignSelf::FlexStart;
    header_center.style.align_self = AlignSelf::Center;
    header_right.style.align_self  = AlignSelf::FlexEnd;

    //レイアウト用の隠しフレームの中に子要素を作成する
    let child_left   = cmds.spawn( ( header_left  , UiStage   ) ).id();
    let child_center = cmds.spawn( ( header_center, UiScore   ) ).id();
    let child_right  = cmds.spawn( ( header_right , UiHiScore ) ).id();
    cmds.entity( hidden_frame ).add_child( child_left   );
    cmds.entity( hidden_frame ).add_child( child_center );
    cmds.entity( hidden_frame ).add_child( child_right  );
}

////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(SCORE)
fn update_score
(   mut qry_text: Query<&mut Text, With<UiScore>>,
    opt_record: Option<Res<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( record ) = opt_record else { return };

    let Some ( index ) = *PLACE_HOLDER_HEADER_CENTER else { return };
    text.sections[ index ].value = format!( "{:05}", record.score() );
}

//UIの表示を更新する(HI-SCORE)
fn update_hi_score
(   mut qry_text: Query<&mut Text, With<UiHiScore>>,
    opt_record: Option<Res<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( record ) = opt_record else { return };

    let Some ( index ) = *PLACE_HOLDER_HEADER_RIGHT else { return };
    text.sections[ index ].value = format!( "{:05}", record.hi_score() );
}

//UIの表示を更新する(STAGE)
fn update_stage
(   mut qry_text: Query<&mut Text, With<UiStage>>,
    opt_record: Option<Res<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( record ) = opt_record else { return };

    let Some ( index ) = *PLACE_HOLDER_HEADER_LEFT else { return };
    text.sections[ index ].value = format!( "{:02}", record.stage() );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.