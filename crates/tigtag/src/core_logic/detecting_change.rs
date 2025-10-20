use super::*;

////////////////////////////////////////////////////////////////////////////////

// ScoreとStageの初期化
pub fn initialize_score_stage(option_record: Option<ResMut<Record>>) -> Result
{
    let mut record = option_record.ok_or("Resource not found.")?;

    // scoreとstageをゼロクリア（hi_scoreは対象外）
    *record.stage_mut() = 0;
    *record.score_mut() = 0;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// スコアリングとステージクリアの判定
#[allow(clippy::too_many_arguments)]
pub fn scoring_and_stage_clear(
    query_player: Query<&player::Player>,
    option_map: Option<ResMut<map::Map>>,
    option_record: Option<ResMut<Record>>,
    option_state: Option<Res<State<MyState>>>,
    mut message_clear: MessageWriter<DotsAllEaten>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut message_eatdot: MessageWriter<DotEaten>, // demo用Message
) -> Result
{
    // 準備
    let player = query_player.single()?;
    let mut map = option_map.ok_or("Resource not found.")?;
    let mut record = option_record.ok_or("Resource not found.")?;
    let state = option_state.ok_or("Resource not found.")?;

    // プレイヤーの位置にドットがあるなら
    if let Some(dot) = map.option_entity(player.cell)
    {
        // ドットの削除とスコア更新
        cmds.entity(dot).despawn();
        *map.option_entity_mut(player.cell) = None;
        map.remaining_dots -= 1;
        message_eatdot.write(DotEaten(player.cell)); // DotEatenのフィールドはtigtag3d用
        *record.score_mut() += 1;

        // 1度beepを鳴らす(自動despawn処理付き)
        let sound_beep = AudioPlayer::new(asset_svr.load(ASSETS_SOUND_BEEP));
        let setting = PlaybackSettings::DESPAWN.with_volume(VOLUME_SOUND_BEEP);
        cmds.spawn((sound_beep, setting));

        // ハイスコアの更新（Demoでプレイヤーの記録が壊されないように）
        if record.score() > record.hi_score() && !state.get().is_demoplay()
        {
            *record.hi_score_mut() = record.score();
        }

        // 全ドットを拾ったらEventでステージクリアを通知
        if map.remaining_dots <= 0
        {
            message_clear.write(DotsAllEaten);
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 衝突判定
pub fn collisions_and_gameover(
    query_player: Query<&player::Player>,
    query_chaser: Query<&chaser::Chaser>,
    mut message_game_over: MessageWriter<PlayerCaught>,
) -> Result
{
    // 準備
    let player = query_player.single()?;

    // 衝突判定が真なら
    if is_collision(player, query_chaser)
    {
        // 後続の処理にゲームオーバーを伝える
        message_game_over.write(PlayerCaught);
    }

    Ok(())
}

// 衝突判定関数
fn is_collision(
    player: &player::Player,
    query_chaser: Query<&chaser::Chaser>,
) -> bool
{
    let mut is_collision = false;

    // プレイヤーの移動区間を a1➜a2 とする
    let mut a1 = player.px_start;
    let mut a2 = player.px_end;
    if a1.x > a2.x
    {
        // a1.x < a2.xにする
        (a1.x, a2.x) = (a2.x, a1.x)
    }
    if a1.y > a2.y
    {
        // a1.y < a2.yにする
        (a1.y, a2.y) = (a2.y, a1.y)
    }

    // 各チェイサー毎に
    for chaser in query_chaser.iter()
    {
        // 同じセルにいる場合 衝突
        if player.px_end == chaser.px_end
        {
            is_collision = true;
            break;
        }

        // チェイサーの移動区間を b1➜b2 とする
        let mut b1 = chaser.px_start;
        let mut b2 = chaser.px_end;
        if b1.x > b2.x
        {
            // b1.x < b2.xにする
            (b1.x, b2.x) = (b2.x, b1.x)
        }
        if b1.y > b2.y
        {
            // b1.y < b2.yにする
            (b1.y, b2.y) = (b2.y, b1.y)
        }

        // 移動した微小区間の重なりを判定する
        if player.px_end.y == chaser.px_end.y
        {
            // Y軸が一致する場合
            is_collision = is_overlap(
                a1.x,
                a2.x,
                b1.x,
                b2.x,
                player.direction,
                chaser.direction,
            );
        }
        else if player.px_end.x == chaser.px_end.x
        {
            // X軸が一致する場合
            is_collision = is_overlap(
                a1.y,
                a2.y,
                b1.y,
                b2.y,
                player.direction,
                chaser.direction,
            );
        }
        if is_collision
        {
            break;
        }
    }

    // 衝突判定の結果を返す
    is_collision
}

// 線分上の移動した微小区間の重なりで衝突を判定
fn is_overlap(a1: f32, a2: f32, b1: f32, b2: f32, a_side: News, b_side: News)
    -> bool
{
    // a1➜a2 と b1➜b2 が重ならないなら衝突しない(この条件が一番多いので先にはじく)
    if a2 < b1 || b2 < a1
    {
        return false;
    }

    // 1つ目、2つ目の条件: a1➜a2 と b1➜b2 が包含関係なら衝突する
    // 3つ目の条件: 部分的に重なる場合 移動が対向なら衝突する(同一方向なら衝突しない)
    if a1 < b1 && b2 < a2 || b1 < a1 && a2 < b2 || a_side != b_side
    {
        return true;
    }

    false
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
