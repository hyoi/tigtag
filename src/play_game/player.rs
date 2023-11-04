use super::*;

////////////////////////////////////////////////////////////////////////////////

//プレイヤーをspawnする
pub fn spawn_sprite
(   qry_player: Query<Entity, With<Player>>,
    opt_map: Option<ResMut<Map>>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{   let Some ( mut map ) = opt_map else { return };

    //スプライトがあれば削除する
    qry_player.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //乱数で初期位置を決める(マップ中央付近の通路)
    let half_w = MAP_GRIDS_WIDTH  / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };
    let x1 = short_side - 1;
    let y1 = short_side - 1;
    let x2 = MAP_GRIDS_WIDTH  - short_side;
    let y2 = MAP_GRIDS_HEIGHT - short_side;

    let mut grid = IVec2::new( 0, 0 );
    loop
    {   grid.x = map.rng.gen_range( x1..=x2 );
        grid.y = map.rng.gen_range( y1..=y2 );
        if map.is_passage( grid ) { break }
    }
    let vec2 = grid.to_sprite_pixels() + ADJUSTER_MAP_SPRITES;
    let translation = vec2.extend( DEPTH_SPRITE_PLAYER );

    //プレイヤーのスプライトを配置する
    let player = Player
    {   grid,
        next         : grid,
        px_start     : vec2,
        px_end       : vec2,
        opt_autodrive: Some ( title_demo::algorithm::which_way_player_goes ), //default()に任せるとNone
        ..default()
    };
    let triangle = MaterialMesh2dBundle
    {   mesh: meshes.add( shape::RegularPolygon::new( PIXELS_PER_GRID * MAGNIFY_SPRITE_PLAYER, 3 ).into() ).into(),
        material: materials.add( ColorMaterial::from( COLOR_SPRITE_PLAYER ) ),
        ..default()
    };
    cmds.spawn( ( triangle, player ) )
    .insert( Transform::from_translation( translation ) )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//プレイヤーを移動させる
#[allow(clippy::too_many_arguments)]
pub fn move_sprite
(   mut qry_player: Query<( &mut Player, &mut Transform )>,
    qry_chasers: Query<&Chaser>,
    opt_map: Option<Res<Map>>,
    opt_demo: Option<Res<DemoMapParams>>,
    state: ResMut<State<MyState>>,
    mut evt_clear: EventReader<EventClear>,
    mut evt_over: EventReader<EventOver>,
    time: Res<Time>,
    cross: Res<input::CrossDirection>,
)
{   let Ok ( ( mut player, mut transform ) ) = qry_player.get_single_mut() else { return };
    let Some ( map ) = opt_map else { return };
    
    //直前の判定でクリア／オーバーしていたらスプライトを移動させない
    if evt_clear.iter().next().is_some() { return }
    if evt_over .iter().next().is_some() { return }

    //前回からの経過時間✕スピードアップ係数
    let time_delta = time.delta().mul_f32( player.speedup );

    //待ち時間が完了したら
    if player.wait.tick( time_delta ).finished()
    {   //スプライトの表示位置をグリッドにそろえる
        if player.px_start != player.px_end
        {   player.px_start = player.px_end;
            player.px_end   = player.next.to_sprite_pixels() + ADJUSTER_MAP_SPRITES;
            transform.translation = player.px_end.extend( DEPTH_SPRITE_PLAYER );
        }

        //自機の進行方向を決める
        let mut new_side = player.side;
        player.stop = true; //停止フラグを立てる

        if ! state.get().is_demoplay() //demoでないなら
        {   let sides = cross.sides();
            for &side in sides
            {   //道なら
                if map.is_passage( player.next + side )
                {   new_side = side;
                    player.stop = false;
                    break;
                }

                //道ではない場合でも向きは変える
                if side == sides[ 0 ]
                {   new_side = side;
                }
            }
        }
        else
        {   //demoの場合
            let mut sides = map.get_byways_list( player.next );         //脇道のリスト
            sides.retain( | side | player.next + side != player.grid ); //戻り路を排除

            //demoなのでプレイヤーのキー入力を詐称する
            player.stop = false;
            new_side = match sides.len().cmp( &1 )
            {   Ordering::Equal => //一本道 ⇒ 道なりに進む
                    sides[ 0 ],
                Ordering::Greater => //三叉路または十字路
                    if let ( Some ( autodrive ), Some ( demo ) ) = ( player.opt_autodrive, opt_demo )
                    {   autodrive( &player, qry_chasers, map, demo, &sides ) //外部関数で進行方向を決める
                    }
                    else
                    {   let mut rng = rand::thread_rng();
                        sides[ rng.gen_range( 0..sides.len() ) ] //外部関数がない(None)なら乱数で決める
                    },
                Ordering::Less => //行き止まり ⇒ 逆走 (このゲームに行き止まりはないけど)
                    match player.side
                    {   News::North    => News::South ,
                        News::South  => News::North   ,
                        News::East => News::West ,
                        News::West  => News::East,
                    },
            };
        }

        //自機の向きが変化したらスプライトを回転させる
        if player.side != new_side
        {   rotate_player_sprite( &player, &mut transform, new_side );
            player.side = new_side;
        }

        //現在の位置と次の位置を更新する
        player.grid = player.next;
        if ! player.stop { player.next += new_side; }

        //waitをリセットする
        player.wait.reset();
    }
    else if ! player.stop
    {   //移動中ならスプライトを中割の位置に移動する
        let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
        match player.side
        {   News::North => transform.translation.y += delta,
            News::South => transform.translation.y -= delta,
            News::East  => transform.translation.x += delta,
            News::West  => transform.translation.x -= delta,
        }
        player.px_start = player.px_end;
        player.px_end   = transform.translation.truncate();
    }
}

//自機の向きとキー入力から角度の差分を求めてスプライトを回転させる
fn rotate_player_sprite
(   player: &Player,
    transform: &mut Mut<Transform>,
    input: News
)
{   let angle: f32 = match player.side
    {   News::North =>
                 if input == News::West {  90.0 }
            else if input == News::East { -90.0 }
            else                        { 180.0 },
        News::South =>
                 if input == News::East {  90.0 }
            else if input == News::West { -90.0 }
            else                        { 180.0 },
        News::East =>
                 if input == News::North {  90.0 }
            else if input == News::South { -90.0 }
            else                         { 180.0 },
        News::West =>
                 if input == News::South {  90.0 }
            else if input == News::North { -90.0 }
            else                         { 180.0 },
    };

    let quat = Quat::from_rotation_z( angle.to_radians() );
    transform.rotate( quat );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.