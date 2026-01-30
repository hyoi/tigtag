use super::*;

////////////////////////////////////////////////////////////////////////////////

// フルスクリーン切替処理キーとボタンの設定
#[cfg(not(target_arch = "wasm32"))]
pub const TRIGGER_FULLSCREEN: controller::InputDeviceConfig =
    controller::InputDeviceConfig {
        keys: &[
            (KeyCode::Enter, Some(MODIFIERS_ALT)), //
            (KeyCode::F11, None),                  //
        ],
        buttons: &[
            GamepadButton::Select, //ps4[SHAREボタン]
        ],
    };

////////////////////////////////////////////////////////////////////////////////

// アプリ終了キーとボタンの設定
pub const TRIGGER_APP_EXIT: controller::InputDeviceConfig =
    controller::InputDeviceConfig {
        keys: &[
            (KeyCode::Escape, None),            //
            (KeyCode::F4, Some(MODIFIERS_ALT)), //
        ],
        buttons: &[
            GamepadButton::Mode, //ps4[PSボタン]
        ],
    };

////////////////////////////////////////////////////////////////////////////////

// UIアウトライン表示／非表示の切替キーとボタンの設定
pub const TRIGGER_UI_OUTLINE: controller::InputDeviceConfig =
    controller::InputDeviceConfig {
        keys: &[
            (KeyCode::Tab, Some(MODIFIERS_CTRL)), //
        ],
        buttons: &[],
    };

////////////////////////////////////////////////////////////////////////////////

// 修飾キーのグループ化
use constants::*;

#[allow(dead_code)]
mod constants
{
    use super::{KeyCode, KeyCode::*};
    pub const MODIFIERS_ALT: &[KeyCode] = &[AltLeft, AltRight];
    pub const MODIFIERS_CTRL: &[KeyCode] = &[ControlLeft, ControlRight];
    pub const MODIFIERS_SHIFT: &[KeyCode] = &[ShiftLeft, ShiftRight];
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
