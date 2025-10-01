use super::*;

////////////////////////////////////////////////////////////////////////////////

// ヘッダー・フッターの表示を更新する
pub fn update_header_footer(
    query_text_spans: Query<(
        Entity,
        &header_footer::Position,
        &header_footer::UpdateInfo,
    )>,
    option_record: Option<Res<Record>>,
    diag_store: Res<DiagnosticsStore>,
    mut text_writer: TextUiWriter,
) -> Result
{
    // 準備
    let record = option_record.ok_or("Resource not found.")?;

    // ヘッダー・フッターの中でupdate_infoがSomeのものを処理する
    for (root_entity, position, update_info) in query_text_spans
    {
        if let Some((index, fn_format)) = update_info.0
        {
            // 表示文字列を作る
            let value = match position
            {
                // 各種Record
                header_footer::Position::TopLeft => fn_format(&record.stage()),
                header_footer::Position::TopCenter => fn_format(&record.score()),
                header_footer::Position::TopRight => fn_format(&record.hi_score()),
                // FPS
                header_footer::Position::BottomLeft => diag_store
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .map_or(NUM3_2.to_string(), |fps| {
                        fps.average()
                            .map_or(NUM3_2.to_string(), |avg| fn_format(&avg))
                    }),
                // update_info.0がSomeなのでここには来ないはず
                _ => unimplemented!("Configuration error."),
            };

            // 表示を更新する
            let mut text = text_writer.get_text(root_entity, index).ok_or(
                format!("No entity ({position:?}) with a matching index: {index}"),
            )?;
            *text = value;
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
