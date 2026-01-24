use super::*;

////////////////////////////////////////////////////////////////////////////////

// 事前ロード対象
pub const PRELOAD_ASSETS: &[&str] = &[
    ASSETS_FONT_ORBITRON_BLACK,
    ASSETS_FONT_PRESSSTART2P_REGULAR,
    ASSETS_FONT_REGGAEONE_REGULAR,
    ASSETS_SPRITE_KANI_DOTOWN,
    ASSETS_SPRITE_BRICK_WALL,
    ASSETS_SPRITESHEET_PLAYER,
    ASSETS_SPRITESHEET_CHASER_RED,
    ASSETS_SPRITESHEET_CHASER_GREEN,
    ASSETS_SPRITESHEET_CHASER_BLUE,
    ASSETS_SPRITESHEET_CHASER_PINK,
    ASSETS_SOUND_BEEP,
];

// assets（フォント）
pub const ASSETS_FONT_ORBITRON_BLACK: &str = "font/Orbitron-Black.ttf";
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "font/PressStart2P-Regular.ttf";
pub const ASSETS_FONT_REGGAEONE_REGULAR: &str = "font/ReggaeOne-Regular.ttf";

// assets（スプライト）
pub const ASSETS_SPRITE_KANI_DOTOWN: &str = "sprite/kani_DOTOWN.png";
pub const ASSETS_SPRITE_BRICK_WALL: &str = "sprite/brick_wall.png";

// assets（スプライトシート）
pub const ASSETS_SPRITESHEET_PLAYER: &str = "spritesheet/player.png";
pub const ASSETS_SPRITESHEET_CHASER_RED: &str = "spritesheet/chaser_red.png";
pub const ASSETS_SPRITESHEET_CHASER_GREEN: &str = "spritesheet/chaser_green.png";
pub const ASSETS_SPRITESHEET_CHASER_BLUE: &str = "spritesheet/chaser_blue.png";
pub const ASSETS_SPRITESHEET_CHASER_PINK: &str = "spritesheet/chaser_pink.png";

// assets（サウンド）
pub const ASSETS_SOUND_BEEP: &str = "audio/sound/beep.ogg";
pub const VOLUME_SOUND_BEEP: Volume = Volume::Linear(0.1); //SEボリューム

////////////////////////////////////////////////////////////////////////////////

// End of code.
