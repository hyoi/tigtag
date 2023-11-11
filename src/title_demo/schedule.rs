use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .init_resource::<DemoMapParams>() //demo用のマップ情報

        .add_systems( Update, update_demo_record ) //フッターの更新(demo)
 
        ////////////////////////////////////////////////////////////////////////
        //デモプレイ
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   play_game::map::make_new_data,        //マップデータの作成
                make_data_for_demo,                   //デモ用のマップ情報を作成
                (   play_game::map::spawn_sprite,     //マップをspawnする
                    play_game::player::spawn_sprite,  //プレイヤーをspawnする
                    play_game::chasers::spawn_sprite, //チェイサーをspawnする
                ),
            ).chain()
        )
        .add_systems
        (   Update,
            (   play_game::judge::scoring_and_stageclear, //スコアリング＆クリア判定
                update_data_for_demo,                     //デモ用のマップ情報を更新
                play_game::judge::detect_collisions,      //衝突判定
                (   play_game::player::move_sprite,       //スプライト移動
                    play_game::chasers::move_sprite,      //スプライト移動
                )
            )
            .chain()
            .run_if( in_state( MyState::TitleDemo ) )
        )

        ////////////////////////////////////////////////////////////////////////
        //一旦TitleDemoから抜けてDemoLoopに入り、その後TitleDemoへ戻る
        //(Stateを変更することでOnEnter等を実行させる)
        .add_systems
        (   OnEnter ( MyState::DemoLoop ),
            (   update_demo_record, //フッターの更新(demo)
                misc::change_state::<TitleDemo>, //無条件遷移
            )
            .chain()
        );

        //demo用情報のdebug表示
        #[cfg( debug_assertions )]
        app.add_systems
        (   Update,
            view_min_rect_contains_dots.run_if( in_state( MyState::TitleDemo ) )
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

//フッターの更新
fn update_demo_record
(   mut qry_text: Query<&mut Text, With<init_app::UiFps>>,
    opt_record: Option<ResMut<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( mut record ) = opt_record else { return };

    //demoのハイスコアの場合、記録を残す
    if record.score() > record.demo_hi_score()
    {   *record.demo_hi_score_mut() = record.score();
        *record.demo_stage_mut()    = record.stage();
    }

    let value = format!( "{:02}-{:05}", record.demo_stage(), record.demo_hi_score(), );
    text.sections[ 3 ].value = value;
}

////////////////////////////////////////////////////////////////////////////////

//デモ用のマップ情報を作成する
fn make_data_for_demo
(   opt_map: Option<Res<Map>>,
    opt_demo: Option<ResMut<DemoMapParams>>,
)
{   let Some ( map ) = opt_map else { return };
    let Some ( mut demo ) = opt_demo else { return };

    //dotではなく道を数える(初期状態では必ず道にdotがある)
    MAP_GRIDS_Y_RANGE.for_each
    (   | y |
        *demo.dots_sum_y_mut( y ) =
        {   MAP_GRIDS_X_RANGE
            .filter( | &x | map.is_space( IVec2::new( x, y ) ) )
            .count() as i32
        }
    );
    MAP_GRIDS_X_RANGE.for_each
    (   | x |
        *demo.dots_sum_x_mut( x ) =
        {   MAP_GRIDS_Y_RANGE
            .filter( | &y | map.is_space( IVec2::new( x, y ) ) )
            .count() as i32
        }
    );

    //dotsを内包する最小の矩形の初期値は決め打ちでいい(Mapをそう作っているから)
    *demo.dots_rect_min_mut() = IVec2::new( 1, 1 );
    *demo.dots_rect_max_mut() = IVec2::new( MAP_GRIDS_WIDTH - 2, MAP_GRIDS_HEIGHT - 2 );
}

////////////////////////////////////////////////////////////////////////////////

//デモ用のマップ情報を更新する
fn update_data_for_demo
(   qry_player: Query<&Player>,
    opt_demo: Option<ResMut<DemoMapParams>>,
    mut evt_eatdot: EventReader<EventEatDot>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };
    let Some ( mut demo ) = opt_demo else { return };

    //直前のスコアリングでドットを削除していない場合
    if evt_eatdot.read().next().is_none() { return }

    //プレイヤーの位置の列・行のdotsを減らす
    *demo.dots_sum_x_mut( player.grid.x ) -= 1;
    *demo.dots_sum_y_mut( player.grid.y ) -= 1;

    //dotsを内包する最小の矩形のminを更新する
    let ( mut x, mut y ) = ( 0, 0 );
    for _ in MAP_GRIDS_X_RANGE
    {   if demo.dots_sum_x( x ) != 0 { break } else { x += 1; }
    }
    for _ in MAP_GRIDS_Y_RANGE
    {   if demo.dots_sum_y( y ) != 0 { break } else { y += 1; }
    }
    *demo.dots_rect_min_mut() = IVec2::new( x, y );

    //dotsを内包する最小の矩形のmaxを更新する
    ( x, y ) = ( MAP_GRIDS_WIDTH - 1, MAP_GRIDS_HEIGHT - 1 );
    for _ in MAP_GRIDS_X_RANGE
    {   if demo.dots_sum_x( x ) != 0 { break } else { x -= 1; }
    }
    for _ in MAP_GRIDS_Y_RANGE
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
#[cfg( debug_assertions )]
fn view_min_rect_contains_dots
(   opt_demo: Option<Res<DemoMapParams>>,
    mut gizmos: Gizmos
)
{   let Some ( demo ) = opt_demo else { return };
    let min = demo.dots_rect_min().to_screen_pixels() + ADJUSTER_MAP_SPRITES;
    let max = demo.dots_rect_max().to_screen_pixels() + ADJUSTER_MAP_SPRITES;
    let width  = max.x - min.x + PIXELS_PER_GRID;
    let height = max.y - min.y - PIXELS_PER_GRID;
    let size = Vec2::new( width, height );
    let position = min + size / 2.0;

    gizmos.rect_2d( position, 0.0, size, Color::GREEN );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.