// external crates
use bevy::{
    prelude::*,               //
    ecs::system::SystemParam, //
};

////////////////////////////////////////////////////////////////////////////////

// ゲームパッド接続状態の管理
pub struct GamepadConnectionCheck;

impl Plugin for GamepadConnectionCheck
{
    fn build(&self, application: &mut App)
    {
        application
            // gamepadの接続状態の変化を検知して必要なら切り替える
            .insert_resource(TargetGamepad(None))
            .add_systems(Update, check_gamepad_connections);
    }
}

//------------------------------------------------------------------------------

// キーとゲームパッドボタンの入力設定に使う型
#[derive(Clone, Copy)]
pub struct InputDeviceConfig
{
    pub keys: &'static [KeyWithModifiers],
    pub buttons: &'static [GamepadButton],
}
type KeyWithModifiers = (KeyCode, Option<&'static [KeyCode]>);

// InputDeviceConfigのトレイト境界とその実装
pub trait ConfigIter
{
    fn keys_iter(&self) -> std::slice::Iter<'_, KeyWithModifiers>;
    fn buttons_iter(&self) -> std::slice::Iter<'_, GamepadButton>;
}
impl ConfigIter for InputDeviceConfig
{
    fn keys_iter(&self) -> std::slice::Iter<'_, KeyWithModifiers>
    {
        self.keys.iter()
    }
    fn buttons_iter(&self) -> std::slice::Iter<'_, GamepadButton>
    {
        self.buttons.iter()
    }
}

//------------------------------------------------------------------------------

// 入力関係をまとめたSystemParamを作る
#[derive(SystemParam)]
pub struct InputDevicePack<'w, 's>
{
    input_keycode: ResMut<'w, ButtonInput<KeyCode>>,
    option_target_gamepad: Option<ResMut<'w, TargetGamepad>>,
    query_gamepads: Query<'w, 's, &'static mut Gamepad>,
}

// InputDevicePackにis_pressed系メソッドを実装する
#[allow(dead_code)]
impl<'w, 's> InputDevicePack<'w, 's>
{
    // キー入力とゲームパッドのボタン入力をチェックする
    pub fn is_pressed<T: ConfigIter>(&self, config: &T) -> bool
    {
        // キーが押下されているか
        let is_keys_pressed = config
            .keys_iter()
            .any(|key| self.input_keycode.is_just_pressed(key));

        // キーの押下がないなら
        let mut is_buttons_pressed = false;
        if !is_keys_pressed
            && let Some(target_gamepad) = &self.option_target_gamepad
            && let Some(gamepad_entity) = target_gamepad.entity()
            && let Ok(gamepad) = self.query_gamepads.get(gamepad_entity)
        {
            // ゲームパッドのボタンが押下されているか
            is_buttons_pressed = config
                .buttons_iter()
                .any(|&button| gamepad.just_pressed(button));
        }

        // 戻り値
        is_keys_pressed || is_buttons_pressed
    }

    // .is_pressed()の拡張版。戻り値は同じ。副作用で押下された入力をキャンセルする
    pub fn is_pressed_with_reset<T: ConfigIter>(&mut self, config: &T) -> bool
    {
        // キーが押下されているか
        let is_keys_pressed = config.keys_iter().any(|key| {
            let check = self.input_keycode.is_just_pressed(key);
            if check
            {
                // 押下された入力（主キーのみ）をリセットする
                let (main_key, _) = key;
                let input_keycode = &mut self.input_keycode;
                input_keycode.reset(*main_key);
            }
            check
        });

        // ゲームパッドが接続されているなら
        let mut is_buttons_pressed = false;
        if let Some(target_gamepad) = &self.option_target_gamepad
            && let Some(gamepad_entity) = target_gamepad.entity()
            && let Ok(mut gamepad) = self.query_gamepads.get_mut(gamepad_entity)
        {
            // ゲームパッドのボタンが押下されているか
            is_buttons_pressed = config.buttons_iter().any(|&button| {
                let check = gamepad.just_pressed(button);
                if check
                {
                    // 押下されたゲームパッドのボタンをリセットする
                    gamepad.digital_mut().reset(button);
                }
                check
            });
        }

        // 戻り値
        is_keys_pressed || is_buttons_pressed
    }
}

////////////////////////////////////////////////////////////////////////////////

// 主キー＋装飾キーの押下判定をButtonInput<KeyCode>に追加する
trait ButtonInputKeyCodeExt
{
    fn is_just_pressed(&self, keys: &KeyWithModifiers) -> bool;
}

impl ButtonInputKeyCodeExt for ButtonInput<KeyCode>
{
    // 主キーと装飾キーの組み合わせが押下されているか
    fn is_just_pressed(&self, keys: &KeyWithModifiers) -> bool
    {
        let (main_key, option_modifiers) = keys;

        // 主キーが押下されているか
        let is_main_key_pressed = self.just_pressed(*main_key);

        // 主キーが押下されているなら
        let mut is_modifiers_pressed = false;
        if is_main_key_pressed
        {
            // 修飾キー指定があれば押下状態を調べる
            is_modifiers_pressed = match option_modifiers
            {
                Some(keys) => keys.iter().any(|&key| self.pressed(key)),
                None => true, // 修飾キー指定がない場合
            };
        }

        // 戻り値
        is_main_key_pressed && is_modifiers_pressed
    }
}

////////////////////////////////////////////////////////////////////////////////

// gamepadのEntityを保存するResource
#[derive(Resource, Default)]
struct TargetGamepad(Option<Entity>);

impl TargetGamepad
{
    // アクセス用メソッド
    fn entity(&self) -> Option<Entity> { self.0 }
    fn entity_mut(&mut self) -> &mut Option<Entity> { &mut self.0 }
}

// gamepadの接続を検出して必要なら切り替える
fn check_gamepad_connections(
    mut target_gamepad: ResMut<TargetGamepad>,
    mut query_gamepads: Query<(Entity, &Name), With<Gamepad>>,
)
{
    // gamepadのEntityが保存されているなら
    if let Some(entity) = target_gamepad.entity()
    {
        // そのEntityが存在しないなら（切断）
        if !query_gamepads.contains(entity)
        {
            // Entityを探す（結果はSome(X)とNoneの場合両方ある）
            let (new_entity, name) = query_gamepads.iter_mut().next().unzip();
            *target_gamepad.entity_mut() = new_entity;

            // Some⇒Some or Some⇒None
            info!("A) Gamepad: {} => {:?} ({:?})", entity, name, new_entity);
        }
    }
    // 現在gamepadの接続があるなら
    else if let Some((entity, name)) = query_gamepads.iter_mut().next()
    {
        *target_gamepad.entity_mut() = Some(entity);

        // None⇒Some
        info!("B) Gamepad: None => {} ({})", name, entity);
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
