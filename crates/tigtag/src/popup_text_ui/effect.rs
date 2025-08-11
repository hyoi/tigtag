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

//効果「テキスト明滅」のトレイト
pub trait Blinking
{
    fn alpha(&mut self, time_delta: f32) -> f32;
    fn blink_index(&self) -> usize;
}

//効果「Hit Any Key」のトレイト
pub trait HitAnyKey {}

////////////////////////////////////////////////////////////////////////////////

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

//カウントダウンを表示しゼロになったらEventをセットする
pub fn count_down<T>(
    mut query: Query<(&Children, &mut T)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
    mut event: EventWriter<EventCountDown>,
) -> Result
where
    T: Component<Mutability = Mutable> + CountDown,
{
    //準備
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

//テキストを明滅させる
pub fn blinking_text<T>(
    mut query: Query<(&Children, &mut T)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
) -> Result
where
    T: Component<Mutability = Mutable> + Blinking,
{
    //準備
    let (children, mut count_params) = query.single_mut()?;
    let entity = children.iter().next().ok_or("Child Entity not found.")?;
    let index = count_params.blink_index();

    //透明度を変化させる
    let alpha = count_params.alpha(time.delta().as_secs_f32());
    let mut text_color = text_writer
        .get_color(entity, index)
        .ok_or(format!("No entity with a matching index: {index}"))?;

    text_color.set_alpha(alpha);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//Hit ANY Keyの処理で無視するキーとボタン
#[rustfmt::skip]
const IGNORE_KEYS: &[KeyCode] = &[
    KeyCode::AltLeft    , KeyCode::AltRight,
    KeyCode::ControlLeft, KeyCode::ControlRight,
    KeyCode::ShiftLeft  , KeyCode::ShiftRight,
    KeyCode::SuperLeft  , KeyCode::SuperRight,
    KeyCode::ArrowUp    , KeyCode::ArrowDown,
    KeyCode::ArrowRight , KeyCode::ArrowLeft,
    KeyCode::CapsLock   , KeyCode::Fn,
    KeyCode::Unidentified(NativeKeyCode::Windows(57443)), //ThinkPad [Fn]
];
// const IGNORE_BUTTONS: &[ GamepadButtonType ] =
// &[
//     GamepadButtonType::DPadUp,    GamepadButtonType::DPadDown,
//     GamepadButtonType::DPadRight, GamepadButtonType::DPadLeft,
// ];

//入力があればStateを変更する
pub fn hit_any_key<T>(
    input_keycode: Res<ButtonInput<KeyCode>>,
    opt_target_gamepad: Option<ResMut<my_utils::misc::TargetGamepad>>,
    qry_gamepads: Query<&Gamepad>,
    mut event: EventWriter<EventHitAnyKey>,
) -> Result
where
    T: Component<Mutability = Mutable> + HitAnyKey,
{
    //無視キー以外のキー入力はあるか
    if input_keycode.any_pressed(IGNORE_KEYS.iter().copied())
    {
        return Ok(());
    }
    if input_keycode.any_just_pressed(IGNORE_KEYS.iter().copied())
    {
        return Ok(());
    } //[Fn]対策
    let mut is_pressed = input_keycode.get_just_pressed().len();

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

    //Stateを遷移させる
    if is_pressed > 0
    {
        event.write(EventHitAnyKey);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
