use super::*;

//プラグインの設定
pub struct DemoPlay;
impl Plugin for DemoPlay
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   init_demoplay_record,                //demoでのrecordの初期化
                map::make_new_data,                  //新マップのデータ作成
                map::spawn_sprite,                   //スプライトをspawnする
                player::spawn_sprite,                //スプライトをspawnする
                chasers::spawn_sprite,               //スプライトをspawnする
                debug::spawn_sprite.run_if( DEBUG ), //スプライトをspawnする
            )
            .chain() //実行順を固定
            // .in_schedule( ENTER_TITLEDEMO )
        )
        .add_systems
        (   Update,
            (   player::scoring_and_clear_stage,      //スコアリング＆クリア判定⇒DemoLoop
                chasers::detect_collisions,           //衝突判定⇒DemoLoop
                player::move_sprite,                  //スプライト移動
                chasers::move_sprite,                 //スプライト移動
                debug::update_sprite.run_if( DEBUG ), //スプライト更新
            )
            .chain() //実行順を固定
            .run_if( in_state( MyState::TitleDemo ) )
            // .in_set( UPDATE_TITLEDEMO )
        )
        //------------------------------------------------------------------------------------------
        .add_systems
        (   Update,
            goto_title //無条件⇒TitleDemo
            .run_if( in_state( MyState::DemoLoop ) )
            // .in_set( UPDATE_DEMOLOOP )
        );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//demoクリアを除き、recordを初期化する
fn init_demoplay_record
(   mut record: ResMut<Record>,
)
{   if ! record.demo.clear_flag
    {   //GameOver後replayしなかった場合、demoで追手につかまった場合
        record.score = 0;
        record.stage = 0;
    }
    else
    {   //demoでステージクリアした場合
        record.demo.clear_flag = false;
    }
}

//無条件でStateを更新⇒TitleDemo
fn goto_title
(   mut state: ResMut<NextState<MyState>>,
)
{   state.set( MyState::TitleDemo );
}

//End of code.