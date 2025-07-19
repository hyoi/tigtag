use super::*;

////////////////////////////////////////////////////////////////////////////////

// チェイサーをspawnする
pub fn spawn_sprite(
    qry_entity: Query<Entity, With<Chaser>>,
    opt_record: Option<Res<Record>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut texture_atlases_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result
{
    // 準備
    qry_entity.iter().for_each(|id| cmds.entity(id).despawn()); // 既存スプライトがあれば削除する
    let record = opt_record.ok_or("Res<Record> not found.")?; // 必須のResource

    // チェイサーを初期位置に配置する
    (0..).zip(CHASER_START_POSITION).for_each(|(idx, &grid)| {
        // 初期位置
        let vec2 = grid.to_vec2_on_game_map();
        let vec3 = vec2.extend(DEPTH_SPRITE_CHASER);
        let transform = Transform::from_translation(vec3);

        // index（0,1,2,3）を作る
        let index = ((record.stage() - 1 + idx) % 4) as usize;

        //チェイサーの情報
        let (asset_file, color, opt_fn_autochase) = CHASERS_SPRITE_INFO[index];

        // チェイサーのデータを初期化する
        let chaser = Chaser {
            grid,
            next_grid: grid,
            px_start: vec2,
            px_end: vec2,
            color,
            opt_fn_autochase,
            ..default()
        };

        if SPRITE_OFF()
        {
            // 正方形のメッシュを作る
            let shape = RegularPolygon::new(CHASER_SPRITE_RADIUS, 4).mesh();
            cmds.spawn((
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(materials.add(Color::Srgba(color))),
                transform,
                chaser,
                // move_sprite()の引数qry_chaserのQueryを満たす為に必要なダミーのSprite
                Sprite::default(),
            ));
        }
        else
        {
            // アニメーションするスプライトをspawnする
            let layout = texture_atlases_layout.add(SPRITESHEET_LAYOUT.clone());
            let index = chaser.sprite_sheet_offset(chaser.direction()) as usize;

            let mut sprite = Sprite::from_atlas_image(
                asset_svr.load(asset_file),
                TextureAtlas { layout, index },
            );
            sprite.custom_size = Some(GRID_CUSTOM_SIZE);

            cmds.spawn((sprite, transform, chaser));
        }
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// チェイサーを移動させる
pub fn move_sprite(
    mut qry_chaser: Query<(&mut Transform, &mut Sprite, &mut Chaser)>,
    opt_map: Option<Res<Map>>,
    qry_player: Query<&player::Player>,
    // mut evt_timer: EventWriter<EventTimerChasers>,
    time: Res<Time>,
) -> Result
{
    //準備
    let player = qry_player.single()?;
    let map = opt_map.ok_or("Res<Map> not found.")?;
    let time_delta = time.delta();
    let mut chaser_timer_finished = Vec::new();

    //敵キャラは複数なのでループ処理する
    for (mut transform, mut sprite, mut chaser) in qry_chaser.iter_mut()
    {
        // 前回からの経過時間 × スピードアップ係数
        // let time_delta = time_delta.mul_f32( chaser.speedup ); //??? speedupが1.x？？

        // 移動タイマーがfinishしたなら
        if chaser.timer.tick(time_delta).finished()
        {
            chaser_timer_finished.push(chaser.color); //後続の処理にtimer finishedを伝達する

            // スプライトがグリッド間の中途に位置したら
            if chaser.px_start != chaser.px_end
            {
                // グリッドにフィットさせる
                chaser.px_start = chaser.px_end;
                chaser.px_end = chaser.next_grid.to_vec2_on_game_map();
                transform.translation = chaser.px_end.extend(DEPTH_SPRITE_CHASER);
            }

            //四方の脇道を取得する
            let mut sides = map.get_side_spaces_list(chaser.next_grid); //脇道のリスト
            sides.retain(|side| chaser.next_grid + side != chaser.grid); //戻り路を取り除く

            //敵キャラが次に進む方向を決める
            chaser.is_stop = false; //停止フラグを倒す(敵キャラはスタート後は止まらない)
            let count = sides.len();
            let new_side = match count
            {
                // 一本道（直線やコーナーを道なりに進む）
                1 => sides[0],
                // 三叉路か十字路なので進行方向を考える
                2.. =>
                {
                    //自動追尾の関数がセットされているなら
                    if let Some(autochase) = chaser.opt_fn_autochase
                    {
                        //追尾関数
                        autochase(&mut chaser, player, &sides)
                    }
                    else
                    {
                        //進行方向をランダムに決める
                        sides[rand::rng().random_range(0..count)]
                    }
                }
                // ここには来ない（このゲームにはマップ上行き止まりはないので）
                _ => chaser.direction.back(), // 逆走
            };

            // チェイサーの向きが変わったなら
            if new_side != chaser.direction
            {
                if let Some(sprite_sheet) = &mut sprite.texture_atlas.as_mut()
                    && !SPRITE_OFF()
                {
                    // スプライトシートのindexを変更してスプライトの向きを変更
                    let old_news = chaser.direction;
                    let old_offset = chaser.sprite_sheet_offset(old_news) as usize;
                    let new_offset = chaser.sprite_sheet_offset(new_side) as usize;
                    let index = &mut sprite_sheet.index;
                    *index = *index + new_offset - old_offset;
                }

                // チェイサーの向きの情報の更新
                chaser.direction = new_side;
            }

            // 位置を更新
            chaser.grid = chaser.next_grid; //現在の位置を更新
            if !chaser.is_stop
            {
                let side = chaser.direction; //✕ chaser.direction += chaser.next_grid
                chaser.next_grid += side; //次の位置を更新
            }

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

            //当たり判定用の微小区間の座標更新
            chaser.px_start = chaser.px_end;
            chaser.px_end = transform.translation.truncate();
        }
    }

    //後続の処理にtimer finishedを伝達する
    // if ! chaser_timer_finished.is_empty()
    // {   evt_timer.send( EventTimerChasers ( chaser_timer_finished ) ); //tigtag3d用の追加フィールド
    // }

    //敵キャラは重なるとスピードアップする
    let mut color_grid = Vec::with_capacity(qry_chaser.iter().len());
    for (_, _, mut chaser) in qry_chaser.iter_mut()
    {
        color_grid.push((chaser.color, chaser.next_grid));
        chaser.speedup = 1.0;
    }
    for (color, grid) in color_grid
    {
        for (_, _, mut chaser) in qry_chaser.iter_mut()
        {
            if grid != chaser.next_grid || color == chaser.color
            {
                continue;
            }
            chaser.speedup += CHASER_ACCEL;
        }
    }

    Ok(())
}

// チェイサー（正方形）を回転させる
pub fn rotate_chaser_shape(
    mut qry_chaser: Query<&mut Transform, With<Chaser>>,
    time: Res<Time>,
)
{
    let time_delta = time.delta().as_secs_f32();
    let radian = TAU * time_delta;
    let quat = Quat::from_rotation_z(radian);

    // 回転させる
    qry_chaser
        .iter_mut()
        .for_each(|mut transform| transform.rotate(quat));
}

////////////////////////////////////////////////////////////////////////////////

// 進む方向を決める(赤)
pub const CHOICE_WAY_RED: Option<FnAutoChase> = None; // Some( choice_way_red );
// fn choice_way_red( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
// {   if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
//     if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
//     if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
//     if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
//     sides[ rand::rng().random_range( 0..sides.len() ) ]
// }

// 進む方向を決める(青)
pub const CHOICE_WAY_BLUE: Option<FnAutoChase> = None; // Some( choice_way_blue );
// fn choice_way_blue( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
// {   if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
//     if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
//     if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
//     if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
//     sides[ rand::rng().random_range( 0..sides.len() ) ]
// }

// 進む方向を決める(緑)
pub const CHOICE_WAY_GREEN: Option<FnAutoChase> = None; // Some( choice_way_green );
// fn choice_way_green( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
// {   if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
//     if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
//     if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
//     if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
//     sides[ rand::rng().random_range( 0..sides.len() ) ]
// }

// 進む方向を決める(ピンク)
pub const CHOICE_WAY_PINK: Option<FnAutoChase> = None; // Some( choice_way_pink );
// fn choice_way_pink( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
// {   if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
//     if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
//     if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
//     if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
//     sides[ rand::rng().random_range( 0..sides.len() ) ]
                                                       // }

////////////////////////////////////////////////////////////////////////////////

// End of code.
