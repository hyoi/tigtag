use super::*;

////////////////////////////////////////////////////////////////////////////////

//敵キャラのComponent
#[derive( Component )]
pub struct Chaser
{   pub grid     : IVec2, //移動中は移動元の座標、停止中はその場の座標
    pub next_grid: IVec2, //移動中は移動先の座標、停止中はその場の座標
    pub direction: News,  //移動の向き
    pub timer    : Timer, //移動のタイマー
    pub is_stop  : bool,  //移動停止フラグ
    pub speedup  : f32,   //スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub px_start : Vec2,  //1フレーム時間に移動した微小区間の始点
    pub px_end   : Vec2,  //1フレーム時間に移動した微小区間の終点
    pub opt_fn_autochase: Option<FnAutoChase>, //敵キャラの移動方向を決める関数
    pub color    : Color, //敵キャラの表示色
    pub anime_timer: Timer,                       //アニメーションのタイマー
    pub sprite_sheet_frame: u32,                  //アニメーションのフレーム数
    pub sprite_sheet_indexes: HashMap<News, u32>, //アニメーションの先頭位置(offset値)
}

impl Default for Chaser
{   fn default() -> Self
    {   Self
        {   grid     : IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer    : Timer::from_seconds( CHASER_TIME_PER_GRID, TimerMode::Once ),
            is_stop  : true,
            speedup  : 1.0,
            px_start : Vec2::default(),
            px_end   : Vec2::default(),
            opt_fn_autochase: None,
            color    : Color::NONE,
            anime_timer: Timer::from_seconds( ANIME_TIMER_CHASER, TimerMode::Repeating ),
            sprite_sheet_frame: SPRITE_SHEET_COLS_CHASER,
            sprite_sheet_indexes: ( *SPRITE_SHEET_IDXS_CHASER ).clone(),
        }
    }
}

//関数ポインタ型(敵キャラの移動方向を決める関数)
type FnAutoChase = fn( &mut Chaser, &player::Player, &[News] ) -> News;

//スプライトシートでアニメーションするためのトレイト実装
impl CharacterAnimation for Chaser
{   fn anime_timer_mut( &mut self ) -> &mut Timer
    {   &mut self.anime_timer
    }
    fn sprite_sheet_frame( &self ) -> u32
    {   self.sprite_sheet_frame
    }
    fn sprite_sheet_offset( &self, news: News ) -> u32
    {   *self.sprite_sheet_indexes.get( &news ).unwrap()
    }
    fn direction( &self ) -> News
    {   self.direction
    }
}

////////////////////////////////////////////////////////////////////////////////

//敵キャラの設定値
pub const CHASER_TIME_PER_GRID: f32 = 0.20;//0.13; //１グリッド進むために必要な時間
const CHASER_SPEED: f32 = PIXELS_PER_GRID / CHASER_TIME_PER_GRID; //速度
const CHASER_SPRITE_SCALING: f32 = 0.5; //primitive shape表示時の縮小係数
const CHASER_ACCEL: f32 = 0.4; //スピードアップの割増
const CHASER_START_POSITION: &[ IVec2 ] = //スタート座標
&[  IVec2::new( 1    , 1     ),
    IVec2::new( 1    , MAX_Y ),
    IVec2::new( MAX_X, 1     ),
    IVec2::new( MAX_X, MAX_Y ),
];
const MAX_X: i32 = map::MAP_GRIDS_WIDTH  - 2;
const MAX_Y: i32 = map::MAP_GRIDS_HEIGHT - 2;

//スプライトシートを使ったアニメーションの情報
const  SPRITE_SHEET_SIZE_CHASER: UVec2 = UVec2::new( 8, 8 );
const  SPRITE_SHEET_COLS_CHASER: u32 = 4;
const  SPRITE_SHEET_ROWS_CHASER: u32 = 4;
static SPRITE_SHEET_IDXS_CHASER: Lazy<HashMap<News,u32>> = Lazy::new
(   ||
    HashMap::from
    (   [   ( News::North,  0 ),
            ( News::East ,  4 ),
            ( News::West ,  8 ),
            ( News::South, 12 ),
        ]
    )
);
const ANIME_TIMER_CHASER: f32 = 0.15;

//各色ごとの情報（色と移動方向の決定関数とassetファイル名）
const CHASERS_SPRITE_INFO: &[ ( Color, Option<FnAutoChase>, &str ) ] =
&[  ( Color::RED,   Some ( choice_way_red   ), ASSETS_SPRITE_SHEET_CHASER_RED   ),
    ( Color::GREEN, Some ( choice_way_green ), ASSETS_SPRITE_SHEET_CHASER_GREEN ),
    ( Color::PINK,  Some ( choice_way_pink  ), ASSETS_SPRITE_SHEET_CHASER_PINK  ),
    ( Color::BLUE,  Some ( choice_way_blue  ), ASSETS_SPRITE_SHEET_CHASER_BLUE  ),
];

////////////////////////////////////////////////////////////////////////////////

//敵キャラをspawnする
pub fn spawn_sprite
(   qry_chaser: Query<Entity, With<Chaser>>,
    opt_record: Option<Res<Record>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut texture_atlases_layout: ResMut<Assets<TextureAtlasLayout>>,
)
{   let Some ( record ) = opt_record else { return };

    //スプライトがあれば削除する
    qry_chaser.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );

    //敵キャラをマップの四隅に配置する
    ( 0.. ).zip( CHASER_START_POSITION ).for_each
    (   | ( i, start_grid ) |
        {   //ステージ数を4で割ったあまりをindex（0,1,2,3）にする
            let index = ( ( record.stage() - 1 + i ) % 4 ) as usize;
            let ( color, opt_fn_autochase, asset_file ) = CHASERS_SPRITE_INFO[ index ];

            //初期位置
            let vec2 = start_grid.to_vec2_on_game_map();
            let translation = vec2.extend( DEPTH_SPRITE_CHASER );

            //Componentを初期化する
            let chaser = Chaser
            {   grid     : *start_grid,
                next_grid: *start_grid,
                px_start : vec2,
                px_end   : vec2,
                color,
                opt_fn_autochase,
                ..default()
            };

            if SPRITE_SHEET_OFF()
            {   //正方形のメッシュ
                let custom_size = Some ( GRID_CUSTOM_SIZE * CHASER_SPRITE_SCALING );
                cmds.spawn( ( SpriteBundle::default(), chaser ) )
                .insert( Sprite { color, custom_size, ..default() } )
                .insert( Transform::from_translation( translation ) )
                .insert( TextureAtlas::default() ) //move_sprite()のqry_chaserの検索条件を満たすためのdummy
                ;
            }
            else
            {   //アニメーションするスプライトをspawnする
                let custom_size = Some( GRID_CUSTOM_SIZE );
                let layout = texture_atlases_layout.add
                (   TextureAtlasLayout::from_grid
                    (   SPRITE_SHEET_SIZE_CHASER,
                        SPRITE_SHEET_COLS_CHASER,
                        SPRITE_SHEET_ROWS_CHASER,
                        None, None
                    )
                );
                let index = chaser.sprite_sheet_offset( chaser.direction() ) as usize;
                cmds.spawn( ( SpriteBundle::default(), chaser ) )
                .insert( Sprite { custom_size, ..default() } )
                .insert( asset_svr.load( asset_file ) as Handle<Image> )
                .insert( TextureAtlas { layout, index } )
                .insert( Transform::from_translation( translation ) )
                ;
            }
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//敵キャラ（正方形の場合）のスプライトを回転させる
pub fn rotate_chaser_shape
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

//敵キャラを移動させる
pub fn move_sprite
(   mut qry_chaser: Query<( &mut Chaser, &mut Transform, &mut TextureAtlas )>,
    opt_map: Option<Res<map::Map>>,
    qry_player: Query<&player::Player>,
    mut evt_timer: EventWriter<EventTimerChasers>,
    time: Res<Time>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };
    let Some ( map ) = opt_map else { return };

    //前回からの経過時間
    let time_delta = time.delta();
    let mut chaser_timer_finished = Vec::new();

    //敵キャラは複数なのでループ処理する
    for ( mut chaser, mut transform, mut sprite_sheet ) in qry_chaser.iter_mut()
    {   //自動追尾の関数がセットされているか？
        let Some ( autochase ) = chaser.opt_fn_autochase else { return };

        //経過時間にスピードアップを反映する
        let time_delta = time_delta.mul_f32( chaser.speedup );

        //グリッドのマス間を移動中か？
        if ! chaser.timer.tick( time_delta ).finished()
        {   if ! chaser.is_stop //スタート直後だけ意味があるif文
            {   //移動中の中割座標
                let delta = CHASER_SPEED * time_delta.as_secs_f32();
                match chaser.direction
                {   News::North => transform.translation.y += delta,
                    News::South => transform.translation.y -= delta,
                    News::East  => transform.translation.x += delta,
                    News::West  => transform.translation.x -= delta,
                }
                chaser.px_start = chaser.px_end;
                chaser.px_end   = transform.translation.truncate();
            }
        }
        else
        {   chaser_timer_finished.push( chaser.color ); //後続の処理にtimer finishedを伝達する

            //スプライトをグリッドに配置する
            if chaser.px_start != chaser.px_end
            {   chaser.px_start = chaser.px_end;
                chaser.px_end   = chaser.next_grid.to_vec2_on_game_map();
                transform.translation = chaser.px_end.extend( DEPTH_SPRITE_CHASER );
            }

            //四方の脇道を取得する
            let mut sides = map.get_side_spaces_list( chaser.next_grid );    //脇道のリスト
            sides.retain( | side | chaser.next_grid + side != chaser.grid ); //戻り路を取り除く

            //敵キャラが次に進む方向を決める
            chaser.is_stop = false; //停止フラグを倒す(敵キャラはスタート後は止まらない)

            let new_side = match sides.len().cmp( &1 ) //sides要素数は１以上(マップに行き止まりが無いので)
            {   //一本道 ⇒ 道なりに進む
                Ordering::Equal => sides[ 0 ],

                //三叉路または十字路 ⇒ 外部関数で自動追尾する
                Ordering::Greater => autochase( &mut chaser, player, &sides ),

                //行き止まり ⇒ 逆走 (このゲームに行き止まりはないのでここには来ないけど)
                Ordering::Less => chaser.direction.back_side(),
            };

            //進行方向が変わったらスプライトの見栄えを変える（スプライトシートのindexを変える）
            if ! SPRITE_SHEET_OFF() && chaser.direction != new_side
            {   let old_offset = chaser.sprite_sheet_offset( chaser.direction ) as usize;
                let new_offset = chaser.sprite_sheet_offset( new_side         ) as usize;
                sprite_sheet.index = sprite_sheet.index + new_offset - old_offset;
            }
            chaser.direction = new_side;

            //現在の位置と次の位置を更新する
            chaser.grid = chaser.next_grid;
            if ! chaser.is_stop
            {   let side = chaser.direction;
                chaser.next_grid += side; //✕ chaser.direction += chaser.next_grid
            }

            //タイマーをリセットする
            chaser.timer.reset();
        }
    }

    //後続の処理にtimer finishedを伝達する
    if ! chaser_timer_finished.is_empty()
    {   evt_timer.send( EventTimerChasers ( chaser_timer_finished ) );
    }

    //敵キャラは重なるとスピードアップする
    let mut color_grid = Vec::with_capacity( qry_chaser.iter().len() );
    for ( mut chaser, _, _ ) in qry_chaser.iter_mut()
    {   color_grid.push( ( chaser.color, chaser.next_grid ) );
        chaser.speedup = 1.0;
    }
    for ( color, grid ) in color_grid
    {   for ( mut chaser, _, _ ) in qry_chaser.iter_mut()
        {   if grid != chaser.next_grid || color == chaser.color { continue }
            chaser.speedup += CHASER_ACCEL;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//進む方向を決める(赤)
fn choice_way_red( chaser: &mut Chaser, player: &player::Player, sides: &[ News ] ) -> News
{   if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    sides[ rand::thread_rng().gen_range( 0..sides.len() ) ]
}

//進む方向を決める(青)
fn choice_way_blue( chaser: &mut Chaser, player: &player::Player, sides: &[ News ] ) -> News
{   if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    sides[ rand::thread_rng().gen_range( 0..sides.len() ) ]
}

//進む方向を決める(緑)
fn choice_way_green( chaser: &mut Chaser, player: &player::Player, sides: &[ News ] ) -> News
{   if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    sides[ rand::thread_rng().gen_range( 0..sides.len() ) ]
}

//進む方向を決める(ピンク)
fn choice_way_pink( chaser: &mut Chaser, player: &player::Player, sides: &[ News ] ) -> News
{   if sides.contains( &News::East  ) && player.next_grid.x > chaser.grid.x { return News::East  }
    if sides.contains( &News::North ) && player.next_grid.y < chaser.grid.y { return News::North }
    if sides.contains( &News::South ) && player.next_grid.y > chaser.grid.y { return News::South }
    if sides.contains( &News::West  ) && player.next_grid.x < chaser.grid.x { return News::West  }
    sides[ rand::thread_rng().gen_range( 0..sides.len() ) ]
}

////////////////////////////////////////////////////////////////////////////////

//End of code.