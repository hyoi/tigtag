use super::*;

////////////////////////////////////////////////////////////////////////////////

// Text Spansの一部を明滅させる表示効果のパラメータ
#[derive(Clone)]
pub struct BlinkingParams
{
    pub cycle: f32,
    pub spans_index: usize,
}

// Blinkingトレイト境界
pub trait Blinking
{
    fn alpha(&mut self, time_delta: f32) -> f32;
    fn index(&self) -> usize;
}

// Text Spansの一部を明滅させるSystem
pub fn blinking_text<T>(
    mut query_blinking: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
) -> Result
where
    T: Component<Mutability = Mutable> + OverlayMessage + Blinking,
{
    // 準備
    let (mut params, text_spans) = query_blinking.single_mut()?;

    let root_entity = text_spans.iter().next().ok_or("Root span not found.")?;
    let span_index = params.index();
    let alpha = params.alpha(time.delta().as_secs_f32());

    // indexで指定したText spanの透明度を変える
    let mut text_color = text_writer
        .get_color(root_entity, span_index)
        .ok_or(format!("No entity with a matching index: {span_index}"))?;
    text_color.set_alpha(alpha);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// Text Spansの一部にカウントダウンを表示しそれが終了したら状態変化させる効果のパラメータ
#[derive(Clone)]
pub struct CountDownParams
{
    pub start_value: i32,
    pub timer: Timer,
    pub counter: i32,
    pub spans_index: usize,
}

// CountDownトレイト境界
pub trait CountDown
{
    fn index(&self) -> usize;
    fn timer(&mut self) -> &mut Timer;
    fn counter(&mut self) -> &mut i32;
    fn start_value(&self) -> i32;
}

// Text Spansの一部にカウントダウンを表示しそれが終了したら状態変化させるSystem
pub fn countdown<T>(
    mut query_countdown: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
    mut event: MessageWriter<CountDownEnded>,
) -> Result
where
    T: Component<Mutability = Mutable> + CountDown,
{
    // 準備
    let (mut params, text_spans) = query_countdown.single_mut()?;

    let root_entity = text_spans.iter().next().ok_or("Root span not found.")?;
    let span_index = params.index();

    // 1秒経過したら
    if params.timer().tick(time.delta()).is_finished()
    {
        *params.counter() += 1; // カウントを減らす
        params.timer().reset(); // 1秒タイマーを再実行する
    }

    // カウントダウンが終わっていないなら
    let count = params.start_value() - *params.counter();
    if count >= 0
    {
        // indexで指定したText spanのカウントダウン表示を更新する
        let mut text_value = text_writer
            .get_text(root_entity, span_index)
            .ok_or(format!("No entity with a matching index: {span_index}"))?;
        *text_value = count.to_string();
    }
    else
    {
        // カウントダウンが終わったならイベントを発行
        event.write(CountDownEnded);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
