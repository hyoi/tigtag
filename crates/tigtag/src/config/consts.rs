#![allow(dead_code)]
use super::*;

////////////////////////////////////////////////////////////////////////////////

// 修飾キーのグループ化
use KeyCode::*;
pub const MODIFIERS_ALT: &[KeyCode] = &[AltLeft, AltRight];
pub const MODIFIERS_CTRL: &[KeyCode] = &[ControlLeft, ControlRight];
pub const MODIFIERS_SHIFT: &[KeyCode] = &[ShiftLeft, ShiftRight];

////////////////////////////////////////////////////////////////////////////////

// Gridに関連する定数
pub const GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;
pub const GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;
pub const CELL_CUSTOM_SIZE: Vec2 = Vec2::new(PIXELS_PER_GRID, PIXELS_PER_GRID);

////////////////////////////////////////////////////////////////////////////////

// 各色名の単純な表記
pub const COLOR_NONE: Color = Color::NONE;
pub const COLOR_WHITE: Color = Color::WHITE;
pub const COLOR_BLACK: Color = Color::BLACK;
pub const COLOR_YELLOW: Color = Color::Srgba(css::YELLOW);
pub const COLOR_GOLD: Color = Color::Srgba(css::GOLD);
pub const COLOR_TEAL: Color = Color::Srgba(css::TEAL);
pub const COLOR_SILVER: Color = Color::Srgba(css::SILVER);
pub const COLOR_CYAN: Color = Color::Srgba(css::AQUA);
pub const COLOR_GRAY: Color = Color::Srgba(css::GRAY);
pub const COLOR_RED: Color = Color::Srgba(css::RED);
pub const COLOR_BLUE: Color = Color::Srgba(css::BLUE);

////////////////////////////////////////////////////////////////////////////////

// 十字ボタンの単純な表記
pub const GAMEPAD_UP: GamepadInput = GamepadInput::Button(GamepadButton::DPadUp);
pub const GAMEPAD_DOWN: GamepadInput = GamepadInput::Button(GamepadButton::DPadDown);
pub const GAMEPAD_LEFT: GamepadInput = GamepadInput::Button(GamepadButton::DPadLeft);
pub const GAMEPAD_RIGHT: GamepadInput = GamepadInput::Button(GamepadButton::DPadRight);
// ショルダーの単純な表記
pub const GAMEPAD_TRIG1_LEFT: GamepadInput = GamepadInput::Button(GamepadButton::LeftTrigger);
pub const GAMEPAD_TRIG1_RIGHT: GamepadInput = GamepadInput::Button(GamepadButton::RightTrigger);
pub const GAMEPAD_TRIG2_LEFT: GamepadInput = GamepadInput::Button(GamepadButton::LeftTrigger2);
pub const GAMEPAD_TRIG2_RIGHT: GamepadInput = GamepadInput::Button(GamepadButton::RightTrigger2);
// スティックの単純な表記
pub const GAMEPAD_STICK_LEFT_Y: GamepadInput = GamepadInput::Axis(GamepadAxis::LeftStickY);
pub const GAMEPAD_STICK_LEFT_X: GamepadInput = GamepadInput::Axis(GamepadAxis::LeftStickX);
pub const GAMEPAD_STICK_RIGHT_Y: GamepadInput = GamepadInput::Axis(GamepadAxis::RightStickY);
pub const GAMEPAD_STICK_RIGHT_X: GamepadInput = GamepadInput::Axis(GamepadAxis::RightStickX);

////////////////////////////////////////////////////////////////////////////////

// マウス入力の単純な表記
pub const MOUSE_BUTTON_LEFT: handle_input::MouseInput = handle_input::MouseInput::ButtonLeft;
pub const MOUSE_BUTTON_RIGHT: handle_input::MouseInput = handle_input::MouseInput::ButtonRight;
pub const MOUSE_WHEEL_Y: handle_input::MouseInput = handle_input::MouseInput::WheelY;
pub const MOUSE_WHEEL_X: handle_input::MouseInput = handle_input::MouseInput::WheelX;
pub const MOUSE_MOTION_Y: handle_input::MouseInput = handle_input::MouseInput::MotionY;
pub const MOUSE_MOTION_X: handle_input::MouseInput = handle_input::MouseInput::MotionX;

////////////////////////////////////////////////////////////////////////////////

// End of code.
