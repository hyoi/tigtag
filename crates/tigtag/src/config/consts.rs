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

// 色名の単純な表記
pub const COLOR_YELLOW: Color = Color::Srgba(css::YELLOW);
pub const COLOR_GOLD: Color = Color::Srgba(css::GOLD);
pub const COLOR_WHITE: Color = Color::Srgba(css::WHITE);
pub const COLOR_TEAL: Color = Color::Srgba(css::TEAL);
pub const COLOR_SILVER: Color = Color::Srgba(css::SILVER);
pub const COLOR_CYAN: Color = Color::Srgba(css::AQUA);
pub const COLOR_GRAY: Color = Color::Srgba(css::GRAY);
pub const COLOR_RED: Color = Color::Srgba(css::RED);
pub const COLOR_NONE: Color = Color::NONE;

////////////////////////////////////////////////////////////////////////////////

// End of code.
