use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの入力を保存するResource
#[derive(Resource)]
pub struct InputDirection(pub Vec<News>, pub LockFlag);

impl Default for InputDirection
{
    fn default() -> Self
    {
        Self(Vec::with_capacity(4), LockFlag::default()) // 十字方向
    }
}

// スピードバグ防止フラグ
#[derive(Default, Clone)]
pub struct LockFlag
{
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

// コールバック関数の型
pub type CallBack = fn(&mut InputDirection, f32);

// コールバック関数
pub use callback::*;

#[allow( dead_code )]
#[rustfmt::skip]
pub mod callback
{   use super::*;

    pub fn move_up( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.up || value == 0.0 { return; }
        // orbit.position.theta += value;
        orbit.0.push( News::North );
        orbit.1.up = true;
    }
    pub fn move_down( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.down || value == 0.0 { return; }
        // orbit.position.theta -= value;
        orbit.0.push( News::South );
        orbit.1.down = true;
    }
    pub fn move_right( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.right || value == 0.0 { return; }
        // orbit.position.phi += value;
        orbit.0.push( News::East );
        orbit.1.right = true;
    }
    pub fn move_left( orbit: &mut InputDirection, value: f32 )
    {
        if orbit.1.left || value == 0.0 { return; }
        // orbit.position.phi -= value;
        orbit.0.push( News::West );
        orbit.1.left = true;
    }

    pub fn axis_x_normal( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.right )
            || ( value < 0.0 && orbit.1.left  ) { return; }
        // orbit.position.phi += value;
        if value > 0.0
        {
            orbit.0.push( News::East );
            orbit.1.right = true;
        } else
        {
            orbit.0.push( News::West );
            orbit.1.left = true;
        }
    }
    pub fn axis_x_reverse( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.left  )
            || ( value < 0.0 && orbit.1.right ) { return; }
        // orbit.position.phi += - value;
        if value >= 0.0
        {
            orbit.0.push( News::West );
            orbit.1.left = true;
        } else
        {
            orbit.0.push( News::East );
            orbit.1.right = true;
        }
    }
    pub fn axis_y_normal( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.up   )
            || ( value < 0.0 && orbit.1.down ) { return; }
        // orbit.position.theta += value;
        if value >= 0.0
        {
            orbit.0.push( News::North );
            orbit.1.up = true;
        } else
        {
            orbit.0.push( News::South );
            orbit.1.down = true;
        }
    }
    pub fn axis_y_reverse( orbit: &mut InputDirection, value: f32 )
    {
        if ( value == 0.0 )
            || ( value > 0.0 && orbit.1.down )
            || ( value < 0.0 && orbit.1.up   ) { return; }
        // orbit.position.theta += - value;
        if value >= 0.0
        {
            orbit.0.push( News::South );
            orbit.1.down = true;
        } else
        {
            orbit.0.push( News::North );
            orbit.1.up = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// キーマップ登録用Resource
#[derive(Resource, Deref)]
pub struct KeyMap(pub FxHashMap<KeyCode, CallBack>);

// ゲームパッドボタンマップ登録用Resource
#[derive(Resource, Deref)]
pub struct PadMap(pub FxHashMap<GamepadInput, CallBack>);

////////////////////////////////////////////////////////////////////////////////

// 極座標カメラの位置をキー入力で操作
pub fn input_from_keyboard(
    // opt_orbit_camera: Option<ResMut<OrbitCamera>>,
    opt_input_direction: Option<ResMut<InputDirection>>,
    opt_keymap: Option<Res<KeyMap>>,
    time: Res<Time>,
    input_keycode: Res<ButtonInput<KeyCode>>,
) -> Result
{
    // 準備
    let mut camera =
        opt_input_direction.ok_or("ResMut<InputDirection> not found.")?;
    let keymap = opt_keymap.ok_or("Res<KeyMap> not found.")?;

    // 前回の実行からの経過時間（感度調整の係数あり）
    let time_delta = time.delta_secs() /* * COEF_KEY_TIME_DELTA */;

    // キー入力とキーマップを使って極座標を更新する
    input_keycode.get_pressed().for_each(|keycode| {
        if let Some(callback) = keymap.get(keycode)
        {
            callback(&mut camera, time_delta);
        }
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 極座標カメラの位置をゲームパッドで操作
pub fn input_from_gamepad(
    // opt_orbit_camera: Option<ResMut<OrbitCamera>>,
    opt_input_direction: Option<ResMut<InputDirection>>,
    opt_padmap: Option<Res<PadMap>>,
    opt_target_gamepad: Option<ResMut<misc::TargetGamepad>>,
    time: Res<Time>,
    qry_gamepads: Query<&Gamepad>,
    mut axis_events: EventReader<GamepadAxisChangedEvent>,
    mut axis_values: Local<FxHashMap<GamepadAxis, f32>>, // スティックの角度の変化量
) -> Result
{
    // 準備
    // let mut camera = opt_orbit_camera.ok_or("opt_orbit_camera is None.")?;
    let mut camera =
        opt_input_direction.ok_or("ResMut<InputDirection> not found.")?;
    let padmap = opt_padmap.ok_or("Res<PadMap> not found.")?;
    let target_gamepad =
        opt_target_gamepad.ok_or("ResMut<misc::TargetGamepad> not found.")?;

    // ゲームパッドが接続されていれば
    if let Some(entity) = target_gamepad.entity()
    {
        if let Ok(gamepad) = qry_gamepads.get(entity)
        {
            // 前回の実行からの経過時間（感度調整の係数あり）
            let time_delta = time.delta_secs() /* * COEF_PAD_TIME_DELTA */;

            // 押されているボタンを調べてコールバック関数を実行する
            // ToDo： eventの利用を検討する（GamepadButtonChangedEvent）
            gamepad.get_pressed().for_each(|&button| {
                let device = GamepadInput::Button(button);
                if let Some(callback) = padmap.get(&device)
                {
                    // ボタンはアナログの可能性があるので変化量を考慮する
                    let value = gamepad.get(button).unwrap_or(1.0);
                    callback(&mut camera, value * time_delta);
                }
            });

            // 変化があった時だけ発火するイベントのスティック角度を保存する
            axis_events.read().for_each(|change_axis| {
                if change_axis.entity == entity
                {
                    axis_values.insert(change_axis.axis, change_axis.value);
                }
            });

            // 保存した角度の情報を使いスティックのコールバック関数を実行する
            axis_values.iter().for_each(|(axis, value)| {
                let device = GamepadInput::Axis(*axis);
                if let Some(callback) = padmap.get(&device)
                {
                    callback(&mut camera, value * time_delta);
                }
            });
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの入力を捕まえる
// pub fn catch_input_direction
// (   qry_player: Query<&player::Player>,
//     opt_input_direction: Option<ResMut<InputDirection>>,
//     opt_gamepad: Option<Res<TargetGamepad>>,
//     input_gamepad: Res<ButtonInput<GamepadButton>>,
//     input_keyboard: Res<ButtonInput<KeyCode>>,
// )
// {   let Ok ( player ) = qry_player.get_single() else { return };
//     let Some ( mut input_direction ) = opt_input_direction else { return };

//     //初期化
//     input_direction.0.clear();
//     let mut pressed_news = HashSet::new();

//     //ゲームパッドが接続されているか
//     if let Some ( gamepad ) = opt_gamepad
//     {   if let Some ( target_id ) = gamepad.id()
//         {   //ゲームパッドの入力をチェックする
//             pressed_news = input_gamepad
//             .get_pressed()
//             .filter_map
//             (   | x |
//                 if x.gamepad != target_id
//                 { None } //ゲームパッドは複数接続できるので、id不一致なら無視
//                 else
//                 {   match x.button_type
//                     {   GamepadButtonType::DPadUp    => Some ( News::North ),
//                         GamepadButtonType::DPadRight => Some ( News::East  ),
//                         GamepadButtonType::DPadLeft  => Some ( News::West  ),
//                         GamepadButtonType::DPadDown  => Some ( News::South ),
//                         _ => None,
//                     }
//                 }
//             )
//             .collect();
//         }
//     }

//     //ゲームパッドの入力がないならキー入力をチェックする
//     if pressed_news.is_empty()
//     {   pressed_news = input_keyboard
//         .get_pressed()
//         .filter_map
//         (   | keycode |
//             match keycode
//             {   KeyCode::ArrowUp    => Some ( News::North ),
//                 KeyCode::ArrowRight => Some ( News::East  ),
//                 KeyCode::ArrowLeft  => Some ( News::West  ),
//                 KeyCode::ArrowDown  => Some ( News::South ),
//                 _ => None,
//             }
//         )
//         .collect();
//     }

//     //要素数０～１なら
//     if pressed_news.is_empty() { return }
//     if pressed_news.len() == 1
//     {   input_direction.0.push( *pressed_news.iter().next().unwrap() );
//         return;
//     }

//     //取得した入力とプレイヤーの向きから、前進・後進入力があったか調べる
//     //.take()するので、pressed_newsは右折・左折の入力だけが（あれば）残る
//     let opt_front = pressed_news.take( &player.direction );
//     let opt_back  = pressed_news.take( &player.direction.back_side() );

//     //優先する方向を考慮して入力をVecにまとめる
//     if let Some ( back ) = opt_back { input_direction.0.push( back ); }
//     input_direction.0.extend( pressed_news.iter() );
//     if let Some ( front ) = opt_front { input_direction.0.push( front ); }
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
