use super::*;
use appctrl_input::{AppCtrl, IsPressed, MainKeyAndModifiers};

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        application
            // Pauseメニューから戻るStateを保管するResource（初期値は意味なし）
            .init_resource::<BackTo>()
            // Pauseメニューの準備（MyState::Initialize）
            .add_systems(
                OnExit(MyState::Initialize), // 他の全画面メッセージの準備と同じタイミング
                spawn_pause_menu,
            )
            // ループ処理１
            .add_systems(
                Update, // without MyState
                // Pauseメニューの表示／非表示（トグル動作）
                hook_input_and_toggle_pause
                    .in_set(misc::execution_order::Before::HitAnyKey)
                    .before(appctrl_input::send_exit_app_message),
            )
            // ループ処理２
            .add_systems(
                Update, // within MyState::MyState::Pause
                (
                    // 入力に従い、メニューアイテムを選択状態にし、適用する
                    select_and_apply_menuitem,
                    // メニューアイテムを拡縮表示する
                    scale_selected_text,
                )
                    .run_if(in_state(MyState::Pause)),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////

// メニューの設定を格納する変数
pub type MenuItemSetting = (PauseMenuItem, messages::TextUiSpan);
pub type MenuItemSettings = Vec<&'static MenuItemSetting>;

// Pauseメニューアイテムの識別子
#[allow(dead_code)]
#[rustfmt::skip]
pub enum PauseMenuItem { Label, Exit, Config }
impl PauseMenuItem
{
    fn is_label(&self) -> bool { matches!(self, Self::Label) }
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メニューのデータを格納する型
#[derive(Clone)]
pub struct OverlayMenuParams
{
    pub settings: MenuItemSettings,
    pub selected_index: i32,
}

// 全画面メニューのデータを格納する型（アイテムに追加した拡縮効果用）
#[derive(Clone, Default)]
pub struct ScalingItemParams
{
    pub scale_cycle: f32,
}

// 全画面メニューComponentのトレイト境界
pub trait OverlayMenu
where
    Self: Component<Mutability = Mutable> + Send + Sync + 'static,
{
    fn init(&mut self);
    fn settings(&self) -> &MenuItemSettings;
    fn selected_index_mut(&mut self) -> &mut i32;
}

// 全画面メニューComponentのトレイト境界（アイテムに拡縮効果を追加する）
pub trait ScalingItem
where
    Self: Component<Mutability = Mutable> + Send + Sync + 'static,
{
    fn scale_cycle_mut(&mut self) -> &mut f32;
}

////////////////////////////////////////////////////////////////////////////////

// メニューアイテムのマーカーComponent
#[derive(Component, Clone)]
pub struct MenuItem(i32);

////////////////////////////////////////////////////////////////////////////////

// 全画面メニュー（Pause）をspawnする
pub fn spawn_pause_menu(mut cmds: Commands, asset_svr: Res<AssetServer>) -> Result
{
    // 他の全画面メッセージと同じトレイトでspawnする
    let boxed_component = Box::new(OverlayPauseMenu::default());

    boxed_component.spawn_overlay_menu(&mut cmds, &asset_svr);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メニューspawn用のトレイト
pub trait BoxedOverlayMessage: Send + Sync + 'static
{
    fn spawn_overlay_menu(
        self: Box<Self>,
        cmds: &mut Commands,
        asset_svr: &Res<AssetServer>,
    );
}

// トレイトの実装
impl<T: Component + Clone + 'static + OverlayMenu> BoxedOverlayMessage for T
{
    fn spawn_overlay_menu(
        self: Box<Self>,
        cmds: &mut Commands,
        asset_svr: &Res<AssetServer>,
    )
    {
        let id = cmds
            .spawn((
                *self.clone(), // マーカーComponent
                Visibility::Hidden,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column, //アイテムを列方向へ並べる
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(PAUSE_MENU_BG_COLOR),
                GlobalZIndex(999),
            ))
            .id();

        // configファイルの設定ではスライスにまとめられているメニューアイテムを
        // 1つずつばらばらにspawnする。すべてTextになる（TextSpanではなくなる）
        self.settings().iter().enumerate().for_each(|(n, setting)| {
            cmds.entity(id).add_text_span(setting, n as i32, asset_svr);
        });
    }
}

////////////////////////////////////////////////////////////////////////////////

// bevyのEntityCommands型を拡張し、TextSpansを扱いやすくするトレイト
pub trait AddOverlatMenuItem
{
    fn add_text_span(
        &mut self,
        setting: &MenuItemSetting,
        index: i32,
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self;
}

// bevyのEntityCommands型を拡張し、TextSpansを扱いやすくするトレイトの実装
impl AddOverlatMenuItem for EntityCommands<'_>
{
    fn add_text_span(
        &mut self,
        setting: &MenuItemSetting,
        index: i32,
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self
    {
        // 準備
        let parent = self.id();
        let (_, (span, file, size, color)) = setting;

        // テキストブロックをspawnする
        self.commands_mut().spawn((
            MenuItem(index),  // マーカーComponent
            ChildOf(parent),  // 親Entity
            Text::new(*span), // 先頭はTextをspawn
            TextFont {
                font: asset_svr.load(*file),
                font_size: *size,
                ..default()
            },
            TextLayout {
                justify: Justify::Center,
                linebreak: LineBreak::NoWrap,
            },
            TextColor(*color),
            // Visibility::Visible, // ★debug時はVisible
        ));

        self // method-chain
    }
}

////////////////////////////////////////////////////////////////////////////////

// Pauseメニューから戻るStateを保管するResource（初期値は意味なし）
#[derive(Resource, Default, Deref, DerefMut)]
pub struct BackTo(pub MyState);

// Pauseメニューの表示／非表示（トグル動作）
fn hook_input_and_toggle_pause(
    appctrl_setting: Local<PauseMenuOpen>, //初回のみdefault()で初期化
    mut input_device: appctrl_input::InputDevicePack,
    option_state: Option<ResMut<State<MyState>>>,
    mut back_to: ResMut<BackTo>,
    mut query_overlay_menu: Query<(&mut Visibility, &mut OverlayPauseMenu)>,
    mut query_menuitems: Query<(
        &MenuItem,
        Entity,
        &mut UiTransform,
        &mut TextColor,
    )>,
    mut cmds: Commands,
) -> Result
{
    // 準備
    let mut state = option_state.ok_or("Resource not found.")?;

    // アプリ終了キーが押下されているなら（入力リセット付き）
    if input_device.is_pressed_with_reset(&*appctrl_setting)
        && let Ok((mut visibility, mut menu_setteing)) =
            query_overlay_menu.single_mut()
    {
        // State が MyState::Pause なら
        *visibility = if state.get().is_pause()
        {
            // OnEnter／OnExitを実行せずに元のStateへ戻ってメニューを非表示
            *state = State::new(**back_to);
            Visibility::Hidden
        }
        else
        {
            // Pausemニューの状態を初期化
            menu_setteing.init();

            // ラベルを除いたメニューアイテムのHash集合を作る
            let indexes: HashSet<_> = (0..)
                .zip(menu_setteing.settings())
                .filter_map(|(index, setting)| match setting.0.is_label()
                {
                    true => None,
                    false => Some(index),
                })
                .collect();

            // メニューアイテムを初期表示にする
            query_menuitems
                .iter_mut()
                .filter(|(MenuItem(index), _, _, _)| indexes.contains(index)) //ラベル以外
                .for_each(|(MenuItem(index), entity, mut transform, mut color)| {
                    if index == menu_setteing.selected_index_mut()
                    {
                        // 選択されているアイテム（初期位置）
                        **color = PAUSE_SELECTED_COLOR; // 選択状態カラー
                        cmds.entity(entity).insert(ScalingText); // 拡縮効果用Component追加
                        transform.scale = Vec2::ONE; // 拡縮リセット
                    }
                    else
                    {
                        // 選択されていないいアイテム
                        **color = PAUSE_NORMAL_COLOR; // 非選択状態カラー
                        cmds.entity(entity).remove::<ScalingText>(); // 拡縮効果用Component削除
                        transform.scale = Vec2::ONE; // 拡縮リセット
                    }
                });

            // OnEnter／OnExitを実行せずにStateをPauseへ変えてメニューを表示
            **back_to = *state.get(); // 元のStateをローカルに保存
            *state = State::new(MyState::Pause);
            Visibility::Visible
        };

        #[cfg(debug_assertions)]
        dbg!(state);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// キーとボタンの設定
#[derive_appctrl_input]
pub struct PauseMenuOpen;
#[derive_appctrl_input]
pub struct PauseMenuUp;
#[derive_appctrl_input]
pub struct PauseMenuDown;
#[derive_appctrl_input]
pub struct PauseMenuApply;
#[derive_appctrl_input]
pub struct PauseMenuCancel;

// 入力に従い、メニューアイテムを選択状態にし、適用する
#[allow(clippy::too_many_arguments)]
pub fn select_and_apply_menuitem(
    (up, down, apply, cancel): (
        Local<PauseMenuUp>,
        Local<PauseMenuDown>,
        Local<PauseMenuApply>,
        Local<PauseMenuCancel>,
    ),
    mut input_device: appctrl_input::InputDevicePack,
    mut query_overlay_menu: Query<(&mut OverlayPauseMenu, &mut Visibility)>,
    mut query_menuitems: Query<(
        &MenuItem,
        Entity,
        &mut UiTransform,
        &mut TextColor,
    )>,
    mut cmds: Commands,
    mut event_app_exit: MessageWriter<AppExit>,
    option_state: Option<ResMut<State<MyState>>>,
    back_to: ResMut<BackTo>,
) -> Result
{
    // 準備
    let (mut menu_setteing, mut visibility) = query_overlay_menu.single_mut()?;
    let mut state = option_state.ok_or("Resource not found.")?;

    // 入力が何もないなら
    let is_pressed_up = input_device.is_pressed(&*up);
    let is_pressed_down = input_device.is_pressed(&*down);
    let is_pressed_apply = input_device.is_pressed(&*apply);
    let is_pressed_cancel = input_device.is_pressed_with_reset(&*cancel); //[Esc]二重処理対策のreset
    if !(is_pressed_up || is_pressed_down || is_pressed_apply || is_pressed_cancel)
    {
        return Ok(());
    }

    // 移動量がゼロではないなら（入力が「上下」なら）
    let index_up = if is_pressed_up { -1 } else { 0 };
    let index_down = if is_pressed_down { 1 } else { 0 };
    let delta_index = index_up + index_down;
    if delta_index != 0
    {
        // ラベルを除いたメニューアイテムのindexでVecを作る(Vecで並び順を維持する)
        let indexes: Vec<_> = (0..)
            .zip(menu_setteing.settings())
            .filter_map(|(index, setting)| match setting.0.is_label()
            {
                true => None,
                false => Some(index),
            })
            .collect();

        // メニューアイテムの通常のindexをラベルを除いたindexへ変換する
        let mapped_index_old = indexes
            .iter()
            .position(|x| x == menu_setteing.selected_index_mut())
            .unwrap() as i32;
        let mapped_index_new = mapped_index_old + delta_index;

        // ラベルを除いた状態で移動可能なら
        if mapped_index_new >= 0 && mapped_index_new < indexes.len() as i32
        {
            // メニューアイテムの表示を変更する
            query_menuitems.iter_mut().for_each(
                |(MenuItem(index), entity, mut transform, mut color)| {
                    if *index == indexes[mapped_index_new as usize]
                    {
                        // 選択されているアイテム
                        **color = PAUSE_SELECTED_COLOR; // 選択状態カラー
                        cmds.entity(entity).insert(ScalingText); // 拡縮効果用Component追加
                        transform.scale = Vec2::ONE; // 拡縮リセット

                        *menu_setteing.scale_cycle_mut() = 0.0; // 拡縮サイクルのリセット
                        *menu_setteing.selected_index_mut() =
                            indexes[mapped_index_new as usize]; // 選択中index保存
                    }
                    else if *index == indexes[mapped_index_old as usize]
                    {
                        // 選択されていないいアイテム
                        **color = PAUSE_NORMAL_COLOR; // 非選択状態カラー
                        cmds.entity(entity).remove::<ScalingText>(); // 拡縮効果用Component削除
                        transform.scale = Vec2::ONE; // 拡縮リセット
                    }
                },
            );
        }
    }

    // 入力が「適用」なら
    if is_pressed_apply
    {
        let selected_index = *menu_setteing.selected_index_mut();
        let menuitem_doing = &menu_setteing.settings()[selected_index as usize].0;
        match *menuitem_doing
        {
            PauseMenuItem::Exit =>
            {
                event_app_exit.write(AppExit::Success); // アプリ終了イベントを送信
            }
            PauseMenuItem::Config => (), // No config now.
            _ => unreachable!("Bad Config"),
        }
    }

    // 入力が「キャンセル」なら
    if is_pressed_cancel
    {
        *state = State::new(**back_to);
        *visibility = Visibility::Hidden;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 表示効果（拡大縮小）を与えるトレイト
#[derive(Component)]
pub struct ScalingText;

// テキストを拡大縮小する
fn scale_selected_text(
    mut query_overlay_menu: Query<&mut OverlayPauseMenu>,
    mut query_menuitems: Query<&mut UiTransform, With<ScalingText>>,
    time: Res<Time>,
) -> Result
{
    if let Ok(mut pause_menu) = query_overlay_menu.single_mut()
    {
        query_menuitems.iter_mut().for_each(|mut transform| {
            let radian = pause_menu.scale_cycle_mut();
            *radian += TAU * time.delta().as_secs_f32();
            *radian -= if *radian > TAU { TAU } else { 0.0 };

            let size = 1.0 + (*radian).sin() * 0.2 + 0.2;
            transform.scale = Vec2::ONE * size;
        });
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
