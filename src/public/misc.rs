use super::*;

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用
pub const DEBUG: fn() -> bool = || cfg!( debug_assertions );
pub const WASM : fn() -> bool = || cfg!( target_arch = "wasm32" );

////////////////////////////////////////////////////////////////////////////////

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() )
    // .insert( Camera { order: CAMERA2D_ORDER, ..default() } )
    // .insert( Camera2d { clear_color: CAMERA2D_BGCOLOR } )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut q_window: Query<&mut Window>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    o_now_gamepad: Option<Res<NowGamepad>>,
)
{   //トラブル除け
    let Ok( mut window ) = q_window.get_single_mut() else { return };

    //キーの状態
    let mut is_key_pressed = false;
    for ( o_modifier, key ) in FULL_SCREEN_KEYS
    {   if inkey.just_pressed( key )
        {   if let Some ( modifier ) = o_modifier
            {   //修飾キーが設定されている＆押されていない
                if ! inkey.pressed( modifier ) { continue }
            }
            is_key_pressed = true;
            break;
        }
    }

    //パッドのボタンの状態
    let mut is_btn_pressed = false;
    if let Some ( now_gamepad ) = o_now_gamepad
    {   if let Some ( gamepad ) = now_gamepad.0
        {   if inbtn.just_pressed( GamepadButton::new( gamepad, FULL_SCREEN_BUTTON ) )
            {   is_btn_pressed = true;
            }
        }
    }

    //入力がないなら
    if ! is_key_pressed && ! is_btn_pressed { return }

    //ウィンドウとフルスクリーンを切り替える(トグル処理)
    window.mode = match window.mode
    {   Windowed => SizedFullscreen,
        _        => Windowed,
    };
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたEnityを再帰的に削除する
pub fn despawn_entity<T: Component>
(   q: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   q.for_each( | ent | cmds.entity( ent ).despawn_recursive() );
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたComponentを見せる
pub fn show_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{// q.for_each_mut( | mut vis | *vis = Visibility::Inherited );
    q.for_each_mut( | mut vis | *vis = Visibility::Visible );
}

//QueryしたComponentを隠す
pub fn hide_component<T: Component>
(   mut q: Query<&mut Visibility, With<T>>,
)
{   q.for_each_mut( | mut vis | *vis = Visibility::Hidden );
}

////////////////////////////////////////////////////////////////////////////////

//指定の情報からTextBundleを作る
pub fn text_ui
(   message: &[ MessageSect ],
    alignment: TextAlignment,
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
    let position_type = PositionType::Absolute;

    let text  = Text { sections, alignment, ..default() };
    let style = Style { position_type, ..default() };
    TextBundle { text, style, ..default() }
}

////////////////////////////////////////////////////////////////////////////////

//操作を受け付けるgamepadを切り替える
pub fn catch_gamepad_connection_changes
(   o_now_gamepad: Option<ResMut<NowGamepad>>,
    gamepads: Res<Gamepads>,
)
{   //トラブル除け
    let Some ( mut now_gamepad ) = o_now_gamepad else { return };
    
    //記録済のgamepadがまだ接続中なら
    if now_gamepad.0
        .is_some_and( | gamepad | gamepads.contains( gamepad ) ) { return }

    //gamepadが一つも接続されていないなら
    if gamepads.iter().count() == 0
    {   if now_gamepad.0.is_some()
        {   //gamepadが取り外された場合
            now_gamepad.0 = None;
        }
        return;
    }

    //gamepadsの中から1つ取り出して記録する
    if let Some ( gamepad ) = gamepads.iter().next()
    {   now_gamepad.0 = Some ( gamepad );
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of cooe.