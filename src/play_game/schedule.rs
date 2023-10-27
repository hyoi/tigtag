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

        .init_resource::<CrossButton>() //十字ボタンの入力状態保存用

        //ヘッダーの表示(Stage、Score、HiScore)
        .add_plugins( header::Schedule )

        //Pause処理
        .add_plugins( pause::Schedule )

        //チェイサーの回転アニメーション
        .add_systems( Update, chasers::rotate )

        //GameStart＜仮＞
        .add_systems
        (   OnEnter ( MyState::GameStart ),
            misc::change_state::<StageStart> //＜仮＞無条件遷移
        )

        //StageStart
        .add_systems
        (   OnEnter ( MyState::StageStart ),
            (   // zero_clear_score_and_stage, //ゲーム開始時(非クリア時)の初期化
                map::make_new_data,         //新マップのデータ作成
                (   map::spawn_sprite,      //スプライトをspawnする
                    player::spawn_sprite,   //スプライトをspawnする
                    chasers::spawn_sprite,  //スプライトをspawnする
                ),
                // set_countdown_params::<TextUiStart>, //カウントダウン初期化
                // misc::show_component::<TextUiStart>, //カウントダウン表示

                misc::change_state::<MainLoop> //＜仮＞無条件遷移
            )
            .chain() //実行順を固定
        )

        //MainLoop
        .add_systems
        (   Update,
            (   // player::scoring_and_clear_stage, //スコアリング＆クリア判定⇒StageClear
                // chasers::detect_collisions,      //衝突判定⇒GameOver
                (   player::move_sprite,         //スプライト移動
                    // chasers::move_sprite,        //スプライト移動
                )
            )
            .chain() //実行順を固定
            .run_if( in_state( MyState::MainLoop ) )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.



// //プラグインの設定
// pub struct Schedule;
// impl Plugin for Schedule
// {   fn build( &self, app: &mut App )
//     {   app

        // //ResourceとEvent
        // .init_resource::<CountDown>()   //カウントダウンタイマーの初期化

//         .add_systems( Update, player::catch_cross_button_pressed ) //十字ボタンの入力読み取り

//         //----------------------------------------------------------------------
//         //タイトルを表示する
//         //(assetsロード後に[AfterInitApp]の値を参照して遷移する)
//         .insert_resource( AfterInitApp ( MyState::Title ) )
//         .add_systems
//         (   OnEnter ( MyState::Title ),
//             (   misc::show_component::<TextUiTitle>, //タイトル表示
//             )
//         )
//         .add_systems
//         (   Update,
//             (   goto_nextstate_with_hitanykey::<TextUiTitle>, //Hit ANY Key
//             )
//             .run_if( in_state( MyState::Title ) )
//         )
//         .add_systems
//         (   OnExit ( MyState::Title ),
//             (   misc::hide_component::<TextUiTitle>, //タイトル非表示
//             )
//         )

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
//                 set_countdown_params::<TextUiStart>, //カウントダウン初期化
//                 misc::show_component::<TextUiStart>, //カウントダウン表示
//             )
//             .chain() //実行順を固定
//         )
//         .add_systems
//         (   Update,
//             (   countdown_message::<TextUiStart>, //カウントダウン後MainLoopへ
//             )
//             .run_if( in_state( MyState::StageStart ) )
//         )
//         .add_systems
//         (   OnExit ( MyState::StageStart ),
//             (   misc::hide_component::<TextUiStart>, //カウントダウン非表示
//             )
//         )

//         //----------------------------------------------------------------------
//         //メインループ
//         .add_systems
//         (   Update,
//             (   player::scoring_and_clear_stage, //スコアリング＆クリア判定⇒StageClear
//                 chasers::detect_collisions,      //衝突判定⇒GameOver
//                 (   player::move_sprite,         //スプライト移動
//                     chasers::move_sprite,        //スプライト移動
//                 )
//             )
//             .chain() //実行順を固定
//             .run_if( in_state( MyState::MainLoop ) )
//         )

//         //----------------------------------------------------------------------
//         //ステージクリアの処理
//         .add_systems
//         (   OnEnter ( MyState::StageClear ),
//             (   set_countdown_params::<TextUiClear>, //カウントダウン初期化
//                 misc::show_component::<TextUiClear>, //カウントダウン表示
//             )
//             .chain() //実行順を固定
//         )
//         .add_systems
//         (   Update,
//             (   countdown_message::<TextUiClear>, //カウントダウン後StageStartへ
//             )
//             .run_if( in_state( MyState::StageClear ) )
//         )
//         .add_systems
//         (   OnExit ( MyState::StageClear ),
//             (   misc::hide_component::<TextUiClear>, //カウントダウン非表示
//             )
//         )

//         //----------------------------------------------------------------------
//         //ゲームオーバーの処理
//         .add_systems
//         (   OnEnter ( MyState::GameOver ),
//             (   set_countdown_params::<TextUiOver>, //カウントダウン初期化
//                 misc::show_component::<TextUiOver>, //カウントダウン表示
//             )
//             .chain() //実行順を固定
//         )
//         .add_systems
//         (   Update,
//             (   countdown_message::<TextUiOver>,             //カウントダウン後Titleへ
//                 goto_nextstate_with_hitanykey::<TextUiOver>, //Hit ANY keyでReplay
//             )
//             .run_if( in_state( MyState::GameOver ) )
//         )
//         .add_systems
//         (   OnExit ( MyState::GameOver ),
//             (   misc::hide_component::<TextUiOver>, //カウントダウン非表示
//             )
//         )
//         ;
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //キー入力さたらStateを遷移させる
// fn goto_nextstate_with_hitanykey<T: Component + WithHitAnyKey>
// (   q_text_ui: Query<&T>,
//     mut next_state: ResMut<NextState<MyState>>,
//     inkey: Res<Input<KeyCode>>,
//     inbtn: Res<Input<GamepadButton>>,
//     o_now_gamepad: Option<Res<NowGamepad>>,
// )
// {   //トラブル除け
//     let Ok ( text_ui ) = q_text_ui.get_single() else { return };
//     let Some ( now_gamepad ) = o_now_gamepad else { return };

//     //HIT ANY KEY処理の例外になる入力があるなら
//     for key in HAK_IGNORE_KEYS
//     {   if inkey.pressed( key ) { return }
//     }
//     if let Some ( gamepad ) = now_gamepad.0
//     {   for button in HAK_IGNORE_BUTTONS
//         {   if inbtn.pressed( GamepadButton::new( gamepad, button ) )
//             {   return;
//             }
//         }
//     }

//     //ゲームパッドの接続があるか、入力があるか
//     let gamepad = now_gamepad.0.map
//     (   |gamepad|
//         {   //接続がある(入力は0個以上)
//             inbtn.get_just_pressed()
//             .filter( |button| button.gamepad == gamepad )
//             .count()
//         }
//     );

//     //入力がないなら
//     if inkey.get_just_pressed().len() == 0
//     {   if gamepad.is_none() { return } //ゲームパッドの接続がない
//         if gamepad.is_some_and( |x| x == 0 ) { return } //ゲームパッドの入力がない
//     }

//     //Stateを遷移させる
//     next_state.set( text_ui.next_state() );
// }

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

// ////////////////////////////////////////////////////////////////////////////////

// //カウントダウンを初期化する
// fn set_countdown_params<T: Component + WithCountDown>
// (   mut q_text_ui: Query<&T>,
//     mut countdown: ResMut<CountDown>,
// )
// {   if let Ok ( text_ui ) = q_text_ui.get_single_mut()
//     {   countdown.count = text_ui.initial_value() + 1;
//         countdown.timer.reset();
//     }
// }

// //カウントダウンを表示しゼロになったらStateを変更する
// fn countdown_message<T: Component + WithCountDown>
// (   mut q_text_ui: Query<(&mut Text, &T)>,
//     mut countdown: ResMut<CountDown>,
//     mut state: ResMut<NextState<MyState>>,
//     time: Res<Time>,
// )
// {   if let Ok ( ( mut text, text_ui ) ) = q_text_ui.get_single_mut()
//     {   let finished = countdown.timer.tick( time.delta() ).finished();
        
//         //1秒経過したら
//         if finished
//         {   countdown.count -= 1;    //カウントダウン
//             countdown.timer.reset(); //1秒タイマーリセット
//         }

//         //カウントダウンが終わったら、次のStateへ遷移する
//         if countdown.count <= 0
//         {   state.set( text_ui.next_state() );
//             return;
//         }

//         //カウントダウンの表示を更新
//         let message = text_ui.cd_string( countdown.count - 1 );
//         text.sections[ text_ui.placeholder() ].value = message;
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //End of code.