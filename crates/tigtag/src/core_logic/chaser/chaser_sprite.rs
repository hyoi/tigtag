use super::*;

////////////////////////////////////////////////////////////////////////////////

// チェイサーをspawnする
pub fn spawn_sprite(
    query_entity: Query<Entity, With<Chaser>>,
    option_record: Option<Res<Record>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut texture_atlases_layout: ResMut<Assets<TextureAtlasLayout>>,
) -> Result
{
    // 準備
    let record = option_record.ok_or("Resource not found.")?;

    // 既存スプライトがあれば削除する
    query_entity.iter().for_each(|id| cmds.entity(id).despawn());

    // チェイサーを初期位置に配置する
    (0..)
        .zip(CHASER_START_POSITION)
        .for_each(|(loop_index, &chaser_cell)| {
            // 初期位置
            let translation = chaser_cell
                .to_screen_pixels_map_adjusted()
                .extend(DEPTH_SPRITE_CHASER);
            let transform = Transform::from_translation(translation);

            // 四隅のチェイサーを4ステージで１周ローテーションさせるためにindex（0,1,2,3）を利用
            let index = ((loop_index + record.stage() - 1) % 4) as usize;
            let (asset_file, color, option_fn_autochase) =
                CHASERS_SPRITE_INFO[index];

            // チェイサーのデータを初期化する
            let chaser = Chaser {
                cell: chaser_cell,
                next_cell: chaser_cell,
                px_start: translation,
                px_end: translation,
                color,
                option_fn_autochase,
                ..default()
            };

            if SPRITE_OFF()
            {
                // 正方形のメッシュを作る
                cmds.spawn((
                    Sprite {
                        // imageを指定しないと正方形のメッシュを表示するのを利用する
                        color: Color::Srgba(color),
                        custom_size: Some(CELL_CUSTOM_SIZE * CHASER_SPRITE_SCALING),
                        ..default()
                    },
                    transform,
                    chaser,
                ));
            }
            else
            {
                // アニメーションするスプライトをspawnする
                let layout =
                    texture_atlases_layout.add(MySpriteSheetLayout::default().0);
                let index = chaser.sprite_sheet_offset(chaser.direction()) as usize;

                let mut sprite = Sprite::from_atlas_image(
                    asset_svr.load(asset_file),
                    TextureAtlas { layout, index },
                );
                sprite.custom_size = Some(CELL_CUSTOM_SIZE);

                cmds.spawn((sprite, transform, chaser));
            }
        });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 進む方向を決める(赤)
// pub const SELECT_PATH_RED: Option<FnAutoChase> = None;
pub const SELECT_PATH_RED: Option<FnAutoChase> = Some(select_path_red);
fn select_path_red(
    chaser: &mut Chaser,
    player: &player::Player,
    sides: &[News],
) -> News
{
    let priority = [
        (News::West, player.next_cell.x < chaser.cell.x),
        (News::East, player.next_cell.x > chaser.cell.x),
        (News::North, player.next_cell.y < chaser.cell.y),
        (News::South, player.next_cell.y > chaser.cell.y),
    ];
    for (direction, condition) in priority
    {
        if condition && sides.contains(&direction)
        {
            return direction;
        }
    }

    // sidesが2要素以上であることを呼び出し側関数で確認している為、心置きなく.unwrap()できる
    *sides.choose(&mut rand::rng()).unwrap()
}

// 進む方向を決める(青)
// pub const SELECT_PATH_BLUE: Option<FnAutoChase> = None;
pub const SELECT_PATH_BLUE: Option<FnAutoChase> = Some(select_path_blue);
fn select_path_blue(
    chaser: &mut Chaser,
    player: &player::Player,
    sides: &[News],
) -> News
{
    let priority = [
        (News::South, player.next_cell.y > chaser.cell.y),
        (News::West, player.next_cell.x < chaser.cell.x),
        (News::East, player.next_cell.x > chaser.cell.x),
        (News::North, player.next_cell.y < chaser.cell.y),
    ];
    for (direction, condition) in priority
    {
        if condition && sides.contains(&direction)
        {
            return direction;
        }
    }

    // sidesが2要素以上であることを呼び出し側関数で確認している為、心置きなく.unwrap()できる
    *sides.choose(&mut rand::rng()).unwrap()
}

// 進む方向を決める(緑)
// pub const SELECT_PATH_GREEN: Option<FnAutoChase> = None;
pub const SELECT_PATH_GREEN: Option<FnAutoChase> = Some(select_path_green);
fn select_path_green(
    chaser: &mut Chaser,
    player: &player::Player,
    sides: &[News],
) -> News
{
    let priority = [
        (News::North, player.next_cell.y < chaser.cell.y),
        (News::South, player.next_cell.y > chaser.cell.y),
        (News::West, player.next_cell.x < chaser.cell.x),
        (News::East, player.next_cell.x > chaser.cell.x),
    ];
    for (direction, condition) in priority
    {
        if condition && sides.contains(&direction)
        {
            return direction;
        }
    }

    // sidesが2要素以上であることを呼び出し側関数で確認している為、心置きなく.unwrap()できる
    *sides.choose(&mut rand::rng()).unwrap()
}

// 進む方向を決める(ピンク)
// pub const SELECT_PATH_PINK: Option<FnAutoChase> = None;
pub const SELECT_PATH_PINK: Option<FnAutoChase> = Some(select_path_pink);
fn select_path_pink(
    chaser: &mut Chaser,
    player: &player::Player,
    sides: &[News],
) -> News
{
    let priority = [
        (News::East, player.next_cell.x > chaser.cell.x),
        (News::North, player.next_cell.y < chaser.cell.y),
        (News::South, player.next_cell.y > chaser.cell.y),
        (News::West, player.next_cell.x < chaser.cell.x),
    ];
    for (direction, condition) in priority
    {
        if condition && sides.contains(&direction)
        {
            return direction;
        }
    }

    // sidesが2要素以上であることを呼び出し側関数で確認している為、心置きなく.unwrap()できる
    *sides.choose(&mut rand::rng()).unwrap()
}

////////////////////////////////////////////////////////////////////////////////

// チェイサー（正方形）を回転させる
pub fn rotate_chaser_shape(
    mut query_chaser: Query<&mut Transform, With<Chaser>>,
    time: Res<Time>,
)
{
    let time_delta = time.delta().as_secs_f32();
    let radian = TAU * time_delta;
    let quat = Quat::from_rotation_z(radian);

    // 回転させる
    query_chaser
        .iter_mut()
        .for_each(|mut transform| transform.rotate(quat));
}

////////////////////////////////////////////////////////////////////////////////

// チェイサーを移動させる
pub fn move_sprite(
    mut query_chaser: Query<(&mut Transform, &mut Sprite, &mut Chaser)>,
    query_player: Query<&player::Player>,
    option_map: Option<Res<map::Map>>,
    time: Res<Time>,
    mut message_writer: MessageWriter<ChaserPositionAdjusted>,
) -> Result
{
    // 準備
    let player = query_player.single()?;
    let map = option_map.ok_or("Resource not found.")?;
    let time_delta = time.delta();
    let mut chaser_timer_finished = Vec::new();

    // 複数のチェイサーをループで処理する
    for (mut transform, mut sprite, mut chaser) in query_chaser.iter_mut()
    {
        // スピードアップ係数をかけて、経過時間を割り増しする
        let time_delta = time_delta.mul_f32(chaser.speedup); //speedup > 1.0

        // 移動タイマーがfinishしたなら
        if chaser.timer.tick(time_delta).is_finished()
        {
            // 移動タイマーがfinishしたチェイサーの色を保存する
            chaser_timer_finished.push(chaser.color);

            // セルの間を移動中のスプライトが半端な位置にいるなら
            if chaser.px_start != chaser.px_end
            {
                // 移動先のセルにフィットさせる
                chaser.px_start = chaser.px_end;
                chaser.px_end = chaser
                    .next_cell
                    .to_screen_pixels_map_adjusted()
                    .extend(DEPTH_SPRITE_CHASER);
                transform.translation = chaser.px_end;
            }

            // 後退を除く三方の道を取得する
            let mut sides = map.get_side_spaces_list(chaser.next_cell); //脇道のリスト
            sides.retain(|side| chaser.next_cell + *side != chaser.cell); //戻り路を削除

            // チェイサーが次に進む方向を決める
            chaser.is_stop = false; //停止フラグを倒す(停止はスタート時のみ)
            let count = sides.len();
            let new_side = match count
            {
                // 一本道（直線やコーナーを道なりに進む）
                1 => sides[0],
                // 三叉路か十字路なので進行方向を考える
                2.. =>
                {
                    // 自動追尾の関数がセットされているなら
                    if let Some(autochase) = chaser.option_fn_autochase
                    {
                        // 追尾関数
                        autochase(&mut chaser, player, &sides)
                    }
                    else
                    {
                        // 進行方向をランダムに決める
                        sides[rand::rng().random_range(0..count)]
                    }
                }
                // ここには来ない（このゲームにはマップ上行き止まりはないので）
                _ => chaser.direction.back(), // 逆走
            };

            // チェイサーの向きが変わったなら
            if new_side != chaser.direction
            {
                // スプライトシートのアニメなら
                if !SPRITE_OFF()
                    && let Some(sprite_sheet) = &mut sprite.texture_atlas
                {
                    // スプライトシートのindexを変更してスプライトの向きを変更
                    let old_news = chaser.direction;
                    let old_offset = chaser.sprite_sheet_offset(old_news) as usize;
                    let new_offset = chaser.sprite_sheet_offset(new_side) as usize;
                    let index = &mut sprite_sheet.index;
                    *index = *index + new_offset - old_offset;
                }

                // チェイサーの向き情報を更新
                chaser.direction = new_side;
            }

            // 位置を更新
            chaser.cell = chaser.next_cell; //現在の位置を更新
            chaser.next_cell += new_side; //次の位置を更新

            // 移動タイマーをリセットする
            chaser.timer.reset();
        }
        else if !chaser.is_stop
        {
            // 移動中の中割表示の座標更新
            let delta = chaser.base_speed * time_delta.as_secs_f32();
            match chaser.direction
            {
                News::North => transform.translation.y += delta,
                News::South => transform.translation.y -= delta,
                News::East => transform.translation.x += delta,
                News::West => transform.translation.x -= delta,
            }

            // 当たり判定用の微小区間の座標更新
            chaser.px_start = chaser.px_end;
            chaser.px_end = transform.translation;
        }
    }

    // 移動タイマーがfinishしたチェイサーがいるなら
    if !chaser_timer_finished.is_empty()
    {
        // メッセージでパッシングする
        message_writer.write(ChaserPositionAdjusted(chaser_timer_finished));
    }

    // チェイサーは重なるとスピードアップする
    let mut colors = Vec::with_capacity(query_chaser.iter().len());
    for (_, _, mut chaser) in query_chaser.iter_mut()
    {
        colors.push((chaser.color, chaser.next_cell));
        chaser.speedup = 1.0; // スピードアップ係数初期化
    }
    for (color, cell) in colors
    {
        for (_, _, mut chaser) in query_chaser.iter_mut()
        {
            // セルが一致し、自分以外の色なら
            if cell == chaser.next_cell && color != chaser.color
            {
                chaser.speedup += CHASER_ACCEL; // スピードアップ係数を割り増し
            }
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
