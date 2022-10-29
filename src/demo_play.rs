use super::*;

//プラグインの設定
pub struct DemoPlay;
impl Plugin for DemoPlay
{   fn build( &self, app: &mut App )
    {   //GameState::TitleDemo
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::TitleDemo )         //<ENTER>
            .label( Mark::MakeMapNewData )                      //<label>
            .with_system( map::make_new_data )                  //新マップのデータ作成
        )
        .add_system_set
        (   SystemSet::on_enter( GameState::TitleDemo )         //<ENTER>
            .after( Mark::MakeMapNewData )                      //<after>
            .with_system( map::spawn_sprite )                   //スプライトをspawnする
            .with_system( player::spawn_sprite )                //スプライトをspawnする
            .with_system( chasers::spawn_sprite )               //スプライトをspawnする
        )
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .before( Mark::DetectCollisions )                   //<before>
            .with_system( player::scoring_and_clear_stage )     //スコアリング＆クリア判定⇒DemoLoop
        )
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .label( Mark::DetectCollisions )                    //<label>
            .with_system( chasers::detect_collisions )          //衝突判定⇒DemoLoop
        )
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .after( Mark::DetectCollisions )                    //<after>
            .with_system( player::move_sprite )                 //スプライト移動
            .with_system( chasers::move_sprite )                //スプライト移動
        )
        ;
        //------------------------------------------------------------------------------------------

        //debugで表示するスプライトの削除
        #[cfg( debug_assertions )]
        app
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .before( Mark::DetectCollisions )                   //<before>
            .with_system( despawn_entity::<PathFinder> )        //スプライト削除
        )
        ;

        //GameState::DemoNext
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_update( GameState::DemoLoop )         //<UPDATE>
            .with_system( goto_title )                          //無条件⇒TitleDemo
        )
        ;
        //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//無条件でStateを更新⇒TitleDemo
fn goto_title
(   mut state: ResMut<State<GameState>>,
)
{   let _ = state.overwrite_set( GameState::TitleDemo );
}

//End of code.