use super::*;

////////////////////////////////////////////////////////////////////////////////

//自キャラのComponent
#[derive( Component )]
pub struct Player
{   pub grid     : IVec2, //移動中は移動元の座標、停止中はその場の座標
    pub next_grid: IVec2, //移動中は移動先の座標、停止中はその場の座標
    pub direction: News,  //移動の向き
    pub timer    : Timer, //移動のタイマー
    pub is_stop  : bool,  //移動停止フラグ
    pub speedup  : f32,   //スピードアップ係数
    pub px_start : Vec2,  //1フレーム時間に移動した微小区間の始点
    pub px_end   : Vec2,  //1フレーム時間に移動した微小区間の終点
    pub opt_fn_autodrive: Option<FnAutoDrive>, //デモ時に自キャラの移動方向を決める関数
    pub anime_timer: Timer,                         //キャラアニメーションのタイマー
    pub sprite_sheet_frame: usize,                  //キャラアニメーションのフレーム数
    pub sprite_sheet_indexes: HashMap<News, usize>, //キャラアニメーションの先頭位置(offset値)
}

impl Default for Player
{   fn default() -> Self
    {   Self
        {   grid     : IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer    : Timer::from_seconds( PLAYER_TIME_PER_GRID, TimerMode::Once ),
            is_stop  : true,
            speedup  : 1.0,
            px_start : Vec2::default(),
            px_end   : Vec2::default(),
            opt_fn_autodrive: None,
            anime_timer: Timer::from_seconds( ANIME_TIMER_PLAYER, TimerMode::Repeating ),
            sprite_sheet_frame: SPRITE_SHEET_COLS_PLAYER,
            sprite_sheet_indexes: ( *SPRITE_SHEET_IDXS_PLAYER ).clone(),
        }
    }
}

//関数ポインタ型(デモ時の自走自キャラの移動方向を決める関数)
type FnAutoDrive = fn( &Player, Query<&chasers::Chaser>, Res<map::Map>, Res<demo::schedule::DemoMapParams>, &[News] ) -> News;

//自キャラの三角スプライトのComponent
#[derive( Component )]
pub struct PlayerTriangle;

//自キャラの入力を保存するResource
#[derive( Resource )]
pub struct InputDirection ( Vec<News> );

impl Default for InputDirection
{   fn default() -> Self
    {   Self ( Vec::with_capacity( 4 ) ) //十字方向
    }
}

//スプライトシートでアニメーションするためのトレイト実装
impl CharacterAnimation for Player
{   fn anime_timer_mut( &mut self ) -> &mut Timer
    {   &mut self.anime_timer
    }
    fn sprite_sheet_frame( &self ) -> usize
    {   self.sprite_sheet_frame
    }
    fn sprite_sheet_offset( &self, news: News ) -> usize
    {   *self.sprite_sheet_indexes.get( &news ).unwrap()
    }
    fn direction( &self ) -> News
    {   self.direction
    }
}

////////////////////////////////////////////////////////////////////////////////

//自キャラの設定値
pub const PLAYER_TIME_PER_GRID: f32 = 0.15;//0.09; //１グリッド進むために必要な時間
const PLAYER_SPEED: f32 = PIXELS_PER_GRID / PLAYER_TIME_PER_GRID; //速度
const PLAYER_SPRITE_SCALING: f32 = 0.4; //primitive shape表示時の縮小係数
const PLAYER_SPRITE_COLOR: Color = Color::YELLOW;

//スプライトシートを使ったアニメーションの情報
pub const  SPRITE_SHEET_SIZE_PLAYER: Vec2 = Vec2::new( 8.0, 8.0 );
pub const  SPRITE_SHEET_COLS_PLAYER: usize = 4;
pub const  SPRITE_SHEET_ROWS_PLAYER: usize = 4;
pub static SPRITE_SHEET_IDXS_PLAYER: Lazy<HashMap<News,usize>> = Lazy::new
(   ||
    HashMap::from
    (   [   ( News::North,  0 ),
            ( News::East ,  4 ),
            ( News::West ,  8 ),
            ( News::South, 12 ),
        ]
    )
);
pub const ANIME_TIMER_PLAYER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

//自キャラをspawnする
pub fn spawn_sprite
(   qry_player: Query<Entity, With<Player>>,
    opt_map: Option<ResMut<map::Map>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut texture_atlases_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{   let Some ( mut map ) = opt_map else { return };

    //スプライトがあれば削除する
    qry_player.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );

    //乱数で初期位置を決める(マップ中央付近の通路)
    let half_w = map::MAP_GRIDS_WIDTH  / 2;
    let half_h = map::MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };
    let x1 = short_side - 1;
    let y1 = short_side - 1;
    let x2 = map::MAP_GRIDS_WIDTH  - short_side;
    let y2 = map::MAP_GRIDS_HEIGHT - short_side;

    let mut player_grid = IVec2::new( 0, 0 );
    loop
    {   player_grid.x = map.rng.gen_range( x1..=x2 );
        player_grid.y = map.rng.gen_range( y1..=y2 );
        if map.is_space( player_grid ) { break }
    }
    let vec2 = player_grid.to_vec2_on_game_map();
    let translation = vec2.extend( DEPTH_SPRITE_PLAYER );

    //Componentを初期化する
    let player = Player
    {   grid     : player_grid,
        next_grid: player_grid,
        px_start : vec2,
        px_end   : vec2,
        opt_fn_autodrive: Some ( demo::auto_drive::choice_way ), //default()に任せるとNone
        ..default()
    };

    if SPRITE_SHEET_OFF()
    {   //三角形のメッシュを作る
        let radius = PIXELS_PER_GRID * PLAYER_SPRITE_SCALING;
        let shape = RegularPolygon::new( radius, 3 ).mesh();
        let triangle = MaterialMesh2dBundle
        {   mesh: meshes.add( shape ).into(),
            material: materials.add( PLAYER_SPRITE_COLOR ),
            ..default()
        };
        let quat = Quat::from_rotation_z( PI ); //News::South
        let triangle =
            cmds.spawn( ( triangle, PlayerTriangle ) )
            .insert( Transform::from_rotation( quat ) )
            .id()
            ;

        //三角形を不可視ルートノードの子にする
        cmds.spawn( ( PbrBundle::default(), player ) )
        .insert( Transform::from_translation( translation ) )
        .insert( TextureAtlas::default() ) //move_sprite()のqry_playerの検索条件を満たすためのdummy
        .push_children( &[ triangle ] )
        ;
    }
    else
    {   //アニメーションするスプライトをspawnする
        let custom_size = Some( GRID_CUSTOM_SIZE );
        let layout = texture_atlases_layout.add
        (   TextureAtlasLayout::from_grid
            (   SPRITE_SHEET_SIZE_PLAYER,
                SPRITE_SHEET_COLS_PLAYER,
                SPRITE_SHEET_ROWS_PLAYER,
                None, None
            )
        );
        let index = player.sprite_sheet_offset( player.direction() );
        cmds.spawn( ( SpriteSheetBundle::default(), player ) )
        .insert( Sprite { custom_size, ..default() } )
        .insert( asset_svr.load( ASSETS_SPRITE_SHEET_PLAYER ) as Handle<Image> )
        .insert( TextureAtlas { layout, index } )
        .insert( Transform::from_translation( translation ) )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//自キャラを移動させる
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn move_sprite
(   mut qry_player: Query<( &mut Player, &mut TextureAtlas )>,
    mut pst_transform: ParamSet
    <(  Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<PlayerTriangle>>,
    )>,
    opt_map: Option<Res<map::Map>>,
    opt_input_direction: Option<Res<InputDirection>>,
    opt_demo: Option<Res<demo::schedule::DemoMapParams>>,
    qry_chasers: Query<&chasers::Chaser>,
    state: ResMut<State<MyState>>,
    mut evt_timer: EventWriter<EventTimerPlayer>,
    time: Res<Time>,
)
{   let Ok ( ( mut player, mut sprite_sheet ) ) = qry_player.get_single_mut() else { return };
    let mut qry_transform = pst_transform.p0();
    let Ok ( mut transform ) = qry_transform.get_single_mut() else { return };
    let Some ( map ) = opt_map else { return };
    let Some ( input_direction ) = opt_input_direction else { return };

    //前回からの経過時間にスピードアップ係数をかける
    let time_delta = time.delta().mul_f32( player.speedup );

    //グリッドのマス間を移動中か？
    if ! player.timer.tick( time_delta ).finished()
    {   if ! player.is_stop
        {   //移動中の中割座標
            let delta = PLAYER_SPEED * time_delta.as_secs_f32();
            match player.direction
            {   News::North => transform.translation.y += delta,
                News::South => transform.translation.y -= delta,
                News::East  => transform.translation.x += delta,
                News::West  => transform.translation.x -= delta,
            }
            player.px_start = player.px_end;
            player.px_end   = transform.translation.truncate();
        }
    }
    else
    {   evt_timer.send( EventTimerPlayer ); //後続の処理にtimer finishedを伝達する

        //スプライトをグリッドに配置する
        if player.px_start != player.px_end
        {   player.px_start = player.px_end;
            player.px_end   = player.next_grid.to_vec2_on_game_map();
            transform.translation = player.px_end.extend( DEPTH_SPRITE_PLAYER );
        }

        //自キャラが次に進む方向を決める
        let mut new_side = player.direction;
        player.is_stop = true; //停止フラグを立てておく

        if ! state.get().is_demoplay()
        {   //入力に対応する
            for side in input_direction.0.iter() //input_direction.0は優先順に並んでいる前提
            {   //壁でない場合
                if map.is_space( player.next_grid + side )
                {   new_side = *side;
                    player.is_stop = false;
                    break;
                }

                //ループの先頭要素では、向きを必ず変える
                if *side == ( input_direction.0 )[ 0 ] //データの性質上 値が重複する要素はない
                {   new_side = *side;
                }
            }
        }
        else
        {   //demoの場合 入力相当のデータをアルゴリズムで作る
            player.is_stop = false; //demoでは自機は停止しない

            let mut sides = map.get_side_spaces_list( player.next_grid ); //脇道のリスト
            sides.retain( | side | player.next_grid + side != player.grid ); //戻り路を取り除く

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

        //進行方向が変わったらスプライトの見栄えを変える
        if player.direction != new_side
        {   if SPRITE_SHEET_OFF()
            {   //三角形を回転させる
                if let Ok ( mut transform ) = pst_transform.p1().get_single_mut()
                {   rotate_player_sprite( &player, &mut transform, new_side );
                }
            }
            else
            {   //スプライトシートのindexを変更する
                let old_offset = player.sprite_sheet_offset( player.direction );
                let new_offset = player.sprite_sheet_offset( new_side         );
                sprite_sheet.index = sprite_sheet.index - old_offset + new_offset;
            }
            player.direction = new_side;
        }

        //現在の位置と次の位置を更新する
        player.grid = player.next_grid;
        if ! player.is_stop { player.next_grid += new_side; }

        //タイマーをリセットする
        player.timer.reset();
    }
}

//自機の向きと入力から角度の差分を求めてスプライトを回転させる
fn rotate_player_sprite
(   player: &Player,
    transform: &mut Mut<Transform>,
    input: News
)
{   let angle: f32 = match player.direction
    {   News::North => match input
        {   News::West => PI /  2.0,
            News::East => PI / -2.0,
            _ => PI,
        }
        News::South => match input
        {   News::East => PI /  2.0,
            News::West => PI / -2.0,
            _  => PI,
        }
        News::East => match input
        {   News::North => PI /  2.0,
            News::South => PI / -2.0,
            _ => PI,
        }
        News::West => match input
        {   News::South => PI /  2.0,
            News::North => PI / -2.0,
            _ => PI,
        }
    };

    let quat = Quat::from_rotation_z( angle );
    transform.rotate( quat );
}

////////////////////////////////////////////////////////////////////////////////

//自キャラの入力を捕まえる
pub fn catch_input_direction
(   qry_player: Query<&player::Player>,
    opt_input_direction: Option<ResMut<InputDirection>>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    input_gamepad: Res<ButtonInput<GamepadButton>>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };
    let Some ( mut input_direction ) = opt_input_direction else { return };

    //初期化
    input_direction.0.clear();
    let mut pressed_news = HashSet::new();

    //ゲームパッドが接続されているか
    if let Some ( gamepad ) = opt_gamepad
    {   if let Some ( target_id ) = gamepad.id()
        {   //ゲームパッドの入力をチェックする
            pressed_news = input_gamepad
            .get_pressed()
            .filter_map
            (   | x |
                if x.gamepad != target_id
                { None } //ゲームパッドは複数接続できるので、id不一致なら無視
                else
                {   match x.button_type
                    {   GamepadButtonType::DPadUp    => Some ( News::North ),
                        GamepadButtonType::DPadRight => Some ( News::East  ),
                        GamepadButtonType::DPadLeft  => Some ( News::West  ),
                        GamepadButtonType::DPadDown  => Some ( News::South ),
                        _ => None,
                    }
                }
            )
            .collect();
        }
    }

    //ゲームパッドの入力がないならキー入力をチェックする
    if pressed_news.is_empty()
    {   pressed_news = input_keyboard
        .get_pressed()
        .filter_map
        (   | keycode |
            match keycode
            {   KeyCode::ArrowUp    => Some ( News::North ),
                KeyCode::ArrowRight => Some ( News::East  ),
                KeyCode::ArrowLeft  => Some ( News::West  ),
                KeyCode::ArrowDown  => Some ( News::South ),
                _ => None,
            }
        )
        .collect();
    }

    //要素数０～１なら
    if pressed_news.is_empty() { return }
    if pressed_news.len() == 1
    {   input_direction.0.push( *pressed_news.iter().next().unwrap() );
        return;
    }

    //取得した入力と自キャラの向きから、前進・後進入力があったか調べる
    //.take()するので、pressed_newsは右折・左折の入力だけが（あれば）残る
    let opt_front = pressed_news.take( &player.direction );
    let opt_back  = pressed_news.take( &player.direction.back_side() );

    //優先する方向を考慮して入力をVecにまとめる
    if let Some ( back ) = opt_back { input_direction.0.push( back ); }
    input_direction.0.extend( pressed_news.iter() );
    if let Some ( front ) = opt_front { input_direction.0.push( front ); }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.