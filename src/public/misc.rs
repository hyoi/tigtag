use super::*;

////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut qry_window: Query<&mut Window>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
)
{   let Ok( mut window ) = qry_window.get_single_mut() else { return };
    let mut is_pressed = false;

    //キーの状態
    if inkey.just_pressed( FULL_SCREEN_KEY )
    {   for key in FULL_SCREEN_KEY_MODIFIER
        {   if inkey.pressed( key )
            {   is_pressed = true;
                break;
            }
        }
    }

    //ゲームパッドのボタンの状態
    if ! is_pressed
    {   for id in gamepads.iter()
        {   if inbtn.just_pressed( GamepadButton::new( id, FULL_SCREEN_BUTTON ) )
            {   is_pressed = true;
                break;
            }
        }
    }

    //ウィンドウとフルスクリーンを切り替える
    if is_pressed
    {   window.mode = match window.mode
        {   WindowMode::Windowed => WindowMode::SizedFullscreen,
            _                    => WindowMode::Windowed,
        };
    }
}

////////////////////////////////////////////////////////////////////////////////

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   //2Dカメラを第四象限に移動する
    //左↑隅が(0,0)、X軸はプラス方向へ伸び、Y軸はマイナス方向へ上がる
    let translation = Vec3::X * SCREEN_PIXELS_WIDTH  * 0.5
                    - Vec3::Y * SCREEN_PIXELS_HEIGHT * 0.5;

    //タイトルバーのWクリックや最大化ボタンによるウィンドウ最大化時に
    //表示が著しく崩れることを緩和するためviewportを設定しておく
    let zero = UVec2::new( 0, 0 );
    let size = Vec2::new( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).as_uvec2();
    let viewport = Some
    (   camera::Viewport
        {   physical_position: zero,
            physical_size    : size,
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

//QueryしたComponentを見せる
pub fn show<T: Component>
(   mut qry: Query<&mut Visibility, With<T>>,
)
{   qry.for_each_mut( | mut vis | *vis = Visibility::Visible );
}

//QueryしたComponentを隠す
pub fn hide<T: Component>
(   mut qry: Query<&mut Visibility, With<T>>,
)
{   qry.for_each_mut( | mut vis | *vis = Visibility::Hidden );
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
    let alignment = TextAlignment::Center;
    let position_type = PositionType::Absolute;

    let text  = Text { sections, alignment, ..default() };
    let style = Style { position_type, ..default() };
    TextBundle { text, style, ..default() }
}

////////////////////////////////////////////////////////////////////////////////

//操作を受け付けるgamepadを切り替える
pub fn catch_gamepad_connection
(   opt_gamepad_id: Option<ResMut<EnabledGamepadId>>,
    gamepads: Res<Gamepads>,
)
{   let Some ( mut gamepad_id ) = opt_gamepad_id else { return };
    
    //記録済のgamepadがまだ接続中なら
    if gamepad_id.0.is_some_and( | id | gamepads.contains( id ) ) { return }

    //一つも接続されていないなら
    if gamepads.iter().count() == 0
    {   if gamepad_id.0.is_some() { gamepad_id.0 = None }
        return;
    }

    //gamepadsの中から1つ取り出して記録する
    if let Some ( id ) = gamepads.iter().next()
    {   gamepad_id.0 = Some ( id );
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
