use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        //MyState::InitGame スケジュール
        application.add_systems(
            OnExit(MyState::InitGame),
            //既存のフッターを改造する
            add_footerleft_text_spans.after(header_footer::spawn_header_footer),
        );

        //MyState::TitleDemo スケジュール
        application.add_systems(
            Update,
            //demo record表示の更新
            update_demo_record.run_if(in_state(MyState::TitleDemo)),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

//変更するヘッダー／フッター
const _DRPH_: &str = "##-#####";
const TARGET: header_footer::Position = header_footer::Position::BottomLeft;

//追加するtext spans
#[rustfmt::skip]
const ADDITIONAL_DEMO_RECORD: &[header_footer::MessageSpan] = &[
    ( " demo ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.35, COLOR_TEAL   ),
    ( _DRPH_  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.26, COLOR_SILVER ),
];

////////////////////////////////////////////////////////////////////////////////

//フッターのUIを改造する
fn add_footerleft_text_spans(
    qry_text_block: Query<(Entity, &header_footer::Position)>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
) -> Result
{
    //準備
    let (entity, _) =
        qry_text_block
            .iter()
            .find(|(_, x)| **x == TARGET)
            .ok_or(format!(
                "header_footer::Position::{:?} Component not found.",
                TARGET
            ))?;

    //demo record表示sectionを、既存のtext.sectionsに追加する
    cmds.entity(entity).with_children(|cmds| {
        for (span, file, size, color) in ADDITIONAL_DEMO_RECORD
        {
            cmds.spawn((
                TextSpan::new(*span), // 先頭以外はTextSpanをspawn
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

//UIの表示を更新する(demo record)
fn update_demo_record(
    qry_text_block: Query<(Entity, &header_footer::Position)>,
    opt_record: Option<ResMut<Record>>,
    mut text_writer: TextUiWriter,
) -> Result
{
    //準備
    let (entity, _) =
        qry_text_block
            .iter()
            .find(|(_, x)| **x == TARGET)
            .ok_or(format!(
                "header_footer::Position::{:?} Component not found.",
                TARGET
            ))?;
    let index = FOOTER_FPS.spans.len()
        + ADDITIONAL_DEMO_RECORD
            .iter()
            .position(|x| x.0 == _DRPH_)
            .ok_or(format!("Placeholder &str \"{}\" not found.", _DRPH_))?;
    let mut record = opt_record.ok_or("ResMut<Record> not found.")?;

    //demo中スコアがdemoのハイスコアを超えた場合 記録を更新する
    if record.score() > record.demo_hi_score()
    {
        *record.demo_hi_score_mut() = record.score();
        *record.demo_stage_mut() = record.stage();
    }

    //表示を更新する
    let value = format!("{:02}-{:05}", record.demo_stage(), record.demo_hi_score(),);
    let mut text = text_writer
        .get_text(entity, index)
        .ok_or(format!("No entity with a matching index: {index}"))?;
    *text = value;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
