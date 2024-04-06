use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   OnExit ( MyState::InitGame ),
            (   //PAUSE用UIをspawnする
                spawn_ui_text,
            )
        )
        .add_systems
        (   Update,
            (   //PAUSEメニュー表示／非表示のトグル処理（close_on_escより前に実行する）
                show_and_hide_pause_menu.before( bevy::window::close_on_esc ),
            )
        )
        .add_systems
        (   Update,
            (   //選択中のメニューアイテムの演出
                effect::repeat_scaling_text::<SelectedMenuItem>,

                //メニューアイテムの選択と決定
                select_and_apply,
            )
            .chain() //実行順の固定
            .run_if( in_state( MyState::Pause ) )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//メニューアイテムの色
const MENU_ITEM_COLOR_SELECTED: Color = Color::YELLOW;
const MENU_ITEM_COLOR_NORMAL  : Color = Color::CYAN;

//メニューアイテムの設定
const UI_PAUSE: &[ MessageSect ] =
&[  ( "PAUSE", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 2.0, MENU_ITEM_COLOR_SELECTED ),
];

const UI_EXIT: &[ MessageSect ] =
&[  ( " \n" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 2.0, Color::NONE            ),
    ( "EXIT", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 2.0, MENU_ITEM_COLOR_NORMAL ),
];

//PAUSEメニュー表示切替のキー／ボタン
pub const ESC_KEY: KeyCode = KeyCode::Escape;
pub const PAUSE_BUTTON: GamepadButtonType = GamepadButtonType::Select; //PS4[SHARE]

//PAUSEメニューアイテム選択に使うキー／ボタン
const KEY_UP  : KeyCode = KeyCode::ArrowUp;
const KEY_DOWN: KeyCode = KeyCode::ArrowDown;

const PAD_UP  : GamepadButtonType = GamepadButtonType::DPadUp;
const PAD_DOWN: GamepadButtonType = GamepadButtonType::DPadDown;

const APPLY_KEYS: &[ KeyCode ] = &[ KeyCode::Enter, KeyCode::Space ];
const APPLY_IGNORE_KEYS: &[ KeyCode ] = &[ KeyCode::AltLeft, KeyCode::AltRight ]; //無視するキー
const APPLY_PAD: GamepadButtonType = GamepadButtonType::East;

////////////////////////////////////////////////////////////////////////////////

//PAUSEメニューのComponent（可視化制御用）
#[derive( Component, Default )]
struct PauseMenu
{   selected_item: PauseMenuItem,
    back_to: MyState,
}

//PAUSEメニューアイテムのComponent
#[derive( Component, PartialEq, Clone, Copy, Default )]
enum PauseMenuItem
{   #[default] Pause,
    Exit,
}

//強調表示で必要なQueryの定義
type ParamsMenuItem<'a> = ( Entity, &'a mut Transform, &'a mut Text, &'a PauseMenuItem );

//PAUSEメニューのメソッド
impl PauseMenu
{   //選択されたアイテムの表示を強調する
    fn highlight_selected_item( &self, mut qry_menu_items: Query<ParamsMenuItem>, cmds: &mut Commands )
    {   for ( id, mut transform, mut text, menu_item ) in qry_menu_items.iter_mut()
        {   transform.scale = Vec3::ONE; //表示スケールを1倍へ
            if menu_item == &self.selected_item
            {   //選択中なら
                text.set_color( MENU_ITEM_COLOR_SELECTED );
                cmds.entity( id ).insert( SelectedMenuItem::default() );
            }
            else
            {   //非選択なら
                text.set_color( MENU_ITEM_COLOR_NORMAL );
                cmds.entity( id ).remove::<SelectedMenuItem>();
            }
        }
    }
}

//選択中メニューアイテムの拡縮のComponent
#[derive( Component, Default )]
struct SelectedMenuItem { cycle: f32 }

//拡縮させるためのトレイトの実装
impl effect::Scaling for SelectedMenuItem
{   fn scale( &mut self, time_delta: f32 ) -> f32
    {   let radian = &mut self.cycle;
        *radian += TAU * time_delta / 2.0;
        *radian -= if *radian > TAU { TAU } else { 0.0 };

        ( *radian ).sin() * 0.2 + 1.2 //1.0 ～ 1.4
    }
}

////////////////////////////////////////////////////////////////////////////////

//PAUSEメニューをspawnする
fn spawn_ui_text
(   qry_hidden_node: Query<Entity, With<HiddenNode>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( hidden_node ) = qry_hidden_node.get_single() else { return };

    //メニューアイテムの準備
    let items = &mut Vec::new();
    let item = misc::text_ui( UI_PAUSE, &asset_svr );
    items.push( cmds.spawn( ( item, PauseMenuItem::Pause ) ).id() );

    if ! WASM()
    {   //WASMの時はEXITのメニューアイテムを作らないようにする
        let item = misc::text_ui( UI_EXIT , &asset_svr );
        items.push( cmds.spawn( ( item, PauseMenuItem::Exit ) ).id() );
    }

    //レイアウト用ノードの準備
    let mut layout_node = NodeBundle
    {   style: Style
        {   flex_direction : FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items    : AlignItems::Center,
            grid_column    : GridPlacement::start_end( 1, 4 ), //３列連結
            grid_row       : GridPlacement::start_end( 2, 3 ), //２行目
            ..default()
        },
        background_color: Color::rgba( 0.2, 0.2, 0.2, 0.9).into(),
        visibility: Visibility::Hidden, //初期非表示
        ..default()
    };

    if DEBUG()
    {   //debug時にborderを可視化
        layout_node.style.border = UiRect::all( Val::Px( 1.0 ) );
        layout_node.border_color = Color::RED.into();
    }

    //PAUSEメニューのspawn
    let child = cmds
        .spawn( ( layout_node, PauseMenu::default() ) )
        .push_children( items )
        .id();
    cmds.entity( hidden_node ).add_child( child );
}

////////////////////////////////////////////////////////////////////////////////

//PAUSEメニューの表示／非表示 切り替え
fn show_and_hide_pause_menu
(   mut qry_menu: Query<( &mut Visibility, &mut PauseMenu )>,
    qry_menu_items: Query<ParamsMenuItem>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    mut input_keyboard: ResMut<ButtonInput<KeyCode>>,
    input_gamepad: Res<ButtonInput<GamepadButton>>,
    mut state: ResMut<State<MyState>>,
    mut cmds: Commands,
)
{   //Note: MyState::InitGameでUIをspawnする前も実行される。（Updateスケジュールに登録するので）
    //そのため let ( mut visibility, mut menu ) = qry_menu.single_mut(); と書くと panic する。
    let Ok ( ( mut visibility, mut menu ) ) = qry_menu.get_single_mut() else { return };

    //キーの状態
    let mut is_pressed = input_keyboard.just_pressed( ESC_KEY );
    input_keyboard.reset( ESC_KEY ); //Note:[Esc]押下げをリセットする（close_on_esc対策）

    //ゲームパッドのボタンの状態
    if ! is_pressed
    {   let Some ( gamepad ) = opt_gamepad else { return }; //Resource未登録
        let Some ( id ) = gamepad.id() else { return };     //gamepad未接続

        is_pressed = input_gamepad.just_pressed( GamepadButton::new( id, PAUSE_BUTTON ) );
    }

    //表示／非表示トグル処理
    if is_pressed
    {   if state.get().is_pause()
        {   //PAUSEメニューを非表示
            *visibility = Visibility::Hidden; //メニューを不可視化
            *state = State::new( menu.back_to ); //OnEnter／OnExitを実行せす遷移する
        }
        else
        {   //メニューアイテムを初期化
            menu.selected_item = PauseMenuItem::Pause;
            menu.highlight_selected_item( qry_menu_items, &mut cmds );

            //PAUSEメニューを表示
            *visibility = Visibility::Visible; //メニューを可視化
            menu.back_to = *state.get(); //遷移元のStateを保存する
            *state = State::new( MyState::Pause ); //OnEnter／OnExitを実行せす遷移する
        }
    }
}

//PAUSEメニューアイテムの選択と決定
#[allow(clippy::too_many_arguments)]
fn select_and_apply
(   mut qry_menu: Query<( &mut Visibility, &mut PauseMenu )>,
    qry_menu_items: Query<ParamsMenuItem>,
    mut qry_windows: Query<Entity, With<Window>>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
    input_gamepad: Res<ButtonInput<GamepadButton>>,
    mut state: ResMut<State<MyState>>,
    mut cmds: Commands,
)
{   let Ok ( ( mut visibility, mut menu ) ) = qry_menu.get_single_mut() else { return };
    let old = &menu.selected_item;
    let mut new = *old;
    let mut apply = false;

    //キーの状態
    for keycode in input_keyboard.get_pressed()
    {   match *keycode
        {   KEY_UP =>
                if *old == PauseMenuItem::Exit
                {   new = PauseMenuItem::Pause
                },
            KEY_DOWN =>
                if *old == PauseMenuItem::Pause && ! WASM()
                {   new = PauseMenuItem::Exit
                },
            _ =>
                if APPLY_KEYS.contains( keycode )
                && ! input_keyboard.any_pressed( APPLY_IGNORE_KEYS.iter().copied() )
                {   apply = true;
                },
        }
    }

    //ゲームパッドのボタンの状態
    if new == *old && ! apply
    {   let Some ( gamepad ) = opt_gamepad else { return }; //Resource未登録
        let Some ( id ) = gamepad.id() else { return };     //gamepad未接続

        for button in input_gamepad.get_pressed()
        {   if button.gamepad != id { continue } //ターゲットのゲームパッド以外は飛ばす
            match button.button_type
            {   PAD_UP =>
                    if *old == PauseMenuItem::Exit
                    {   new = PauseMenuItem::Pause
                    },
                PAD_DOWN =>
                    if *old == PauseMenuItem::Pause && ! WASM()
                    {   new = PauseMenuItem::Exit
                    },
                APPLY_PAD =>
                    apply = true,
                _ => (),
            }
        }
    }

    //強調表示するメニューアイテムを変える
    if new != *old
    {   menu.selected_item = new;
        menu.highlight_selected_item( qry_menu_items, &mut cmds );
    }

    //選択されたメニューアイテムを実行
    if apply
    {   match menu.selected_item
        {   PauseMenuItem::Pause =>
            {   //PAUSEメニューから脱出
                *visibility = Visibility::Hidden; //メニューを不可視化
                *state = State::new( menu.back_to ); //OnEnter／OnExitを実行せす遷移する
            },
            PauseMenuItem::Exit =>
            {   //ゲーム終了
                qry_windows.iter_mut().for_each( |id| cmds.entity( id ).despawn_recursive() );
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.