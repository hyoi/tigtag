use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーキャラクター操作入力のコールバック関数の型
pub type FnCallback = fn(&mut HashNews, f32);
pub type HashNews = FxHashSet<News>;

// キー入力マッピング登録用のResource
#[derive(Resource, Deref)]
pub struct KeyMap(pub FxHashMap<KeyCode, FnCallback>);
impl KeyMap
{
    pub fn from(key_map: KeyMapSlice) -> Self
    {
        Self(FxHashMap::from_iter(key_map.iter().copied()))
    }
}
pub type KeyMapSlice = &'static [(KeyCode, FnCallback)];

// ゲームパッド入力マッピング登録用のResource
// #[derive(Resource, Deref)]
// pub struct GamepadMap(pub FxHashMap<GamepadInput, FnCallback>);
// impl GamepadMap
// {
//     pub fn from(gamepad_map: GamepadMapSlice) -> Self
//     {
//         Self(FxHashMap::from_iter(gamepad_map.iter().copied()))
//     }
// }
// pub type GamepadMapSlice = &'static [(GamepadInput, player::FnCallback)];

////////////////////////////////////////////////////////////////////////////////

// コールバック関数
#[allow( dead_code )]
#[rustfmt::skip]
pub mod callback
{   use super::*;

    // 十字キー／ボタン
    pub fn move_up   ( hash: &mut HashNews, _: f32 ) { hash.insert( News::North ); }
    pub fn move_down ( hash: &mut HashNews, _: f32 ) { hash.insert( News::South ); }
    pub fn move_right( hash: &mut HashNews, _: f32 ) { hash.insert( News::East  ); }
    pub fn move_left ( hash: &mut HashNews, _: f32 ) { hash.insert( News::West  ); }

    // スタティックのノーマル
    pub fn axis_x_normal( hash: &mut HashNews, value: f32 )
    {
        let news = if value >= 0.0 { News::East } else { News::West };
        hash.insert( news );
    }
    pub fn axis_y_normal( hash: &mut HashNews, value: f32 )
    {
        let news = if value >= 0.0 { News::North } else { News::South };
        hash.insert( news );
    }

    // スティックのリバース
    pub fn axis_x_reverse( hash: &mut HashNews, value: f32 )
    {
        axis_x_normal( hash, -value );
    }
    pub fn axis_y_reverse( hash: &mut HashNews, value: f32 )
    {
        axis_y_normal( hash, -value );
    }
}

////////////////////////////////////////////////////////////////////////////////

// キー入力をEventで送信する
pub fn input_from_keyboard(
    option_keymap: Option<Res<KeyMap>>,
    mut event_input: MessageWriter<PlayerMovementInput>,
    time: Res<Time>,
    input_keycode: Res<ButtonInput<KeyCode>>,
) -> Result
{
    // 準備
    let keymap = option_keymap.ok_or("Resource not found.")?;
    let time_delta = time.delta_secs();

    // キー入力をNewsへ変換して記録する
    let mut hash_news: HashNews = FxHashSet::default();
    input_keycode.get_pressed().for_each(|keycode| {
        if let Some(callback) = keymap.get(keycode)
        {
            callback(&mut hash_news, time_delta);
        }
    });

    // Eventで送信する
    event_input.write(PlayerMovementInput(hash_news));

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// ゲームパッドの入力を保存する
// pub fn input_from_gamepad(
//     option_gamepad_map: Option<Res<GamepadMap>>,
//     option_target_gamepad: Option<ResMut<my_utils::misc::TargetGamepad>>,
//     time: Res<Time>,
//     query_gamepads: Query<&Gamepad>,
//     mut event_input: EventWriter<EventPlayerInputNews>,
//     mut axis_events: EventReader<GamepadAxisChangedEvent>,
//     mut axis_values: Local<FxHashMap<GamepadAxis, f32>>, // スティックの角度の変化量
// ) -> Result
// {
//     // 準備
//     let gamepad_map = option_gamepad_map.ok_or("Resource not found.")?;
//     let target_gamepad = option_target_gamepad.ok_or("Resource not found.")?;

//     // ゲームパッドが接続中なら
//     if let Some(gamepad_entity) = target_gamepad.entity()
//         && let Ok(gamepad) = query_gamepads.get(gamepad_entity)
//     {
//         let time_delta = time.delta_secs();
//         let mut hash_news: HashNews = FxHashSet::default();

//         // 押されているボタンを調べてコールバック関数を実行する
//         gamepad.get_pressed().for_each(|&button| {
//             let device = GamepadInput::Button(button);
//             if let Some(callback) = gamepad_map.get(&device)
//             {
//                 // ボタンはアナログの可能性があるので変化量を考慮する
//                 let value = gamepad.get(button).unwrap_or(1.0);
//                 callback(&mut hash_news, value * time_delta);
//             }
//         });

//         // スティックの処理
//         // 変化時だけ発火するイベントの情報を保存する（スティックの角度）
//         axis_events.read().for_each(|change_axis| {
//             // ゲームパッドが一致するなら（ゲームパッドは複数接続できるので）
//             if change_axis.entity == gamepad_entity
//             {
//                 // スティックが中央に戻ったなら（0.0）
//                 if change_axis.value == 0.0
//                 {
//                     axis_values.remove(&change_axis.axis);
//                 }
//                 else
//                 {
//                     // スティックが傾いた場合は変更量を保存する
//                     axis_values.insert(change_axis.axis, change_axis.value);
//                 }
//             }
//         });
//         // 保存した情報を使ってコールバックを実行する
//         axis_values.iter().for_each(|(axis, value)| {
//             let device = GamepadInput::Axis(*axis);
//             if let Some(callback) = gamepad_map.get(&device)
//             {
//                 callback(&mut hash_news, value * time_delta);
//             }
//         });

//         // Eventで送信する
//         event_input.write(EventPlayerInputNews(hash_news));
//     }

//     Ok(())
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
