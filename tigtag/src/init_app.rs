use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //assetsのロード完了後にInitAppへ遷移させる
        .insert_resource( load_assets::AfterLoadAssets ( MyState::InitApp ) )

        //日時表示の更新
        .add_systems( Update, uapdate_real_clock )

        //経過時間表示の更新
        .add_systems( PreStartup, change_passed_clock_wrap_period ) //wrap変更(3600s->60s)
        .add_systems( Update, uapdate_passed_clock )

        //FPS表示の更新
        .add_plugins( FrameTimeDiagnosticsPlugin ) //FPSプラグイン
        .add_systems( Update, update_fps )

        //ヘッダー／フッターを表示する
        .add_systems
        (   OnEnter ( MyState::InitApp ),
            (   spawn_header_footer, //ヘッダー／フッターのspawn

                //テスト用
                (   // debug::spawn_2d_sprites, //debug用グリッド表示
                    debug::spawn_3d_objects, //3D表示テスト
                )
                .run_if( DEBUG )
                .run_if( not( resource_exists::<AfterInitApp> ) )
            )
        )
        .add_systems
        (   Update,
            (   change_state_by::<AfterInitApp> //State遷移
                    .run_if( resource_exists::<AfterInitApp> ), //遷移先State

                //テスト用
                (   debug::update_gizmo, //Gizmo描画

                    (   (   debug::catch_input_keyboard, //キー入力
                            debug::catch_input_mouse,    //マウス
                            debug::catch_input_gamepad,  //ゲームパッド
                        ),
                        debug::move_orbit_camera, //極座標用3Dカメラの移動
                    )
                    .chain() //実行順の固定
                )
                .run_if( DEBUG )
                .run_if( not( resource_exists::<AfterInitApp> ) )
            )
            .run_if( in_state( MyState::InitApp ) )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//InitAppの後の遷移先を登録するResource
#[derive( Resource )]
pub struct AfterInitApp ( pub MyState );
impl ChangeMyState for AfterInitApp
{   fn state( &self ) -> MyState { self.0 }
}

//隠しノードのComponent
#[derive( Component )] pub struct HiddenNode;

//ヘッダー／フッターのComponent
#[derive( Component )] pub struct UiHeaderLeft;   //日時表示
#[derive( Component )] pub struct UiHeaderCenter; //タイトル
#[derive( Component )] pub struct UiHeaderRight;  //経過時間表示
#[derive( Component )] pub struct UiFooterLeft;   //FPS表示
#[derive( Component )] pub struct UiFooterCenter; //auther
#[derive( Component )] pub struct UiFooterRight;  //Powered by

//ヘッダー／フッターのテキスト情報
const HEADER_LEFT: &[ MessageSect ] =
&[  ( "  Clock ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, Color::TEAL   ),
    ( ""        , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::SILVER ),
];

const HEADER_CENTER: &[ MessageSect ] =
&[  ( APP_TITLE, ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL ),
];

const HEADER_RIGHT: &[ MessageSect ] =
&[  ( "  Passed ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, Color::TEAL   ),
    ( ""         , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::SILVER ),
];

const FOOTER_LEFT: &[ MessageSect ] =
&[  ( "  FPS ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, Color::TEAL   ),
    ( ""      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::SILVER ),
];

const FOOTER_CENTER: &[ MessageSect ] =
&[  ( "hyoi 2023 - XXXX", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL ),
];

const FOOTER_RIGHT: &[ MessageSect ] =
&[  ( "Powered by ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
    ( "RUST"       , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
    ( " & "        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
    ( "BEVY  "     , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
];

//おまけ(蟹)
const GRID_X_KANI: i32 = SCREEN_GRIDS_WIDTH  - 4;
const GRID_Y_KANI: i32 = SCREEN_GRIDS_HEIGHT - 1;
const MAGNIFY_SPRITE_KANI: f32 = 0.9;
const COLOR_SPRITE_KANI: Color = Color::rgba( 1.0, 1.0, 1.0, 0.6 );

////////////////////////////////////////////////////////////////////////////////

//ヘッダー／フッターをspawnする
fn spawn_header_footer
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //ウィンドウ全体の隠しノードを作成する(グリッドレイアウト３列)
    let style = Style
    {   width : Val::Percent( 100.0 ),
        height: Val::Percent( 100.0 ),
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::fr( 3, 1.0 ), //３列指定
        grid_template_rows   : RepeatedGridTrack::fr( 3, 1.0 ), //３行指定
        ..default()
    };
    let mut hidden_node = NodeBundle
    {   style,
        background_color: Color::NONE.into(),
        ..default()
    };

    //debug時のボーダーライン表示
    if DEBUG()
    {   hidden_node.style.border = UiRect::all( Val::Px( 1.0 ) );
        hidden_node.border_color = Color::RED.into()
    }

    //ヘッダー／フッターの準備
    let mut header_left   = misc::text_ui( HEADER_LEFT  , &asset_svr );
    let mut header_center = misc::text_ui( HEADER_CENTER, &asset_svr );
    let mut header_right  = misc::text_ui( HEADER_RIGHT , &asset_svr );
    let mut footer_left   = misc::text_ui( FOOTER_LEFT  , &asset_svr );
    let mut footer_center = misc::text_ui( FOOTER_CENTER, &asset_svr );
    let mut footer_right  = misc::text_ui( FOOTER_RIGHT , &asset_svr );

    header_left.style.grid_row       = GridPlacement::start( 1 ); //ヘッダー
    header_left.style.grid_column    = GridPlacement::start( 1 ); //左端のセル
    header_left.style.align_self     = AlignSelf::FlexStart;      //セル内の上寄せ
    header_left.style.justify_self   = JustifySelf::Start;        //セル内の左寄せ
    header_left.background_color     = Color::BLUE.into();

    header_center.style.grid_row     = GridPlacement::start( 1 ); //ヘッダー
    header_center.style.grid_column  = GridPlacement::start( 2 ); //中央のセル
    header_center.style.align_self   = AlignSelf::FlexStart;      //セル内の上寄せ
    header_center.style.justify_self = JustifySelf::Center;       //セル内の中央寄せ
    header_center.background_color   = Color::BLUE.into();

    header_right.style.grid_row      = GridPlacement::start( 1 ); //ヘッダー
    header_right.style.grid_column   = GridPlacement::start( 3 ); //右端のセル
    header_right.style.align_self    = AlignSelf::FlexStart;      //セル内の上寄せ
    header_right.style.justify_self  = JustifySelf::End;          //セル内の右寄せ
    header_right.background_color    = Color::BLUE.into();

    footer_left.style.grid_row       = GridPlacement::start( 3 ); //フッター
    footer_left.style.grid_column    = GridPlacement::start( 1 ); //左端のセル
    footer_left.style.align_self     = AlignSelf::FlexEnd;        //セル内の下寄せ
    footer_left.style.justify_self   = JustifySelf::Start;        //セル内の左寄せ

    footer_center.style.grid_row     = GridPlacement::start( 3 ); //フッター
    footer_center.style.grid_column  = GridPlacement::start( 2 ); //中央のセル
    footer_center.style.align_self   = AlignSelf::FlexEnd;        //セル内の下寄せ
    footer_center.style.justify_self = JustifySelf::Center;       //セル内の中央寄せ

    footer_right.style.grid_row      = GridPlacement::start( 3 ); //フッター
    footer_right.style.grid_column   = GridPlacement::start( 3 ); //右端のセル
    footer_right.style.align_self    = AlignSelf::FlexEnd;        //セル内の下寄せ
    footer_right.style.justify_self  = JustifySelf::End;          //セル内の右寄せ

    //隠しノードの中に子要素を作成する
    cmds.spawn( ( hidden_node, HiddenNode ) ).with_children
    (   | cmds |
        {   cmds.spawn( ( header_left  , UiHeaderLeft   ) );
            cmds.spawn( ( header_center, UiHeaderCenter ) );
            cmds.spawn( ( header_right , UiHeaderRight  ) );
            cmds.spawn( ( footer_left  , UiFooterLeft   ) );
            cmds.spawn( ( footer_center, UiFooterCenter ) );
            cmds.spawn( ( footer_right , UiFooterRight  ) );
        }
    );

    //おまけ(蟹)
    let custom_size = Some ( GRID_CUSTOM_SIZE * MAGNIFY_SPRITE_KANI );
    let color = COLOR_SPRITE_KANI;
    let vec2 = IVec2::new( GRID_X_KANI, GRID_Y_KANI ).to_vec2_on_screen();
    let vec3 = vec2.extend( DEPTH_SPRITE_KANI_DOTOWN );
    cmds.spawn( SpriteBundle::default() )
    .insert( Sprite { custom_size, color, ..default() } )
    .insert( Transform::from_translation( vec3 ) )
    .insert( asset_svr.load( ASSETS_SPRITE_KANI_DOTOWN ) as Handle<Image> )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//日時表示を更新する
fn uapdate_real_clock
(   mut qry_text: Query<&mut Text, With<UiHeaderLeft>>,
)
{   let Ok( mut text ) = qry_text.get_single_mut() else { return };

    text.sections[ 1 ].value =
        chrono::Local::now().format("%m/%d %H:%M:%S").to_string();
}

////////////////////////////////////////////////////////////////////////////////

//経過時間を記録するResource
#[derive( Resource, Default )]
struct ElapsedTime { d: i32, h: i32, m: i32, s: f32, }

//経過時間を記録する準備
fn change_passed_clock_wrap_period
(   mut time: ResMut<Time<Real>>,
    mut cmd: Commands,
)
{   //wrapを60秒へ変更
    time.set_wrap_period( Duration::new( 60, 0 ) );

    //経過時間を記録するResourceを登録
    cmd.insert_resource
    (   ElapsedTime
        {   s: time.elapsed_seconds_wrapped(),
            ..default()
        }
    );
}

//経過時間を更新する
fn uapdate_passed_clock
(   mut qry_text: Query<&mut Text, With<UiHeaderRight>>,
    opt_clock: Option<ResMut<ElapsedTime>>,
    time: Res<Time<Real>>,
)
{   let Ok( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( mut clock ) = opt_clock else { return };

    let new_s = time.elapsed_seconds_wrapped();
    if  new_s < clock.s //wrapしたら
    {   clock.m += 1;
        if clock.m > 59 { clock.m = 0; clock.h += 1 }
        if clock.h > 23 { clock.h = 0; clock.d += 1 }
    }
    clock.s = new_s.floor(); //小数点以下切り捨て(format!が切り上げ表示する対策)

    text.sections[ 1 ].value =
        format!( "{:02}:{:02}:{:02}:{:02.0} ", clock.d, clock.h, clock.m, clock.s );
}

////////////////////////////////////////////////////////////////////////////////

//FPSを更新する
fn update_fps
(   mut qry_text: Query<&mut Text, With<UiFooterLeft>>,
    diag_store: Res<DiagnosticsStore>,
)
{   let Ok( mut text ) = qry_text.get_single_mut() else { return };
    const NA3_2: &str = "###.##";

    text.sections[ 1 ].value =
        diag_store
        .get( &FrameTimeDiagnosticsPlugin::FPS )
        .map_or
        (   NA3_2.to_string(),
            | fps |
            {   fps.average().map_or
                (   NA3_2.to_string(),
                    | avg | format!( "{avg:06.2}" ),
                )
            }
        );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.