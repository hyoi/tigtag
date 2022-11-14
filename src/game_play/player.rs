use super::*;

//submodules
mod demo_algorithm;

//スプライトをspawnして自機を表示する
pub fn spawn_sprite
(   q: Query<Entity, With<Player>>,
    mut map: ResMut<Map>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{   //スプライトがあれば削除する
    q.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //乱数で自機の初期位置を決める(マップ中央付近の通路)
    let half_w = MAP_GRIDS_WIDTH  / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };
    let x1 = short_side - 1;
    let y1 = short_side - 1;
    let x2 = MAP_GRIDS_WIDTH  - short_side;
    let y2 = MAP_GRIDS_HEIGHT - short_side;

    let mut grid = Grid::new( 0, 0 );
    loop
    {   grid.x = map.rng.gen_range( x1..=x2 );
        grid.y = map.rng.gen_range( y1..=y2 );
        if map.is_passage( grid ) { break }
    }
    let pixel = grid.into_pixel_map();

    //自機のスプライトを配置する
    let triangle = MaterialMesh2dBundle
    {   mesh: meshes.add( shape::RegularPolygon::new( PIXELS_PER_GRID * MAGNIFY_SPRITE_PLAYER, 3 ).into() ).into(),
        material: materials.add( ColorMaterial::from( COLOR_SPRITE_PLAYER ) ),
        ..default()
    };
    let player = Player
    {   grid,
        next    : grid,
        px_start: pixel,
        px_end  : pixel,
        fn_runaway: Some ( demo_algorithm::which_way_player_goes ), //default()に任せるとNone 
        ..default()
    };
    cmds
    .spawn( ( triangle, player ) )
    .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_PLAYER ) ) )
    ;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機の移動
pub fn move_sprite
(   ( mut q_player, q_chasers ): ( Query<(&mut Player, &mut Transform)>, Query<&Chaser> ),
    map: Res<Map>,
    state: ResMut<State<GameState>>,
    ( mut ev_clear, mut ev_over ): ( EventReader<EventClear>, EventReader<EventOver> ),
    ( inkey, time ): ( Res<Input<KeyCode>>, Res<Time> ),
)
{   //直前の判定でクリア／オーバーしていたらスプライトの表示を変更しない
    if ev_clear.iter().next().is_some() { return }
    if ev_over .iter().next().is_some() { return }

    let ( mut player, mut transform ) = q_player.get_single_mut().unwrap(); //プレイヤーの情報
    let time_delta = time.delta().mul_f32( player.speedup ); //前回からの経過時間×スピードアップ係数

    //待ち時間が完了したら
    if player.wait.tick( time_delta ).finished()
    {   //スプライトの表示位置をグリッドにそろえる
        if player.px_start != player.px_end
        {   player.px_start = player.px_end;
            player.px_end   = player.next.into_pixel_map();
            transform.translation = player.px_end.extend( DEPTH_SPRITE_PLAYER );
        }

        //自機の進行方向を決める
        player.stop = false;
        let mut side = player.side;
        if ! state.current().is_demoplay()
        {   //demoでなければプレイヤーのキー入力を確認(入力がなければ停止)
                 if inkey.pressed( KeyCode::Up    ) { side = DxDy::Up;    }
            else if inkey.pressed( KeyCode::Down  ) { side = DxDy::Down;  }
            else if inkey.pressed( KeyCode::Right ) { side = DxDy::Right; }
            else if inkey.pressed( KeyCode::Left  ) { side = DxDy::Left;  }
            else { player.stop = true }

            //キー入力があってもその向きに壁があれば停止
            if ! player.stop
            {   player.stop = map.is_wall( player.next + side )
            }
        }
        else
        {   //四方の脇道を取得する
            let mut sides = map.get_byways_list( player.next );         //脇道のリスト
            sides.retain( | side | player.next + side != player.grid ); //戻り路を排除

            //demoなのでプレイヤーのキー入力を詐称する
            use std::cmp::Ordering;
            side = match sides.len().cmp( &1 )
            {   Ordering::Equal => //一本道 ⇒ 道なりに進む
                    sides[ 0 ],
                Ordering::Greater => //三叉路または十字路
                    if let Some ( fnx ) = player.fn_runaway
                    {   fnx( &player, q_chasers, map, &sides ) //外部関数で進行方向を決める
                    }
                    else
                    {   let mut rng = rand::thread_rng();
                        sides[ rng.gen_range( 0..sides.len() ) ] //外部関数がない(None)なら乱数で決める
                    },
                Ordering::Less => //行き止まり ⇒ 逆走 (このゲームに行き止まりはないけど)
                    match player.side
                    {   DxDy::Up    => DxDy::Down ,
                        DxDy::Down  => DxDy::Up   ,
                        DxDy::Right => DxDy::Left ,
                        DxDy::Left  => DxDy::Right,
                    },
            };
        }

        //自機の向きが変化したらスプライトを回転させる
        if player.side != side
        {   rotate_player_sprite( &player, &mut transform, side );
            player.side = side;
        }

        //現在の位置と次の位置を更新する
        player.grid = player.next;
        if ! player.stop { player.next += side; }

        //waitをリセットする
        player.wait.reset();
    }
    else if ! player.stop
    {   //移動中ならスプライトを中割の位置に移動する
        let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
        match player.side
        {   DxDy::Up    => transform.translation.y += delta,
            DxDy::Down  => transform.translation.y -= delta,
            DxDy::Right => transform.translation.x += delta,
            DxDy::Left  => transform.translation.x -= delta,
        }
        player.px_start = player.px_end;
        player.px_end   = transform.translation.truncate();
    }
}

//自機の向きとキー入力から角度の差分を求めてスプライトを回転させる
fn rotate_player_sprite
(   player: &Player,
    transform: &mut Mut<Transform>,
    input: DxDy
)
{   let angle: f32 = match player.side
    {   DxDy::Up =>
                 if input == DxDy::Left  {  90.0 }
            else if input == DxDy::Right { -90.0 }
            else                         { 180.0 },
        DxDy::Down =>
                 if input == DxDy::Right {  90.0 }
            else if input == DxDy::Left  { -90.0 }
            else                         { 180.0 },
        DxDy::Right =>
                 if input == DxDy::Up    {  90.0 }
            else if input == DxDy::Down  { -90.0 }
            else                         { 180.0 },
        DxDy::Left =>
                 if input == DxDy::Down  {  90.0 }
            else if input == DxDy::Up    { -90.0 }
            else                         { 180.0 },
    };

    let quat = Quat::from_rotation_z( angle.to_radians() );
    transform.rotate( quat );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//スコアの処理とクリア判定
pub fn scoring_and_clear_stage
(   q1: Query<&Player>,
    mut _q2: Query<( &mut Text, &TextUiNumTile )>,
    mut record: ResMut<Record>,
    mut map: ResMut<Map>,
    mut state: ResMut<State<GameState>>,
    mut ev_clear: EventWriter<EventClear>,
    ( mut cmds, asset_svr, audio ): ( Commands, Res<AssetServer>, Res<Audio> ),
)
{   if let Ok ( player ) = q1.get_single()
    {   //自機の位置にドットがあるなら
        if let Some ( id ) = map.o_entity( player.grid )
        {   //スプライト削除
            cmds.entity( id ).despawn();
            *map.o_entity_mut( player.grid ) = None;

            //スコア更新
            record.score += 1;
            map.remaining_dots -= 1;
            audio.set_volume( VOLUME_SOUND_BEEP );
            audio.play( asset_svr.load( ASSETS_SOUND_BEEP ) );

            //ハイスコアの更新
            if ! state.current().is_demoplay() && record.score > record.hi_score
            {   record.hi_score = record.score;
            }

            //全ドットを拾ったら、Clearへ遷移する
            if map.remaining_dots <= 0
            {   let next =
                {   if state.current().is_demoplay()
                    {   record.demo.clear_flag = true;
                        GameState::DemoLoop
                    }
                    else
                    {   GameState::StageClear
                    }
                };
                let _ = state.overwrite_set( next );
                ev_clear.send( EventClear );    //後続の処理にクリアを伝える
            }
        }
    }
}

//End of code.