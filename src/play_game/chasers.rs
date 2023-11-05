use super::*;

////////////////////////////////////////////////////////////////////////////////

//チェイサーの色と移動方向の決定関数
const COLOR_SPRITE_CHASERS: [ ( Color, Option<FnChasing> ); 4 ] = 
[   ( Color::RED,   Some ( which_way_red_goes   ) ),
    ( Color::GREEN, Some ( which_way_green_goes ) ),
    ( Color::PINK,  Some ( which_way_pink_goes  ) ),
    ( Color::BLUE,  Some ( which_way_blue_goes  ) ),
];

//移動方向を決める(赤)
fn which_way_red_goes( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::West  ) && player.next.x < chaser.grid.x { return News::West  }
    else if sides.contains( &News::East  ) && player.next.x > chaser.grid.x { return News::East  }
    else if sides.contains( &News::North ) && player.next.y < chaser.grid.y { return News::North }
    else if sides.contains( &News::South ) && player.next.y > chaser.grid.y { return News::South }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//移動方向を決める(青)
fn which_way_blue_goes( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::South ) && player.next.y > chaser.grid.y { return News::South }
    else if sides.contains( &News::West  ) && player.next.x < chaser.grid.x { return News::West  }
    else if sides.contains( &News::East  ) && player.next.x > chaser.grid.x { return News::East  }
    else if sides.contains( &News::North ) && player.next.y < chaser.grid.y { return News::North }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//移動方向を決める(緑)
fn which_way_green_goes( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::North ) && player.next.y < chaser.grid.y { return News::North }
    else if sides.contains( &News::South ) && player.next.y > chaser.grid.y { return News::South }
    else if sides.contains( &News::West  ) && player.next.x < chaser.grid.x { return News::West  }
    else if sides.contains( &News::East  ) && player.next.x > chaser.grid.x { return News::East  }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//移動方向を決める(ピンク)
fn which_way_pink_goes( chaser: &mut Chaser, player: &Player, sides: &[ News ] ) -> News
{        if sides.contains( &News::East  ) && player.next.x > chaser.grid.x { return News::East  }
    else if sides.contains( &News::North ) && player.next.y < chaser.grid.y { return News::North }
    else if sides.contains( &News::South ) && player.next.y > chaser.grid.y { return News::South }
    else if sides.contains( &News::West  ) && player.next.x < chaser.grid.x { return News::West  }
    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

////////////////////////////////////////////////////////////////////////////////

//チェイサーをspawnする
pub fn spawn_sprite
(   qry_player: Query<Entity, With<Chaser>>,
    opt_record: Option<Res<Record>>,
    mut cmds: Commands,
)
{   let Some ( record ) = opt_record else { return };

    //スプライトがあれば削除する
    qry_player.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //追手のスプライトを配置する
    let custom_size = Some ( SIZE_GRID * MAGNIFY_SPRITE_CHASER );

    ( 0.. ).zip( CHASER_INIT_POSITION ).for_each
    (   | ( i, ( x, y ) ) |
        {   let grid  = IVec2::new( x, y );
            let pixel = grid.to_sprite_pixels() + ADJUSTER_MAP_SPRITES;
            let index = ( ( i + record.stage() - 1 ) % 4 ) as usize;
            let ( color, fn_chasing ) = COLOR_SPRITE_CHASERS[ index ];
            let chaser = Chaser
            {   grid,
                next    : grid,
                px_start: pixel,
                px_end  : pixel,
                color,
                fn_chasing,
                ..default()
            };

            cmds
            .spawn( ( SpriteBundle::default(), chaser ) )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_CHASER ) ) )
            ;
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//チェイサーを回転させる
pub fn rotate
(   mut qry_chaser: Query<&mut Transform, With<Chaser>>,
    time: Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32();
    let angle = 360.0 * time_delta;
    let quat = Quat::from_rotation_z( angle.to_radians() );

    //回転させる
    qry_chaser.for_each_mut( | mut transform | transform.rotate( quat ) );
}

////////////////////////////////////////////////////////////////////////////////

//チェイサーを移動させる
pub fn move_sprite
(   qry_player: Query<&Player>,
    mut qry_chaser: Query<(&mut Chaser, &mut Transform)>,
    opt_map: Option<Res<Map>>,
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
    for ( mut chaser, mut transform ) in qry_chaser.iter_mut()
    {   //スピードアップを反映する
        let time_delta = time_delta.mul_f32( chaser.speedup );

        //待ち時間が完了したら or ストップ状態だったら ⇒ 移動方向を決めて移動開始
        if chaser.wait.tick( time_delta ).finished() || chaser.stop
        {   //スプライトの表示位置をグリッドにそろえる
            if chaser.px_start != chaser.px_end
            {   chaser.px_start = chaser.px_end;
                chaser.px_end   = chaser.next.to_sprite_pixels() + ADJUSTER_MAP_SPRITES;
                transform.translation = chaser.px_end.extend( DEPTH_SPRITE_CHASER );
            }
    
            //四方の脇道を取得する
            let mut sides = map.get_byways_list( chaser.next );         //脇道のリスト
            sides.retain( | side | chaser.next + side != chaser.grid ); //戻り路を排除

            //チェイサーの向きを決める（プレーヤーのキー入力に相当）
            chaser.stop = false;
            chaser.side =
                match sides.len().cmp( &1 ) //sides要素数は１以上(このゲームのマップに行き止まりが無いので)
                {   Ordering::Equal =>
                        sides[ 0 ], //一本道なら道なりに進む
                    Ordering::Greater =>
                        if let Some ( fnx ) = chaser.fn_chasing
                        {   fnx( &mut chaser, player, &sides ) //分かれ道なら外部関数で進行方向を決める
                        }
                        else
                        {   chaser.stop = true; //外部関数がない(None)なら停止フラグを立てる
                            chaser.side
                        },
                    Ordering::Less =>
                        match chaser.side //行き止まりなら逆走する(このゲームに行き止まりはないけど)
                        {   News::North => News::South,
                            News::South => News::North,
                            News::East  => News::West ,
                            News::West  => News::East ,
                        },
                };

            //現在の位置と次の位置を更新する
            chaser.grid = chaser.next;
            if ! chaser.stop
            {   let side = chaser.side; //chaser.side += chaser.next すると、
                chaser.next += side;    //error[E0502]: cannot borrow `chaser` as 
            }                           //immutable because it is also borrowed as mutable

            //waitをリセットする
            chaser.wait.reset();
        }
        else if ! chaser.stop
        {   //移動中ならスプライトを中割の位置に移動する
            let delta = CHASER_MOVE_COEF * time_delta.as_secs_f32();
            match chaser.side
            {   News::North => transform.translation.y += delta,
                News::South => transform.translation.y -= delta,
                News::East  => transform.translation.x += delta,
                News::West  => transform.translation.x -= delta,
            }
            chaser.px_start = chaser.px_end;
            chaser.px_end   = transform.translation.truncate();
        }
    }

    //チェイサーは重なるとスピードアップする
    let mut color_grid = Vec::with_capacity( qry_chaser.iter().len() );
    for ( mut chaser, _ ) in qry_chaser.iter_mut()
    {   color_grid.push( ( chaser.color, chaser.next ) );
        chaser.speedup = 1.0;
    }
    for ( color, grid ) in color_grid
    {   for ( mut chaser, _ ) in qry_chaser.iter_mut()
        {   if grid != chaser.next || color == chaser.color { continue }
            chaser.speedup += CHASER_ACCEL;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.