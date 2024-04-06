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
            (   start_loading, //Assetsのロード開始
                spawn_sprite,  //アニメ用スプライトの生成
                debug::spawn_2d_sprites.run_if( DEBUG ), //debug用グリッド表示
            )
        )
        //ループ
        .add_systems
        (   Update,
            (   is_loading_done, //ロード完了ならフラグを立てる
                move_sprite,     //ロード中のアニメーション

                change_state_by::<AfterLoadAssets> //ループ脱出
                    .run_if( resource_exists::<IsLoadingFinished> ) //ローディング完了フラグ
                    .run_if( resource_exists::<AfterLoadAssets> ), //遷移先State
            )
            .run_if( in_state( MyState::LoadAssets ) )
        )
        //後処理
        .add_systems
        (   OnExit ( MyState::LoadAssets ),
            (   misc::despawn_component::<Sprite>, //スプライトの削除
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//ロードしたAssetsのハンドルの保存先
#[derive( Resource )]
struct LoadedAssets { handles: Vec<Handle<LoadedUntypedAsset>> }

//ローディング完了フラグ
#[derive( Resource )]
struct IsLoadingFinished;

//ローディングが完了した後の遷移先
#[derive( Resource )]
pub struct AfterLoadAssets ( pub MyState );
impl ChangeMyState for AfterLoadAssets
{   fn state( &self ) -> MyState { self.0 }
}

//ローディングアニメのスプライトのComponent
#[derive( Component )]
struct SpriteTile { goal: IVec2 }

//ローディングメッセージ
struct LoadingMessage<'a>
{   design: Vec<&'a str>,
    width : f32,
    height: f32,
}
static NOWLOADING: Lazy<LoadingMessage> = Lazy::new
(   ||
    {   let design = vec!
        [//  0123456789 123456789 123456789 123456789 12345
            " ##  #           #                            ", //0
            " ##  #  #  #   # #     #   #  ##  # #  #  ##  ", //1
            " # # # # # # # # #    # # # # # #   ## # #    ", //2
            " # # # # # # # # #    # # # # # # # #### # ## ", //3
            " #  ## # #  # #  #    # # ### # # # # ## #  # ", //4
            " #  ##  #   # #  ####  #  # # ##  # #  #  ##  ", //5
            "",                                               //6
            "",                                               //7
            " ###                      #   #           # # ", //8
            " #  # #   ###  #  ### ### # # #  #  # ### # # ", //9
            " #  # #   #   # # #   #   # # # # #    #  # # ", //10
            " ###  #   ### # # ### ### # # # # # #  #  # # ", //11
            " #    #   #   ###   # #    # #  ### #  #      ", //12
            " #    ### ### # # ### ###  # #  # # #  #  # # ", //13
        ];// 0123456789_123456789_123456789_123456789_12345

        let width  = design[ 0 ].len() as f32 * PIXELS_PER_GRID;
        let height = design.len() as f32 * PIXELS_PER_GRID;

        LoadingMessage { design, width, height,}
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

//Assetsのロードは完了したか？
fn is_loading_done
(   assets: Res<LoadedAssets>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //事前ロードが完了したか？
    for handle in assets.handles.iter()
    {   match asset_svr.get_load_state( handle )
        {   Some ( LoadState::Loaded ) => (), //ロード完了
            Some ( LoadState::Failed ) =>
            {   //ロード失敗⇒パニック
                let mut filename = "Unknown".to_string();
                if let Some ( asset_path ) = handle.path()
                {   if let Some ( s ) = asset_path.path().to_str()
                    {   filename = s.to_string();
                    }
                }
                panic!( "AppEr: Failed loading asset-file \"{filename}\"" );
            },
            _ => return, //UPDATEなので関数は繰り返し呼び出される
        }
    }

    //ロード完了フラグを立てる
    cmds.insert_resource( IsLoadingFinished );
}

////////////////////////////////////////////////////////////////////////////////

//ローディングアニメ用スプライトを生成する
fn spawn_sprite( mut cmds: Commands )
{   let mut rng = rand::thread_rng();
    let color = Color::YELLOW;
    let custom_size = Some ( GRID_CUSTOM_SIZE * 0.9 );

    for ( goal_y, line ) in NOWLOADING.design.iter().enumerate()
    {   for ( goal_x, char ) in line.chars().enumerate()
        {   //空白文字は無視
            if char == ' ' { continue }

            //スプライトの移動先座標(ゴール)
            let goal = IVec2::new( goal_x as i32, goal_y as i32 );

            //スプライトの初期座標(スタート)
            let rnd_x = rng.gen_range( GRIDS_X_RANGE );
            let rnd_y = rng.gen_range( GRIDS_Y_RANGE );
            let vec2 = IVec2::new( rnd_x, rnd_y ).to_vec2_on_screen();
            let vec3 = vec2.extend( DEPTH_SPRITE_LOADING_MSG );

            //スプライトをspawnする
            cmds.spawn( ( SpriteBundle::default(), SpriteTile { goal } ) )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            ;
        }
    }
}

//スプライトを動かしてローディングアニメを見せる
fn move_sprite
(   mut qry_transform: Query<( &mut Transform, &SpriteTile )>,
    time: Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32() * 2.0;

    let scaling = SCREEN_PIXELS_WIDTH / NOWLOADING.width; //横方向に長いのでWidthを使う
    let adjuster_y = ( SCREEN_PIXELS_HEIGHT - NOWLOADING.height * scaling ) * 0.5;

    qry_transform.iter_mut().for_each
    (   | ( mut transform, sprite ) |
        {   //座標の調整
            let mut goal = sprite.goal.to_vec2_on_screen() * scaling;
            goal.y -= adjuster_y;

            //ゴールへ向けてスプライトを移動
            let now = &mut transform.translation;
            now.x += ( goal.x - now.x ) * time_delta;
            now.y += ( goal.y - now.y ) * time_delta;
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.