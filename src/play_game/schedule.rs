use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //ResourceとEvent
        .init_resource::<Record>()  //ゲームの成績
        .init_resource::<Map>()     //マップ情報
        .add_event::<EventClear>()  //ステージクリアイベントの伝達
        .add_event::<EventOver>()   //ゲームオーバーイベントの伝達
        .add_event::<EventEatDot>() //スコアリングイベントの伝達（demo用）
        .init_resource::<input::CrossDirection>()      //十字方向の入力状態
        .init_resource::<ui::effect::CountDownTimer>() //カウントダウンタイマー

        //submoduleのplugin
        .add_plugins( ui::header::Schedule ) //ヘッダー(Stage、Score、HiScore)
        .add_plugins( pause::Schedule )      //Pause処理

        //チェイサーの回転アニメーション
        .add_systems( Update, chasers::rotate )

        ////////////////////////////////////////////////////////////////////////
        //ゲーム開始
        .add_systems
        (   OnEnter ( MyState::GameStart ),
            (   //中央に表示するメッセージの作成
                ui::center::spawn_title,
                ui::center::spawn_in_hidden_frame::<ui::center::Start>,
                ui::center::spawn_in_hidden_frame::<ui::center::Clear>,
                ui::center::spawn_in_hidden_frame::<ui::center::Over>,

                misc::change_state::<TitleDemo>, //無条件遷移
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //タイトルを表示する
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   misc::show::<ui::center::Title>, //タイトル表示
            )
        )
        .add_systems
        (   Update,
            (   ui::effect::hit_any_key::<ui::center::Title>,  //Hit ANY Key
                ui::effect::blinking_text::<ui::center::Demo>, //文字の明滅
            )
            .run_if( in_state( MyState::TitleDemo ) )
        )
        .add_systems
        (   OnExit ( MyState::TitleDemo ),
            (   misc::hide::<ui::center::Title>,   //タイトル非表示
                judge::init_record_except_hiscore, //scoreとstageをゼロクリア
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ステージ初期化
        .add_systems
        (   OnEnter ( MyState::StageStart ),
            (   //マップ作成とスプライト表示
                (   map::make_new_data,        //マップデータの作成
                    (   map::spawn_sprite,     //マップをspawnする
                        player::spawn_sprite,  //プレイヤーをspawnする
                        chasers::spawn_sprite, //チェイサーをspawnする
                    ),
                ).chain(),

                //ステージ開始メッセージの表示
                (   ui::effect::init_countdown::<ui::center::Start>, //カウントダウン初期化
                    misc::show::<ui::center::Start>, //メッセージ表示
                ).chain(),
            )
        )
        .add_systems
        (   Update,
            (   ui::effect::counting_down::<ui::center::Start>, //カウントダウン
            )
            .run_if( in_state( MyState::StageStart ) )
        )
        .add_systems
        (   OnExit ( MyState::StageStart ),
            (   misc::hide::<ui::center::Start>, //メッセージ非表示
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //メインループ
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

        ////////////////////////////////////////////////////////////////////////
        //ステージクリアの処理
        .add_systems
        (   OnEnter ( MyState::StageClear ),
            (   ui::effect::init_countdown::<ui::center::Clear>, //カウントダウン初期化
                misc::show::<ui::center::Clear>, //メッセージ表示
            )
            .chain()
        )
        .add_systems
        (   Update,
            (   ui::effect::counting_down::<ui::center::Clear>, //カウントダウン
            )
            .run_if( in_state( MyState::StageClear ) )
        )
        .add_systems
        (   OnExit ( MyState::StageClear ),
            (   misc::hide::<ui::center::Clear>, //メッセージ非表示
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //ゲームオーバーの処理
        .add_systems
        (   OnEnter ( MyState::GameOver ),
            (   ui::effect::init_countdown::<ui::center::Over>, //カウントダウン初期化
                misc::show::<ui::center::Over>, //メッセージ表示
            )
            .chain()
        )
        .add_systems
        (   Update,
            (   ui::effect::counting_down::<ui::center::Over>, //カウントダウン後Titleへ
                ui::effect::hit_any_key::<ui::center::Over>,   //Hit ANY keyでReplay
            )
            .run_if( in_state( MyState::GameOver ) )
        )
        .add_systems
        (   OnExit ( MyState::GameOver ),
            (   misc::hide::<ui::center::Over>,    //メッセージ非表示
                judge::init_record_except_hiscore, //scoreとstageをゼロクリア
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.