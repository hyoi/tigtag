use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //前処理
        .add_systems
        (   OnEnter ( MyState::LoadAssets ),
            (   start_loading, //Assetのロード開始
                spawn_sprite,  //アニメ用スプライトの生成
            )
        )
        //ループ
        .add_systems
        (   Update,
            (   is_loading_done, //ロード完了ならフラグを立てる
                move_sprite,     //ロード中のアニメーション

                //ループ脱出：事前ロードが完了した ＆ 遷移先がセットされている
                misc::change_state_with_res::<AfterLoadAssetsTo<MyState>>
                .run_if( resource_exists::<LoadingFinished>() ) //フラグ
                .run_if( resource_exists::<AfterLoadAssetsTo<MyState>>() ), //遷移先
            )
            .run_if( in_state( MyState::LoadAssets ) )
        )
        //後処理
        .add_systems
        (   OnExit ( MyState::LoadAssets ),
            (   misc::despawn::<SpriteTile>, //スプライトの削除
            )
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

//ロードしたAssetsのハンドルの保存先
#[derive( Resource )]
struct LoadedAssets { handles: Vec<HandleUntyped> }

//ローディング完了フラグ
#[derive( Resource )]
struct LoadingFinished;

//スプライトのComponent
#[derive( Component )]
struct SpriteTile { grid: IVec2 }

//スプライトの設定
const  COLOR_SPRITE_TILE: Color = Color::YELLOW; //色
const  DEPTH_SPRITE_TILE: f32 = 900.0; //重なり

//ローディングメッセージ
struct LoadingMessage<'a>
{   message: Vec<&'a str>,
    width  : f32,
    height : f32,
}
static NOWLOADING: Lazy<LoadingMessage> = Lazy::new
(   ||
    {   let message = vec!
        [//  0123456789_123456789_123456789_123456789_12345
            " ##  #           #                            ", //0
            " ##  # ### #   # #    ###  #  ##  # #  #  ##  ", //1
            " # # # # # # # # #    # # # # # #   ## # #    ", //2
            " # # # # # # # # #    # # # # # # # #### # ## ", //3
            " #  ## # #  # #  #    # # ### # # # # ## #  # ", //4
            " #  ## ###  # #  #### ### # # ##  # #  #  ##  ", //5
            "",                                               //6
            "",                                               //7
            " ###                      #   #           # # ", //8
            " #  # #   ###  #  ### ### # # #  #  # ### # # ", //9
            " #  # #   #   # # #   #   # # # # #    #  # # ", //10
            " ###  #   ### # # ### ### # # # # # #  #  # # ", //11
            " #    #   #   ###   # #    # #  ### #  #      ", //12
            " #    ### ### # # ### ###  # #  # # #  #  # # ", //13
        ];// 0123456789_123456789_123456789_123456789_12345

        let width  = message[ 0 ].len() as f32 * PIXELS_PER_GRID;
        let height = message.len()      as f32 * PIXELS_PER_GRID;

        LoadingMessage { message, width, height,}
    }
);

////////////////////////////////////////////////////////////////////////////////

//Assetsのロードを開始する
fn start_loading
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //Assetsのロードを開始
    let mut handles = Vec::new();
    PRELOAD_ASSETS.iter()
    .for_each( | fname | handles.push( asset_svr.load_untyped( *fname ) ) );

    //解放しないようリソースに登録する
    cmds.insert_resource( LoadedAssets { handles } );
}

//Assetsのロードが完了したら、Stateを変更する
fn is_loading_done
(   assets: Res<LoadedAssets>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //事前ロードが完了したか？
    for handle in assets.handles.iter()
    {   match asset_svr.get_load_state( handle )
        {   LoadState::Loaded => (), //ロード完了
            LoadState::Failed =>
            {   //ロード失敗⇒パニック
                let mut filename = "Unknown".to_string();
                if let Some ( asset_path ) = asset_svr.get_handle_path( handle )
                {   if let Some ( s ) = asset_path.path().to_str()
                    {   filename = s.to_string();
                    }
                }
                panic!( "Pre-loading failed asset-file \"{filename}\"" );
            },
            _ => return, //UPDATEなので関数は繰り返し呼び出される
        }
    }

    //次のStateへ遷移する
    cmds.insert_resource( LoadingFinished );
}

////////////////////////////////////////////////////////////////////////////////

//ローディングアニメ用スプライトを生成する
fn spawn_sprite( mut cmds: Commands )
{   let mut rng = rand::thread_rng();
    let color = COLOR_SPRITE_TILE;
    let custom_size = Some ( SIZE_GRID * 0.9 );

    for ( goal_y, line ) in NOWLOADING.message.iter().enumerate()
    {   for ( goal_x, char ) in line.chars().enumerate()
        {   //空白文字は無視
            if char == ' ' { continue }

            //スプライトの初期座標を乱数で決める
            let rnd_x = rng.gen_range( SCREEN_GRIDS_X_RANGE );
            let rnd_y = rng.gen_range( SCREEN_GRIDS_Y_RANGE );
            let start = IVec2::new( rnd_x, rnd_y ).to_sprite_pixels();
            let px3d  = start.extend( DEPTH_SPRITE_TILE );

            //スプライトの移動先座標(ゴール)
            let grid = IVec2::new( goal_x as i32, goal_y as i32 );

            //スプライトをspawnする
            cmds.spawn( ( SpriteBundle::default(), SpriteTile { grid } ) )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( px3d ) )
            ;
        }
    }
}

//スプライトを動かしてローディングアニメを見せる
fn move_sprite
(   mut qry_transform: Query<( &mut Transform, &SpriteTile )>,
    time: Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32() * 5.0;

    let scaling = SCREEN_PIXELS_WIDTH / NOWLOADING.width; //横方向に長いので
    let adjuster_y = ( SCREEN_PIXELS_HEIGHT - NOWLOADING.height * scaling ) * -0.5;

    qry_transform.for_each_mut
    (   | ( mut transform, goal ) |
        {   //座標の調整
            let mut goal = goal.grid.to_sprite_pixels() * scaling;
            goal.y += adjuster_y;

            //ゴールへ向けてスプライトを移動
            let now = &mut transform.translation;
            now.x += ( goal.x - now.x ) * time_delta;
            now.y += ( goal.y - now.y ) * time_delta;
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.