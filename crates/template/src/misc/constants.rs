#![allow( dead_code )]

use super::*;

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用
pub const DEBUG: fn() -> bool = || cfg!( debug_assertions       );
pub const WASM : fn() -> bool = || cfg!( target_arch = "wasm32" );

////////////////////////////////////////////////////////////////////////////////

//v0.14.0でカラー定数がmodule移動したので、とりあえず自力で追加

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
    const MAROON   : Color;
}

//カラー定数を bevy::prelude::Color へ追加（とりあえず）
impl ColorConstants for bevy::prelude::Color
{   const BISQUE   : Color = Color::Srgba( css::BISQUE    );
    const DARK_GRAY: Color = Color::Srgba( css::DARK_GRAY );
    const RED      : Color = Color::Srgba( css::RED       );
    const GREEN    : Color = Color::Srgba( css::GREEN     );
    const BLUE     : Color = Color::Srgba( css::BLUE      );
    const GRAY     : Color = Color::Srgba( css::GRAY      );
    const YELLOW   : Color = Color::Srgba( css::YELLOW    );
    const TEAL     : Color = Color::Srgba( css::TEAL      );
    const SILVER   : Color = Color::Srgba( css::SILVER    );
    const SEA_GREEN: Color = Color::Srgba( css::SEA_GREEN );
    const GOLD     : Color = Color::Srgba( css::GOLD      );
    const CYAN     : Color = Color::Srgba( css::AQUA      );
    const PINK     : Color = Color::Srgba( css::PINK      );
    const MAROON   : Color = Color::Srgba( css::MAROON    );

}

////////////////////////////////////////////////////////////////////////////////

//End of code.