use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //InitAppの後にGameStartへ遷移させる
        .insert_resource( init_app::AfterInitApp ( MyState::InitGame ) )

        //ResourceとEvent
        .init_resource::<Record>()   //ゲームの成績
        .init_resource::<map::Map>() //マップ情報
        .init_resource::<player::InputDirection>() //プレイヤーの入力(十字方向)

        .add_event::<EventClear>()  //ステージクリアイベントの伝達
        .add_event::<EventOver>()   //ゲームオーバーイベントの伝達
        .add_event::<EventEatDot>() //スコアリングイベントの伝達（demo用）

        //plugin
        .add_plugins( header::Schedule )     //ヘッダー(Stage、Score、HiScore)
        .add_plugins( title_demo::Schedule ) //デモ
        .add_plugins( pause::Schedule )      //Pause処理

        //State縛りなくアニメーションさせる(ゲーム中もPAUSE中も)
        .add_systems
        (   Update,
            (   //スプライトシートアニメーション
                animating_sprites::<player::Player>,
                animating_sprites::<chasers::Chaser>,

                //チェイサーの回転(スプライトシートがOFFの場合)
                chasers::rotate_chaser_shape.run_if( SPRITE_SHEET_OFF ),
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ゲーム初期化
        .add_systems
        (   OnEnter ( MyState::InitGame ),
            (   //UIの準備
                game_title ::spawn_text,
                stage_start::spawn_text,
                stage_clear::spawn_text,
                game_over  ::spawn_text,

                //無条件遷移
                misc::change_state::<TitleDemo>,
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //タイトル画面
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   //ゲームタイトの表示
                misc::show_component::<game_title::GameTitle>, //UI可視化
            )
        )
        .add_systems
        (   Update,
            (   //演出＆入力待ち
                effect::blinking::<game_title::Blinking>, //Demo の明滅
                effect::hit_any_key::<StageStart>,        //Hit ANY Key
            )
            .run_if( in_state( MyState::TitleDemo ) )
        )
        .add_systems
        (   OnExit ( MyState::TitleDemo ),
            (   //ゲームタイトの消去
                misc::hide_component::<game_title::GameTitle>, //UI不可視化
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ステージ初期化
        .add_systems
        (   OnEnter ( MyState::StageStart ),
            (   //ゲーム画面表示
                (   //scoreとstageをゼロクリア
                    initialize_record_except_hi_score, //StageClearの場合何もしない

                    //マップデータの作成
                    map::make_new_data,

                    //スプライトのspawn
                    (   map::spawn_sprite,
                        player::spawn_sprite,
                        chasers::spawn_sprite,
                    ),
                )
                .chain(), //実行順の固定

                (   //プレー開始メッセージの表示
                    effect::init_count::<stage_start::CountDown>,    //カウント初期化
                    misc::show_component::<stage_start::StageStart>, //UI可視化
                )
                .chain(), //実行順の固定
            )
        )
        .add_systems
        (   Update,
            (   //演出
                effect::count_down::<stage_start::CountDown>, //カウントダウン
            )
            .run_if( in_state( MyState::StageStart ) )
        )
        .add_systems
        (   OnExit ( MyState::StageStart ),
            (   //プレー開始メッセージの消去
                misc::hide_component::<stage_start::StageStart>, //UI不可視化
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //メインループ
        .add_systems
        (   Update,
            (   //ループ脱出条件
                detection::scoring_and_stage_clear, //スコアリング＆クリア判定
                detection::collisions_and_gameover, //衝突判定

                //スプライトの移動
                (   //自キャラ
                    (   player::catch_input_direction,
                        player::move_sprite,
                    )
                    .chain(), //実行順の固定

                    //敵キャラ
                    chasers::move_sprite,
                )
            )
            .chain() //実行順の固定
            .run_if( in_state( MyState::MainLoop ) )
        )

        ////////////////////////////////////////////////////////////////////////
        //ステージクリアの処理
        .add_systems
        (   OnEnter ( MyState::StageClear ),
            (   //ステージクリアの表示
                effect::init_count::<stage_clear::CountDown>,    //カウント初期化
                misc::show_component::<stage_clear::StageClear>, //UI可視化
            )
            .chain(), //実行順の固定
        )
        .add_systems
        (   Update,
            (   //演出
                effect::count_down::<stage_clear::CountDown>, //カウントダウン
            )
            .run_if( in_state( MyState::StageClear ) )
        )
        .add_systems
        (   OnExit ( MyState::StageClear ),
            (   //ステージクリアの消去
                misc::hide_component::<stage_clear::StageClear>, //UI不可視化
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ゲームオーバーの処理
        .add_systems
        (   OnEnter ( MyState::GameOver ),
            (   //ゲームオーバーの表示
                effect::init_count::<game_over::CountDown>,  //カウント初期化
                misc::show_component::<game_over::GameOver>, //UI可視化
            )
            .chain() //実行順の固定
        )
        .add_systems
        (   Update,
            (   //演出＆入力待ち
                effect::count_down::<game_over::CountDown>, //カウントダウン
                effect::blinking::<game_over::Blinking>,    //Replay? の明滅
                effect::hit_any_key::<StageStart>,          //Hit ANY Key
            )
            .run_if( in_state( MyState::GameOver ) )
        )
        .add_systems
        (   OnExit ( MyState::GameOver ),
            (   //ゲームオーバーの消去
                misc::hide_component::<game_over::GameOver>, //UI不可視化
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//キャラクターをアニメーションさせる
fn animating_sprites<T: Component + CharacterAnimation>
(   mut qry_target: Query<( &mut TextureAtlas, &mut T )>,
    time: Res<Time>,
)
{   for ( mut sprite, mut character ) in &mut qry_target
    {   if character.anime_timer_mut().tick( time.delta() ).just_finished()
        {   sprite.index += 1;
            let offset = character.sprite_sheet_offset( character.direction() );
            let frame  = character.sprite_sheet_frame();
            if sprite.index >= offset + frame { sprite.index = offset }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//ScoreとStageの初期化
pub fn initialize_record_except_hi_score
(   opt_record: Option<ResMut<Record>>,
)
{   let Some ( mut record ) = opt_record else { return };

    //クリアフラグが立っていた場合
    if record.is_clear()
    {   *record.is_clear_mut() = false;
        return;
    }

    //scoreとstageをゼロクリア
    *record.score_mut() = 0;
    *record.stage_mut() = 0;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.