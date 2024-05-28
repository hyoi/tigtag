#![allow( dead_code )]

use super::*;

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
// pub const CARGO_TOML_NAME: &str = env!( "CARGO_PKG_NAME" ); //shareになってしまう
pub const CARGO_TOML_VER: &str = env!( "CARGO_PKG_VERSION" );

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用
pub const DEBUG           : fn() -> bool = || cfg!( debug_assertions             );
pub const WASM            : fn() -> bool = || cfg!( target_arch = "wasm32"       );
pub const SPRITE_SHEET_OFF: fn() -> bool = || cfg!( feature = "sprite_sheet_off" );

////////////////////////////////////////////////////////////////////////////////

//v0.14.0でWEBカラーの定数が廃止されたので、とりあえずの対応として自力で定数を追加
//本来なら bevy::color::palettes::css の定数を使うべき

//オーファンルール対策用trait
pub trait ColorConstants
{   const BISQUE   : Color;
    const DARK_GRAY: Color;
    const RED      : Color;
    const GREEN    : Color;
    const BLUE     : Color;
    const GRAY     : Color;
    const YELLOW   : Color;
    const TEAL     : Color;
    const SILVER   : Color;
    const SEA_GREEN: Color;
    const GOLD     : Color;
    const CYAN     : Color;
    const PINK     : Color;
}

//WEBカラーの定数
impl ColorConstants for bevy::prelude::Color
{   const BISQUE   : Color = Color::srgb( 1.0 , 0.89, 0.77 );
    const DARK_GRAY: Color = Color::srgb( 0.25, 0.25, 0.25 );
    const RED      : Color = Color::srgb( 1.0 , 0.0 , 0.0  );
    const GREEN    : Color = Color::srgb( 0.0 , 1.0 , 0.0  );
    const BLUE     : Color = Color::srgb( 0.0 , 0.0 , 1.0  );
    const GRAY     : Color = Color::srgb( 0.5 , 0.5 , 0.5  );
    const YELLOW   : Color = Color::srgb( 1.0 , 1.0 , 0.0  );
    const TEAL     : Color = Color::srgb( 0.0 , 0.5 , 0.5  );
    const SILVER   : Color = Color::srgb( 0.75, 0.75, 0.75 );
    const SEA_GREEN: Color = Color::srgb( 0.18, 0.55, 0.34 );
    const GOLD     : Color = Color::srgb( 1.0 , 0.84, 0.0  );
    const CYAN     : Color = Color::srgb( 0.0 , 1.0 , 1.0  );
    const PINK     : Color = Color::srgb( 1.0 , 0.08, 0.58 );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.