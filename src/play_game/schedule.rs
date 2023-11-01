use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //InitApp処理後にGameStartへ遷移させる
        .insert_resource( AfterInitAppTo ( MyState::GameStart ) )

        //ResourceとEvent
        .init_resource::<Stage>()   //ステージの初期化
        .init_resource::<Score>()   //スコアの初期化
        .init_resource::<HiScore>() //ハイスコアの初期化
        .init_resource::<Map>()     //迷路の初期化
        .add_event::<EventClear>()  //ステージクリアイベントの登録
        .add_event::<EventOver>()   //ゲームオーバーイベントの登録

        .init_resource::<input::CrossDirection>() //十字方向の入力状態
        .init_resource::<ui_center::CountDownTimer>() //カウントダウンタイマーの初期化

        //submoduleのplugin
        .add_plugins( ui_header::Schedule ) //ヘッダー(Stage、Score、HiScore)
        .add_plugins( pause::Schedule )     //Pause処理

        //チェイサーの回転アニメーション
        .add_systems( Update, chasers::rotate )

        //GameStart-------------------------------------------------------------
        .add_systems
        (   OnEnter ( MyState::GameStart ),
            (   ui_center::spawn_in_hidden_frame::<ui_center::Title>,
                ui_center::spawn_in_hidden_frame::<ui_center::Start>,
                ui_center::spawn_in_hidden_frame::<ui_center::Clear>,
                ui_center::spawn_in_hidden_frame::<ui_center::Over>,

                misc::change_state::<TitleDemo>, //無条件遷移
            )
        )
        //GameStart-------------------------------------------------------------

        //タイトルを表示する
        //TitleDemo-------------------------------------------------------------
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   misc::show::<ui_center::Title>, //タイトル表示
            )
        )
        .add_systems
        (   Update,
            (   ui_center::hit_any_key::<ui_center::Title>, //Hit ANY Key
            )
            .run_if( in_state( MyState::TitleDemo ) )
        )
        .add_systems
        (   OnExit ( MyState::TitleDemo ),
            (   misc::hide::<ui_center::Title>, //タイトル非表示
            )
        )
        //TitleDemo-------------------------------------------------------------

        //ステージ初期化
        //StageStart------------------------------------------------------------
        .add_systems
        (   OnEnter ( MyState::StageStart ),
            (   // zero_clear_score_and_stage, //ゲーム開始時(非クリア時)の初期化

                //マップ作成とスプライト初期表示
                (   map::make_new_data,         //新マップのデータ作成
                    (   map::spawn_sprite,      //スプライトをspawnする
                        player::spawn_sprite,   //スプライトをspawnする
                        chasers::spawn_sprite,  //スプライトをspawnする
                    ),
                ).chain(),

                //Startメッセージの表示
                (   ui_center::init_countdown::<ui_center::Start>, //カウントダウン初期化
                    misc::show::<ui_center::Start>, //メッセージ表示
                ).chain(),
            )
        )
        .add_systems
        (   Update,
            (   ui_center::counting_down::<ui_center::Start>, //カウントダウン
            )
            .run_if( in_state( MyState::StageStart ) )
        )
        .add_systems
        (   OnExit ( MyState::StageStart ),
            (   misc::hide::<ui_center::Start>, //メッセージ非表示
            )
        )
        //StageStart------------------------------------------------------------

        //メインループ
        //MainLoop--------------------------------------------------------------
        .add_systems
        (   Update,
            (   judge::scoring_and_stageclear, //スコアリング＆クリア判定
                judge::detect_collisions,      //衝突判定
                (   //プレイヤーの移動
                    (   input::catch_player_operation,
                        player::move_sprite,
                    )
                    .chain(),

                    //チェイサー移動
                    chasers::move_sprite,
                )
            )
            .chain()
            .run_if( in_state( MyState::MainLoop ) )
        )
        //MainLoop--------------------------------------------------------------

        //ステージクリアの処理
        //StageClear------------------------------------------------------------
        .add_systems
        (   OnEnter ( MyState::StageClear ),
            (   ui_center::init_countdown::<ui_center::Clear>, //カウントダウン初期化
                misc::show::<ui_center::Clear>, //メッセージ表示
            )
            .chain()
        )
        .add_systems
        (   Update,
            (   ui_center::counting_down::<ui_center::Clear>, //カウントダウン
            )
            .run_if( in_state( MyState::StageClear ) )
        )
        .add_systems
        (   OnExit ( MyState::StageClear ),
            (   misc::hide::<ui_center::Clear>, //メッセージ非表示
            )
        )
        //StageClear------------------------------------------------------------

        //ゲームオーバーの処理
        //GameOver--------------------------------------------------------------
        .add_systems
        (   OnEnter ( MyState::GameOver ),
            (   ui_center::init_countdown::<ui_center::Over>, //カウントダウン初期化
                misc::show::<ui_center::Over>, //メッセージ表示
            )
            .chain()
        )
        .add_systems
        (   Update,
            (   ui_center::counting_down::<ui_center::Over>, //カウントダウン後Titleへ
                ui_center::hit_any_key::<ui_center::Over>,   //Hit ANY keyでReplay
            )
            .run_if( in_state( MyState::GameOver ) )
        )
        .add_systems
        (   OnExit ( MyState::GameOver ),
            (   misc::hide::<ui_center::Over>, //メッセージ非表示
            )
        )
        //GameOver--------------------------------------------------------------
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.

//         //----------------------------------------------------------------------
//         //ステージ初期化＆ゲーム開始のカウントダウン
//         .add_systems
//         (   OnEnter ( MyState::StageStart ),
//             (   zero_clear_score_and_stage, //ゲーム開始時(非クリア時)の初期化
//                 map::make_new_data,         //新マップのデータ作成
//                 (   map::spawn_sprite,      //スプライトをspawnする
//                     player::spawn_sprite,   //スプライトをspawnする
//                     chasers::spawn_sprite,  //スプライトをspawnする
//                 ),
//                 set_countdown_params::<Textui::Start>, //カウントダウン初期化
//                 misc::show_component::<Textui::Start>, //カウントダウン表示
//             )
//             .chain() //実行順を固定
//         )
//         .add_systems
//         (   Update,
//             (   countdown_message::<Textui::Start>, //カウントダウン後MainLoopへ
//             )
//             .run_if( in_state( MyState::StageStart ) )
//         )
//         .add_systems
//         (   OnExit ( MyState::StageStart ),
//             (   misc::hide_component::<Textui::Start>, //カウントダウン非表示
//             )
//         )

// ////////////////////////////////////////////////////////////////////////////////

// //ScoreとStageの初期化
// pub fn zero_clear_score_and_stage
// (   o_record: Option<ResMut<Record>>,
// )
// {   //トラブル除け
//     let Some ( mut record ) = o_record else { return };

//     //クリアした場合、scoreとstageをゼロクリアしない
//     if record.is_clear
//     {   record.is_clear = false;
//         return;
//     }

//     //scoreとstageをゼロクリア
//     record.score = 0;
//     record.stage = 0;
// }

// //End of code.