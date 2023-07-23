use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems( Update, update_footer_demo_record ) //フッタの情報更新(demo record)
 
        //----------------------------------------------------------------------
        //デモプレイ
        .add_systems
        (   OnEnter ( MyState::Title ),
            (   game_play::zero_clear_score_and_stage, //ScoreとStageをゼロクリア
                game_play::map::make_new_data,         //新マップのデータ作成
                (   game_play::map::spawn_sprite,      //スプライトをspawnする
                    game_play::player::spawn_sprite,   //スプライトをspawnする
                    game_play::chasers::spawn_sprite,  //スプライトをspawnする
                )
            )
            .chain() //実行順を固定
        )
        .add_systems
        (   Update,
            (   game_play::player::scoring_and_clear_stage, //スコアリング＆クリア判定⇒DemoLoop
                game_play::chasers::detect_collisions,      //衝突判定⇒DemoLoop
                (   game_play::player::move_sprite,         //スプライト移動
                    game_play::chasers::move_sprite,        //スプライト移動
                )
            )
            .chain() //実行順を固定
            .run_if( in_state( MyState::Title ) )
        )

        //----------------------------------------------------------------------
        //一旦Titleから抜けてDemoLoopに入り、その後Titleへ戻る
        //(Stateを変更することでOnEnter等を実行させる)
        .add_systems
        (   OnEnter ( MyState::DemoLoop ),
            (   update_footer_demo_record, //フッタの情報更新(demo record)
                goto_title,
            )
            .chain() //実行順を固定
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//フッタの情報更新
fn update_footer_demo_record
(   mut q_text: Query<&mut Text, With<FooterLeft>>,
    o_record: Option<Res<Record>>,
)
{   //トラブル除け
    let Ok ( mut ui ) = q_text.get_single_mut() else { return };
    let Some ( record ) = o_record else { return };

    ui.sections[ 3 ].value = format!
    (   "{:02}-{:05}",
        record.demo.stage,
        record.demo.hi_score,
    );
}

//無条件でNextStateを更新
fn goto_title
(   mut state: ResMut<NextState<MyState>>,
)
{   state.set( MyState::Title );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.