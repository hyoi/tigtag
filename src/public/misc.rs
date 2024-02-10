use super::*;

////////////////////////////////////////////////////////////////////////////////

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   //2Dカメラを第四象限に移動する。
    //左↑隅が(0,0)、X軸はプラス方向へ伸び、Y軸はマイナス方向へ伸びる
    let translation = Vec3::X * SCREEN_PIXELS_WIDTH  * 0.5
                    - Vec3::Y * SCREEN_PIXELS_HEIGHT * 0.5;

    cmds.spawn( Camera2dBundle::default() )
    .insert( Transform::from_translation( translation) )
    ;
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
        let Some ( id ) = gamepad.id() else { return };     //gamepad未接続
        is_pressed = inbtn.just_pressed( GamepadButton::new( id, FULL_SCREEN_BUTTON ) )
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
{   let Some ( state ) = opt_state else { warn!( "opt_state is None." ); return };

    next_state.set( state.next() );
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたEnityを再帰的に削除する
pub fn despawn<T: Component>
(   qry_entity: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   qry_entity.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );
}

//QueryしたComponentを見せる
pub fn show<T: Component>
(   mut qry: Query<&mut Visibility, With<T>>,
)
{   qry.iter_mut().for_each( | mut vis | *vis = Visibility::Visible );
}

//QueryしたComponentを隠す
pub fn hide<T: Component>
(   mut qry: Query<&mut Visibility, With<T>>,
)
{   qry.iter_mut().for_each( | mut vis | *vis = Visibility::Hidden );
}

////////////////////////////////////////////////////////////////////////////////

//UI用のTextBundleを作る
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
    let justify = JustifyText::Center;
    let position_type = PositionType::Absolute;

    let text  = Text { sections, justify, ..default() };
    let style = Style { position_type, ..default() };
    TextBundle { text, style, ..default() }
}

//UIレイアウト用の隠しフレームを作る
pub fn hidden_ui_frame
(   justify_content: JustifyContent,
) -> NodeBundle
{   let mut style = Style
    {   width : Val::Px( SCREEN_PIXELS_WIDTH  ),
        height: Val::Px( SCREEN_PIXELS_HEIGHT ),
        flex_direction : FlexDirection::Column,
        align_items    : AlignItems::Center,
        justify_content,
        ..default()
    };
    if misc::DEBUG() { style.border = UiRect::all( Val::Px( 1.0 ) ); }

    let mut hidden_frame = NodeBundle
    {   style,
        background_color: Color::NONE.into(),
        ..default()
    };
    if misc::DEBUG() { hidden_frame.border_color = Color::RED.into() }

    hidden_frame
}

////////////////////////////////////////////////////////////////////////////////

// //キャラクターをアニメーションさせる
// pub fn animating_sprites<T: Component + CharacterAnimation>
// (   mut qry_target: Query<( &mut TextureAtlasSprite, &mut T )>,
//     time: Res<Time>,
// )
// {   for ( mut sprite, mut character ) in &mut qry_target
//     {   if character.anime_timer_mut().tick( time.delta() ).just_finished()
//         {   sprite.index += 1;
//             let offset = character.sprite_sheet_offset( character.direction() );
//             let frame  = character.sprite_sheet_frame();
//             if sprite.index >= offset + frame { sprite.index = offset }
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////

//End of code.