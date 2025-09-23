use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーをspawnする
pub fn spawn_sprite(
    qry_entity: Query<Entity, With<Player>>,
    opt_map: Option<ResMut<map::Map>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut texture_atlases_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result
{
    // 準備
    let mut map = opt_map.ok_or("Resource not found.")?;

    // 既存スプライトがあれば削除する
    qry_entity.iter().for_each(|id| cmds.entity(id).despawn());

    // 乱数で初期位置を決める(マップ中央付近の通路)
    let half_w = map::MAP_WIDTH_IN_CELLS / 2;
    let half_h = map::MAP_HEIGHT_IN_CELLS / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };
    let x1 = short_side - 1;
    let y1 = short_side - 1;
    let x2 = map::MAP_WIDTH_IN_CELLS - short_side;
    let y2 = map::MAP_HEIGHT_IN_CELLS - short_side;

    let player_cell = loop
    {
        let x = map.rng.random_range(x1..=x2);
        let y = map.rng.random_range(y1..=y2);
        let cell = IVec2::new(x, y);

        if map.is_space(cell)
        {
            break cell;
        }
    };
    let translation = player_cell
        .to_screen_pixels_map_adjusted()
        .extend(DEPTH_SPRITE_PLAYER);
    let transform = Transform::from_translation(translation);

    // プレイヤーのデータを初期化する
    let player = Player {
        cell: player_cell,
        next_cell: player_cell,
        px_start: translation,
        px_end: translation,
        ..default()
    };

    if SPRITE_OFF()
    {
        // 三角形のメッシュを作る
        let shape = RegularPolygon::new(PLAYER_SPRITE_RADIUS, 3).mesh();
        let quat = Quat::from_rotation_z(PI); // 回転 News::South
        cmds.spawn((
            Mesh2d(meshes.add(shape)),
            MeshMaterial2d(materials.add(PLAYER_SPRITE_COLOR)),
            transform.with_rotation(quat),
            player,
        ));
    }
    else
    {
        // アニメーションするスプライトをspawnする
        let layout = texture_atlases_layout.add(MySpriteSheetLayout::default().0);
        let index = player.sprite_sheet_offset(player.direction()) as usize;

        let mut sprite = Sprite::from_atlas_image(
            asset_svr.load(ASSETS_SPRITESHEET_PLAYER),
            TextureAtlas { layout, index },
        );
        sprite.custom_size = Some(CELL_CUSTOM_SIZE);

        cmds.spawn((sprite, transform, player));
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// プレイヤーを移動させる
pub fn move_sprite(
    mut query_player: Query<(&mut Player, &mut Transform)>,
    mut query_sprite: Query<&mut Sprite, With<Player>>, //SPRITE_OFFだとspawnされないので空
    option_map: Option<Res<map::Map>>,
    option_state: Option<Res<State<MyState>>>,
    mut event_reader: EventReader<EventPlayerInputNews>,
    time: Res<Time>,
    query_chaser: Query<&chaser::Chaser>,
    option_demo_params: Option<Res<DemoMapParams>>,
    option_autodrive_fn: Option<Res<DemoAutoDriveFn>>,
) -> Result
{
    //準備
    let (mut player, mut transform) = query_player.single_mut()?;
    let map = option_map.ok_or("Resource not found.")?;
    let state = option_state.ok_or("Resource not found.")?;

    // 入力（NEWS）のイベントをハッシュ集合へ統合する
    let mut input_news = FxHashSet::<News>::default();
    event_reader
        .read()
        .for_each(|event| input_news.extend(&event.0));

    // 前回からの経過時間 × スピードアップ係数（プレイヤーのスピードアップは未実装）
    let time_delta = time.delta().mul_f32(player.speedup); //speedup > 1.0

    // 移動タイマーがfinishしたなら
    if player.timer.tick(time_delta).finished()
    {
        // セルの間を移動中のスプライトが半端な位置にいるなら
        if player.px_start != player.px_end
        {
            // 移動先のセルにフィットさせる
            player.px_start = player.px_end;
            player.px_end = player
                .next_cell
                .to_screen_pixels_map_adjusted()
                .extend(DEPTH_SPRITE_PLAYER);
            transform.translation = player.px_end;
        }

        // プレイヤーが次に進む方向を決める
        let mut new_side = player.direction;
        player.is_stop = true; // 停止フラグを立てておく

        // Demoなら
        if state.get().is_demoplay()
            && let Some(demo_params) = option_demo_params
            && let Some(autodrive) = option_autodrive_fn
        {
            //demoの場合 入力相当のデータを作る
            player.is_stop = false; //demoでは自機は停止しない
            let mut sides = map.get_side_spaces_list(player.next_cell); //脇道のリスト
            sides.retain(|side| player.next_cell + *side != player.cell); //戻り路を取り除く
            let count = sides.len(); // 1～4（通常は1～3。スタート地点が十字路の場合のみ4）

            new_side = match count
            {
                1 => sides[0], //一本道 ⇒ 道なりに進む
                2 | 3 | 4 =>
                    autodrive.0(&player, query_chaser, map, demo_params, &sides), //三叉路または十字路
                _ =>
                {
                    // countが4になってpanicする不具合アリ！
                    unreachable!("Bad count of autodrive paths. count: {count}")
                },
            };
        }
        // Demoではないなら
        else
        {
            // 準備
            let count = input_news.len(); // cout: 0～4
            let input_direction = &mut Vec::<News>::with_capacity(4);

            // 入力が１つなら
            if count == 1
            {
                input_direction.push(*input_news.iter().next().unwrap());
            }
            // 入力が２つ以上なら
            else if count >= 2
            {
                // プレイヤーの正面（前方）、背面（後方）、それ以外（左右）として、
                // 後方➡左右➡前方の順に優先して並べ替えたVecを作る。
                let option_front = input_news.take(&player.direction);
                let option_back = input_news.take(&player.direction.back());
                input_direction.extend(option_back);
                input_direction.extend(input_news.drain()); //(あれば)左右
                input_direction.extend(option_front);
            }

            // 入力を処理する
            for news in &*input_direction
            {
                // 入力された向きが壁でないなら
                if map.is_space(player.next_cell + *news)
                {
                    new_side = *news;
                    player.is_stop = false;
                    break; // directionの要素は優先順に並んでいるので脱出
                }

                // ループの先頭要素なら（入力された向きが壁でも）
                if *news == input_direction[0]
                {
                    //向きだけ変える（アニメ演出用）
                    new_side = *news;
                }
            }
        }

        // プレイヤーの向きが変わったなら
        if new_side != player.direction
        {
            // スプライトシートのアニメなら
            if !SPRITE_OFF()
                && let Ok(mut sprite) = query_sprite.single_mut()
                && let Some(sprite_sheet) = &mut sprite.texture_atlas
            {
                // スプライトシートのindexを変更してスプライトの向きを変更
                let old_news = player.direction;
                let old_offset = player.sprite_sheet_offset(old_news) as usize;
                let new_offset = player.sprite_sheet_offset(new_side) as usize;
                let index = &mut sprite_sheet.index;
                *index = *index + new_offset - old_offset;
            }
            else
            {
                //三角形を回転して向きを変更
                let angle = rotate_player_triangle(&player, new_side);
                let quat = Quat::from_rotation_z(angle);
                transform.rotate(quat);
            }

            //プレイヤーの向き情報を更新
            player.direction = new_side;
        }

        // 位置を更新
        player.cell = player.next_cell; //現在の位置を更新
        if !player.is_stop
        {
            player.next_cell += new_side; //次の位置を更新
        }

        // 移動タイマーをリセットする
        player.timer.reset();
    }
    else if !player.is_stop
    {
        // 移動中の中割表示の座標更新
        let delta = player.base_speed * time_delta.as_secs_f32();
        match player.direction
        {
            News::North => transform.translation.y += delta,
            News::South => transform.translation.y -= delta,
            News::East => transform.translation.x += delta,
            News::West => transform.translation.x -= delta,
        }

        //当たり判定用の微小区間の座標更新
        player.px_start = player.px_end;
        player.px_end = transform.translation;
    }

    //入力のクリア
    input_news.clear();

    Ok(())
}

// プレイヤー（三角形）を回転させる
fn rotate_player_triangle(player: &Player, new_side: News) -> f32
{
    //入力と現在の向きから回転角を決める
    match player.direction
    {
        News::North => match new_side
        {
            News::West => PI / 2.0,
            News::East => PI / -2.0,
            _ => PI,
        },
        News::South => match new_side
        {
            News::East => PI / 2.0,
            News::West => PI / -2.0,
            _ => PI,
        },
        News::East => match new_side
        {
            News::North => PI / 2.0,
            News::South => PI / -2.0,
            _ => PI,
        },
        News::West => match new_side
        {
            News::South => PI / 2.0,
            News::North => PI / -2.0,
            _ => PI,
        },
    }
}

////////////////////////////////////////////////////////////////////////////////

// demo用に入力相当のデータを作る
// fn autodrive(
//     player: &mut Player,
//     map: Res<map::Map>,
//     query_chasers: Query<&chaser::Chaser>,
//     option_demo_params: Option<Res<DemoMapParams>>,
// ) -> News
// {
//     // demoでは停止しない
//     player.is_stop = false;

//     //進入できる脇道のリストを作る
//     let mut sides = map.get_side_spaces_list(player.next_grid);
//     sides.retain(|side| player.next_grid + side != player.grid); // 戻り路は除く
//     let count = sides.len();

//     //進入できる脇道の数で処理を分ける
//     if count == 1
//     {
//         sides[0] // 一本道 ⇒ 道なりに進む
//     }
//     else if count > 1
//     {
//         // 三叉路または十字路 ⇒ 外部関数で進行方向を決める
//         if let (Some(autodrive), Some(demo)) = (player.fn_autodrive, opt_demo)
//         {
//             autodrive(player, qry_chasers, map, demo, &sides)
//         }
//         else
//         {
//             // 外部関数を使えないなら乱数で決める
//             let mut rng = rand::rng();
//             sides[rng.random_range(0..sides.len())]
//         }
//     }
//     else
//     {
//         // 行き止まり ⇒ 逆走 (このゲームに行き止まりはないが)
//         match player.direction
//         {
//             News::North => News::South,
//             News::South => News::North,
//             News::East => News::West,
//             News::West => News::East,
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
