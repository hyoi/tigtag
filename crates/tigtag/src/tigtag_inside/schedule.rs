use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //InitAppの後にGameStartへ遷移させる
        .insert_resource( AfterInitApp ( MyState::InitGame ) )

        //Resource
        .init_resource::<Record>()   //ゲームの成績
        .init_resource::<map::Map>() //マップ情報
        .init_resource::<player::InputDirection>() //プレイヤーの入力(十字方向)

        //Event
        .add_event::<EventClear>()  //ステージクリアの伝達
        .add_event::<EventOver>()   //ゲームオーバーの伝達
        .add_event::<EventEatDot>() //スコアリングの伝達
        .add_event::<EventTimerPlayer>()  //自キャラ移動タイマーのfinishedの伝達
        .add_event::<EventTimerChasers>() //敵キャラ移動タイマーのfinishedの伝達

        //plugin
        .add_plugins( header::Schedule ) //ヘッダー更新(Stage、Score、HiScore)
        .add_plugins( demo::Schedule   ) //タイトル画面のデモプレイ
        .add_plugins( pause::Schedule  ) //Pause処理

        //State縛りなくアニメーションさせる(ゲーム中もPAUSE中も)
        .add_systems
        (   Update,
            (   //スプライトシートアニメーション
                animating_sprites::<player::Player>,
                animating_sprites::<chasers::Chaser>,

                //チェイサーの回転(スプライトシートがOFFの場合)
                chasers::rotate_chaser_shape.run_if( SPRITE_OFF ),
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ゲーム初期化
        .add_systems
        (   OnEnter ( MyState::InitGame ),
            (   //TextUIの準備
                title_demo ::spawn_text,
                stage_start::spawn_text,
                stage_clear::spawn_text,
                game_over  ::spawn_text,

                //無条件遷移
                change_state_to::<TitleDemo>,
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //タイトル画面
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   //TextUIの可視化
                misc::show_component::<title_demo::Message>,
            )
        )
        .add_systems
        (   Update,
            (   //TextUIの演出＆入力待ち
                effect::blinking_text::<title_demo::TextDEMO>, //Demo の明滅
                effect::hit_any_key::<StageStart>, //Hit ANY Key
            )
            .run_if( in_state( MyState::TitleDemo ) )
        )
        .add_systems
        (   OnExit ( MyState::TitleDemo ),
            (   //TextUIの不可視化
                misc::hide_component::<title_demo::Message>,

                //scoreとstageをゼロクリアする(DEMOでステージクリアの時はしない)
                initialize_record_except_hi_score,
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ステージ初期化
        .add_systems
        (   OnEnter ( MyState::StageStart ),
            (   (   //マップデータ生成
                    map::make_new_data,

                    //スプライトのspawn
                    (   map::spawn_sprite,
                        player::spawn_sprite,
                        chasers::spawn_sprite,
                    ),
                )
                .chain(), //実行順の固定

                //TextUIの可視化
                (   effect::init_count::<stage_start::CountDown>, //カウント初期化
                    misc::show_component::<stage_start::Message>,
                )
                .chain(), //実行順の固定
            )
        )
        .add_systems
        (   Update,
            (   //TextUIの演出
                effect::count_down::<stage_start::CountDown>, //カウントダウン
            )
            .run_if( in_state( MyState::StageStart ) )
        )
        .add_systems
        (   OnExit ( MyState::StageStart ),
            (   //TextUIの不可視化
                misc::hide_component::<stage_start::Message>,
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //メインループ
        .add_systems
        (   Update,
            (   //ループ脱出条件
                detection::scoring_and_stage_clear, //スコアリング＆クリア判定
                change_state_to::<StageClear>.run_if( on_event::<EventClear>() ),

                detection::collisions_and_gameover, //衝突判定
                change_state_to::<GameOver>.run_if( on_event::<EventOver>() ),

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
            (   //TextUIの可視化
                effect::init_count::<stage_clear::CountDown>, //カウント初期化
                misc::show_component::<stage_clear::Message>,
            )
            .chain(), //実行順の固定
        )
        .add_systems
        (   Update,
            (   //TextUIの演出
                effect::count_down::<stage_clear::CountDown>, //カウントダウン
            )
            .run_if( in_state( MyState::StageClear ) )
        )
        .add_systems
        (   OnExit ( MyState::StageClear ),
            (   //TextUIの不可視化
                misc::hide_component::<stage_clear::Message>,
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ゲームオーバーの処理
        .add_systems
        (   OnEnter ( MyState::GameOver ),
            (   //TextUIの可視化
                effect::init_count::<game_over::CountDown>,//カウント初期化
                misc::show_component::<game_over::Message>,
            )
            .chain() //実行順の固定
        )
        .add_systems
        (   Update,
            (   //TextUIの演出＆入力待ち
                effect::count_down::<game_over::CountDown>, //カウントダウン
                effect::blinking_text::<game_over::TextREPLAY>, //Replay? の明滅
                effect::hit_any_key::<StageStart>, //Hit ANY Key
            )
            .run_if( in_state( MyState::GameOver ) )
        )
        .add_systems
        (   OnExit ( MyState::GameOver ),
            (   //TextUIの不可視化
                misc::hide_component::<game_over::Message>,

                //scoreとstageをゼロクリアする
                initialize_record_except_hi_score
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
            if sprite.index as u32 >= offset + frame { sprite.index = offset as usize }
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