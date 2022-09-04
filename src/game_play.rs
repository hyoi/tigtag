use super::*;

//submodules
mod ui_update;              //text UI表示
use ui_update::UiUpdate;    //プラグイン

pub mod map;        //pub必須(demoplayモジュールからmap::spawn_spriteのように呼び出す都合上)
pub mod player;     //pub必須(demoplayモジュールからplayer::spawn_spriteのように呼び出す都合上)
pub mod chasers;    //pub必須(demoplayモジュールからchasers::spawn_spriteのように呼び出す都合上)

pub use map::*;
pub use player::*;
pub use chasers::*;

//プラグインの設定
pub struct GamePlay;
impl Plugin for GamePlay
{   fn build( &self, app: &mut App )
    {   //etc.
        app
        .add_plugin( UiUpdate )                                       //上下のtext UIの表示更新
        .add_system( chasers::rotate_sprite )                         //追手スプライトがあれば回転させる
        .insert_resource( MarkAfterFetchAssets ( GameState::Title ) ) //Assetsロード後のState変更先
        ;

        //GameState::Title
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::Title )                 //<ENTER>
            .with_system( show_component::<TextUiTitle> )           //text UI（Title）表示
        )
        .add_system_set
        (   SystemSet::on_update( GameState::Title )                //<UPDATE>
            .with_system( into_next_state_with_key::<TextUiTitle> ) //SPACEキー入力⇒Replay
        )
        .add_system_set
        (   SystemSet::on_exit( GameState::Title )                  //<EXIT>
            .with_system( hide_component::<TextUiTitle> )           //text UI（Title）消去
            .with_system( init_gameplay_record )                    //プレイ開始時の初期化
        )
        ;
        //------------------------------------------------------------------------------------------

        //GameState::Start
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::Start )             //<ENTER>
            .with_system( show_component::<TextUiStart> )       //text UI（Start）表示
            .with_system( set_countdown_params::<TextUiStart> ) //カウントダウンタイマー初期化
            .label( Mark::MakeMapNewData )                      //<label>
            .with_system( map::make_new_data )                  //新マップのデータ作成
        )
        .add_system_set
        (   SystemSet::on_enter( GameState::Start )             //<ENTER>
            .after( Mark::MakeMapNewData )                      //<after>
            .with_system( map::spawn_sprite )                   //スプライトをspawnする
            .with_system( player::spawn_sprite )                //スプライトをspawnする
            .with_system( chasers::spawn_sprite )               //スプライトをspawnする
        )
        .add_system_set
        (   SystemSet::on_update( GameState::Start )            //<UPDATE>
            .with_system( countdown_message::<TextUiStart> )    //カウントダウン後⇒MainLoop
        )
        .add_system_set
        (   SystemSet::on_exit( GameState::Start )              //<EXIT>
            .with_system( hide_component::<TextUiStart> )       //text UI（Start）消去
        )
        ;
        //------------------------------------------------------------------------------------------

        //GameState::MainLoop
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_update( GameState::MainLoop )         //<UPDATE>
            .before( Mark::DetectCollisions )                   //<before>
            .with_system( player::scoring_and_clear_stage )     //スコアリング＆クリア判定⇒ClearStage
        )
        .add_system_set
        (   SystemSet::on_update( GameState::MainLoop )         //<UPDATE>
            .label( Mark::DetectCollisions )                    //<label>
            .with_system( chasers::detect_collisions )          //衝突判定⇒GameOver
        )
        .add_system_set
        (   SystemSet::on_update( GameState::MainLoop )         //<UPDATE>
            .after( Mark::DetectCollisions )                    //<after>
            .with_system( player::move_sprite )                 //スプライト移動
            .with_system( chasers::move_sprite )                //スプライト移動
        )
        ;
        //------------------------------------------------------------------------------------------

        //GameState::GameOver
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::GameOver )              //<ENTER>
            .with_system( show_component::<TextUiOver> )            //text UI（GameOver）表示
            .with_system( set_countdown_params::<TextUiOver> )      //カウントダウンタイマー初期化
        )
        .add_system_set
        (   SystemSet::on_update( GameState::GameOver )             //<UPDATE>
            .with_system( countdown_message::<TextUiOver> )         //カウントダウン後⇒Title
            .with_system( into_next_state_with_key::<TextUiOver> )  //SPACEキー入力⇒Replay
        )
        .add_system_set
        (   SystemSet::on_exit( GameState::GameOver )               //<EXIT>
            .with_system( hide_component::<TextUiOver> )            //text UI（GameOver）消去
        )
        ;
        //------------------------------------------------------------------------------------------

        //GameState::Replay
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_update( GameState::Replay )           //<UPDATE>
            .with_system( goto_start )                          //無条件でStateを更新⇒Start 
        )
        .add_system_set
        (   SystemSet::on_exit( GameState::Replay )             //<EXIT>
            .with_system( init_gameplay_record )                //プレイ開始時の初期化
        )
        ;
        //------------------------------------------------------------------------------------------

        //GameState::ClearStage
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::ClearStage )        //<ENTER>
            .with_system( show_component::<TextUiClear> )       //text UI（ClearStage）表示
            .with_system( set_countdown_params::<TextUiClear> ) //カウントダウンタイマー初期化
        )
        .add_system_set
        (   SystemSet::on_update( GameState::ClearStage )       //<UPDATE>
            .with_system( countdown_message::<TextUiClear> )    //カウントダウン後⇒Start
        )
        .add_system_set
        (   SystemSet::on_exit( GameState::ClearStage )         //<EXIT>
            .with_system( hide_component::<TextUiClear> )       //text UI（ClearStage）消去
        )
        ;
        //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//プレイ開始時の初期化処理(スコアのゼロクリア等)
fn init_gameplay_record
(   mut record: ResMut<Record>,
)
{   record.set_mode_play();
    record.score = 0;
    record.stage = 0;
}

//無条件でStateを更新⇒Start
fn goto_start
(   mut state: ResMut<State<GameState>>,
)
{   let _ = state.overwrite_set( GameState::Start );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//キーが入力さたらStateを更新する
fn into_next_state_with_key<T: Component + TextUiWithHitKey>
(   mut q: Query<&T>,
    mut inkey: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
)
{   if let Ok ( target ) = q.get_single_mut()
    {   if ! inkey.just_pressed( target.key_code() ) { return }

        let _ = state.overwrite_set( target.next_state() );
    
        //NOTE: https://bevy-cheatbook.github.io/programming/states.html#with-input
        inkey.reset( target.key_code() );    
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//カウントダウンタイマーを初期化する
fn set_countdown_params<T: Component + TextUiWithCountDown>
(   mut q: Query<&T>,
    mut ctdw: ResMut<CountDown>,
)
{   if let Ok ( target ) = q.get_single_mut()
    {   ctdw.count = target.initial_value() + 1;
        ctdw.timer.reset();
    }
}

//カウントダウンを表示しゼロになったらStateを変更する
fn countdown_message<T: Component + TextUiWithCountDown>
(   mut q: Query<(&mut Text, &T)>,
    mut ctdw: ResMut<CountDown>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
)
{   if let Ok ( ( mut text, target ) ) = q.get_single_mut()
    {   let finished = ctdw.timer.tick( time.delta() ).finished();
        
        //1秒経過したら
        if finished
        {   ctdw.count -= 1;    //カウントダウン
            ctdw.timer.reset(); //1秒タイマーリセット
        }

        //カウントダウンが終わったら、次のStateへ遷移する
        if ctdw.count <= 0
        {   let _ = state.overwrite_set( target.next_state() );
            return;
        }

        //カウントダウンの表示を更新
        let n = ctdw.count - 1;
        text.sections[ target.placeholder() ].value = target.cd_string( n );
    }
}

//End of code.