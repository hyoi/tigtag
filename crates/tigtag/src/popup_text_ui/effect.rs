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
#[derive(Clone)]
pub struct ScalingParams
{
    pub unselected_size: f32,
    pub selected_base_size: f32,
    pub max_scaling: f32,
    pub cycle: f32,
    pub spans_index: usize,
    pub spans_len: usize,
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
pub trait PopupMenu
{
    fn init(&mut self);
    fn resize_font(&mut self, time_delta: f32) -> f32;
    fn selected_menuitem_index(&self) -> usize;
    fn selected_menuitem_index_mut(&mut self) -> &mut usize;
    fn menuitem_len(&self) -> usize;
    fn unselected_size(&self) -> f32;
    fn selected_base_size(&self) -> f32;
}
pub trait PopupMenuItem {}

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
    mut query_blinking_params: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
) -> Result
where
    T: Component<Mutability = Mutable> + Blinking,
{
    // 準備
    let (mut blinking, text_spans) = query_blinking_params.single_mut()?;

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

// pauseメニューのパラメータを初期化する
pub fn init_pause_menu<T>(
    mut query_scaling_params: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
) -> Result
where
    T: Component<Mutability = Mutable> + PopupMenu,
{
    // 準備
    let (mut scaling, text_spans) = query_scaling_params.single_mut()?;
    let root_entity = text_spans.iter().next().ok_or("Text spans not found.")?;

    //初期化
    scaling.init();
    for index in 0..scaling.menuitem_len()
    {
        let (size, color) = if index == 0
        {
            (scaling.selected_base_size(), MENU_ITEM_COLOR_SELECTED)
        }
        else
        {
            (scaling.unselected_size(), MENU_ITEM_COLOR_NORMAL)
        };

        let mut text_font = text_writer
            .get_font(root_entity, index)
            .ok_or(format!("No entity with a matching index: {index}"))?;
        text_font.font_size = size;

        let mut text_color = text_writer
            .get_color(root_entity, index)
            .ok_or(format!("No entity with a matching index: {index}"))?;
        *text_color = color.into();
    }

    Ok(())
}

// pauseメニューのメニューアイテムを拡縮表示する
pub fn scale_selected_text<T>(
    mut query_scaling_params: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
    time: Res<Time>,
) -> Result
where
    T: Component<Mutability = Mutable> + PopupMenu,
{
    // 準備
    let (mut scaling, text_spans) = query_scaling_params.single_mut()?;

    let root_entity = text_spans.iter().next().ok_or("Text spans not found.")?;
    let span_index = scaling.selected_menuitem_index();

    //テキストを拡大縮小させる
    let mut text_font = text_writer
        .get_font(root_entity, span_index)
        .ok_or(format!("No entity with a matching index: {span_index}"))?;

    text_font.font_size = scaling.resize_font(time.delta().as_secs_f32());

    Ok(())
}

// pauseメニューのメニューアイテムを選択し適用する
pub fn select_menu_item<T>(
    mut query_scaling_params: Query<(&mut T, &Children)>,
    mut text_writer: TextUiWriter,
    input_keycode: Res<ButtonInput<KeyCode>>,
    // opt_target_gamepad: Option<ResMut<my_utils::misc::TargetGamepad>>,
    // qry_gamepads: Query<&Gamepad>,
    mut event_exit: EventWriter<EventAppExit>,
    // mut event_config: EventWriter<EventAppConfig>,
) -> Result
where
    T: Component<Mutability = Mutable> + PopupMenu,
{
    // 準備
    let (mut scaling, text_spans) = query_scaling_params.single_mut()?;
    let root_entity = text_spans.iter().next().ok_or("Text spans not found.")?;

    let mut index = scaling.selected_menuitem_index() as i32;
    let old_index = index;
    let max_index = scaling.menuitem_len() as i32 - 1;
    let mut apply = false;
    input_keycode.get_just_pressed().for_each(|keycode| {
        (index, apply) = match keycode
        {
            KeyCode::ArrowUp | KeyCode::KeyW => ((index - 1).max(0), apply),
            KeyCode::ArrowDown | KeyCode::KeyS =>
                ((index + 1).min(max_index), apply),
            KeyCode::Enter | KeyCode::Space => (index, true),
            _ => (index, apply),
        }
    });

    if index != old_index
    {
        let mut text_font = text_writer
            .get_font(root_entity, old_index as usize)
            .ok_or(format!("No entity with a matching index: {old_index}"))?;
        text_font.font_size = scaling.unselected_size();

        let mut text_color = text_writer
            .get_color(root_entity, old_index as usize)
            .ok_or(format!("No entity with a matching index: {old_index}"))?;
        *text_color = MENU_ITEM_COLOR_NORMAL.into();

        let mut text_color = text_writer
            .get_color(root_entity, index as usize)
            .ok_or(format!("No entity with a matching index: {index}"))?;
        *text_color = MENU_ITEM_COLOR_SELECTED.into();

        *scaling.selected_menuitem_index_mut() = index as usize;
    }

    if apply && index == 1
    {
        event_exit.write(EventAppExit);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
