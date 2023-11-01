use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   //初期表示
            OnExit ( MyState::GameStart ),
            (   header_stage,    //ステージ
                header_score,    //スコア
                header_hi_score, //ハイスコア
            )
        )
        .add_systems
        (   //表示更新
            Update,
            (   header_stage   .run_if( resource_changed::<Stage  >() ), //ステージ
                header_score   .run_if( resource_changed::<Score  >() ), //スコア
                header_hi_score.run_if( resource_changed::<HiScore>() ), //ハイスコア
            )
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(SCORE)
fn header_score
(   mut qry_text: Query<&mut Text, With<UiScore>>,
    opt_score: Option<Res<Score>>,
)
{   let Ok ( mut ui ) = qry_text.get_single_mut() else { return };
    let Some ( score ) = opt_score else { return };
    ui.sections[ 1 ].value = format!( "{:05}", score.get() );
}

//UIの表示を更新する(HI-SCORE)
fn header_hi_score
(   mut qry_text: Query<&mut Text, With<UiHiScore>>,
    opt_hi_score: Option<Res<HiScore>>,
)
{   let Ok ( mut ui ) = qry_text.get_single_mut() else { return };
    let Some ( hi_score ) = opt_hi_score else { return };
    ui.sections[ 1 ].value = format!( "{:05}", hi_score.get() );
}

//UIの表示を更新する(STAGE)
fn header_stage
(   mut qry_text: Query<&mut Text, With<UiStage>>,
    opt_stage: Option<Res<Stage>>,
)
{   let Ok ( mut ui ) = qry_text.get_single_mut() else { return };
    let Some ( stage ) = opt_stage else { return };
    ui.sections[ 1 ].value = format!( "{:02}", stage.get() );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.