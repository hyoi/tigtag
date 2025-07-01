use super::*;

////////////////////////////////////////////////////////////////////////////////

// スプライトシートでアニメーションするためのトレイト実装
impl CharacterAnimation for Chaser
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

// チェイサーをspawnする
pub fn spawn_sprite(
    opt_record: Option<Res<Record>>,
    qry_entity: Query<Entity, With<Chaser>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut texture_atlases_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result
{
    // 準備
    let record = opt_record.ok_or("Resource <Record> not found.")?; // 必須のResource
    qry_entity.iter().for_each(|id| cmds.entity(id).despawn()); // 既存スプライトがあれば削除する

    // 敵キャラをマップの四隅に配置する
    (0..)
        .zip(CHASER_START_POSITION)
        .for_each(|(i, start_grid)| {
            // ステージ数を4で割ったあまりをindex（0,1,2,3）にする
            let index = ((record.stage() - 1 + i) % 4) as usize;
            let (color, opt_fn_autochase, asset_file) = CHASERS_SPRITE_INFO[index];

            // 初期位置
            let vec2 = start_grid.to_vec2_on_game_map();
            let translation = vec2.extend(DEPTH_SPRITE_CHASER);

            // Componentを初期化する
            let chaser = Chaser {
                grid: *start_grid,
                next_grid: *start_grid,
                px_start: vec2,
                px_end: vec2,
                color,
                opt_fn_autochase,
                ..default()
            };

            if SPRITE_OFF()
            {
                // 正方形のメッシュ
                let radius = PIXELS_PER_GRID * CHASER_SPRITE_SCALING;
                let shape = RegularPolygon::new(radius, 4).mesh();
                cmds.spawn((
                    Mesh2d(meshes.add(shape)),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(translation),
                    chaser, // データ
                ));
            }
            else
            {
                // アニメーションするスプライトをspawnする
                let custom_size = Some(GRID_CUSTOM_SIZE);
                let layout =
                    texture_atlases_layout.add(TextureAtlasLayout::from_grid(
                        SPRITE_SHEET_SIZE_CHASER,
                        SPRITE_SHEET_COLS_CHASER,
                        SPRITE_SHEET_ROWS_CHASER,
                        None,
                        None,
                    ));
                let index = chaser.sprite_sheet_offset(chaser.direction()) as usize;
                let mut sprite = Sprite::from_atlas_image(
                    asset_svr.load(asset_file),
                    TextureAtlas { layout, index },
                );
                sprite.custom_size = custom_size;
                cmds.spawn((
                    sprite,
                    Transform::from_translation(translation),
                    chaser, // データ
                ));
            }
        });

    Ok(())
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

// チェイサーのスプライトを回転させる（SPRITE OFFの場合）
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

// 敵キャラを移動させる
// pub fn move_sprite
// (   mut qry_chaser: Query<( &mut Chaser, &mut Transform, &mut TextureAtlas )>,
//     opt_map: Option<Res<map::Map>>,
//     qry_player: Query<&player::Player>,
//     mut evt_timer: EventWriter<EventTimerChasers>,
//     time: Res<Time>,
// )
// {   let Ok ( player ) = qry_player.get_single() else { return };
//     let Some ( map ) = opt_map else { return };

//     //前回からの経過時間
//     let time_delta = time.delta();
//     let mut chaser_timer_finished = Vec::new();

//     //敵キャラは複数なのでループ処理する
//     for ( mut chaser, mut transform, mut sprite_sheet ) in qry_chaser.iter_mut()
//     {   //自動追尾の関数がセットされているか？
//         let Some ( autochase ) = chaser.opt_fn_autochase else { return };

//         //経過時間にスピードアップを反映する
//         let time_delta = time_delta.mul_f32( chaser.speedup );

//         //グリッドのマス間を移動中か？
//         if ! chaser.timer.tick( time_delta ).finished()
//         {   if ! chaser.is_stop //スタート直後だけ意味があるif文
//             {   //移動中の中割座標
//                 let delta = CHASER_SPEED * time_delta.as_secs_f32();
//                 match chaser.direction
//                 {   News::North => transform.translation.y += delta,
//                     News::South => transform.translation.y -= delta,
//                     News::East  => transform.translation.x += delta,
//                     News::West  => transform.translation.x -= delta,
//                 }
//                 chaser.px_start = chaser.px_end;
//                 chaser.px_end   = transform.translation.truncate();
//             }
//         }
//         else
//         {   chaser_timer_finished.push( chaser.color ); //後続の処理にtimer finishedを伝達する

//             //スプライトをグリッドに配置する
//             if chaser.px_start != chaser.px_end
//             {   chaser.px_start = chaser.px_end;
//                 chaser.px_end   = chaser.next_grid.to_vec2_on_game_map();
//                 transform.translation = chaser.px_end.extend( DEPTH_SPRITE_CHASER );
//             }

//             //四方の脇道を取得する
//             let mut sides = map.get_side_spaces_list( chaser.next_grid );    //脇道のリスト
//             sides.retain( | side | chaser.next_grid + side != chaser.grid ); //戻り路を取り除く

//             //敵キャラが次に進む方向を決める
//             chaser.is_stop = false; //停止フラグを倒す(敵キャラはスタート後は止まらない)

//             let new_side = match sides.len().cmp( &1 ) //sides要素数は１以上(マップに行き止まりが無いので)
//             {   //一本道 ⇒ 道なりに進む
//                 Ordering::Equal => sides[ 0 ],

//                 //三叉路または十字路 ⇒ 外部関数で自動追尾する
//                 Ordering::Greater => autochase( &mut chaser, player, &sides ),

//                 //行き止まり ⇒ 逆走 (このゲームに行き止まりはないのでここには来ないけど)
//                 Ordering::Less => chaser.direction.back_side(),
//             };

//             //進行方向が変わったらスプライトの見栄えを変える（スプライトシートのindexを変える）
//             if ! SPRITE_OFF() && chaser.direction != new_side
//             {   let old_offset = chaser.sprite_sheet_offset( chaser.direction ) as usize;
//                 let new_offset = chaser.sprite_sheet_offset( new_side         ) as usize;
//                 sprite_sheet.index = sprite_sheet.index + new_offset - old_offset;
//             }
//             chaser.direction = new_side;

//             //現在の位置と次の位置を更新する
//             chaser.grid = chaser.next_grid;
//             if ! chaser.is_stop
//             {   let side = chaser.direction;
//                 chaser.next_grid += side; //✕ chaser.direction += chaser.next_grid
//             }

//             //タイマーをリセットする
//             chaser.timer.reset();
//         }
//     }

//     //後続の処理にtimer finishedを伝達する
//     if ! chaser_timer_finished.is_empty()
//     {   evt_timer.send( EventTimerChasers ( chaser_timer_finished ) ); //tigtag3d用の追加フィールド
//     }

//     //敵キャラは重なるとスピードアップする
//     let mut color_grid = Vec::with_capacity( qry_chaser.iter().len() );
//     for ( mut chaser, _, _ ) in qry_chaser.iter_mut()
//     {   color_grid.push( ( chaser.color, chaser.next_grid ) );
//         chaser.speedup = 1.0;
//     }
//     for ( color, grid ) in color_grid
//     {   for ( mut chaser, _, _ ) in qry_chaser.iter_mut()
//         {   if grid != chaser.next_grid || color == chaser.color { continue }
//             chaser.speedup += CHASER_ACCEL;
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
