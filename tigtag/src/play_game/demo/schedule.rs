use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //Resource
        .init_resource::<DemoMapParams>() //demo用マップ情報

        //plugin
        .add_plugins( footer::Schedule ) //フッター(demo record)

        //debug表示(Gizumo)
        .add_systems
        (   Update,
            view_data_for_demo
                .run_if( DEBUG )
                .run_if( in_state( MyState::TitleDemo ) )
        )

        ////////////////////////////////////////////////////////////////////////
        //デモプレイ
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   //マップデータ生成
                map::make_new_data,
                make_data_for_demo, //デモ用マップ情報を収集

                //スプライトのspawn
                (   map::spawn_sprite,
                    player::spawn_sprite,
                    chasers::spawn_sprite,
                ),
            )
            .chain() //実行順の固定
        )
        .add_systems
        (   Update,
            (   //ループ脱出条件
                detection::scoring_and_stage_clear, //スコアリング＆クリア判定
                change_state_to::<DemoLoop>.run_if( on_event::<EventClear>() ),
                update_data_for_demo.run_if( on_event::<EventEatDot>() ),

                detection::collisions_and_gameover, //衝突判定
                change_state_to::<DemoLoop>.run_if( on_event::<EventOver>() ),

                //スプライトの移動
                (   player::move_sprite,  //自キャラ
                    chasers::move_sprite, //敵キャラ
                )
            )
            .chain() //実行順の固定
            .run_if( in_state( MyState::TitleDemo ) )
        )
        .add_systems
        (   OnEnter ( MyState::DemoLoop ),
            (   change_state_to::<TitleDemo>, //無条件遷移
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//demo用のマップ情報Resource
#[derive( Resource, Default )]
pub struct DemoMapParams
{   dots_rect : IVec2Rect,                               //dotsを内包する最小の矩形
    dots_sum_x: [ i32; map::MAP_GRIDS_WIDTH  as usize ], //列に残っているdotsを数えた配列
    dots_sum_y: [ i32; map::MAP_GRIDS_HEIGHT as usize ], //行に残っているdotsを数えた配列
}

#[derive( Default )]
struct IVec2Rect { min: IVec2, max: IVec2 }

impl DemoMapParams
{   pub fn dots_sum_x    ( &    self, x: i32 ) ->      i32 {      self.dots_sum_x[ x as usize ] }
    pub fn dots_sum_x_mut( &mut self, x: i32 ) -> &mut i32 { &mut self.dots_sum_x[ x as usize ] }
    pub fn dots_sum_y    ( &    self, y: i32 ) ->      i32 {      self.dots_sum_y[ y as usize ] }
    pub fn dots_sum_y_mut( &mut self, y: i32 ) -> &mut i32 { &mut self.dots_sum_y[ y as usize ] }

    pub fn dots_rect_min    ( &    self ) ->       IVec2 {      self.dots_rect.min }
    pub fn dots_rect_min_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.min }
    pub fn dots_rect_max    ( &    self ) ->       IVec2 {      self.dots_rect.max }
    pub fn dots_rect_max_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.max }
}

////////////////////////////////////////////////////////////////////////////////

//デモ用のマップ情報を作成する
fn make_data_for_demo
(   opt_map: Option<Res<map::Map>>,
    opt_demo: Option<ResMut<DemoMapParams>>,
)
{   let Some ( map ) = opt_map else { return };
    let Some ( mut demo ) = opt_demo else { return };

    //dotではなく道を数える(マップデータ作成の直後なら必ず道にdotがある)
    map::MAP_GRIDS_Y_RANGE.for_each
    (   | y |
        *demo.dots_sum_y_mut( y ) =
        {   map::MAP_GRIDS_X_RANGE
            .filter( | &x | map.is_space( IVec2::new( x, y ) ) )
            .count() as i32
        }
    );
    map::MAP_GRIDS_X_RANGE.for_each
    (   | x |
        *demo.dots_sum_x_mut( x ) =
        {   map::MAP_GRIDS_Y_RANGE
            .filter( | &y | map.is_space( IVec2::new( x, y ) ) )
            .count() as i32
        }
    );

    //dotsを内包する最小の矩形の初期値は決め打ちでいい(Mapをそう作っているから)
    *demo.dots_rect_min_mut() = IVec2::new( 1, 1 );
    *demo.dots_rect_max_mut() = IVec2::new( map::MAP_GRIDS_WIDTH - 2, map::MAP_GRIDS_HEIGHT - 2 );
}

////////////////////////////////////////////////////////////////////////////////

//デモ用のマップ情報を更新する
fn update_data_for_demo
(   qry_player: Query<&player::Player>,
    opt_demo: Option<ResMut<DemoMapParams>>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };
    let Some ( mut demo ) = opt_demo else { return };

    //プレイヤーの位置の列・行のdotsを減らす
    *demo.dots_sum_x_mut( player.grid.x ) -= 1;
    *demo.dots_sum_y_mut( player.grid.y ) -= 1;

    //dotsを内包する最小の矩形のminを更新する
    let ( mut x, mut y ) = ( 0, 0 );
    for _ in map::MAP_GRIDS_X_RANGE
    {   if demo.dots_sum_x( x ) != 0 { break } else { x += 1; }
    }
    for _ in map::MAP_GRIDS_Y_RANGE
    {   if demo.dots_sum_y( y ) != 0 { break } else { y += 1; }
    }
    *demo.dots_rect_min_mut() = IVec2::new( x, y );

    //dotsを内包する最小の矩形のmaxを更新する
    ( x, y ) = ( map::MAP_GRIDS_WIDTH - 1, map::MAP_GRIDS_HEIGHT - 1 );
    for _ in map::MAP_GRIDS_X_RANGE
    {   if demo.dots_sum_x( x ) != 0 { break } else { x -= 1; }
    }
    for _ in map::MAP_GRIDS_Y_RANGE
    {   if demo.dots_sum_y( y ) != 0 { break } else { y -= 1; }
    }
    *demo.dots_rect_max_mut() = IVec2::new( x, y );
}

////////////////////////////////////////////////////////////////////////////////

//demo時のプレイヤー自走に使うメソッド
impl DemoMapParams
{   //指定のマスが、残dotsの最小矩形の中か？
    pub fn is_inside_rect( &self, grid: IVec2 ) -> bool
    {   let IVec2 { x: x1, y: y1 } = self.dots_rect_min();
        let IVec2 { x: x2, y: y2 } = self.dots_rect_max();

        ( x1..=x2 ).contains( &grid.x ) && ( y1..=y2 ).contains( &grid.y )
    }

    //指定のマスから残dotsの最小矩形までの単純距離(dx+dy)を求める
    pub fn how_far_to_rect( &self, grid: IVec2 ) -> i32
    {   let IVec2 { x: x1, y: y1 } = self.dots_rect_min();
        let IVec2 { x: x2, y: y2 } = self.dots_rect_max();

        let dx = if grid.x < x1 { x1 - grid.x } else if grid.x > x2 { grid.x - x2 } else { 0 };
        let dy = if grid.y < y1 { y1 - grid.y } else if grid.y > y2 { grid.y - y2 } else { 0 };

        dx + dy
    }
}

////////////////////////////////////////////////////////////////////////////////

//demo用情報のdebug表示
fn view_data_for_demo
(   opt_demo: Option<Res<DemoMapParams>>,
    mut gizmos: Gizmos
)
{   let Some ( demo ) = opt_demo else { return };
    let adjuster = Vec2::Y     * PIXELS_PER_GRID / 2.0
                 + Vec2::NEG_X * PIXELS_PER_GRID / 2.0;
    let min = demo.dots_rect_min().to_vec2_on_game_map() + adjuster;
    let max = demo.dots_rect_max().to_vec2_on_game_map() + adjuster;
    let width  = max.x - min.x + PIXELS_PER_GRID;
    let height = max.y - min.y - PIXELS_PER_GRID;
    let size = Vec2::new( width, height );
    let position = min + size / 2.0;

    gizmos.rect_2d( position, 0.0, size, Color::BLUE );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.