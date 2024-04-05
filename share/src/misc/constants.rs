#![allow( dead_code )]

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

//End of code.