use super::*;

////////////////////////////////////////////////////////////////////////////////

// 拡張するヘッダー／フッター
const FOOTER_LEFT: header_footer::Position = header_footer::Position::BottomLeft;
#[rustfmt::skip]
const ADDITIONAL_DEMO_RECORD: &[header_footer::MessageSpan] = &[
    ( " demo ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.35, COLOR_TEAL   ),
    ( _DRPH_  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.26, COLOR_SILVER ),
];
const _DRPH_: &str = "##-#####"; // Placeholder

////////////////////////////////////////////////////////////////////////////////

// フッターのUIを改造する
pub fn add_text_spans(
    query_text_block: Query<(Entity, &header_footer::Position)>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
) -> Result
{
    // TARGET_FOOTER_LEFTのEntityを取得
    let (root_entity, _) = query_text_block
        .iter()
        .find(|(_, x)| **x == FOOTER_LEFT)
        .ok_or(format!(
            "header_footer::Position::{:?} Component not found.",
            FOOTER_LEFT
        ))?;

    // 既存のText,TextSpanの後ろにdemo recordを追加する
    cmds.entity(root_entity).with_children(|cmds| {
        for (span, file, size, color) in ADDITIONAL_DEMO_RECORD
        {
            cmds.spawn((
                TextSpan::new(*span), // 先頭以外はTextSpan
                TextFont {
                    font: asset_svr.load(*file),
                    font_size: *size,
                    ..default()
                },
                TextColor(*color),
            ));
        }
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// UIの表示を更新する(demo record)
pub fn update_demo_record(
    option_record: Option<ResMut<core_logic::Record>>,
    query_text_block: Query<(Entity, &header_footer::Position)>,
    mut text_writer: TextUiWriter,
) -> Result
{
    // 準備
    let mut record = option_record.ok_or("Resource not found.")?;
    let (root_entity, _) = query_text_block
        .iter()
        .find(|(_, x)| **x == FOOTER_LEFT)
        .ok_or(format!(
            "header_footer::Position::{:?} Component not found.",
            FOOTER_LEFT
        ))?;
    let index = FOOTER_FPS.textspans.len()
        + ADDITIONAL_DEMO_RECORD
            .iter()
            .position(|x| x.0 == _DRPH_)
            .ok_or(format!("Placeholder &str \"{}\" not found.", _DRPH_))?;

    // demo中スコアがdemoのハイスコアを超えた場合 記録を更新する
    if record.score() > record.demo_hi_score()
    {
        *record.demo_hi_score_mut() = record.score();
        *record.demo_stage_mut() = record.stage();
    }

    // 表示を更新する
    let value = format!("{:02}-{:05}", record.demo_stage(), record.demo_hi_score(),);
    let mut text = text_writer
        .get_text(root_entity, index)
        .ok_or(format!("No entity with a matching index: {index}"))?;
    *text = value;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
