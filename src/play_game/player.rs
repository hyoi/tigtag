use super::*;

////////////////////////////////////////////////////////////////////////////////

//プレイヤーをspawnする
pub fn spawn_sprite
(   qry_player: Query<Entity, With<Player>>,
    opt_map: Option<ResMut<Map>>,
    mut cmds: Commands,
    // asset_svr: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{   let Some ( mut map ) = opt_map else { return };

    //スプライトがあれば削除する
    qry_player.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );

    //乱数で初期位置を決める(マップ中央付近の通路)
    let half_w = MAP_GRIDS_WIDTH  / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };
    let x1 = short_side - 1;
    let y1 = short_side - 1;
    let x2 = MAP_GRIDS_WIDTH  - short_side;
    let y2 = MAP_GRIDS_HEIGHT - short_side;

    let mut player_grid = IVec2::new( 0, 0 );
    loop
    {   player_grid.x = map.rng.gen_range( x1..=x2 );
        player_grid.y = map.rng.gen_range( y1..=y2 );
        if map.is_space( player_grid ) { break }
    }
    let sprite_vec2 = player_grid.to_vec2_on_map();
    let translation = sprite_vec2.extend( DEPTH_SPRITE_PLAYER );

    //プレイヤーのスプライトを配置する
    let player = Player
    {   grid     : player_grid,
        next_grid: player_grid,
        dx_start : sprite_vec2,
        dx_end   : sprite_vec2,
//      opt_fn_autodrive: Some ( title_demo::auto_drive::choice_way ), //default()に任せるとNone
        ..default()
    };

    // //アニメーションするスプライトをspawnする
    // let texture_atlas = asset_svr.gen_texture_atlas_player();
    // let texture_atlas_hdl = texture_atlases.add( texture_atlas );
    // let custom_size = Some( SIZE_GRID );
    // let index = player.sprite_sheet_offset( player.direction() );
    // let texture_atlas_sprite = TextureAtlasSprite { custom_size, index, ..default() };
    // cmds.spawn( ( SpriteSheetBundle::default(), player ) )
    // .insert( texture_atlas_hdl )
    // .insert( texture_atlas_sprite )
    // .insert( Transform::from_translation( translation ) )
    // ;

    //三角形のメッシュ
    let radius = PIXELS_PER_GRID * _MAGNIFY_SPRITE_PLAYER;
    let shape = shape::RegularPolygon::new( radius, 3 );
    let triangle = MaterialMesh2dBundle
    {   mesh: meshes.add( shape ).into(),
        material: materials.add( _COLOR_SPRITE_PLAYER ),
        ..default()
    };
    let quat = Quat::from_rotation_z( PI ); //News::South
    cmds.spawn( ( triangle, player ) )
    .insert( Transform::from_translation( translation ).with_rotation( quat ) )
    // .insert( TextureAtlasSprite::default() ) //move_sprite()のqry_playerの検索条件を満たすためのdummy
    ;
}

////////////////////////////////////////////////////////////////////////////////

//プレイヤーを移動させる
#[allow(clippy::too_many_arguments)]
pub fn move_sprite
(   mut qry_player: Query<( &mut Player, &mut Transform, /*&mut TextureAtlasSprite*/ )>,
    opt_input: Option<Res<input::CrossDirection>>,
    opt_map: Option<Res<Map>>,
    opt_demo: Option<Res<DemoMapParams>>,
    qry_chasers: Query<&Chaser>,
    state: ResMut<State<MyState>>,
    mut evt_clear: EventReader<EventClear>,
    mut evt_over: EventReader<EventOver>,
    time: Res<Time>,
)
{   let Ok ( ( mut player, mut transform, /*mut sprite*/ ) ) = qry_player.get_single_mut() else { return };
    let Some ( input ) = opt_input else { return };
    let Some ( map ) = opt_map else { return };

    //直前の判定でクリア／オーバーしていたらスプライトを移動させない
    if evt_clear.read().next().is_some() { return }
    if evt_over .read().next().is_some() { return }

    //前回からの経過時間にスピードアップ係数をかける
    let time_delta = time.delta().mul_f32( player.speedup );

    //タイマーが完了したら
    if player.timer.tick( time_delta ).finished()
    {   //スプライトをplayer.next_gridに配置する
        if player.dx_start != player.dx_end
        {   player.dx_start = player.dx_end;
            player.dx_end   = player.next_grid.to_vec2_on_map();
            transform.translation = player.dx_end.extend( DEPTH_SPRITE_PLAYER );
        }

        //プレイヤーが次に進む方向を決める
        let mut new_side = player.direction;
        player.is_stop = true; //停止フラグを立てる

        if ! state.get().is_demoplay()
        {   //demoではない場合、プレイヤーの十字方向の入力に対応する
            for &side in input.direction() //入力の要素数は０～２
            {   //壁でない場合
                if map.is_space( player.next_grid + side )
                {   new_side = side;
                    player.is_stop = false;
                    break;
                }

                //要素１つ目なら壁でも向きだけは変える
                if side == input.direction()[ 0 ]
                {   new_side = side;
                }
            }
        }
        else
        {   //demoの場合
            let mut sides = map.get_side_spaces_list( player.next_grid ); //脇道のリスト
            sides.retain( | side | player.next_grid + side != player.grid ); //戻り路を取り除く

            //demoなので自動運転する
            player.is_stop = false;
            new_side = match sides.len().cmp( &1 )
            {   Ordering::Equal => //一本道 ⇒ 道なりに進む
                    sides[ 0 ],
                Ordering::Greater => //三叉路または十字路
                    if let ( Some ( autodrive ), Some ( demo ) ) = ( player.opt_fn_autodrive, opt_demo )
                    {   //外部関数で進行方向を決める
                        autodrive( &player, qry_chasers, map, demo, &sides )
                    }
                    else
                    {   //外部関数を使えないなら乱数で決める
                        let mut rng = rand::thread_rng();
                        sides[ rng.gen_range( 0..sides.len() ) ]
                    },
                Ordering::Less => //行き止まり ⇒ 逆走 (このゲームに行き止まりはないけど)
                    match player.direction
                    {   News::North => News::South,
                        News::South => News::North,
                        News::East  => News::West ,
                        News::West  => News::East ,
                    },
            };
        }

        //プレイヤーの進む向きが変わったらスプライトを回転させる
        if player.direction != new_side
        {   // if sprite.custom_size.is_some()
            // {   let old_offset = player.sprite_sheet_offset( player.direction );
            //     let new_offset = player.sprite_sheet_offset( new_side         );
            //     sprite.index = sprite.index - old_offset + new_offset;
            // }
            // else
            {   rotate_player_sprite( &player, &mut transform, new_side );
            }
            player.direction = new_side;
        }

        //現在の位置と次の位置を更新する
        player.grid = player.next_grid;
        if ! player.is_stop { player.next_grid += new_side; }

        //タイマーをリセットする
        player.timer.reset();
    }
    else if ! player.is_stop
    {   //移動中の中割アニメーション
        let delta = PLAYER_SPEED * time_delta.as_secs_f32();
        match player.direction
        {   News::North => transform.translation.y += delta,
            News::South => transform.translation.y -= delta,
            News::East  => transform.translation.x += delta,
            News::West  => transform.translation.x -= delta,
        }
        player.dx_start = player.dx_end;
        player.dx_end   = transform.translation.truncate();
    }
}

//自機の向きとキー入力から角度の差分を求めてスプライトを回転させる
fn rotate_player_sprite
(   player: &Player,
    transform: &mut Mut<Transform>,
    input: News
)
{   let angle: f32 = match player.direction
    {   News::North =>
                 if input == News::West { PI /  2.0 }
            else if input == News::East { PI / -2.0 }
            else                        { PI },
        News::South =>
                 if input == News::East { PI /  2.0 }
            else if input == News::West { PI / -2.0 }
            else                        { PI },
        News::East =>
                 if input == News::North { PI /  2.0 }
            else if input == News::South { PI / -2.0 }
            else                         { PI },
        News::West =>
                 if input == News::South { PI /  2.0 }
            else if input == News::North { PI / -2.0 }
            else                         { PI },
    };

    let quat = Quat::from_rotation_z( angle );
    transform.rotate( quat );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.