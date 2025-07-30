use super::*;

////////////////////////////////////////////////////////////////////////////////

// ScoreとStageの初期化
pub fn initialize_record_except_hi_score(
    opt_record: Option<ResMut<Record>>,
) -> Result
{
    let mut record = opt_record.ok_or("ResMut<Record> not found.")?;

    //scoreとstageをゼロクリア
    *record.score_mut() = 0;
    *record.stage_mut() = 0;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// Pause等の処理の雛形
// fn hook_exit_app_key(
//     mut input_keycode: ResMut<ButtonInput<KeyCode>>,
//     mut state: ResMut<State<MyState>>,
//     mut back_to: Local<MyState>,
// )
// {
//     // キーが押下されているか
//     if input_keycode.just_pressed(EXIT_APP_KEY)
//     {
//         // キー押下をリセットする（misc::app_close_on_keyが実行されないように）
//         input_keycode.reset(EXIT_APP_KEY);

//         // Pauseのトグル処理
//         if state.get().is_pause()
//         {
//             // OnEnter／OnExitを実行せす遷移する
//             *state = State::new(*back_to);
//         }
//         else
//         {
//             // 遷移元のStateをローカルに保存する
//             *back_to = *state.get();

//             // OnEnter／OnExitを実行せす遷移する
//             *state = State::new(MyState::Pause);
//         }

//         #[cfg(debug_assertions)]
//         dbg!(state);
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// 更新対象の位置を指定するためのResource
#[derive(Resource)]
pub struct PlaceHolderFps(pub header_footer::Position, pub usize);

pub trait PlaceHolderFpsTrait
{
    fn position(&self) -> header_footer::Position;
    fn index(&self) -> usize;
}
impl PlaceHolderFpsTrait for PlaceHolderFps
{
    fn position(&self) -> header_footer::Position { self.0 }
    fn index(&self) -> usize { self.1 }
}

// FPSの表示を更新する
pub fn update_fps<T: Resource + PlaceHolderFpsTrait>(
    opt_display_info: Option<Res<T>>,
    qry_text_block: Query<(Entity, &header_footer::Position)>,
    mut text_writer: TextUiWriter,
    diag_store: Res<DiagnosticsStore>,
) -> Result
{
    // 準備
    let display_info = opt_display_info.ok_or("Res<{T}> not found.")?;
    let root_entity = qry_text_block
        .iter()
        .filter(|(_, position)| **position == display_info.position())
        .collect::<Vec<(Entity, &header_footer::Position)>>();

    // 書き換え
    if !root_entity.is_empty()
    {
        let entity = root_entity[0].0;
        let mut fps_text = text_writer
            .get_text(entity, display_info.index())
            .ok_or(format!(
                "No entity with a matching index: {}",
                display_info.index()
            ))?;

        *fps_text = diag_store.get(&FrameTimeDiagnosticsPlugin::FPS).map_or(
            NA3_2.to_string(),
            |fps| {
                fps.average()
                    .map_or(NA3_2.to_string(), |avg| format!("{avg:06.2}"))
            },
        );
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
