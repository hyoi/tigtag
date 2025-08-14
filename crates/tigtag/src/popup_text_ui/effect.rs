use super::*;

////////////////////////////////////////////////////////////////////////////////

// 効果の種類毎のパラメータ
#[derive(Clone)]
pub struct CountDownParams
{
    pub start_value: i32,
    pub timer: Timer,
    pub counter: i32,
    pub spans_index: usize,
}
#[derive(Clone)]
pub struct BlinkingParams
{
    pub cycle: f32,
    pub spans_index: usize,
}
#[derive(Clone)]
pub struct HitAnyKeyParams
{
    pub ignore_keys: &'static [KeyCode],
}

// 効果の種類毎のトレイト
pub trait CountDown
{
    fn init(&mut self);
    fn index(&self) -> usize;
    fn timer(&mut self) -> &mut Timer;
    fn counter(&mut self) -> &mut i32;
    fn start_value(&self) -> i32;
}
pub trait Blinking
{
    fn alpha(&mut self, time_delta: f32) -> f32;
    fn index(&self) -> usize;
}
pub trait HitAnyKey
{
    fn ignore_keys(&self) -> &[KeyCode];
}

////////////////////////////////////////////////////////////////////////////////

//カウントダウンのパラメータを初期化する
pub fn init_countdown<T>(
    mut query_countdown_params: Query<&mut T>,
    mut event_countdown: ResMut<Events<EventCountDown>>,
) -> Result
where
    T: Component<Mutability = Mutable> + CountDown,
{
    //準備
    let mut countdown_params = query_countdown_params.single_mut()?;

    //初期化
    countdown_params.init();
    event_countdown.clear(); //[対策]StageClear等のCountDownイベントが生きているので（v0.16.1）

    Ok(())
}

//カウントダウンを表示しゼロになったらEventをセットする
pub fn countdown<T>(
    mut query_countdown_params: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
    mut event: EventWriter<EventCountDown>,
) -> Result
where
    T: Component<Mutability = Mutable> + CountDown,
{
    // 準備
    let (mut countdown, text_spans) = query_countdown_params.single_mut()?;

    let root_entity = text_spans.iter().next().ok_or("Text spans not found.")?;
    let span_index = countdown.index();

    //1秒経過したら
    if countdown.timer().tick(time.delta()).finished()
    {
        *countdown.counter() += 1; //カウント
        countdown.timer().reset(); //1秒タイマーリセット
    }

    //カウントダウンが続いているなら
    let count = countdown.start_value() - *countdown.counter();

    if count >= 0
    {
        //カウントダウンの表示を更新する
        let mut text = text_writer
            .get_text(root_entity, span_index)
            .ok_or(format!("No entity with a matching index: {span_index}"))?;
        *text = format!("{}", count);
    }
    else
    {
        //そうでないならカウントダウン終了のイベントを発行
        event.write(EventCountDown);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//テキストを明滅させる
pub fn blinking_text<T>(
    mut query_countdown_params: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
) -> Result
where
    T: Component<Mutability = Mutable> + Blinking,
{
    // 準備
    let (mut blinking, text_spans) = query_countdown_params.single_mut()?;

    let root_entity = text_spans.iter().next().ok_or("Text spans not found.")?;
    let span_index = blinking.index();

    //透明度を変化させる
    let alpha = blinking.alpha(time.delta().as_secs_f32());
    let mut text_color = text_writer
        .get_color(root_entity, span_index)
        .ok_or(format!("No entity with a matching index: {span_index}"))?;

    text_color.set_alpha(alpha);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//何かしら入力があればEventをセットする
pub fn hit_any_key<T>(
    query_hitanykey_params: Query<&T>,
    input_keycode: Res<ButtonInput<KeyCode>>,
    // opt_target_gamepad: Option<ResMut<my_utils::misc::TargetGamepad>>,
    // qry_gamepads: Query<&Gamepad>,
    mut event: EventWriter<EventHitAnyKey>,
) -> Result
where
    T: Component<Mutability = Mutable> + HitAnyKey,
{
    // 準備
    let hitanykey = query_hitanykey_params.single()?;
    let ignore_keys = hitanykey.ignore_keys();

    //無視するキーなら
    let check1 = input_keycode.any_pressed(ignore_keys.iter().copied());
    let check2 = input_keycode.any_just_pressed(ignore_keys.iter().copied()); //[Fn]対策
    let mut is_pressed = if check1 || check2
    {
        return Ok(());
    }
    else
    {
        input_keycode.get_just_pressed().len()
    };

    #[cfg(debug_assertions)]
    if is_pressed != 0
    {
        input_keycode.get_just_pressed().for_each(|key| {
            dbg!(key);
        });
    }

    // //無視ボタン以外のボタン入力はあるか
    // if is_pressed == 0
    // {   let Some ( gamepad ) = opt_gamepad else { return };
    //     let Some ( id ) = gamepad.id() else { return };
    //     for buton in HAK_IGNORE_BUTTONS
    //     {   if inbtn.pressed( GamepadButton::new( id, *buton ) ) { return }
    //     }
    //     is_pressed = inbtn.get_just_pressed().filter( |x| x.gamepad == id ).count();
    // }

    // Eventをセットする
    if is_pressed > 0
    {
        event.write(EventHitAnyKey);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
