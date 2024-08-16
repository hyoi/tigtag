#![allow( dead_code )]

use super::*;

////////////////////////////////////////////////////////////////////////////////

//デフォルトカメラのComponent
#[derive( Component )] pub struct CameraDefault2d;
#[derive( Component )] pub struct CameraDefault3d;

//デフォルト2Dカメラをspawnする
pub fn spawn_camera_2d( mut cmds: Commands )
{   //タイトルバーWクリックや最大化ボタンによるウィンドウ最大化、および
    //WASMでCanvasへのfit(最大化)を設定した場合に表示が著しく崩れることがある。
    //それを緩和するためカメラにviewportを設定しておく
    let zero = UVec2::new( 0, 0 );
    let size = Vec2::new( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT );
    let viewport = Some
    (   bevy::render::camera::Viewport
        {   physical_position: zero,
            physical_size    : size.as_uvec2(),
            ..default()
        }
    );

    cmds.spawn( ( Camera2dBundle::default(), CameraDefault2d ) )
    .insert( Camera
    {   order: CAMERA_ORDER_DEFAULT_2D,
        clear_color: CAMERA_BGCOLOR_2D,
        viewport,
        ..default()
    } )
    .insert( Transform::from_translation( CAMERA_POSITION_DEFAULT_2D ) )
    ;
}

//デフォルト3Dカメラをspawnする
pub fn spawn_camera_3d( mut cmds: Commands )
{   //タイトルバーWクリックや最大化ボタンによるウィンドウ最大化、および
    //WASMでCanvasへのfit(最大化)を設定した場合に表示が著しく崩れることがある。
    //それを緩和するためカメラにviewportを設定しておく
    let zero = UVec2::new( 0, 0 );
    let size = Vec2::new( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT );
    let viewport = Some
    (   bevy::render::camera::Viewport
        {   physical_position: zero,
            physical_size    : size.as_uvec2(),
            ..default()
        }
    );

    //3Dカメラの座標を初期化する（オービットカメラ）
    let vec3 = Orbit::default().to_vec3();

    cmds.spawn( ( Camera3dBundle:: default(), CameraDefault3d ) )
    .insert( Camera
    {   order: CAMERA_ORDER_DEFAULT_3D,
        clear_color: CAMERA_BGCOLOR_3D,
        viewport,
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

//[ESC]キー押し下げでアプリを終了する
//※bevy::window::close_on_escがv0.14.0で廃止になったため代替
//  remove close_on_esc #12859 https://github.com/bevyengine/bevy/pull/12859
pub fn close_on_esc
(   mut cmds: Commands,
    windows: Query< ( Entity, &Window ) >,
    input: Res< ButtonInput< KeyCode > >,
)
{   for ( id, window ) in windows.iter()
    {   if window.focused && input.just_pressed( KeyCode::Escape )
        {   cmds.entity( id ).despawn();
        }
    }
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