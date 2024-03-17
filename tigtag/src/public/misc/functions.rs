use super::*;

////////////////////////////////////////////////////////////////////////////////

//操作を受付けるgamepadを切り替える
pub fn change_gamepad_connection
(   opt_gamepad: Option<ResMut<TargetGamepad>>,
    gamepads: Res<Gamepads>,
)
{   let Some ( mut gamepad ) = opt_gamepad else { return };

    //IDが保存されている場合
    if let Some ( id ) = gamepad.id()
    {   //該当gamepadが接続中なら
        if gamepads.contains( id ) { return }

        //gamepadが接続されていない（＝全部外された）
        if gamepads.iter().count() == 0
        {   *gamepad.id_mut() = None;

            #[cfg( debug_assertions )]
            dbg!( gamepad.id() ); //for debug

            return;
        }
    }

    //接続中のものを１つ取り出して切り替える
    *gamepad.id_mut() = gamepads.iter().next();

    #[cfg( debug_assertions )]
    if gamepad.id().is_some() { dbg!( gamepad.id() ); } //for debug
}

////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut qry_window: Query<&mut Window>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    inkey: Res<ButtonInput<KeyCode>>,
    inbtn: Res<ButtonInput<GamepadButton>>,
)
{   let Ok( mut window ) = qry_window.get_single_mut() else { return };

    //キーの状態
    let mut is_pressed =
        inkey.just_pressed( FULL_SCREEN_KEY ) &&
        inkey.any_pressed( FULL_SCREEN_KEY_MODIFIER.iter().copied() );

    //ゲームパッドのボタンの状態
    if ! is_pressed
    {   let Some ( gamepad ) = opt_gamepad else { return }; //Resource未登録
        let Some ( id ) = gamepad.id() else { return };     //ゲームパッド未接続

        is_pressed = inbtn.just_pressed( GamepadButton::new( id, FULL_SCREEN_BUTTON ) )
    }

    if is_pressed
    {   window.mode = match window.mode
        {   WindowMode::Windowed => WindowMode::SizedFullscreen,
            _                    => WindowMode::Windowed,
        };
    }
}

////////////////////////////////////////////////////////////////////////////////

//デフォルトカメラのComponent
#[derive( Component )] pub struct Camera2dDefault;
#[derive( Component )] pub struct Camera3dDefault;

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   //タイトルバーのWクリックや最大化ボタンによるウィンドウ最大化時に
    //表示が著しく崩れることを緩和するためviewportを設定しておく
    // let zero = UVec2::new( 0, 0 );
    // let size = Vec2::new( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT );
    // let viewport = Some
    // (   bevy::render::camera::Viewport
    //     {   physical_position: zero,
    //         physical_size    : size.as_uvec2(),
    //         ..default()
    //     }
    // );

    //2Dカメラを第四象限にスライドする
    //左上隅が(0,0)、X軸はプラス方向へ伸び、Y軸はマイナス方向へ下がる
    let vec3 = Vec3::X     * SCREEN_PIXELS_WIDTH  * 0.5
             + Vec3::NEG_Y * SCREEN_PIXELS_HEIGHT * 0.5;

    cmds.spawn( ( Camera2dBundle::default(), Camera2dDefault ) )
    .insert( Camera
    {   order: CAMERA_ORDER_DEFAULT_2D,
        clear_color: CAMERA_BGCOLOR_2D,
        // viewport,
        ..default()
    } )
    .insert( Transform::from_translation( vec3 ) )
    ;
}

//3D cameraをspawnする
pub fn spawn_3d_camera( mut cmds: Commands )
{   //タイトルバーのWクリックや最大化ボタンによるウィンドウ最大化時に
    //表示が著しく崩れることを緩和するためviewportを設定しておく
    // let zero = UVec2::new( 0, 0 );
    // let size = Vec2::new( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT );
    // let viewport = Some
    // (   bevy::render::camera::Viewport
    //     {   physical_position: zero,
    //         physical_size    : size.as_uvec2(),
    //         ..default()
    //     }
    // );

    //3Dカメラの座標を初期化する（オービットカメラ）
    let vec3 = Orbit::default().to_vec3();

    cmds.spawn( ( Camera3dBundle:: default(), Camera3dDefault ) )
    .insert( Camera
    {   order: CAMERA_ORDER_DEFAULT_3D,
        clear_color: CAMERA_BGCOLOR_3D,
        // viewport,
        ..default()
    } )
    .insert( Transform::from_translation( vec3 ).looking_at( Vec3::ZERO, Vec3::Y ) )
    ;
}

//3D lightをspawnする
pub fn spawn_3d_light( mut cmds: Commands )
{   let illuminance = LIGHT_3D_BRIGHTNESS;
    let shadows_enabled = true;
    let transform = Transform::from_translation( LIGHT_3D_TRANSLATION );

    cmds.spawn( DirectionalLightBundle::default() )
    .insert( DirectionalLight { illuminance, shadows_enabled, ..default() } )
    .insert( transform.looking_at( Vec3::ZERO, Vec3::Y ) )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたEnityを再帰的に削除する
pub fn despawn_component<T: Component>
(   qry_entity: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   qry_entity.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );
}

pub fn despawn_by_filter<T: QueryFilter>
(   qry_entity: Query<Entity, T>,
    mut cmds: Commands,
)
{   qry_entity.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );
}

////////////////////////////////////////////////////////////////////////////////

//TextBundleを作る
pub fn text_ui
(   message: &[ MessageSect ],
    asset_svr: &Res<AssetServer>,
) -> TextBundle
{   let mut sections = Vec::new();
    for ( line, file, size, color ) in message.iter()
    {   let value = line.to_string();
        let style = TextStyle
        {   font     : asset_svr.load( *file ),
            font_size: *size,
            color    : *color
        };
        sections.push( TextSection { value, style } );
    }

    let text = Text { sections, ..default() };
    TextBundle { text, ..default() }
}

////////////////////////////////////////////////////////////////////////////////

// //Stateの無条件遷移
// pub fn change_state<T: Send + Sync + Default + ChangeState>
// (   next: Local<T>,
//     mut next_state: ResMut<NextState<MyState>>
// )
// {   next_state.set( next.state() );
// }

// //Stateの無条件遷移（Resourceで遷移先指定）
// pub fn next_state<T: Resource + ChangeState>
// (   opt_state: Option<Res<T>>,
//     mut next_state: ResMut<NextState<MyState>>
// )
// {   let Some ( next ) = opt_state else { warn!( "opt_state is None." ); return };

//     next_state.set( next.state() );
// }

////////////////////////////////////////////////////////////////////////////////

//QueryしたComponentを可視化する
pub fn show_component<T: Component>
(   mut qry: Query<&mut Visibility, With<T>>,
)
{   qry.iter_mut().for_each( | mut v | *v = Visibility::Visible );
}

//QueryしたComponentを不可視にする
pub fn hide_component<T: Component>
(   mut qry: Query<&mut Visibility, With<T>>,
)
{   qry.iter_mut().for_each( | mut v | *v = Visibility::Hidden );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.