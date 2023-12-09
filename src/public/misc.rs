use super::*;

////////////////////////////////////////////////////////////////////////////////

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   //2Dカメラを第四象限に移動する。
    //左↑隅が(0,0)、X軸はプラス方向へ伸び、Y軸はマイナス方向へ伸びる
    let translation = Vec3::X * SCREEN_PIXELS_WIDTH  * 0.5
                    - Vec3::Y * SCREEN_PIXELS_HEIGHT * 0.5;

    //タイトルバーのWクリックや最大化ボタンによるウィンドウ最大化時に
    //表示が著しく崩れることを緩和するためviewportを設定しておく(根本的な対策ではない)
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

//操作を受け付けるgamepadを切り替える
pub fn choose_gamepad_connection
(   opt_gamepad: Option<ResMut<ConnectedGamepad>>,
    gamepads: Res<Gamepads>,
)
{   let Some ( mut gamepad ) = opt_gamepad else { return };

    //IDが保存されている場合
    if let Some ( id ) = gamepad.id()
    {   //IDのgamepadがまだ接続されている
        if gamepads.contains( id ) { return }

        //gamepadが１つも接続されていない
        if gamepads.iter().count() == 0
        {   *gamepad.id_mut() = None; //IDが無効
            return;
        }
    }

    //gamepadsから１つ取り出してIDを保存する
    *gamepad.id_mut() = gamepads.iter().next();

    #[cfg( debug_assertions )]
    if gamepad.id().is_some() { dbg!( gamepad.id() ); } //for debug
}

////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut qry_window: Query<&mut Window>,
    opt_gamepad: Option<Res<ConnectedGamepad>>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   let Ok( mut window ) = qry_window.get_single_mut() else { return };
    let mut is_pressed = false;

    //キーの状態
    if inkey.just_pressed( FULL_SCREEN_KEY )
    {   //装飾キー
        for key in FULL_SCREEN_KEY_MODIFIER
        {   if inkey.pressed( *key )
            {   is_pressed = true;
                break;
            }
        }
    }

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
    let alignment = TextAlignment::Center;
    let position_type = PositionType::Absolute;

    let text  = Text { sections, alignment, ..default() };
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

//スプライトをアニメーションさせる
pub fn animating_sprites
(   mut qry_target: Query<( &mut TextureAtlasSprite, &mut AnimationParams )>,
    time: Res<Time>,
)
{   for ( mut anime_sprite, mut anime_params ) in &mut qry_target
    {   if anime_params.timer.tick( time.delta() ).just_finished()
        {   anime_sprite.index += 1;
            if anime_sprite.index >= anime_params.frame_count
            {   anime_sprite.index = 0;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
