use super::*;

////////////////////////////////////////////////////////////////////////////////

//マス目状にスプライトを敷き詰める
pub fn spawn_2d_sprites
(   mut cmds: Commands,
)
{   let color = Color::srgba( 0.1, 0.1, 0.1, 0.4 );
    let custom_size = Some ( GRID_CUSTOM_SIZE * 0.9 );

    for x in GRIDS_X_RANGE
    {   for y in GRIDS_Y_RANGE
        {   let vec3 = IVec2::new( x, y )
            .to_vec2_on_screen()
            .extend( DEPTH_SPRITE_DEBUG_GRID );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            .with_children
            (   | cmds |
                {   let value = format!( "{:02}\n{:02}", x, y ).to_string();
                    let style = TextStyle
                    {   font_size: PIXELS_PER_GRID * 0.4,
                        color    : Color::DARK_GRAY,
                        ..default()
                    };
                    let sections  = vec![ TextSection { value, style } ];
                    let justify = JustifyText::Center;

                    cmds.spawn( Text2dBundle::default() )
                    .insert( Text { sections, justify, ..default() } )
                    .insert( Transform::from_translation( Vec3::Z ) ) //親スプライトの手前
                    ;
                }
            );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//3Dオブジェクトの設定
const SIZE_OBJ3D_DEBUG_PLANE: f32 = 5.0; //地面の縦横のサイズ
const SIZE_OBJ3D_DEBUG_CUBE : f32 = 1.0; //立方体の辺のサイズ
const COLOR_OBJ3D_DEBUG_PLANE: Color = Color::SEA_GREEN; //地面の色
const COLOR_OBJ3D_DEBUG_CUBE : Color = Color::BISQUE; //正方形の色

//gizmo描画対象のマーカー
#[derive( Component )]
pub struct TargetGizumo;

//3Dオブジェクトを配置する
pub fn spawn_3d_objects
(   mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   //立方体
    cmds.spawn( ( PbrBundle::default(), TargetGizumo ) )
    .insert( meshes.add( Cube::from_size( SIZE_OBJ3D_DEBUG_CUBE ) ) )
    .insert( Transform::from_translation( Vec3::ZERO ) ) //原点(全軸0.0)
    .insert( materials.add( COLOR_OBJ3D_DEBUG_CUBE ) )
    ;

    //地面
    cmds.spawn( PbrBundle::default() )
    .insert( meshes.add( SquarePlane::from_size( SIZE_OBJ3D_DEBUG_PLANE ) ) )
    .insert( Transform::from_translation( Vec3::Y / -2.0 ) ) //Y軸を0.5下方へ移動
    .insert( materials.add( COLOR_OBJ3D_DEBUG_PLANE ) )
    ;
}

//Gizomを描画する
//※対象の大きさを考慮しない(三辺が1.0と決め打ちしている)のでラインの長さが足りなくなるかも
pub fn update_gizmo
(   qry_target: Query<&Transform, With<TargetGizumo>>,
    mut gizmos: Gizmos,
)
{   qry_target.iter().for_each
    (   | transform |
        {   let origin = transform.translation;
            gizmos.linestrip( [ origin + Vec3::NEG_X, origin + Vec3::X, origin + Vec3::X * 0.8 + ( Vec3::ONE - Vec3::X ) * 0.05 ], Color::RED   );
            gizmos.linestrip( [ origin + Vec3::NEG_Y, origin + Vec3::Y, origin + Vec3::Y * 0.8 + ( Vec3::ONE - Vec3::Y ) * 0.05 ], Color::GREEN );
            gizmos.linestrip( [ origin + Vec3::NEG_Z, origin + Vec3::Z, origin + Vec3::Z * 0.8 + ( Vec3::ONE - Vec3::Z ) * 0.05 ], Color::BLUE  );
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//テキストUIを表示する
pub fn spawn_grid_layout_ui
(   // camera2d_id: Query<Entity, With<misc::CameraDefault2d>>,
    // camera3d_id: Query<Entity, With<misc::CameraDefault3d>>,
    mut cmds: Commands,
)
{   //ウィンドウ全体の隠しノードを作成する(グリッドレイアウト３列×３行)
    let style = Style
    {   width : Val::Percent( 100.0 ),
        height: Val::Percent( 100.0 ),
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::fr( 3, 1.0 ),
        border: UiRect::all( Val::Px( 1.0 ) ), //ボーダーライン表示用
        ..default()
    };
    let hidden_node = NodeBundle
    {   style,
        background_color: Color::NONE.into(),
        border_color    : Color::RED.into(), //ボーダーラインの色
        ..default()
    };

    //UIを描画するカメラを指定する(優先:Camera2dDefault)
    // #[allow(clippy::suspicious_else_formatting)]
    // let ui_camera_id =
    //     if let Ok ( id ) = camera2d_id.get_single() { id } else
    //     if let Ok ( id ) = camera3d_id.get_single() { id } else { return };

    //隠しノードの中に子ノードをspawnする
    cmds.spawn( hidden_node )
    // .insert( TargetCamera ( ui_camera_id ) )
    .with_children
    (   |cmds|
        {   cmds.spawn( text_ui( "TOP/LEFT"    , AlignSelf::Start , JustifySelf::Start  ) );
            cmds.spawn( text_ui( "TOP"         , AlignSelf::Start , JustifySelf::Center ) );
            cmds.spawn( text_ui( "TOP/RIGHT"   , AlignSelf::Start , JustifySelf::End    ) );

            cmds.spawn( text_ui( "LEFT"        , AlignSelf::Center, JustifySelf::Start  ) );
            cmds.spawn( text_ui( "CENTER"      , AlignSelf::Center, JustifySelf::Center ) );
            cmds.spawn( text_ui( "RIGHT"       , AlignSelf::Center, JustifySelf::End    ) );

            cmds.spawn( text_ui( "BOTTOM/LEFT" , AlignSelf::End   , JustifySelf::Start  ) );
            cmds.spawn( text_ui( "BOTTOM"      , AlignSelf::End   , JustifySelf::Center ) );
            cmds.spawn( text_ui( "BOTTOM/RIGHT", AlignSelf::End   , JustifySelf::End    ) );
        }
    );
}

//TextBundleを作る
fn text_ui
(   text: &str,
    align_self  : AlignSelf,
    justify_self: JustifySelf,
) -> TextBundle
{   let style = TextStyle { font_size: PIXELS_PER_GRID, color: Color::GRAY, ..default() };
    let value = text.to_string();
    let sections = vec![ TextSection { style, value } ];

    let text = Text { sections, ..default() };
    let style = Style
    {   align_self,   //グリッドのセルでの縦位置の寄せ
        justify_self, //グリッドのセルでの横位置の寄せ
        ..default()
    };

    TextBundle { text, style,..default() }
}

////////////////////////////////////////////////////////////////////////////////

//極座標に従って3D Cameraを移動する
//＜副作用＞ Res<OrbitCamera>が見つからない場合、Resouceを登録する
pub fn move_orbit_camera
(   mut que_camera: Query<( &mut Transform, &Camera ), With<Camera3d>>,
    opt_orbit_camera: Option<Res<OrbitCamera>>,
    mut cmds: Commands,
)
{   let Ok ( ( mut transform, camera ) ) = que_camera.get_single_mut() else { return };

    //カメラのResourceの有無で処理を分ける
    let mut orbit_camera;
    if let Some ( res_orbit_camera ) = opt_orbit_camera
    {   orbit_camera = *res_orbit_camera; //既存のResourceを使用する
    }
    else
    {   orbit_camera = OrbitCamera::default();     //OrbitCameraを作る
        orbit_camera.is_active = camera.is_active; //現時点のカメラ状態を保存
        cmds.insert_resource( orbit_camera );      //Resourceを登録する
    };

    //アクティブでないなら更新しない
    if ! orbit_camera.is_active { return }

    //カメラの位置と向きを更新する
    let origin = orbit_camera.look_at;
    let vec3 = orbit_camera.orbit.to_vec3() + origin;
    *transform = Transform::from_translation( vec3 ).looking_at( origin, Vec3::Y );
}

////////////////////////////////////////////////////////////////////////////////

//キーの定義
const DBGINP_KEY_UP     : KeyCode = KeyCode::ArrowUp;
const DBGINP_KEY_DOWN   : KeyCode = KeyCode::ArrowDown;
const DBGINP_KEY_LEFT   : KeyCode = KeyCode::ArrowLeft;
const DBGINP_KEY_RIGHT  : KeyCode = KeyCode::ArrowRight;
const DBGINP_KEY_ZOOMIN : KeyCode = KeyCode::KeyZ;
const DBGINP_KEY_ZOOMOUT: KeyCode = KeyCode::KeyX;

//ゲームパッドのボタンの定義
const DBGINP_PAD_UP     : GamepadButtonType = GamepadButtonType::DPadUp;
const DBGINP_PAD_DOWN   : GamepadButtonType = GamepadButtonType::DPadDown;
const DBGINP_PAD_LEFT   : GamepadButtonType = GamepadButtonType::DPadLeft;
const DBGINP_PAD_RIGHT  : GamepadButtonType = GamepadButtonType::DPadRight;
const DBGINP_PAD_ZOOMIN : GamepadButtonType = GamepadButtonType::LeftTrigger2;
const DBGINP_PAD_ZOOMOUT: GamepadButtonType = GamepadButtonType::RightTrigger2;

//ゲームパッドのスティックの定義
const DBGINP_PAD_AXIS_X : GamepadAxisType = GamepadAxisType::LeftStickX;
const DBGINP_PAD_AXIS_Y : GamepadAxisType = GamepadAxisType::LeftStickY;

////////////////////////////////////////////////////////////////////////////////

//キー入力によって極座標カメラの位置を更新する
pub fn catch_input_keyboard
(   opt_orbit_camera: Option<ResMut<OrbitCamera>>,
    time: Res<Time>,
    inkey: Res<ButtonInput<KeyCode>>,
)
{   let Some ( mut camera ) = opt_orbit_camera else { return };
    if ! camera.is_active { return } //アクティブでないなら更新しない

    //準備
    let orbit = &mut camera.orbit;
    let time_delta = time.delta().as_secs_f32() * 5.0; //前回の実行からの経過時間

    //極座標を更新する
    for keycode in inkey.get_pressed()
    {   match *keycode
        {   DBGINP_KEY_ZOOMOUT =>
                orbit.r = ( orbit.r + time_delta ).min( CAMERA_ORBIT_MAX_R ),
            DBGINP_KEY_ZOOMIN =>
                orbit.r = ( orbit.r - time_delta ).max( CAMERA_ORBIT_MIN_R ),
            DBGINP_KEY_UP =>
                orbit.theta = ( orbit.theta + time_delta ).min( CAMERA_ORBIT_MAX_THETA ),
            DBGINP_KEY_DOWN =>
                orbit.theta = ( orbit.theta - time_delta ).max( CAMERA_ORBIT_MIN_THETA ),
            DBGINP_KEY_RIGHT =>
            {   orbit.phi += time_delta;
                orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
            }
            DBGINP_KEY_LEFT =>
            {   orbit.phi -= time_delta;
                orbit.phi += if orbit.phi < 0.0 { TAU } else { 0.0 };
            }
            _ => (),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//マウス入力によって極座標カメラの位置を更新する
pub fn catch_input_mouse
(   opt_orbit_camera: Option<ResMut<OrbitCamera>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut evt_mouse_motion: EventReader<MouseMotion>,
    mut evt_mouse_wheel : EventReader<MouseWheel>,
)
{   let Some ( mut camera ) = opt_orbit_camera else { return };
    if ! camera.is_active { return } //アクティブでないなら更新しない

    //準備
    let orbit = &mut camera.orbit;

    //ホイールで極座標を更新する
    for mouse_wheel in evt_mouse_wheel.read()
    {   orbit.r += mouse_wheel.y * 0.2; //感度良すぎるので
        orbit.r = orbit.r.min( CAMERA_ORBIT_MAX_R ).max( CAMERA_ORBIT_MIN_R );
    }

    //右ボタンが押されていないなら
    if ! mouse_button.pressed( MouseButton::Left ) { return }

    //マウスの上下左右で極座標を更新する
    for mouse_motion in evt_mouse_motion.read()
    {   //上下
        orbit.theta += mouse_motion.delta.y * 0.01; //感度良すぎるので
        orbit.theta = orbit.theta.min( CAMERA_ORBIT_MAX_THETA ).max( CAMERA_ORBIT_MIN_THETA );

        //左右
        orbit.phi -= mouse_motion.delta.x * 0.01; //感度良すぎるので
        orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
        orbit.phi += if orbit.phi <  0.0 { TAU } else { 0.0 };
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームパッドによって極座標カメラの位置を更新する
pub fn catch_input_gamepad
(   opt_orbit_camera: Option<ResMut<OrbitCamera>>,
    gamepad: Res<TargetGamepad>,
    time: Res<Time>,
    inbtn: Res<ButtonInput<GamepadButton>>,
    axis_button: Res<Axis<GamepadButton>>,
    axis_stick : Res<Axis<GamepadAxis>>,
)
{   let Some ( mut camera ) = opt_orbit_camera else { return };
    let Some ( gamepad ) = gamepad.id() else { return };
    if ! camera.is_active { return } //アクティブでないなら更新しない

    //準備
    let orbit = &mut camera.orbit;
    let time_delta = time.delta().as_secs_f32() * 5.0; //前回の実行からの経過時間

    //ズームイン
    let button_type = DBGINP_PAD_ZOOMIN;
    let button = GamepadButton { gamepad, button_type };
    if let Some ( value ) = axis_button.get( button )
    {   orbit.r -= value * time_delta;
        orbit.r = orbit.r.max( CAMERA_ORBIT_MIN_R );
    }

    //ズームアウト
    let button_type = DBGINP_PAD_ZOOMOUT;
    let button = GamepadButton { gamepad, button_type };
    if let Some ( value ) = axis_button.get( button )
    {   orbit.r += value * time_delta;
        orbit.r = orbit.r.min( CAMERA_ORBIT_MAX_R );
    }

    //上下（スティック入力を優先）
    let mut flag = false;
    let stick_y = GamepadAxis { gamepad, axis_type: DBGINP_PAD_AXIS_Y };
    if let Some ( value ) = axis_stick.get( stick_y )
    {   if value != 0.0
        {   orbit.theta += value * time_delta;
            orbit.theta = orbit.theta.min( CAMERA_ORBIT_MAX_THETA ).max( CAMERA_ORBIT_MIN_THETA );
            flag = true;
        }
    }
    if ! flag
    {   let button_type = DBGINP_PAD_UP;
        if inbtn.pressed( GamepadButton { gamepad, button_type } )
        {   orbit.theta = ( orbit.theta + time_delta ).min( CAMERA_ORBIT_MAX_THETA );
        }
        let button_type = DBGINP_PAD_DOWN;
        if inbtn.pressed( GamepadButton { gamepad, button_type } )
        {   orbit.theta = ( orbit.theta - time_delta ).max( CAMERA_ORBIT_MIN_THETA );
        }
    }

    //左右（スティック入力を優先）
    let mut flag = false;
    let stick_x = GamepadAxis { gamepad, axis_type: DBGINP_PAD_AXIS_X };
    if let Some ( value ) = axis_stick.get( stick_x )
    {   if value != 0.0
        {   orbit.phi += value * time_delta;
            orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
            orbit.phi += if orbit.phi <  0.0 { TAU } else { 0.0 };
            flag = true;
        }
    }
    if ! flag
    {   let button_type = DBGINP_PAD_RIGHT;
        if inbtn.pressed( GamepadButton { gamepad, button_type } )
        {   orbit.phi += time_delta;
            orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
        }
        let button_type = DBGINP_PAD_LEFT;
        if inbtn.pressed( GamepadButton { gamepad, button_type } )
        {   orbit.phi -= time_delta;
            orbit.phi += if orbit.phi < 0.0 { TAU } else { 0.0 };
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.