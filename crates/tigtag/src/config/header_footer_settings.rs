use super::*;

////////////////////////////////////////////////////////////////////////////////

// ヘッダー／フッター用の情報
pub const HEADER_FOOTER: &[header_footer::TextBlock] = &[
    HEADER_STAGE,      // ステージ数
    HEADER_SCORE,      // スコア
    HEADER_HI_SCORE,   // ハイスコア
    FOOTER_FPS,        // FPS表示
    FOOTER_AUTHER,     // auther
    FOOTER_POWERED_BY, // Powered by
];

// フォントサイズ
const HEADER_LABEL_SIZE: f32 = PIXELS_PER_GRID * 0.58;
const HEADER_VALUE_SIZE: f32 = PIXELS_PER_GRID * 0.7;
const FOOTER_LABEL_SIZE: f32 = PIXELS_PER_GRID * 0.48;
const FOOTER_VALUE_SIZE: f32 = PIXELS_PER_GRID * 0.4;

// プレースホルダ
pub const NUM2: &str = "##";
pub const NUM5: &str = "#####";
pub const NUM3_2: &str = "###.##";

// 表示する数値を整形する関数
fn format_i32_02(value: &dyn std::fmt::Display) -> String { format!("{:02}", value) }
fn format_i32_05(value: &dyn std::fmt::Display) -> String { format!("{:05}", value) }
fn format_f32_06_2(value: &dyn std::fmt::Display) -> String
{
    format!("{:06.2}", value)
}

//------------------------------------------------------------------------------

pub const HEADER_STAGE: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::TopLeft,
    align_self: AlignSelf::Start,     // セル内の上寄せ
    justify_self: JustifySelf::Start, // セル内の左寄せ
    bg_color: COLOR_NONE,
    #[rustfmt::skip]
    textspans: &[
        ( " STAGE ", ASSETS_FONT_ORBITRON_BLACK      , HEADER_LABEL_SIZE, COLOR_GOLD  ),
        ( NUM2     , ASSETS_FONT_PRESSSTART2P_REGULAR, HEADER_VALUE_SIZE, COLOR_WHITE ),
    ],
    update_info: Some((1, format_i32_02)),
};

pub const HEADER_SCORE: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::TopCenter,
    align_self: AlignSelf::Start,      // セル内の上寄せ
    justify_self: JustifySelf::Center, // セル内の中央寄せ
    bg_color: COLOR_NONE,
    #[rustfmt::skip]
    textspans: &[
        ( " SCORE ", ASSETS_FONT_ORBITRON_BLACK      , HEADER_LABEL_SIZE, COLOR_GOLD  ),
        ( NUM5     , ASSETS_FONT_PRESSSTART2P_REGULAR, HEADER_VALUE_SIZE, COLOR_WHITE ),
    ],
    update_info: Some((1, format_i32_05)),
};

pub const HEADER_HI_SCORE: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::TopRight,
    align_self: AlignSelf::Start,   // セル内の上寄せ
    justify_self: JustifySelf::End, // セル内の右寄せ
    bg_color: COLOR_NONE,
    #[rustfmt::skip]
    textspans: &[
        ( " Hi-SCORE ", ASSETS_FONT_ORBITRON_BLACK      , HEADER_LABEL_SIZE, COLOR_GOLD  ),
        ( NUM5        , ASSETS_FONT_PRESSSTART2P_REGULAR, HEADER_VALUE_SIZE, COLOR_WHITE ),
    ],
    update_info: Some((1, format_i32_05)),
};

//------------------------------------------------------------------------------

pub const FOOTER_FPS: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::BottomLeft,
    align_self: AlignSelf::End,       // セル内の下寄せ
    justify_self: JustifySelf::Start, // セル内の左寄せ
    bg_color: COLOR_NONE,
    #[rustfmt::skip]
    textspans: &[
        ( "  FPS ", ASSETS_FONT_ORBITRON_BLACK      , FOOTER_LABEL_SIZE, COLOR_TEAL   ),
        ( NUM3_2  , ASSETS_FONT_PRESSSTART2P_REGULAR, FOOTER_VALUE_SIZE, COLOR_SILVER ),
    ],
    update_info: Some((1, format_f32_06_2)),
};

pub const FOOTER_AUTHER: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::BottomCenter,
    align_self: AlignSelf::End,        // セル内の下寄せ
    justify_self: JustifySelf::Center, // セル内の中央寄せ
    bg_color: COLOR_NONE,
    #[rustfmt::skip]
    textspans: &[
        ( COPYRIGHT, ASSETS_FONT_ORBITRON_BLACK, FOOTER_LABEL_SIZE, COLOR_TEAL ),
    ],
    update_info: None,
};

pub const FOOTER_POWERED_BY: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::BottomRight,
    align_self: AlignSelf::End,     // セル内の下寄せ
    justify_self: JustifySelf::End, // セル内の右寄せ
    bg_color: COLOR_NONE,
    #[rustfmt::skip]
    textspans: &[
        ( "Powered by ", ASSETS_FONT_ORBITRON_BLACK, FOOTER_LABEL_SIZE, COLOR_TEAL   ),
        ( "RUST"       , ASSETS_FONT_ORBITRON_BLACK, FOOTER_LABEL_SIZE, COLOR_SILVER ),
        ( " & "        , ASSETS_FONT_ORBITRON_BLACK, FOOTER_LABEL_SIZE, COLOR_TEAL   ),
        ( "BEVY  "     , ASSETS_FONT_ORBITRON_BLACK, FOOTER_LABEL_SIZE, COLOR_SILVER ),
    ],
    update_info: None,
};

////////////////////////////////////////////////////////////////////////////////

// おまけ(蟹)
pub const SPRITE_KANI_GRID_X: i32 = SCREEN_GRIDS_WIDTH - 4;
pub const SPRITE_KANI_GRID_Y: i32 = SCREEN_GRIDS_HEIGHT - 1;
pub const SPRITE_KANI_MAGNIFY: f32 = 0.9;
pub const SPRITE_KANI_ALPHA: Color = Color::srgba(1.0, 1.0, 1.0, 0.6);

////////////////////////////////////////////////////////////////////////////////

// End of code.
