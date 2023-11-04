use super::*;

////////////////////////////////////////////////////////////////////////////////

//ScoreとStageの初期化
pub fn init_record_except_hiscore
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

//スコア処理とステージクリア判定
#[allow(clippy::too_many_arguments)]
pub fn scoring_and_stageclear
(   qry_player: Query<&Player>,
    opt_map: Option<ResMut<Map>>,
    opt_record: Option<ResMut<Record>>,
    state: Res<State<MyState>>,
    mut next_state: ResMut<NextState<MyState>>,
    mut evt_clear: EventWriter<EventClear>,
    mut evt_eatdot: EventWriter<EventEatDot>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };
    let Some ( mut map ) = opt_map else { return };
    let Some ( mut record ) = opt_record else { return };

    //プレイヤーの位置にドットがないなら
    let Some ( dot ) = map.opt_entity( player.grid ) else { return };

    //ドットの削除
    cmds.entity( dot ).despawn();
    *map.opt_entity_mut( player.grid ) = None;
    evt_eatdot.send( EventEatDot ); //後続の処理にドット削除を伝達する(demo用)

    //スコア更新
    *record.score_mut() += 1;
    map.remaining_dots -= 1;

    //1度beepを鳴らす(despawn処理付き)
    let volume = Volume::Relative ( VolumeLevel::new( VOLUME_SOUND_BEEP ) );
    let sound_beep = AudioBundle
    {   source: asset_svr.load( ASSETS_SOUND_BEEP ),
        settings: PlaybackSettings::DESPAWN.with_volume( volume ),
    };
    cmds.spawn( sound_beep );

    //ハイスコアの更新
    if ! state.get().is_demoplay() && record.score() > record.hi_score()
    {   *record.hi_score_mut() = record.score();
    }

    //全ドットを拾ったらステージクリア
    if map.remaining_dots <= 0
    {   next_state.set
        (   match state.get()
            {   MyState::MainLoop  => MyState::StageClear,
                MyState::TitleDemo => MyState::DemoLoop,
                _ => unreachable!( "Bad state: {:?}", state.get() ),
            }
        );
        *record.is_clear_mut() = true;
        evt_clear.send( EventClear ); //後続の処理にステージクリアを伝達する
    }
}

////////////////////////////////////////////////////////////////////////////////

//衝突判定
#[allow(clippy::too_many_arguments)]
pub fn detect_collisions
(   qry_player: Query<&Player>,
    qry_chaser: Query<&Chaser>,
    opt_record: Option<Res<Record>>,
    state: Res<State<MyState>>,
    mut next_state: ResMut<NextState<MyState>>,
    mut evt_clear: EventReader<EventClear>,
    mut evt_over: EventWriter<EventOver>,
)
{   let Some ( _record ) = opt_record else { return };

    //直前の判定でクリアしていたら衝突判定しない
    if evt_clear.iter().next().is_some() { return }
    
    //衝突判定が真なら
    if is_collision( qry_player, qry_chaser )
    {   //衝突処理
        next_state.set
        (   match state.get()
            {   MyState::MainLoop  => MyState::GameOver,
                MyState::TitleDemo => MyState::DemoLoop,
                _ => unreachable!( "Bad state: {:?}", state.get() ),
            }
        );
        evt_over.send( EventOver ); //後続の処理にゲームオーバーを伝える
    }
}

//衝突判定関数
fn is_collision
(   qry_player: Query<&Player>,
    qry_chaser: Query<&Chaser>
) -> bool
{   let mut is_collision = false;
    let Ok ( player ) = qry_player.get_single() else { return is_collision };

    //自機の移動区間を a1➜a2 とする
    let mut a1 = player.px_start;
    let mut a2 = player.px_end;
    if a1.x > a2.x { std::mem::swap( &mut a1.x, &mut a2.x ) } //a1.x < a2.xにする
    if a1.y > a2.y { std::mem::swap( &mut a1.y, &mut a2.y ) } //a1.y < a2.yにする

    //各追手ごとの処理
    for chaser in qry_chaser.iter()
    {   //同じグリッドにいる場合
        if player.px_end == chaser.px_end
        {   is_collision = true;
            break;
        }

        //追手の移動区間を b1➜b2 とする
        let mut b1 = chaser.px_start;
        let mut b2 = chaser.px_end;
        if b1.x > b2.x { std::mem::swap( &mut b1.x, &mut b2.x ) } //b1.x < b2.xにする
        if b1.y > b2.y { std::mem::swap( &mut b1.y, &mut b2.y ) } //b1.y < b2.yにする

        //移動した微小区間の重なりを判定する
        if player.px_end.y == chaser.px_end.y
        {   //Y軸が一致する場合
            is_collision = is_overlap( a1.x, a2.x, b1.x, b2.x, player.side, chaser.side );
        }
        else if player.px_end.x == chaser.px_end.x
        {   //X軸が一致する場合
            is_collision = is_overlap( a1.y, a2.y, b1.y, b2.y, player.side, chaser.side );
        }
        if is_collision { break }
    }

    //衝突判定の結果を返す
    is_collision
}

//線分の重なりで衝突を判定
fn is_overlap
(   a1: f32, a2: f32,
    b1: f32, b2: f32,
    a_side: News, b_side: News,
) -> bool
{   //a1➜a2 と b1➜b2 が重ならないなら衝突しない(この条件が一番多いので先にはじく)
    if a2 < b1 || b2 < a1 { return false }

    //1つ目、2つ目の条件: a1➜a2 と b1➜b2 が包含関係なら衝突する
    //3つ目の条件: 部分的に重なる場合 移動が対向なら衝突する(同一方向なら衝突しない)
    if a1 < b1 && b2 < a2 || b1 < a1 && a2 < b2 || a_side != b_side { return true }

    false
}

////////////////////////////////////////////////////////////////////////////////

//End of code.