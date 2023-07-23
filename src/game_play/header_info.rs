use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   Update,
            (   update_header_stage,    //ステージ
                update_header_score,    //スコア
                update_header_hi_score, //ハイスコア
            )
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(STAGE)
fn update_header_stage
(   mut q_text: Query<&mut Text, With<HeaderLeft>>,
    o_record: Option<Res<Record>>,
)
{   let Ok ( mut ui ) = q_text.get_single_mut() else { return };
    let Some ( record ) = o_record else { return };
    ui.sections[ 1 ].value = format!( "{:02}", record.stage );
}

//UIの表示を更新する(SCORE)
fn update_header_score
(   mut q_text: Query<&mut Text, With<HeaderCenter>>,
    o_record: Option<Res<Record>>,
    o_map: Option<Res<Map>>, //デバッグ用
)
{   let Ok ( mut ui ) = q_text.get_single_mut() else { return };
    let Some ( record ) = o_record else { return };
    ui.sections[ 1 ].value = format!( "{:05}", record.score );

    //デバッグ時、残ドット数を表示する
    let Some ( map ) = o_map else { return };
    ui.sections[ 2 ].value = if misc::DEBUG()
    {   format!( "/{:03}", map.remaining_dots )
    }
    else
    {   "".to_string()
    }
}

//UIの表示を更新する(HI-SCORE)
fn update_header_hi_score
(   mut q_text: Query<&mut Text, With<HeaderRight>>,
    o_record: Option<Res<Record>>,
)
{   let Ok ( mut ui ) = q_text.get_single_mut() else { return };
    let Some ( record ) = o_record else { return };
    ui.sections[ 1 ].value = format!( "{:05}", record.hi_score );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.