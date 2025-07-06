use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの三角スプライトのComponent
#[derive(Component)]
pub struct PlayerTriangle;

// スプライトシートでアニメーションするためのトレイト実装
impl CharacterAnimation for Player
{
    fn anime_timer_mut(&mut self) -> &mut Timer { &mut self.anime_timer }
    fn sprite_sheet_frame(&self) -> u32 { self.sprite_sheet_frame }
    fn sprite_sheet_offset(&self, news: News) -> u32
    {
        *self.sprite_sheet_indexes.get(&news).unwrap()
    }
    fn direction(&self) -> News { self.direction }
}

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
    let mut map = opt_map.ok_or("Resource <Map> not found.")?; // 必須のResource
    qry_entity.iter().for_each(|id| cmds.entity(id).despawn()); // 既存スプライトがあれば削除する

    // 乱数で初期位置を決める(マップ中央付近の通路)
    let half_w = MAP_GRIDS_WIDTH / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };
    let x1 = short_side - 1;
    let y1 = short_side - 1;
    let x2 = MAP_GRIDS_WIDTH - short_side;
    let y2 = MAP_GRIDS_HEIGHT - short_side;

    let mut player_grid = IVec2::new(0, 0);
    loop
    {
        player_grid.x = map.rng.random_range(x1..=x2);
        player_grid.y = map.rng.random_range(y1..=y2);
        if map.is_space(player_grid)
        {
            break;
        }
    }
    let vec2 = player_grid.to_vec2_on_game_map();
    let translation = vec2.extend(DEPTH_SPRITE_PLAYER);

    // Componentを初期化する
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
        let radius = PIXELS_PER_GRID * PLAYER_SPRITE_SCALING;
        let shape = RegularPolygon::new(radius, 3).mesh();
        let quat = Quat::from_rotation_z(PI); // News::South
        cmds.spawn((
            Mesh2d(meshes.add(shape)),
            MeshMaterial2d(materials.add(PLAYER_SPRITE_COLOR)),
            Transform::from_translation(translation).with_rotation(quat),
            PlayerTriangle, // マーカー
            player,         // データ
        ));
    }
    else
    {
        // アニメーションするスプライトをspawnする
        let custom_size = Some(GRID_CUSTOM_SIZE);
        let layout = texture_atlases_layout.add(TextureAtlasLayout::from_grid(
            SPRITE_SHEET_SIZE_PLAYER,
            SPRITE_SHEET_COLS_PLAYER,
            SPRITE_SHEET_ROWS_PLAYER,
            None,
            None,
        ));
        let index = player.sprite_sheet_offset(player.direction()) as usize;
        let mut sprite = Sprite::from_atlas_image(
            asset_svr.load(ASSETS_SPRITE_SHEET_PLAYER),
            TextureAtlas { layout, index },
        );
        sprite.custom_size = custom_size;
        cmds.spawn((
            sprite,
            Transform::from_translation(translation),
            player, // データ
        ));
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// プレイヤーを移動させる
// #[allow(clippy::too_many_arguments)]
// #[allow(clippy::type_complexity)]
pub fn move_sprite(
    mut qry_player: Query<(&mut Sprite, &mut Transform, &mut Player)>,
    opt_input_direction: Option<ResMut<InputDirection>>,
    opt_map: Option<Res<Map>>,
    //     mut qry_player: Query<( &mut Player, &mut TextureAtlas )>,
    //     mut pst_transform: ParamSet
    //     <(  Query<&mut Transform, With<Player>>,
    //         Query<&mut Transform, With<PlayerTriangle>>,
    //     )>,
    opt_demo: Option<Res<DemoMapParams>>,
    qry_chasers: Query<&Chaser>,
    state: ResMut<State<MyState>>,
    mut evt_timer: EventWriter<EventTimerPlayer>,
    time: Res<Time>,
) -> Result
{
    let (mut sprite, mut transform, mut player) = qry_player.single_mut()?;
    let mut input_direction =
        opt_input_direction.ok_or("Resource InputDirection not found.")?;
    let map = opt_map.ok_or("Resource Map not found.")?;
    //     let Ok ( ( mut player, mut sprite_sheet ) ) = qry_player.get_single_mut() else { return };
    //     let mut qry_transform = pst_transform.p0();
    //     let Ok ( mut transform ) = qry_transform.get_single_mut() else { return };

    // 前回からの経過時間にスピードアップ係数をかける
    let time_delta = time.delta().mul_f32(player.speedup);

    // グリッドのマス間を移動中か？
    if !player.timer.tick(time_delta).finished()
    {
        if !player.is_stop
        {
            // 移動中の中割座標
            let delta = PLAYER_SPEED * time_delta.as_secs_f32();
            match player.direction
            {
                News::North => transform.translation.y += delta,
                News::South => transform.translation.y -= delta,
                News::East => transform.translation.x += delta,
                News::West => transform.translation.x -= delta,
            }
            player.px_start = player.px_end;
            player.px_end = transform.translation.truncate();
        }
    }
    else
    {
        evt_timer.write(EventTimerPlayer); // 後続の処理にtimer finishedを伝達する

        // スプライトをグリッドに配置する
        if player.px_start != player.px_end
        {
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
            for side in input_direction.0.iter()
            // input_direction.0は優先順に並んでいる前提
            {
                // 壁でない場合
                if map.is_space(player.next_grid + side)
                {
                    new_side = *side;
                    player.is_stop = false;
                    break;
                }

                // ループの先頭要素では、向きを必ず変える
                if *side == (input_direction.0)[0]
                // データの性質上 値が重複する要素はない
                {
                    new_side = *side;
                }
            }
        }
        else
        {
            // demoの場合 入力相当のデータをアルゴリズムで作る
            player.is_stop = false; // demoでは自機は停止しない

            let mut sides = map.get_side_spaces_list(player.next_grid); // 脇道のリスト
            sides.retain(|side| player.next_grid + side != player.grid); // 戻り路を取り除く

            new_side = match sides.len().cmp(&1)
            {
                Ordering::Equal =>
                // 一本道 ⇒ 道なりに進む
                    sides[0],
                Ordering::Greater =>
                // 三叉路または十字路
                    if let (Some(autodrive), Some(demo)) =
                        (player.opt_fn_autodrive, opt_demo)
                    {
                        // 外部関数で進行方向を決める
                        autodrive(&player, qry_chasers, map, demo, &sides)
                    }
                    else
                    {
                        // 外部関数を使えないなら乱数で決める
                        let mut rng = rand::rng();
                        sides[rng.random_range(0..sides.len())]
                    },
                Ordering::Less =>
                // 行き止まり ⇒ 逆走 (このゲームに行き止まりはないけど)
                    match player.direction
                    {
                        News::North => News::South,
                        News::South => News::North,
                        News::East => News::West,
                        News::West => News::East,
                    },
            };
        }

        // 進行方向が変わったらスプライトの見栄えを変える
        if player.direction != new_side
        {
            // if SPRITE_OFF()
            // {   //三角形を回転させる
            //     if let Ok ( mut transform ) = pst_transform.p1().get_single_mut()
            //     {   rotate_player_sprite( &player, &mut transform, new_side );
            //     }
            // }
            // else
            {
                // スプライトシートのindexを変更する
                let old_offset =
                    player.sprite_sheet_offset(player.direction) as usize;
                let new_offset = player.sprite_sheet_offset(new_side) as usize;
                let sprite_sheet = &mut sprite
                    .texture_atlas
                    .as_mut()
                    .ok_or("Sprite TextureAtlas not setting.")?;
                let index = &mut sprite_sheet.index;
                *index = *index + new_offset - old_offset;
            }
            player.direction = new_side;
        }

        // 現在の位置と次の位置を更新する
        player.grid = player.next_grid;
        if !player.is_stop
        {
            player.next_grid += new_side;
        }

        // タイマーをリセットする
        player.timer.reset();
    }

    *input_direction = InputDirection::default();

    Ok(())
}

// 自機の向きと入力から角度の差分を求めてスプライトを回転させる
// fn rotate_player_sprite
// (   player: &Player,
//     transform: &mut Mut<Transform>,
//     input: News
// )
// {   let angle: f32 = match player.direction
//     {   News::North => match input
//         {   News::West => PI /  2.0,
//             News::East => PI / -2.0,
//             _ => PI,
//         }
//         News::South => match input
//         {   News::East => PI /  2.0,
//             News::West => PI / -2.0,
//             _  => PI,
//         }
//         News::East => match input
//         {   News::North => PI /  2.0,
//             News::South => PI / -2.0,
//             _ => PI,
//         }
//         News::West => match input
//         {   News::South => PI /  2.0,
//             News::North => PI / -2.0,
//             _ => PI,
//         }
//     };

//     let quat = Quat::from_rotation_z( angle );
//     transform.rotate( quat );
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
