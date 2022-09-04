use super::*;

//追手のスプライトの色と移動方向決定関数
pub const COLOR_SPRITE_CHASERS: [ ( Color, Option<FnChasing> ); 4 ] =
[   ( Color::RED,   Some ( which_way_red_goes   ) ),
    ( Color::GREEN, Some ( which_way_green_goes ) ),
    ( Color::PINK,  Some ( which_way_pink_goes  ) ),
    ( Color::BLUE,  Some ( which_way_blue_goes  ) ),
];

//追手の移動方向を決める(赤)
fn which_way_red_goes( chaser: &mut Chaser, player: &Player, sides: &[ DxDy ] ) -> DxDy
{        if sides.contains( &DxDy::Left  ) && player.next.x < chaser.grid.x { return DxDy::Left  }
    else if sides.contains( &DxDy::Right ) && player.next.x > chaser.grid.x { return DxDy::Right }
    else if sides.contains( &DxDy::Up    ) && player.next.y < chaser.grid.y { return DxDy::Up    }
    else if sides.contains( &DxDy::Down  ) && player.next.y > chaser.grid.y { return DxDy::Down  }

    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//追手の移動方向を決める(青)
fn which_way_blue_goes( chaser: &mut Chaser, player: &Player, sides: &[ DxDy ] ) -> DxDy
{        if sides.contains( &DxDy::Down  ) && player.next.y > chaser.grid.y { return DxDy::Down  }
    else if sides.contains( &DxDy::Left  ) && player.next.x < chaser.grid.x { return DxDy::Left  }
    else if sides.contains( &DxDy::Right ) && player.next.x > chaser.grid.x { return DxDy::Right }
    else if sides.contains( &DxDy::Up    ) && player.next.y < chaser.grid.y { return DxDy::Up    }

    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//追手の移動方向を決める(緑)
fn which_way_green_goes( chaser: &mut Chaser, player: &Player, sides: &[ DxDy ] ) -> DxDy
{        if sides.contains( &DxDy::Up    ) && player.next.y < chaser.grid.y { return DxDy::Up    }
    else if sides.contains( &DxDy::Down  ) && player.next.y > chaser.grid.y { return DxDy::Down  }
    else if sides.contains( &DxDy::Left  ) && player.next.x < chaser.grid.x { return DxDy::Left  }
    else if sides.contains( &DxDy::Right ) && player.next.x > chaser.grid.x { return DxDy::Right }

    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//追手の移動方向を決める(ピンク)
fn which_way_pink_goes( chaser: &mut Chaser, player: &Player, sides: &[ DxDy ] ) -> DxDy
{        if sides.contains( &DxDy::Right ) && player.next.x > chaser.grid.x { return DxDy::Right }
    else if sides.contains( &DxDy::Up    ) && player.next.y < chaser.grid.y { return DxDy::Up    }
    else if sides.contains( &DxDy::Down  ) && player.next.y > chaser.grid.y { return DxDy::Down  }
    else if sides.contains( &DxDy::Left  ) && player.next.x < chaser.grid.x { return DxDy::Left  }

    let mut rng = rand::thread_rng();
    sides[ rng.gen_range( 0..sides.len() ) ]
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//スプライトをspawnして追手を表示する
pub fn spawn_sprite
(   q: Query<Entity, With<Chaser>>,
    o_record: Option<Res<Record>>,
    mut cmds: Commands,
)
{   //スプライトがあれば削除する
    q.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //追手のスプライトを配置する
    let stage = o_record.as_ref().map_or( 0, | record | record.stage ); //スタート位置をローテーションさせる
    let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) * MAGNIFY_SPRITE_CHASER );

    ( 0.. ).zip( CHASER_INIT_POSITION ).for_each
    (   | ( i, ( x, y ) ) |
        {   let grid  = Grid::new( x, y );
            let pixel = grid.into_pixel_map();
            let ( color, fn_chasing ) = COLOR_SPRITE_CHASERS[ ( ( stage + i ) % 4 ) as usize ];
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
            .spawn_bundle( SpriteBundle::default() )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_CHASER ) ) )
            .insert( chaser )
            ;
        }
    );
}

//追手のスプライトを回転させる
pub fn rotate_sprite
(   mut q: Query<&mut Transform, With<Chaser>>,
    time: Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32();
    let angle = 360.0 * time_delta;
    let quat = Quat::from_rotation_z( angle.to_radians() );

    //回転させる
    q.for_each_mut( | mut transform | transform.rotate( quat ) );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//追手の移動
pub fn move_sprite
(   q_player: Query<&Player>,
    mut q_chaser: Query<(&mut Chaser, &mut Transform)>,
    map: Res<Map>,
    mut ev_clear : EventReader<EventClear>,
    mut ev_over  : EventReader<EventOver>,
    time: Res<Time>,
)
{   //直前の判定でクリア／オーバーしていたらスプライトの表示を変更しない
    if ev_clear.iter().next().is_some() { return }
    if ev_over .iter().next().is_some() { return }

    let player = q_player.get_single().unwrap(); //プレイヤーの情報
    let time_delta = time.delta();               //前回からの経過時間

    //追手は複数なのでループ処理する
    for ( mut chaser, mut transform ) in q_chaser.iter_mut()
    {   //スピードアップを反映する
        let time_delta = time_delta.mul_f32( chaser.speedup );

        //待ち時間が完了したら or ストップ状態だったら ⇒ 移動方向を決めて移動開始
        if chaser.wait.tick( time_delta ).finished() || chaser.stop
        {   //スプライトの表示位置をグリッドにそろえる
            if chaser.px_start != chaser.px_end
            {   chaser.px_start = chaser.px_end;
                chaser.px_end   = chaser.next.into_pixel_map();
                transform.translation = chaser.px_end.extend( DEPTH_SPRITE_CHASER );
            }
    
            //四方で壁がない方向を確認する（逆走防止付き）
            let mut sides = Vec::with_capacity( 4 );
            if map.is_passage( chaser.next + DxDy::Up    ) && chaser.side != DxDy::Down  { sides.push( DxDy::Up    ) }
            if map.is_passage( chaser.next + DxDy::Down  ) && chaser.side != DxDy::Up    { sides.push( DxDy::Down  ) }
            if map.is_passage( chaser.next + DxDy::Right ) && chaser.side != DxDy::Left  { sides.push( DxDy::Right ) }
            if map.is_passage( chaser.next + DxDy::Left  ) && chaser.side != DxDy::Right { sides.push( DxDy::Left  ) }

            //追手の向きを決める（自機のプレーヤーのキー入力に相当）
            chaser.stop = false;
            let count = sides.len(); //countは1以上(このゲームのマップには行き止まりが無いので)

            use std::cmp::Ordering;
            match count.cmp( &1 )
            {   Ordering::Equal =>
                {   //一本道では道なりに進む
                    chaser.side = sides[ 0 ];
                },
                Ordering::Greater =>
                {   //道が複数あるなら外部関数で進行方向を決める
                    if chaser.fn_chasing.is_some()
                    {   chaser.side = ( chaser.fn_chasing.unwrap() )( &mut chaser, player, &sides );
                    }
                    else
                    {   //外部関数が設定されていなければ停止フラグを立てる
                        chaser.stop = true;
                    }
                },
                Ordering::Less =>
                {   //行き止まりでは逆走する(このゲームに行き止まりはないけど)
                    chaser.side = match chaser.side
                    {   DxDy::Up    => DxDy::Down ,
                        DxDy::Down  => DxDy::Up   ,
                        DxDy::Right => DxDy::Left ,
                        DxDy::Left  => DxDy::Right,
                    };
                },
            }

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
            {   DxDy::Up    => transform.translation.y += delta,
                DxDy::Down  => transform.translation.y -= delta,
                DxDy::Right => transform.translation.x += delta,
                DxDy::Left  => transform.translation.x -= delta,
            }
            chaser.px_start = chaser.px_end;
            chaser.px_end   = transform.translation.truncate();
        }
    }

    //追手は重なるとスピードアップする
    let mut color_grid = Vec::with_capacity( q_chaser.iter().len() );
    for ( mut chaser, _ ) in q_chaser.iter_mut()
    {   color_grid.push( ( chaser.color, chaser.next ) );
        chaser.speedup = 1.0;
    }
    for ( color, grid ) in color_grid
    {   for ( mut chaser, _ ) in q_chaser.iter_mut()
        {   if grid != chaser.next || color == chaser.color { continue }
            chaser.speedup += CHASER_ACCEL;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//衝突を判定する。条件を満たしたら、
//デモならTitleへ、衝突ならOverへ遷移する
pub fn detect_collisions
(   q_player: Query<&Player>,
    q_chaser: Query<&Chaser>,
    record: Res<Record>,
    mut state: ResMut<State<GameState>>,
    mut ev_over: EventWriter<EventOver>,
)
{   if cfg!( debug_assertions ) { return }   //debugでは無敵

    //クリアしていなければ衝突判定する
    if ! state.current().is_clearstage() && is_collision( q_player, q_chaser )
    {   let next = if record.is_demoplay()
        {   GameState::DemoNext
        }
        else
        {   GameState::GameOver
        };
        let _ = state.overwrite_set( next );
        ev_over.send( EventOver );    //後続の処理にゲームオーバーを伝える
    }
}

//衝突判定関数
fn is_collision
(   q_player: Query<&Player>,
    q_chaser: Query<&Chaser>
) -> bool
{   let mut is_collision = false;

    if let Ok ( player ) = q_player.get_single()
    {   //自機の移動区間を a1➜a2 とする
        let mut a1 = player.px_start;
        let mut a2 = player.px_end;
        if a1.x > a2.x { std::mem::swap( &mut a1.x, &mut a2.x ) } //a1.x < a2.xにする
        if a1.y > a2.y { std::mem::swap( &mut a1.y, &mut a2.y ) } //a1.y < a2.yにする

        //各追手ごとの処理
        for chaser in q_chaser.iter()
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
    }

    //衝突判定の結果を返す
    is_collision
}

//線分の重なりで衝突を判定
fn is_overlap
(   a1: f32, a2: f32,
    b1: f32, b2: f32,
    a_side: DxDy, b_side: DxDy,
) -> bool
{   //a1➜a2 と b1➜b2 が重ならないなら衝突しない(この条件が一番多いので先にはじく)
    if a2 < b1 || b2 < a1 { return false }

    //1つ目、2つ目の条件: a1➜a2 と b1➜b2 が包含関係なら衝突する
    //3つ目の条件: 部分的に重なる場合 移動が対向なら衝突する(同一方向なら衝突しない)
    if a1 < b1 && b2 < a2 || b1 < a1 && a2 < b2 || a_side != b_side { return true }

    false
}

//Endo of code.