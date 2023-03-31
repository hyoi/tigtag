use super::*;

//submodules
mod ui_update;              //text UI表示
use ui_update::UiUpdate;    //プラグイン

pub mod map;     //pub必須(demoplayモジュールから呼び出すため)
pub mod player;  //pub必須(demoplayモジュールから呼び出すため)
pub mod chasers; //pub必須(demoplayモジュールから呼び出すため)

pub use map::*;     //pub必須(demoplayモジュールから呼び出すため)
pub use player::*;  //pub必須(demoplayモジュールから呼び出すため)
pub use chasers::*; //pub必須(demoplayモジュールから呼び出すため)

mod cross_button;    //ゲームパッドの十字キー入力
use cross_button::*; //プラグイン

//プラグインの設定
pub struct GamePlay;
impl Plugin for GamePlay
{   fn build( &self, app: &mut App )
    {   app
        .add_plugin( UiUpdate )                                         //header & footer UIの表示更新
        .add_system( chasers::rotate_sprite )                           //追手スプライトがあれば回転させる
        .insert_resource( MarkAfterFetchAssets ( MyState::TitleDemo ) ) //Assetsロード後のState変更先
        .add_plugin( CrossButton )                                      //ゲームパッドの十字キー入力
        //------------------------------------------------------------------------------------------
        .add_system
        (   show_component::<TextUiTitle> //text UI（Title）表示
            .in_schedule( ENTER_TITLEDEMO )
        )
        .add_system
        (   into_next_state_with_key::<TextUiTitle> //SPACEキー入力⇒GameStart
            .in_set( UPDATE_TITLEDEMO )
        )
        .add_system
        (   hide_component::<TextUiTitle> //text UI（Title）消去
            .in_schedule( EXIT_TITLEDEMO )
        )
        //------------------------------------------------------------------------------------------
        .add_system
        (   init_gameplay_record //初期化後 無条件⇒StageStart
            .in_set( UPDATE_GAMESTART )
        )
        //------------------------------------------------------------------------------------------
        .add_systems
        (   (   show_component::<TextUiStart>,       //text UI（Start）表示
                set_countdown_params::<TextUiStart>, //カウントダウンタイマー初期化
                map::make_new_data,                  //新マップのデータ作成
                map::spawn_sprite,                   //スプライトをspawnする
                player::spawn_sprite,                //スプライトをspawnする
                chasers::spawn_sprite,               //スプライトをspawnする
                debug::spawn_sprite.run_if( DEBUG ), //スプライトをspawnする
            )
            .chain().in_schedule( ENTER_STAGESTART )
        )
        .add_system
        (   countdown_message::<TextUiStart> //カウントダウン後⇒MainLoop
            .in_set( UPDATE_STAGESTART )
        )
        .add_system
        (   hide_component::<TextUiStart> //text UI（Start）消去
            .in_schedule( EXIT_STAGESTART )
        )
        //------------------------------------------------------------------------------------------
        .add_systems
        (   (   player::scoring_and_clear_stage,      //スコアリング＆クリア判定⇒StageClear
                chasers::detect_collisions,           //衝突判定⇒GameOver
                player::move_sprite,                  //スプライト移動
                chasers::move_sprite,                 //スプライト移動
                debug::update_sprite.run_if( DEBUG ), //スプライト移動
            )
            .chain().in_set( UPDATE_MAINLOOP )
        )
        //------------------------------------------------------------------------------------------
        .add_systems
        (   (   show_component::<TextUiClear>,       //text UI（StageClear）表示
                set_countdown_params::<TextUiClear>, //カウントダウンタイマー初期化
            )
            .in_schedule( ENTER_STAGECLEAR )
        )
        .add_system
        (   countdown_message::<TextUiClear> //カウントダウン後⇒StageStart
            .in_set( UPDATE_STAGECLEAR )
        )
        .add_system
        (   hide_component::<TextUiClear> //text UI（StageClear）消去
            .in_schedule( EXIT_STAGECLEAR )
        )
        //------------------------------------------------------------------------------------------
        .add_systems
        (   (   show_component::<TextUiOver>,       //text UI（GameOver）表示
                set_countdown_params::<TextUiOver>, //カウントダウンタイマー初期化
            )
            .in_schedule( ENTER_GAMEOVER )
        )
        .add_systems
        (   (   countdown_message::<TextUiOver>,        //カウントダウン後⇒TitleDemo
                into_next_state_with_key::<TextUiOver>, //SPACEキー入力⇒GameStart
            )
            .in_set( UPDATE_GAMEOVER )
        )
        .add_system
        (   hide_component::<TextUiOver> //text UI（GameOver）消去
            .in_schedule( EXIT_GAMEOVER )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲーム開始時の初期化とStageStartへの無条件遷移
fn init_gameplay_record
(   mut state: ResMut<NextState<MyState>>,
    mut record: ResMut<Record>,
)
{   //ゲーム開始時の初期化
    record.score = 0;
    record.stage = 0;

    //ステージ初期化へ進む
    state.set( MyState::StageStart );
}

//キーが入力さたらStateを更新する
fn into_next_state_with_key<T: Component + TextUiWithHitKey>
(   mut q: Query<&T>,
    mut record: ResMut<Record>,
    state: Res<State<MyState>>,
    mut next_state: ResMut<NextState<MyState>>,
    mut inkey: ResMut<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   if let Ok ( target ) = q.get_single_mut()
    {   //入力がないなら関数脱出
        if ! inkey.just_pressed( target.key_code() ) 
        && ! inbtn.just_pressed( target.btn_code() )
        { return }

        //MyState::GameStartへ遷移する前にゼロクリアする
        let current = state.0;
        if current == MyState::TitleDemo || current == MyState::GameOver
        {   record.score = 0;
            record.stage = 0;
        }
        
        next_state.set( target.next_state() );
    
        //NOTE: https://bevy-cheatbook.github.io/programming/states.html#with-input
        inkey.reset( target.key_code() );    
    }
}

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
    mut state: ResMut<NextState<MyState>>,
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
        {   state.set( target.next_state() );
            return;
        }

        //カウントダウンの表示を更新
        let n = ctdw.count - 1;
        text.sections[ target.placeholder() ].value = target.cd_string( n );
    }
}

//End of code.