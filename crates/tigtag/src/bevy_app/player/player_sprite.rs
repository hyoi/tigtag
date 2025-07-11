use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーをspawnする
pub fn spawn_sprite(
    opt_map: Option<ResMut<Map>>,
    qry_entity: Query<Entity, With<Player>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut texture_atlases_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result
{
    // 準備
    let mut map = opt_map.ok_or("ResMut<Map> not found.")?; // 必須のResource
    qry_entity.iter().for_each(|id| cmds.entity(id).despawn()); // 既存スプライトがあれば削除する

    // 乱数で初期位置を決める(マップ中央付近の通路)
    let half_w = MAP_GRIDS_WIDTH / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };
    let x1 = short_side - 1;
    let y1 = short_side - 1;
    let x2 = MAP_GRIDS_WIDTH - short_side;
    let y2 = MAP_GRIDS_HEIGHT - short_side;

    let player_grid = loop
    {
        let x = map.rng.random_range(x1..=x2);
        let y = map.rng.random_range(y1..=y2);
        let grid = IVec2::new(x, y);

        if map.is_space(grid)
        {
            break grid;
        }
    };
    let vec2 = player_grid.to_vec2_on_game_map();
    let transform = Transform::from_translation(vec2.extend(DEPTH_SPRITE_PLAYER));

    // プレイヤーデータを初期化する
    let player = Player {
        grid: player_grid,
        next_grid: player_grid,
        px_start: vec2,
        px_end: vec2,
        // opt_fn_autodrive: Some ( demo::auto_drive::choice_way ), //default()に任せるとNone
        ..default()
    };

    if SPRITE_OFF()
    {
        // 三角形のメッシュを作る
        let shape = RegularPolygon::new(PLAYER_SPRITE_RADIUS, 3).mesh();
        let quat = Quat::from_rotation_z(PI); // News::South
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
        let layout = texture_atlases_layout.add(PLAYER_SPRITESHEET_LAYOUT.clone());
        let index = player.sprite_sheet_offset(player.direction()) as usize;

        let mut sprite = Sprite::from_atlas_image(
            asset_svr.load(ASSETS_SPRITE_SHEET_PLAYER),
            TextureAtlas { layout, index },
        );
        sprite.custom_size = Some(GRID_CUSTOM_SIZE);

        cmds.spawn((sprite, transform, player));
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// プレイヤーを移動させる
// #[allow(clippy::too_many_arguments)]
pub fn move_sprite(
    mut qry_player: Query<(&mut Transform, &mut Player)>,
    mut qry_sprite: Query<&mut Sprite, With<Player>>,
    opt_map: Option<Res<Map>>,
    opt_input: Option<ResMut<InputDirection>>,
    opt_demo: Option<Res<DemoMapParams>>,
    qry_chasers: Query<&Chaser>,
    state: ResMut<State<MyState>>,
    // mut evt_timer: EventWriter<EventTimerPlayer>,
    time: Res<Time>,
) -> Result
{
    //準備
    let (mut transform, mut player) = qry_player.single_mut()?;
    let map = opt_map.ok_or("Res<Map> not found.")?;
    let mut input = opt_input.ok_or("ResMut<InputDirection> not found.")?;

    // 前回からの経過時間 × スピードアップ係数
    let time_delta = time.delta().mul_f32(player.speedup);

    // 移動タイマーがfinishしたなら
    if player.timer.tick(time_delta).finished()
    {
        // 後続の処理にtimer finishedを伝達する
        // evt_timer.write(EventTimerPlayer);

        // スプライトがグリッド間の中途に位置したなら
        if player.px_start != player.px_end
        {
            // スプライトをグリッドにフィットさせる
            player.px_start = player.px_end;
            player.px_end = player.next_grid.to_vec2_on_game_map();
            transform.translation = player.px_end.extend(DEPTH_SPRITE_PLAYER);
        }

        // プレイヤーが次に進む方向を決める
        let mut new_side = player.direction;
        player.is_stop = true; // 停止フラグを立てておく

        if !state.get().is_demoplay()
        {
            // 入力に対応する
            for side in input.0.iter()
            // input.0は優先順に並んでいる前提
            {
                // 壁でない場合
                if map.is_space(player.next_grid + side)
                {
                    new_side = *side;
                    player.is_stop = false;
                    break;
                }

                // ループの先頭要素では、向きを必ず変える
                if *side == (input.0)[0]
                // データの性質上 値が重複する要素はない
                {
                    new_side = *side;
                }
            }
        }
        else
        {
            // demoの場合
            autodrive(&mut player, map, opt_demo, qry_chasers);
            //     // demoの場合 入力相当のデータをアルゴリズムで作る
            //     player.is_stop = false; // demoでは自機は停止しない

            //     let mut sides = map.get_side_spaces_list(player.next_grid); // 脇道のリスト
            //     sides.retain(|side| player.next_grid + side != player.grid); // 戻り路を取り除く

            //     new_side = match sides.len().cmp(&1)
            //     {
            //         Ordering::Equal =>
            //         // 一本道 ⇒ 道なりに進む
            //             sides[0],
            //         Ordering::Greater =>
            //         // 三叉路または十字路
            //             if let (Some(autodrive), Some(demo)) =
            //                 (player.fn_autodrive, opt_demo)
            //             {
            //                 // 外部関数で進行方向を決める
            //                 autodrive(&player, qry_chasers, map, demo, &sides)
            //             }
            //             else
            //             {
            //                 // 外部関数を使えないなら乱数で決める
            //                 let mut rng = rand::rng();
            //                 sides[rng.random_range(0..sides.len())]
            //             },
            //         Ordering::Less =>
            //         // 行き止まり ⇒ 逆走 (このゲームに行き止まりはないけど)
            //             match player.direction
            //             {
            //                 News::North => News::South,
            //                 News::South => News::North,
            //                 News::East => News::West,
            //                 News::West => News::East,
            //             },
            //     };
        }

        // プレイヤーの向きが変わったなら
        if new_side != player.direction
        {
            if let Ok(mut sprite) = qry_sprite.single_mut()
                && let Some(sprite_sheet) = &mut sprite.texture_atlas.as_mut()
                && !SPRITE_OFF()
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
                rotate_player_triangle(&player, &mut transform, new_side);
            }

            //プレイヤーの向きの情報の更新
            player.direction = new_side;
        }

        // 位置を更新
        player.grid = player.next_grid; //現在の位置を更新
        if !player.is_stop
        {
            player.next_grid += new_side; //次の位置を更新
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
        player.px_end = transform.translation.truncate();
    }

    //入力のクリア
    *input = InputDirection::default();

    Ok(())
}

// プレイヤー（三角形）を回転させる
fn rotate_player_triangle(
    player: &Player,
    transform: &mut Mut<Transform>,
    input: News,
)
{
    //入力と現在の向きから回転角を決める
    let angle: f32 = match player.direction
    {
        News::North => match input
        {
            News::West => PI / 2.0,
            News::East => PI / -2.0,
            _ => PI,
        },
        News::South => match input
        {
            News::East => PI / 2.0,
            News::West => PI / -2.0,
            _ => PI,
        },
        News::East => match input
        {
            News::North => PI / 2.0,
            News::South => PI / -2.0,
            _ => PI,
        },
        News::West => match input
        {
            News::South => PI / 2.0,
            News::North => PI / -2.0,
            _ => PI,
        },
    };

    //回転させる
    let quat = Quat::from_rotation_z(angle);
    transform.rotate(quat);
}

////////////////////////////////////////////////////////////////////////////////

// demo用に入力相当のデータを作る
fn autodrive(
    player: &mut Player,
    map: Res<Map>,
    opt_demo: Option<Res<DemoMapParams>>,
    qry_chasers: Query<&Chaser>,
) -> News
{
    // demoでは停止しない
    player.is_stop = false;

    //進入できる脇道のリストを作る
    let mut sides = map.get_side_spaces_list(player.next_grid);
    sides.retain(|side| player.next_grid + side != player.grid); // 戻り路は除く
    let count = sides.len();

    //進入できる脇道の数で処理を分ける
    if count == 1
    {
        sides[0] // 一本道 ⇒ 道なりに進む
    }
    else if count > 1
    {
        // 三叉路または十字路 ⇒ 外部関数で進行方向を決める
        if let (Some(autodrive), Some(demo)) = (player.fn_autodrive, opt_demo)
        {
            autodrive(player, qry_chasers, map, demo, &sides)
        }
        else
        {
            // 外部関数を使えないなら乱数で決める
            let mut rng = rand::rng();
            sides[rng.random_range(0..sides.len())]
        }
    }
    else
    {
        // 行き止まり ⇒ 逆走 (このゲームに行き止まりはないが)
        match player.direction
        {
            News::North => News::South,
            News::South => News::North,
            News::East => News::West,
            News::West => News::East,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
