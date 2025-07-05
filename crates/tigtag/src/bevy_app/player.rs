use super::*;

////////////////////////////////////////////////////////////////////////////////

// 自キャラの三角スプライトのComponent
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

// 極座標の型
// #[derive(Default, Clone)]
// pub struct Orbit
// {
//     pub r: f32,     // 極座標のr（中心点から飛翔体までの距離）
//     pub theta: f32, // 極座標のΘ（中心点から見た飛翔体の仰角）
//     pub phi: f32,   // 極座標のφ（中心点から見た飛翔体の平面の回転角）
// }

// 極座標カメラのResource
// #[derive(Resource, Default, Clone)]
// pub struct OrbitCamera
// {
//     pub position: Orbit,         // 極座標上のカメラの位置
//     pub lock: LockFlag,          // スピードバグ防止フラグ
// }

// 自キャラの入力を保存するResource
#[derive(Resource)]
pub struct InputDirection(Vec<News>, LockFlag);

impl Default for InputDirection
{
    fn default() -> Self
    {
        Self(Vec::with_capacity(4), LockFlag::default()) // 十字方向
    }
}

// スピードバグ防止フラグ
#[derive(Default, Clone)]
pub struct LockFlag
{
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

// コールバック関数の型
pub type CallBack = fn(&mut InputDirection, f32);

// コールバック関数
pub use callback::*;

#[allow( dead_code )]
#[rustfmt::skip]
pub mod callback
{   use super::*;

    pub fn move_up( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.up || value == 0.0 { return; }
        // orbit.position.theta += value;
        orbit.0.push( News::North );
        orbit.1.up = true;
    }
    pub fn move_down( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.down || value == 0.0 { return; }
        // orbit.position.theta -= value;
        orbit.0.push( News::South );
        orbit.1.down = true;
    }
    pub fn move_right( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.right || value == 0.0 { return; }
        // orbit.position.phi += value;
        orbit.0.push( News::East );
        orbit.1.right = true;
    }
    pub fn move_left( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.left || value == 0.0 { return; }
        // orbit.position.phi -= value;
        orbit.0.push( News::West );
        orbit.1.left = true;
    }

    pub fn axis_x_normal( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.right )
            || ( value < 0.0 && orbit.1.left  ) { return; }
        // orbit.position.phi += value;
        if value > 0.0
        {
            orbit.0.push( News::East );
            orbit.1.right = true;
        } else
        {
            orbit.0.push( News::West );
            orbit.1.left = true;
        }
    }
    pub fn axis_x_reverse( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.left  )
            || ( value < 0.0 && orbit.1.right ) { return; }
        // orbit.position.phi += - value;
        if value >= 0.0
        {
            orbit.0.push( News::West );
            orbit.1.left = true;
        } else
        {
            orbit.0.push( News::East );
            orbit.1.right = true;
        }
    }
    pub fn axis_y_normal( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.up   )
            || ( value < 0.0 && orbit.1.down ) { return; }
        // orbit.position.theta += value;
        if value >= 0.0
        {
            orbit.0.push( News::North );
            orbit.1.up = true;
        } else
        {
            orbit.0.push( News::South );
            orbit.1.down = true;
        }
    }
    pub fn axis_y_reverse( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.down )
            || ( value < 0.0 && orbit.1.up   ) { return; }
        // orbit.position.theta += - value;
        if value >= 0.0
        {
            orbit.0.push( News::South );
            orbit.1.down = true;
        } else
        {
            orbit.0.push( News::North );
            orbit.1.up = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 自キャラを移動させる
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

        // 自キャラが次に進む方向を決める
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

// キーマップ登録用Resource
#[derive(Resource, Deref)]
pub struct KeyMap(pub FxHashMap<KeyCode, player::CallBack>);

// ゲームパッドボタンマップ登録用Resource
#[derive(Resource, Deref)]
pub struct PadMap(pub FxHashMap<GamepadInput, player::CallBack>);

////////////////////////////////////////////////////////////////////////////////

// 極座標カメラの位置をキー入力で操作
pub fn input_from_keyboard(
    // opt_orbit_camera: Option<ResMut<OrbitCamera>>,
    opt_input_direction: Option<ResMut<InputDirection>>,
    opt_keymap: Option<Res<KeyMap>>,
    time: Res<Time>,
    input_keycode: Res<ButtonInput<KeyCode>>,
) -> Result
{
    // 準備
    let mut camera =
        opt_input_direction.ok_or("Resource InputDirection not found.")?;
    let keymap = opt_keymap.ok_or("opt_keymap is None.")?;

    // 前回の実行からの経過時間（感度調整の係数あり）
    let time_delta = time.delta_secs() /* * COEF_KEY_TIME_DELTA */;

    // キー入力とキーマップを使って極座標を更新する
    input_keycode.get_pressed().for_each(|keycode| {
        if let Some(callback) = keymap.get(keycode)
        {
            callback(&mut camera, time_delta);
        }
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// 自キャラの入力を捕まえる
// pub fn catch_input_direction
// (   qry_player: Query<&player::Player>,
//     opt_input_direction: Option<ResMut<InputDirection>>,
//     opt_gamepad: Option<Res<TargetGamepad>>,
//     input_gamepad: Res<ButtonInput<GamepadButton>>,
//     input_keyboard: Res<ButtonInput<KeyCode>>,
// )
// {   let Ok ( player ) = qry_player.get_single() else { return };
//     let Some ( mut input_direction ) = opt_input_direction else { return };

//     //初期化
//     input_direction.0.clear();
//     let mut pressed_news = HashSet::new();

//     //ゲームパッドが接続されているか
//     if let Some ( gamepad ) = opt_gamepad
//     {   if let Some ( target_id ) = gamepad.id()
//         {   //ゲームパッドの入力をチェックする
//             pressed_news = input_gamepad
//             .get_pressed()
//             .filter_map
//             (   | x |
//                 if x.gamepad != target_id
//                 { None } //ゲームパッドは複数接続できるので、id不一致なら無視
//                 else
//                 {   match x.button_type
//                     {   GamepadButtonType::DPadUp    => Some ( News::North ),
//                         GamepadButtonType::DPadRight => Some ( News::East  ),
//                         GamepadButtonType::DPadLeft  => Some ( News::West  ),
//                         GamepadButtonType::DPadDown  => Some ( News::South ),
//                         _ => None,
//                     }
//                 }
//             )
//             .collect();
//         }
//     }

//     //ゲームパッドの入力がないならキー入力をチェックする
//     if pressed_news.is_empty()
//     {   pressed_news = input_keyboard
//         .get_pressed()
//         .filter_map
//         (   | keycode |
//             match keycode
//             {   KeyCode::ArrowUp    => Some ( News::North ),
//                 KeyCode::ArrowRight => Some ( News::East  ),
//                 KeyCode::ArrowLeft  => Some ( News::West  ),
//                 KeyCode::ArrowDown  => Some ( News::South ),
//                 _ => None,
//             }
//         )
//         .collect();
//     }

//     //要素数０～１なら
//     if pressed_news.is_empty() { return }
//     if pressed_news.len() == 1
//     {   input_direction.0.push( *pressed_news.iter().next().unwrap() );
//         return;
//     }

//     //取得した入力と自キャラの向きから、前進・後進入力があったか調べる
//     //.take()するので、pressed_newsは右折・左折の入力だけが（あれば）残る
//     let opt_front = pressed_news.take( &player.direction );
//     let opt_back  = pressed_news.take( &player.direction.back_side() );

//     //優先する方向を考慮して入力をVecにまとめる
//     if let Some ( back ) = opt_back { input_direction.0.push( back ); }
//     input_direction.0.extend( pressed_news.iter() );
//     if let Some ( front ) = opt_front { input_direction.0.push( front ); }
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
