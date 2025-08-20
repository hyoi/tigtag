use super::*;

////////////////////////////////////////////////////////////////////////////////

// シンプル ヘッダー／フッター用の情報
pub const HEADER_FOOTER: &[header_footer::TextBlock] = &[
    HEADER_STAGE,      // ステージ数
    HEADER_SCORE,      // スコア
    HEADER_HI_SCORE,   // ハイスコア
    FOOTER_FPS,        // FPS表示
    FOOTER_AUTHER,     // auther
    FOOTER_POWERED_BY, // Powered by
];

pub const HEADER_STAGE: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::TopLeft,
    align_self: AlignSelf::Start,     // セル内の上寄せ
    justify_self: JustifySelf::Start, // セル内の左寄せ
    bg_color: Srgba::NONE,
    #[rustfmt::skip]
    spans: &[
        ( " STAGE ", ASSETS_FONT_ORBITRON_BLACK      , HEADER_LABEL_SIZE, COLOR_GOLD  ),
        ( NA2      , ASSETS_FONT_PRESSSTART2P_REGULAR, HEADER_VALUE_SIZE, COLOR_WHITE ),
    ],
};

pub const HEADER_SCORE: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::TopCenter,
    align_self: AlignSelf::Start,      // セル内の上寄せ
    justify_self: JustifySelf::Center, // セル内の中央寄せ
    bg_color: Srgba::NONE,
    #[rustfmt::skip]
    spans: &[
        ( " SCORE ", ASSETS_FONT_ORBITRON_BLACK      , HEADER_LABEL_SIZE, COLOR_GOLD  ),
        ( NA5      , ASSETS_FONT_PRESSSTART2P_REGULAR, HEADER_VALUE_SIZE, COLOR_WHITE ),
    ],
};

pub const HEADER_HI_SCORE: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::TopRight,
    align_self: AlignSelf::Start,   // セル内の上寄せ
    justify_self: JustifySelf::End, // セル内の右寄せ
    bg_color: Srgba::NONE,
    #[rustfmt::skip]
    spans: &[
        ( " Hi-SCORE ", ASSETS_FONT_ORBITRON_BLACK      , HEADER_LABEL_SIZE, COLOR_GOLD  ),
        ( NA5         , ASSETS_FONT_PRESSSTART2P_REGULAR, HEADER_VALUE_SIZE, COLOR_WHITE ),
    ],
};

pub const FOOTER_FPS: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::BottomLeft,
    align_self: AlignSelf::End,       // セル内の下寄せ
    justify_self: JustifySelf::Start, // セル内の左寄せ
    bg_color: Srgba::NONE,
    #[rustfmt::skip]
    spans: &[
        ( "  FPS ", ASSETS_FONT_ORBITRON_BLACK      , FOOTER_FONT_SIZE      , COLOR_TEAL   ),
        ( NA3_2   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4 , COLOR_SILVER ),
    ],
};

pub const FOOTER_AUTHER: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::BottomCenter,
    align_self: AlignSelf::End,        // セル内の下寄せ
    justify_self: JustifySelf::Center, // セル内の中央寄せ
    bg_color: Srgba::NONE,
    #[rustfmt::skip]
    spans: &[
        ( COPYRIGHT, ASSETS_FONT_ORBITRON_BLACK, FOOTER_FONT_SIZE, COLOR_TEAL ),
    ],
};

pub const FOOTER_POWERED_BY: header_footer::TextBlock = header_footer::TextBlock {
    position: header_footer::Position::BottomRight,
    align_self: AlignSelf::End,     // セル内の下寄せ
    justify_self: JustifySelf::End, // セル内の右寄せ
    bg_color: Srgba::NONE,
    #[rustfmt::skip]
    spans: &[
        ( "Powered by ", ASSETS_FONT_ORBITRON_BLACK, FOOTER_FONT_SIZE, COLOR_TEAL   ),
        ( "RUST"       , ASSETS_FONT_ORBITRON_BLACK, FOOTER_FONT_SIZE, COLOR_SILVER ),
        ( " & "        , ASSETS_FONT_ORBITRON_BLACK, FOOTER_FONT_SIZE, COLOR_TEAL   ),
        ( "BEVY  "     , ASSETS_FONT_ORBITRON_BLACK, FOOTER_FONT_SIZE, COLOR_SILVER ),
    ],
};

const HEADER_LABEL_SIZE: f32 = PIXELS_PER_GRID * 0.58;
const HEADER_VALUE_SIZE: f32 = PIXELS_PER_GRID * 0.7;
const FOOTER_FONT_SIZE: f32 = PIXELS_PER_GRID * 0.48;

pub const NA2: &str = "##";
pub const NA5: &str = "#####";
pub const NA3_2: &str = "###.##";

////////////////////////////////////////////////////////////////////////////////

//ヘッダー情報を表示する位置と更新するtext spanのindexの指定
pub const PLACE_HOLDER: &[header_info::PlaceHolderLabel] = &[
    header_info::Stage(header_footer::TopLeft, 1), //表示位置（Stage）
    header_info::Score(header_footer::TopCenter, 1), //表示位置（Score）
    header_info::HiScore(header_footer::TopRight, 1), //表示位置（HiScore）
];

////////////////////////////////////////////////////////////////////////////////

// End of code.
