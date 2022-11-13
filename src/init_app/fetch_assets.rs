use super::*;

//Assetsのロードを開始する
pub fn start_fetching_assets
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //Assetsのロードを開始
    let mut preload = Vec::new();
    FETCH_ASSETS.iter().for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

    //リソースに登録して解放しないようにする
    cmds.insert_resource( LoadedAssets { preload } );
}

//ローディングアニメ用スプライトを生成する
pub fn spawn_sprite_now_loading
(   mut cmds: Commands,
)
{   let mut rng = rand::thread_rng();
    let color = COLOR_SPRITE_TILE;
    let custom_size = Some ( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) * 0.75 );
 
    for ( goal_y, line ) in DESIGN_NOWLOADING_MESSAGE.iter().enumerate()
    {   for ( goal_x, chara ) in line.chars().enumerate()
        {   if chara == ' ' { continue }    //空白文字は無視

            //スプライトの初期座標と最終座標
            let rnd_x = rng.gen_range( SCREEN_GRIDS_RANGE_X );
            let rnd_y = rng.gen_range( SCREEN_GRIDS_RANGE_Y );
            let start = Grid::new( rnd_x, rnd_y ).into_pixel_screen();
            let goal  = Grid::new( goal_x as i32, goal_y as i32 );

            //スプライトを作成する
            cmds
            .spawn( ( SpriteBundle::default(), SpriteTile ( goal ) ) )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( start.extend( DEPTH_SPRITE_TILE ) ) )
            ;
        } 
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetsのロードが完了したら、Stateを変更する
pub fn change_state_after_loading
(   assets   : Res<LoadedAssets>,
    mut state: ResMut<State<GameState>>,
    asset_svr: Res<AssetServer>,
    o_marker : Option<Res<MarkAfterFetchAssets>>,
)
{   //プリロードが完了したか？
    // for handle in assets.preload.iter()
    // {   use bevy::asset::LoadState::*;
    //     match asset_svr.get_load_state( handle )
    //     {   Loaded => {} //完了
    //         Failed => panic!( "Can't load assets" ), //ロード失敗⇒パニック
    //         _      => return, //on_update()の中なので関数は繰り返し呼び出される
    //     }
    // }

    //次のStateへ遷移する
    if let Some ( x ) = o_marker
    {   let _ = state.overwrite_set( x.0 );
    }
}

//スプライトを動かしてローディングアニメを見せる
pub fn move_sprite_now_loading
(   mut q: Query<( &mut Transform, &SpriteTile )>,
    time : Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32() * 5.0;

    let mess_width  = DESIGN_NOWLOADING_MESSAGE[ 0 ].len() as f32 * PIXELS_PER_GRID;
    let mess_height = DESIGN_NOWLOADING_MESSAGE.len() as f32 * PIXELS_PER_GRID;
    let half_screen_w = SCREEN_PIXELS_WIDTH  / 2.0;
    let half_screen_h = ( SCREEN_PIXELS_HEIGHT - mess_height ) / 2.0;
    let scale = SCREEN_PIXELS_WIDTH / mess_width;

    q.for_each_mut
    (   | ( mut transform, goal_xy ) |
        {   let mut goal = goal_xy.0.into_pixel_screen();
            goal.x = ( goal.x + half_screen_w ) * scale - half_screen_w; //横幅の調整
            goal.y = ( goal.y + half_screen_h ) * scale - half_screen_h; //縦位置の調整

            let position = &mut transform.translation;
            position.x += ( goal.x - position.x ) * time_delta;
            position.y += ( goal.y - position.y ) * time_delta;
        }
    );
}

//End of code.