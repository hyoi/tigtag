#![allow(dead_code)]
use super::*;

////////////////////////////////////////////////////////////////////////////////

// 名前空間のトップレベルへ輸出する識別子
#[rustfmt::skip]
#[allow(unused_imports)]
pub mod prelude
{
    use super::*;

    // ゲームパッド十字ボタンの簡略化定数
    pub const GAMEPAD_UP   : GamepadInput = GamepadInput::Button(GamepadButton::DPadUp   );
    pub const GAMEPAD_DOWN : GamepadInput = GamepadInput::Button(GamepadButton::DPadDown );
    pub const GAMEPAD_LEFT : GamepadInput = GamepadInput::Button(GamepadButton::DPadLeft );
    pub const GAMEPAD_RIGHT: GamepadInput = GamepadInput::Button(GamepadButton::DPadRight);

    // ゲームパッドショルダーボタンの簡略化定数
    pub const GAMEPAD_TRIG1_LEFT : GamepadInput = GamepadInput::Button(GamepadButton::LeftTrigger  );
    pub const GAMEPAD_TRIG1_RIGHT: GamepadInput = GamepadInput::Button(GamepadButton::RightTrigger );
    pub const GAMEPAD_TRIG2_LEFT : GamepadInput = GamepadInput::Button(GamepadButton::LeftTrigger2 );
    pub const GAMEPAD_TRIG2_RIGHT: GamepadInput = GamepadInput::Button(GamepadButton::RightTrigger2);

    // ゲームパッドスティックの簡略化定数
    pub const GAMEPAD_STICK_LEFT_Y : GamepadInput = GamepadInput::Axis(GamepadAxis::LeftStickY );
    pub const GAMEPAD_STICK_LEFT_X : GamepadInput = GamepadInput::Axis(GamepadAxis::LeftStickX );
    pub const GAMEPAD_STICK_RIGHT_Y: GamepadInput = GamepadInput::Axis(GamepadAxis::RightStickY);
    pub const GAMEPAD_STICK_RIGHT_X: GamepadInput = GamepadInput::Axis(GamepadAxis::RightStickX);

    // マウスの各種センサーを抽象化するenum
    pub use super::MouseInput;
}

////////////////////////////////////////////////////////////////////////////////

// gamepadのEntityを保存するResource
#[derive(Resource, Default)]
pub struct TargetGamepad(Option<Entity>);

// アクセス用メソッド
impl TargetGamepad
{
    pub fn entity(&self) -> Option<Entity> { self.0 }
    pub fn entity_mut(&mut self) -> &mut Option<Entity> { &mut self.0 }
}

// gamepadの接続を検出して必要なら切り替える
pub fn check_gamepad_connections(
    option_target_gamepad: Option<ResMut<TargetGamepad>>,
    mut query_gamepads: Query<(Entity, &Name), With<Gamepad>>,
    mut cmds: Commands,
)
{
    // gamepadの接続状態を調べてResourceを更新する（クロージャ）
    let mut check_gamepad = |target_gamepad: &mut TargetGamepad| {
        // gamepadのEntityが保存されているなら
        if let Some(entity) = target_gamepad.entity()
        {
            // そのEntityが存在しないなら（切断）
            if !query_gamepads.contains(entity)
            {
                // Entityを探す（結果はNoneかもしれない）
                let (entity, _name) = query_gamepads.iter_mut().next().unzip();
                *target_gamepad.entity_mut() = entity;

                #[cfg(debug_assertions)]
                dbg!(&_name, target_gamepad.entity()); // Some⇒Some、Some⇒None
            }
        }
        else if !query_gamepads.is_empty()
        // 現在gamepadの接続があるなら
        {
            // Entityを探す（結果は必ずSome）
            let (entity, _name) = query_gamepads.iter_mut().next().unzip();
            *target_gamepad.entity_mut() = entity;

            #[cfg(debug_assertions)]
            dbg!(&_name, target_gamepad.entity()); // None⇒Some
        }
    };

    // Resourceが登録済みなら
    if let Some(mut target_gamepad) = option_target_gamepad
    {
        check_gamepad(&mut target_gamepad);
    }
    else
    {
        // Resourceが未登録なら、初期化した後に登録する
        let mut target_gamepad = TargetGamepad::default();
        check_gamepad(&mut target_gamepad);
        cmds.insert_resource(target_gamepad);
    }
}

////////////////////////////////////////////////////////////////////////////////

// デバイスからの入力を抽象化するためのUserAction
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum UserAction
{
    // ６軸方向へ＋１単位で移動するアクション
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    ZoomIn,
    ZoomOut,
    // ３軸方向へ軸の傾き（＋－）で移動するアクション（f32はデバイスの感度調整）
    AxisVertNormal(f32),
    AxisVertReverse(f32),
    AxisHorizNormal(f32),
    AxisHorizReverse(f32),
    AxisZoomNormal(f32),
    AxisZoomReverse(f32),
    // カーソルがオーバーラップした画面内の対象へのインタラクト
    ObjectPick,
}

// マウスデバイス上にある各素子を抽象化する為のEnum
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseInput
{
    ButtonLeft,
    ButtonRight,
    WheelY,
    WheelX,
    MotionY,
    MotionX,
}

// 入力とUserActionをマッピングする型
pub type ConnfigKeyboard = &'static [(KeyCode, UserAction)];
pub type ConnfigGamepad = &'static [(GamepadInput, UserAction)];
pub type ConnfigMouse = &'static [(MouseInput, UserAction)];

// マッピングをResourceに保存する為の型とメソッド
#[derive(Resource, Deref)]
pub struct MappingKeyboard(pub HashMap<KeyCode, UserAction>);
#[derive(Resource, Deref)]
pub struct MappingGamepad(pub HashMap<GamepadInput, UserAction>);
#[derive(Resource, Deref)]
pub struct MappingMouse(pub HashMap<MouseInput, UserAction>);

impl MappingKeyboard
{
    pub fn from(key_map: ConnfigKeyboard) -> Self
    {
        Self(HashMap::from_iter(key_map.iter().copied()))
    }
}
impl MappingGamepad
{
    pub fn from(gamepad_map: ConnfigGamepad) -> Self
    {
        Self(HashMap::from_iter(gamepad_map.iter().copied()))
    }
}
impl MappingMouse
{
    pub fn from(mouse_map: ConnfigMouse) -> Self
    {
        Self(HashMap::from_iter(mouse_map.iter().copied()))
    }
}

////////////////////////////////////////////////////////////////////////////////

// デバイスからの入力を伝えるメッセージ
#[derive(Message, Debug)]
pub struct MessUserAction(pub Vec<(UserAction, f32)>);

////////////////////////////////////////////////////////////////////////////////

// キーボードからの入力をメッセージで送信する
pub fn check_keyboard(
    option_mapping_keyboard: Option<Res<MappingKeyboard>>,
    mut message_writer: MessageWriter<MessUserAction>,
    input_keycode: Res<ButtonInput<KeyCode>>,
) -> Result
{
    // キー入力のマッピングが登録されているなら
    if let Some(mapping_keyboard) = option_mapping_keyboard
    {
        let actions: Vec<(UserAction, f32)> = input_keycode
            .get_pressed()
            .filter_map(|key| mapping_keyboard.get(key))
            .map(|action| (*action, 1.0))
            .collect();

        // 送信メッセージがあるなら
        if !actions.is_empty()
        {
            message_writer.write(MessUserAction(actions));
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// ゲームパッドからの入力をメッセージで送信する
pub fn check_gamepad(
    option_mapping_gamepad: Option<Res<MappingGamepad>>,
    option_target_gamepad: Option<ResMut<TargetGamepad>>,
    query_gamepads: Query<&Gamepad>,
    mut message_writer: MessageWriter<MessUserAction>,
) -> Result
{
    // ゲームパッドの入力のマッピングが登録されているなら
    if let Some ( mapping_gamepad ) = option_mapping_gamepad
        // ゲームパッドが接続中なら
        && let Some ( target_gamepad ) = option_target_gamepad
        && let Some(gamepad_entity) = target_gamepad.entity()
        && let Ok(gamepad) = query_gamepads.get(gamepad_entity)
    {
        // 押下中のゲームパッドのボタン（デジタルとアナログ共に）
        let actions: Vec<(UserAction, f32)> = gamepad
            .get_pressed()
            .filter_map(|&control| {
                let button = GamepadInput::Button(control);
                if let Some(action) = mapping_gamepad.get(&button)
                {
                    // .unwrap()失敗なら、デジタルと判断して1.0を与える
                    let value = gamepad.get(button).unwrap_or(1.0);
                    Some((*action, value))
                }
                else
                {
                    None
                }
            })
            .collect();

        // 送信メッセージがあるなら
        if !actions.is_empty()
        {
            message_writer.write(MessUserAction(actions));
        }

        // スティックの傾き
        const ZERO_TOLERANCE: f32 = 0.05;
        let left = gamepad.left_stick();
        let right = gamepad.right_stick();
        #[rustfmt::skip]
        let actions: Vec<(UserAction, f32)> = mapping_gamepad
            .iter()
            .filter_map(|(input, action)| match input
            {
                GamepadInput::Axis(GamepadAxis::LeftStickY)
                    if left.y.abs() > ZERO_TOLERANCE =>
                        Some((*action, left.y)),
                GamepadInput::Axis(GamepadAxis::LeftStickX)
                    if left.x.abs() > ZERO_TOLERANCE =>
                        Some((*action, left.x)),
                GamepadInput::Axis(GamepadAxis::RightStickY)
                    if right.y.abs() > ZERO_TOLERANCE =>
                        Some((*action, right.y)),
                GamepadInput::Axis(GamepadAxis::RightStickX)
                    if right.x.abs() > ZERO_TOLERANCE =>
                        Some((*action, right.x)),
                _ => None,
            })
            .collect();

        // 送信メッセージがあるなら
        if !actions.is_empty()
        {
            message_writer.write(MessUserAction(actions));
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// マウスからの入力をメッセージで送信する
pub fn check_mouse(
    option_mapping_mouse: Option<Res<MappingMouse>>,
    input_mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut motion_events: MessageReader<MouseMotion>,
    mut wheel_events: MessageReader<MouseWheel>,
    mut message_writer: MessageWriter<MessUserAction>,
) -> Result
{
    // マウス入力のマッピングが登録されているなら
    if let Some(mapping_mouse) = option_mapping_mouse
    {
        // ピックアップにマップされたボタンの押下状態を調べる
        let is_pickup_button_pressed =
            input_mouse_buttons.get_pressed().any(|button| match *button
            {
                MouseButton::Left
                    if mapping_mouse.get(&MouseInput::ButtonLeft)
                        == Some(&UserAction::ObjectPick) =>
                    true,
                MouseButton::Right
                    if mapping_mouse.get(&MouseInput::ButtonRight)
                        == Some(&UserAction::ObjectPick) =>
                    true,
                // 他のマウスボタンは無視
                _ => false,
            });

        // ピックアップボタンが押下されているなら
        if is_pickup_button_pressed
        {
            // マウスモーション（Ｘ軸、Ｙ軸）
            let mut actions = Vec::<(UserAction, f32)>::new();
            motion_events.read().for_each(|motion| {
                if let Some(action) = mapping_mouse.get(&MouseInput::MotionY)
                    && motion.delta.y != 0.0
                {
                    actions.push((*action, motion.delta.y));
                }
                if let Some(action) = mapping_mouse.get(&MouseInput::MotionX)
                    && motion.delta.x != 0.0
                {
                    actions.push((*action, motion.delta.x));
                }
            });

            // 送信メッセージがあるなら
            if !actions.is_empty()
            {
                message_writer.write(MessUserAction(actions));
            }
        }

        // ホイール（Ｙ）のマッピングがあるなら
        if let Some(action) = mapping_mouse.get(&MouseInput::WheelY)
        {
            // 変化があった時だけ発火するホイールのイベントを拾って送信する
            let actions: Vec<(UserAction, f32)> = wheel_events
                .read()
                .map(|wheel| (*action, wheel.y))
                .collect();

            // 送信メッセージがあるなら
            if !actions.is_empty()
            {
                message_writer.write(MessUserAction(actions));
            }
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
