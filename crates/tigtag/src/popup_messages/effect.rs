use super::*;

////////////////////////////////////////////////////////////////////////////////

//効果「カウントダウン」のトレイト
pub trait CountDown
{
    fn init(&mut self);
    fn placeholder(&self) -> Option<usize>;
    fn timer(&mut self) -> &mut Timer;
    fn counter(&mut self) -> &mut i32;
    fn start_value(&self) -> i32;
}

//カウントダウンのパラメータを初期化する
pub fn init_count<T>(
    mut qrt_count_params: Query<&mut T>,
    mut event: ResMut<Events<EventCountDown>>,
) -> Result
where
    T: Component<Mutability = Mutable> + CountDown,
{
    //準備
    let mut count_params = qrt_count_params.single_mut()?;

    //初期化
    count_params.init();
    event.clear(); //[対策]StageClear等のCountDownイベントが生きているので（v0.16.1）

    Ok(())
}

//カウントダウンを表示しゼロになったらStateを変更する
pub fn count_down<T>(
    mut query: Query<(&Children, &mut T)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
    mut event: EventWriter<EventCountDown>,
) -> Result
where
    T: Component<Mutability = Mutable> + CountDown,
{
    let (children, mut count_params) = query.single_mut()?;
    let entity = children.iter().next().ok_or("Child Entity not found.")?;
    let index = count_params.placeholder().ok_or("err")?;

    //1秒経過したら
    if count_params.timer().tick(time.delta()).finished()
    {
        *count_params.counter() += 1; //カウント
        count_params.timer().reset(); //1秒タイマーリセット
    }

    //カウントダウンが続いているなら
    let count_down = count_params.start_value() - *count_params.counter();

    if count_down >= 0
    {
        //カウントダウンの表示を更新する
        let mut text = text_writer
            .get_text(entity, index)
            .ok_or(format!("No entity with a matching index: {index}"))?;
        *text = format!("{}", count_down);
    }
    else
    {
        //そうでないならカウントダウン終了のイベントを発行
        event.write(EventCountDown);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
