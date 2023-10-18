use super::*;

////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut qry_window: Query<&mut Window>,
    keys: Res<Input<KeyCode>>,
    gpdbtn: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
)
{   let Ok( mut window ) = qry_window.get_single_mut() else { return };

    //[Alt]＋[Enter]の状態
    let is_key_pressed =
        ( keys.pressed( KeyCode::AltRight ) || keys.pressed( KeyCode::AltLeft ) )
            && keys.just_pressed( KeyCode::Return );

    //ゲームパッドは抜き挿しでIDが変わるので.iter()で回す
    let button_type = GamepadButtonType::Select; //ps4[SHARE]
    let mut is_gpdbtn_pressed = false;
    for gamepad in gamepads.iter()
    {   if gpdbtn.just_pressed( GamepadButton { gamepad, button_type } )
        {   is_gpdbtn_pressed = true;
            break;
        }
    }

    //入力がないなら
    if ! is_key_pressed && ! is_gpdbtn_pressed { return }

    //ウィンドウとフルスクリーンを切り替える
    window.mode = match window.mode
    {   WindowMode::Windowed => WindowMode::SizedFullscreen,
        _                    => WindowMode::Windowed,
    };
}

////////////////////////////////////////////////////////////////////////////////

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   //2Dカメラを第一象限に移動する
    //左下隅が(0,0)、X軸はプラス方向へ伸び、Y軸はプラス方向へ上がる
    let translation = Vec3::X * SCREEN_PIXELS_WIDTH  * 0.5
                    + Vec3::Y * SCREEN_PIXELS_HEIGHT * 0.5;

    //タイトルバーのWクリックや最大化ボタンによるウィンドウ最大化時に
    //表示が著しく崩れることを緩和するためviewportを設定しておく
    let zero = UVec2::new( 0, 0 );
    let size = Vec2::new( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT );
    let viewport = Some
    (   camera::Viewport
        {   physical_position: zero,
            physical_size    : size.as_uvec2(),
            ..default()
        }
    );

    cmds.spawn( Camera2dBundle::default() )
    .insert( Camera { viewport, ..default() } )
    .insert( Transform::from_translation( translation) )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//Stateの無条件遷移
pub fn change_state<T: Send + Sync + Default + GotoState>
(   state: Local<T>,
    mut next_state: ResMut<NextState<MyState>>
)
{   next_state.set( state.next() );
}

//Stateの無条件遷移（Resourceで遷移先指定）
pub fn change_state_with_res<T: Resource + GotoState>
(   opt_state: Option<Res<T>>,
    mut next_state: ResMut<NextState<MyState>>
)
{   let Some ( state ) = opt_state else { warn!( "No exists State." ); return };

    next_state.set( state.next() );
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたEnityを再帰的に削除する
pub fn despawn<T: Component>
(   qry_entity: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   qry_entity.for_each( | id | cmds.entity( id ).despawn_recursive() );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.




// ////////////////////////////////////////////////////////////////////////////////

// //QueryしたEnityを再帰的に削除する
// pub fn despawn_entity<T: Component>
// (   q: Query<Entity, With<T>>,
//     mut cmds: Commands,
// )
// {   q.for_each( | ent | cmds.entity( ent ).despawn_recursive() );
// }

// ////////////////////////////////////////////////////////////////////////////////

// //QueryしたComponentを見せる
// pub fn show_component<T: Component>
// (   mut q: Query<&mut Visibility, With<T>>,
// )
// {// q.for_each_mut( | mut vis | *vis = Visibility::Inherited );
//     q.for_each_mut( | mut vis | *vis = Visibility::Visible );
// }

// //QueryしたComponentを隠す
// pub fn hide_component<T: Component>
// (   mut q: Query<&mut Visibility, With<T>>,
// )
// {   q.for_each_mut( | mut vis | *vis = Visibility::Hidden );
// }

// ////////////////////////////////////////////////////////////////////////////////

// //指定の情報からTextBundleを作る
// pub fn text_ui
// (   message: &[ MessageSect ],
//     alignment: TextAlignment,
//     asset_svr: &Res<AssetServer>,
// ) -> TextBundle
// {   let mut sections = Vec::new();
//     for ( line, file, size, color ) in message.iter()
//     {   let value = line.to_string();
//         let style = TextStyle
//         {   font     : asset_svr.load( *file ),
//             font_size: *size,
//             color    : *color
//         };
//         sections.push( TextSection { value, style } );
//     }
//     let position_type = PositionType::Absolute;

//     let text  = Text { sections, alignment, ..default() };
//     let style = Style { position_type, ..default() };
//     TextBundle { text, style, ..default() }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //操作を受け付けるgamepadを切り替える
// pub fn catch_gamepad_connection_changes
// (   o_now_gamepad: Option<ResMut<NowGamepad>>,
//     gamepads: Res<Gamepads>,
// )
// {   //トラブル除け
//     let Some ( mut now_gamepad ) = o_now_gamepad else { return };
    
//     //記録済のgamepadがまだ接続中なら
//     if now_gamepad.0
//         .is_some_and( | gamepad | gamepads.contains( gamepad ) ) { return }

//     //gamepadが一つも接続されていないなら
//     if gamepads.iter().count() == 0
//     {   if now_gamepad.0.is_some()
//         {   //gamepadが取り外された場合
//             now_gamepad.0 = None;
//         }
//         return;
//     }

//     //gamepadsの中から1つ取り出して記録する
//     if let Some ( gamepad ) = gamepads.iter().next()
//     {   now_gamepad.0 = Some ( gamepad );
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //End of cooe.