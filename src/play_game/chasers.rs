use super::*;

////////////////////////////////////////////////////////////////////////////////

//チェイサーの色と移動方向の決定関数
const COLOR_SPRITE_CHASERS: &[ ( Color, Option<FnChasing> ) ] =
&[  ( Color::RED,   Some ( choice_way_red   ) ),
    ( Color::GREEN, Some ( choice_way_green ) ),
    ( Color::PINK,  Some ( choice_way_pink  ) ),
    ( Color::BLUE,  Some ( choice_way_blue  ) ),
];

//進む方向を決める(赤)
fn choice_way_red( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    else if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    else if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    else if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//進む方向を決める(青)
fn choice_way_blue( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    else if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    else if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    else if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//進む方向を決める(緑)
fn choice_way_green( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    else if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    else if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    else if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//進む方向を決める(ピンク)
fn choice_way_pink( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    else if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    else if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    else if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

////////////////////////////////////////////////////////////////////////////////

//チェイサーをspawnする
pub fn spawn_sprite
(   qry_chaser: Query<Entity, With<Chaser>>,
    opt_record: Option<Res<Record>>,
    mut cmds: Commands,
    // asset_svr: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{   let Some ( record ) = opt_record else { return };

    //スプライトがあれば削除する
    qry_chaser.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //チェイサーのスプライトを配置する
    ( 0.. ).zip( CHASER_START_POSITION ).zip( SPRITE_SHEET_CHASERS ).for_each
    (   | ( ( i, chaser_grid ), asset ) |
        {   let chaser_vec2 = chaser_grid.to_vec2_on_map();
            let index = ( ( i + record.stage() - 1 ) % 4 ) as usize;
            let ( color, opt_fn_chasing ) = COLOR_SPRITE_CHASERS[ index ];
            let chaser = Chaser
            {   grid     : *chaser_grid,
                next_grid: *chaser_grid,
                dx_start : chaser_vec2,
                dx_end   : chaser_vec2,
                color,
                opt_fn_chasing,
                ..default()
            };
            let translation = chaser_vec2.extend( DEPTH_SPRITE_CHASER );

            // //アニメーションするスプライトをspawnする
            // let texture_atlas = asset_svr.gen_texture_atlas_chaser( asset );
            // let texture_atlas_hdl = texture_atlases.add( texture_atlas );
            // let custom_size = Some( SIZE_GRID );
            // let texture_atlas_sprite = TextureAtlasSprite { custom_size, ..default() };
            // cmds.spawn( ( SpriteSheetBundle::default(), chaser ) )
            // .insert( texture_atlas_hdl.clone() )
            // .insert( texture_atlas_sprite )
            // .insert( Transform::from_translation( translation ) )
            // ;

            //正方形のメッシュ
            let custom_size = Some ( SIZE_GRID * _MAGNIFY_SPRITE_CHASER );
            cmds
            .spawn( ( SpriteBundle::default(), chaser ) )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( translation ) )
            // .insert( TextureAtlasSprite::default() ) //move_sprite()のqry_playerの検索条件を満たすためのdummy
            ;
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//チェイサー（正方形のメッシュの場合）のスプライトを回転させる
pub fn rotate
(   mut qry_chaser: Query<&mut Transform, With<Chaser>>,
    time: Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32();
    let radian = TAU * time_delta;
    let quat = Quat::from_rotation_z( radian );

    //回転させる
    qry_chaser.iter_mut().for_each( | mut transform | transform.rotate( quat ) );
}

////////////////////////////////////////////////////////////////////////////////

//チェイサーを移動させる
pub fn move_sprite
(   mut qry_chaser: Query<( &mut Chaser, &mut Transform, /*&mut TextureAtlasSprite*/ )>,
    opt_map: Option<Res<Map>>,
    qry_player: Query<&Player>,
    mut evt_clear : EventReader<EventClear>,
    mut evt_over  : EventReader<EventOver>,
    time: Res<Time>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };
    let Some ( map ) = opt_map else { return };

    //直前の判定でクリア／オーバーしていたらスプライトを移動させない
    if evt_clear.read().next().is_some() { return }
    if evt_over .read().next().is_some() { return }

    //前回からの経過時間
    let time_delta = time.delta();

    //チェイサーは複数なのでループ処理する
    for ( mut chaser, mut transform, /*mut sprite*/ ) in qry_chaser.iter_mut()
    {   //経過時間にスピードアップを反映する
        let time_delta = time_delta.mul_f32( chaser.speedup );

        //タイマーが完了したら or ストップ状態だったら ⇒ 移動方向を決めて移動開始
        if chaser.timer.tick( time_delta ).finished() || chaser.is_stop
        {   //スプライトをchaser.next_gridにそろえる
            if chaser.dx_start != chaser.dx_end
            {   chaser.dx_start = chaser.dx_end;
                chaser.dx_end   = chaser.next_grid.to_vec2_on_map();
                transform.translation = chaser.dx_end.extend( DEPTH_SPRITE_CHASER );
            }

            //四方の脇道を取得する
            let mut sides = map.get_side_spaces_list( chaser.next_grid ); //脇道のリスト
            sides.retain( | side | chaser.next_grid + side != chaser.grid ); //戻り路を取り除く

            //チェイサーが次に進む方向を決める（プレーヤーのキー入力に相当）
            chaser.is_stop = false;
            let new_side = match sides.len().cmp( &1 ) //sides要素数は１以上(マップに行き止まりが無いので)
            {   Ordering::Equal => //一本道 ⇒ 道なりに進む
                    sides[ 0 ],
                Ordering::Greater => //三叉路または十字路
                    if let Some ( chasing ) = chaser.opt_fn_chasing
                    {   //外部関数で進行方向を決める
                        chasing( &mut chaser, player, &sides )
                    }
                    else
                    {   //外部関数を使えないなら停止フラグを立てる
                        chaser.is_stop = true;
                        chaser.direction
                    },
                Ordering::Less => //行き止まり ⇒ 逆走 (このゲームに行き止まりはないけど)
                    match chaser.direction
                    {   News::North => News::South,
                        News::South => News::North,
                        News::East  => News::West ,
                        News::West  => News::East ,
                    },
            };

            // //チェイサーの向きが変わったらスプライトアニメのテクスチャハンドルを差し替える
            // if chaser.direction != new_side
            // {   let old_offset = chaser.sprite_sheet_offset( chaser.direction );
            //     let new_offset = chaser.sprite_sheet_offset( new_side         );
            //     sprite.index = sprite.index - old_offset + new_offset;
            // }
            chaser.direction = new_side;

            //現在の位置と次の位置を更新する
            chaser.grid = chaser.next_grid;
            if ! chaser.is_stop
            {   let side = chaser.direction; //chaser.direction += chaser.next_grid すると、
                chaser.next_grid += side;    //error[E0502]: cannot borrow `chaser` as
            }                                //immutable because it is also borrowed as mutable

            //タイマーをリセットする
            chaser.timer.reset();
        }
        else if ! chaser.is_stop
        {   //移動中の中割アニメーション
            let delta = CHASER_SPEED * time_delta.as_secs_f32();
            match chaser.direction
            {   News::North => transform.translation.y += delta,
                News::South => transform.translation.y -= delta,
                News::East  => transform.translation.x += delta,
                News::West  => transform.translation.x -= delta,
            }
            chaser.dx_start = chaser.dx_end;
            chaser.dx_end   = transform.translation.truncate();
        }
    }

    //チェイサーは重なるとスピードアップする
    let mut color_grid = Vec::with_capacity( qry_chaser.iter().len() );
    for ( mut chaser, _, /*_*/ ) in qry_chaser.iter_mut()
    {   color_grid.push( ( chaser.color, chaser.next_grid ) );
        chaser.speedup = 1.0;
    }
    for ( color, grid ) in color_grid
    {   for ( mut chaser, _, /*_*/ ) in qry_chaser.iter_mut()
        {   if grid != chaser.next_grid || color == chaser.color { continue }
            chaser.speedup += CHASER_ACCEL;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.