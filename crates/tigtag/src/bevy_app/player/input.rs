use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの入力を保存するResource
#[derive(Resource, Default, Deref, DerefMut)]
pub struct PlayerInput(pub FxHashSet<News>);

////////////////////////////////////////////////////////////////////////////////

// キー入力を保存する
pub fn input_from_keyboard(
    opt_input_direction: Option<ResMut<PlayerInput>>,
    opt_keymap: Option<Res<KeyMap>>,
    input_keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result
{
    // 準備
    let mut input = opt_input_direction.ok_or("ResMut<PlayerInput> not found.")?;
    let keymap = opt_keymap.ok_or("Res<KeyMap> not found.")?;
    let time_delta = time.delta_secs();

    // キー入力をキーマップで変換してプレイヤーの向きとして記録する
    input_keycode.get_pressed().for_each(|keycode| {
        if let Some(callback) = keymap.get(keycode)
        {
            callback(&mut input, time_delta);
        }
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// ゲームパッドの入力を保存する
pub fn input_from_gamepad(
    opt_input_direction: Option<ResMut<PlayerInput>>,
    opt_padmap: Option<Res<PadMap>>,
    opt_target_gamepad: Option<ResMut<misc::TargetGamepad>>,
    qry_gamepads: Query<&Gamepad>,
    mut axis_events: EventReader<GamepadAxisChangedEvent>,
    mut axis_values: Local<FxHashMap<GamepadAxis, f32>>, // スティックの角度の変化量
    time: Res<Time>,
) -> Result
{
    // 準備
    let mut input = opt_input_direction.ok_or("ResMut<PlayerInput> not found.")?;
    let padmap = opt_padmap.ok_or("Res<PadMap> not found.")?;
    let target_gamepad =
        opt_target_gamepad.ok_or("ResMut<misc::TargetGamepad> not found.")?;
    let time_delta = time.delta_secs();

    // ゲームパッドが接続中なら
    if let Some(gamepad_entity) = target_gamepad.entity()
        && let Ok(gamepad) = qry_gamepads.get(gamepad_entity)
    {
        // 押されているボタンを調べてコールバック関数を実行する
        // ToDo： eventの利用を検討する（GamepadButtonChangedEvent）
        gamepad.get_pressed().for_each(|&button| {
            let device = GamepadInput::Button(button);
            if let Some(callback) = padmap.get(&device)
            {
                // ボタンはアナログの可能性があるので変化量を考慮する
                let value = gamepad.get(button).unwrap_or(1.0);
                callback(&mut input, value * time_delta);
            }
        });

        // スティックの処理
        // 変化時だけ発火するイベントの情報を保存する（スティックの角度）
        axis_events.read().for_each(|change_axis| {
            //ゲームパッドが一致するなら（ゲームパッドは複数接続できるので）
            if change_axis.entity == gamepad_entity
            {
                //スティックが中央に戻ったなら（0.0）
                if change_axis.value == 0.0
                {
                    axis_values.remove(&change_axis.axis);
                }
                else
                {
                    // スティックが傾いた場合は変更量を保存する
                    axis_values.insert(change_axis.axis, change_axis.value);
                }
            }
        });

        // 保存した情報を使ってコールバックを実行する
        axis_values.iter().for_each(|(axis, value)| {
            let device = GamepadInput::Axis(*axis);
            if let Some(callback) = padmap.get(&device)
            {
                callback(&mut input, value * time_delta);
            }
        });
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// コールバック関数の型
pub type CallBack = fn(&mut PlayerInput, f32);

// コールバック関数
pub use callback::*;

#[allow( dead_code )]
#[rustfmt::skip]
pub mod callback
{   use super::*;

    // 十字キー／ボタン
    pub fn move_up   ( input: &mut PlayerInput, _: f32 ) { input.insert( News::North ); }
    pub fn move_down ( input: &mut PlayerInput, _: f32 ) { input.insert( News::South ); }
    pub fn move_right( input: &mut PlayerInput, _: f32 ) { input.insert( News::East  ); }
    pub fn move_left ( input: &mut PlayerInput, _: f32 ) { input.insert( News::West  ); }

    // スタティックのノーマル
    pub fn axis_x_normal( input: &mut PlayerInput, value: f32 )
    {
        let news = if value >= 0.0 { News::East } else { News::West };
        input.insert( news );
    }
    pub fn axis_y_normal( input: &mut PlayerInput, value: f32 )
    {
        let news = if value >= 0.0 { News::North } else { News::South };
        input.insert( news );
    }

    // スティックのリバース
    pub fn axis_x_reverse( input: &mut PlayerInput, value: f32 )
    {
        axis_x_normal( input, -value );
    }
    pub fn axis_y_reverse( input: &mut PlayerInput, value: f32 )
    {
        axis_y_normal( input, -value );
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

// End of code.
