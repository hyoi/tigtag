#![allow( dead_code )]

use super::*;

////////////////////////////////////////////////////////////////////////////////

//スプライトの設定
const COLOR_SPRITE_DEBUG_GRID: Color = Color::rgba( 1.0, 1.0, 1.0, 0.01 );

//マス目状にスプライトを敷き詰める
pub fn spawn_2d_sprites
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let color = COLOR_SPRITE_DEBUG_GRID;
    let custom_size = Some ( SIZE_GRID );

    for x in SCREEN_GRIDS_X_RANGE
    {   for y in SCREEN_GRIDS_Y_RANGE
        {   let vec2 = IVec2::new( x, y ).to_sprite_pixels();
            let vec3 = vec2.extend( DEPTH_SPRITE_DEBUG_GRID );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            .insert( asset_svr.load( ASSETS_SPRITE_DEBUG_GRID ) as Handle<Image> )
            .with_children
            (   | cmds |
                {   let value = format!( "{:02}\n{:02}", x, y ).to_string();
                    let style = TextStyle
                    {   font     : asset_svr.load( ASSETS_FONT_PRESSSTART2P_REGULAR ),
                        font_size: PIXELS_PER_GRID * 0.25,
                        color    : Color::DARK_GREEN,
                    };
                    let sections  = vec![ TextSection { value, style } ];
                    let alignment = TextAlignment::Center;

                    cmds.spawn( Text2dBundle::default() )
                    .insert( Text { sections, alignment, ..default() } )
                    .insert( Transform::from_translation( Vec3::Z ) )
                    ;
                }
            );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// //3Dオブジェクトの設定
// const SIZE_OBJ3D_DEBUG_PLANE: f32 = 5.0; //地面の縦横のサイズ
// const SIZE_OBJ3D_DEBUG_CUBE : f32 = 1.0; //立方体の辺のサイズ
// const COLOR_OBJ3D_DEBUG_PLANE: Color = Color::rgb( 0.3, 0.5, 0.3 ); //地面の色
// const COLOR_OBJ3D_DEBUG_CUBE : Color = Color::rgb( 0.8, 0.7, 0.6 ); //正方形の色

// //3Dオブジェクトの配置
// pub fn spawn_3d_objects
// (   mut cmds: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// )
// {   //立方体
//     cmds.spawn( PbrBundle::default() )
//     .insert( meshes.add( shape::Cube::new( SIZE_OBJ3D_DEBUG_CUBE ).into() ) )
//     .insert( Transform::from_translation( Vec3::ZERO ) ) //原点(全軸0.0)
//     .insert( materials.add( COLOR_OBJ3D_DEBUG_CUBE.into() ) )
//     ;

//     //地面
//     cmds.spawn( PbrBundle::default() )
//     .insert( meshes.add( shape::Plane::from_size( SIZE_OBJ3D_DEBUG_PLANE ).into() ) )
//     .insert( Transform::from_translation( Vec3::Y / -2.0 ) ) //Y軸を0.5下方へ移動
//     .insert( materials.add( COLOR_OBJ3D_DEBUG_PLANE.into() ) )
//     ;
// }

// ////////////////////////////////////////////////////////////////////////////////

// //ゲームパッドによって極座標カメラの位置を更新する
// pub fn catch_input_gamepad
// (   o_camera: Option<ResMut<OrbitCamera>>,
//     time: Res<Time>,
//     axis_button: Res<Axis<GamepadButton>>,
//     axis_stick : Res<Axis<GamepadAxis>>,
//     gamepads: Res<Gamepads>,
// )
// {   let Some ( mut camera ) = o_camera else { return };
//     if ! camera.is_active { return } //アクティブでないなら更新しない

//     //準備
//     let orbit = &mut camera.orbit;
//     let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間

//     //ゲームパッドは抜き挿しでIDが変わるので、
//     //.iter()で回しながら極座標を更新する
//     for gamepad in gamepads.iter()
//     {   //左トリガーでズームイン
//         let button_type = GamepadButtonType::LeftTrigger2;
//         let button = GamepadButton { gamepad, button_type };
//         if let Some ( value ) = axis_button.get( button )
//         {   orbit.r -= value * time_delta;
//             orbit.r = orbit.r.max( ORBIT_CAMERA_MIN_R );
//         }

//         //右トリガーでズームアウト
//         let button_type = GamepadButtonType::RightTrigger2; 
//         let button = GamepadButton { gamepad, button_type };
//         if let Some ( value ) = axis_button.get( button )
//         {   orbit.r += value * time_delta;
//             orbit.r = orbit.r.min( ORBIT_CAMERA_MAX_R );
//         }

//         //左スティックのＹ軸で上下首振り
//         let axis_type = GamepadAxisType::LeftStickY;
//         let stick_y = GamepadAxis { gamepad, axis_type };
//         if let Some ( value ) = axis_stick.get( stick_y )
//         {   orbit.theta += value * time_delta;
//             orbit.theta = orbit.theta
//                 .min( ORBIT_CAMERA_MAX_THETA )
//                 .max( ORBIT_CAMERA_MIN_THETA );
//         }

//         //左スティックのＸ軸で左右回転
//         let axis_type = GamepadAxisType::LeftStickX;
//         let stick_x = GamepadAxis { gamepad, axis_type };
//         if let Some ( value ) = axis_stick.get( stick_x )
//         {   orbit.phi += value * time_delta;
//             orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
//             orbit.phi += if orbit.phi <  0.0 { TAU } else { 0.0 };
//         }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //マウス入力によって極座標カメラの位置を更新する
// pub fn catch_input_mouse
// (   o_camera: Option<ResMut<OrbitCamera>>,
//     mouse_button: Res<Input<MouseButton>>,
//     mut e_mouse_motion: EventReader<mouse::MouseMotion>,
//     mut e_mouse_wheel: EventReader<mouse::MouseWheel>,
// )
// {   let Some ( mut camera ) = o_camera else { return };
//     if ! camera.is_active { return } //アクティブでないなら更新しない

//     //準備
//     let orbit = &mut camera.orbit;

//     //ホイールで極座標を更新する
//     for mouse_wheel in e_mouse_wheel.iter()
//     {   orbit.r += mouse_wheel.y * MOUSE_WHEEL_Y_COEF; //感度良すぎるので
//         orbit.r = orbit.r
//             .min( ORBIT_CAMERA_MAX_R )
//             .max( ORBIT_CAMERA_MIN_R );
//     }

//     //右ボタンが押されていないなら
//     if ! mouse_button.pressed( MouseButton::Left ) { return }

//     //マウスの上下左右で極座標を更新する
//     for mouse_motion in e_mouse_motion.iter()
//     {   //上下首振り
//         orbit.theta += mouse_motion.delta.y * MOUSE_MOTION_Y_COEF; //感度良すぎるので
//         orbit.theta = orbit.theta
//             .min( ORBIT_CAMERA_MAX_THETA )
//             .max( ORBIT_CAMERA_MIN_THETA );

//         //左右回転
//         orbit.phi -= mouse_motion.delta.x * MOUSE_MOTION_X_COEF; //感度良すぎるので
//         orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
//         orbit.phi += if orbit.phi <  0.0 { TAU } else { 0.0 };
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //キー入力によって極座標カメラの位置を更新する
// pub fn catch_input_keyboard
// (   o_camera: Option<ResMut<OrbitCamera>>,
//     time: Res<Time>,
//     inkey: Res<Input<KeyCode>>,
// )
// {   let Some ( mut camera ) = o_camera else { return };
//     if ! camera.is_active { return } //アクティブでないなら更新しない

//     //準備
//     let orbit = &mut camera.orbit;
//     let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間

//     //極座標を更新する
//     for keycode in inkey.get_pressed()
//     {   match keycode
//         {   KeyCode::X =>
//                 orbit.r = ( orbit.r + time_delta ).min( ORBIT_CAMERA_MAX_R ),
//             KeyCode::Z =>
//                 orbit.r = ( orbit.r - time_delta ).max( ORBIT_CAMERA_MIN_R ),
//             KeyCode::Up =>
//                 orbit.theta = ( orbit.theta + time_delta ).min( ORBIT_CAMERA_MAX_THETA ),
//             KeyCode::Down =>
//                 orbit.theta = ( orbit.theta - time_delta ).max( ORBIT_CAMERA_MIN_THETA ),
//             KeyCode::Right =>
//             {   orbit.phi += time_delta;
//                 orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
//             }
//             KeyCode::Left =>
//             {   orbit.phi -= time_delta;
//                 orbit.phi += if orbit.phi < 0.0 { TAU } else { 0.0 };
//             }
//             _ => (),
//         }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //極座標に従って3D Cameraを移動する
// //＜副作用＞ OrbitCameraが見つからない場合、Resouceを作成する
// pub fn move_orbit_camera<T: Component>
// (   mut que_camera: Query<(&mut Transform, &Camera), With<T>>,
//     opt_orbit_camera: Option<Res<OrbitCamera>>,
//     mut cmds: Commands,
// )
// {   let Ok ( ( mut transform, camera ) ) = que_camera.get_single_mut() else { return };

//     //カメラのResourceの有無で処理を分ける
//     let mut orbit_camera;
//     if let Some ( res_orbit_camera ) = opt_orbit_camera
//     {   orbit_camera = *res_orbit_camera; //Resourceを使用する
//     }
//     else
//     {   orbit_camera = OrbitCamera::default();
//         orbit_camera.is_active = camera.is_active; //現時点のカメラ状態を保存
//         cmds.insert_resource( orbit_camera ); //Resourceを登録する
//     };

//     //アクティブでないなら更新しない
//     if ! orbit_camera.is_active { return }

//     //カメラの位置と向きを更新する
//     let origin = orbit_camera.look_at;
//     let vec3 = orbit_camera.orbit.to_vec3() + origin;
//     *transform = Transform::from_translation( vec3 ).looking_at( origin, Vec3::Y );
// }

////////////////////////////////////////////////////////////////////////////////

//End of code.