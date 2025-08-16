// #![allow( dead_code )]
use super::*;

////////////////////////////////////////////////////////////////////////////////

// キー入力でアプリ終了
pub fn app_close_on_key(
    qry_window: Query<(Entity, &Window)>,
    input_keycode: Res<ButtonInput<KeyCode>>,
    mut cmds: Commands,
)
{
    qry_window.iter().for_each(|(id, window)| {
        if window.focused && input_keycode.just_pressed(EXIT_APP_KEY)
        {
            cmds.entity(id).despawn();
        }
    });
}

////////////////////////////////////////////////////////////////////////////////

// ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode(
    mut query_window: Query<&mut Window>,
    input_keycode: Res<ButtonInput<KeyCode>>,
    // qry_gamepads: Query<(Entity, &Gamepad)>,
    // opt_target_gamepad: Option<ResMut<TargetGamepad>>,
) -> Result
{
    // 準備
    let mut window = query_window.single_mut()?;

    // キーの押下状態
    let is_pressed = input_keycode.just_pressed(FULL_SCREEN_KEY)
        && input_keycode.any_pressed(FULL_SCREEN_MODIFIER_KEY.iter().copied());

    // ゲームパッドのボタン押下状態
    // if ! is_pressed
    // {   let Some ( target ) = opt_target_gamepad else { return }; //Resource未登録
    //     let Some ( target ) = target.entity() else { return };    //ゲームパッド未接続
    //     for ( entity, gamepad ) in qry_gamepads.iter()
    //     {   if entity != target { continue }
    //         is_pressed = gamepad.pressed( FULL_SCREEN_BUTTON );
    //     }
    // }

    // 切換(トグル動作)
    if is_pressed
    {
        #[cfg(debug_assertions)]
        dbg!("before", &window.mode, &window.resolution);

        match window.mode
        {
            WindowMode::Windowed =>
            {
                window.resolution.set_scale_factor(2.0);
                window.mode = WindowMode::Fullscreen(
                    MonitorSelection::Primary,
                    VideoModeSelection::Current,
                );
            }
            _ =>
            {
                window.resolution.set_scale_factor(1.0);
                window.mode = WindowMode::Windowed;
            }
        };

        #[cfg(debug_assertions)]
        dbg!("after", &window.mode, &window.resolution);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// QueryしたEnityを削除する（条件がComponent）
pub fn despawn_component<T: Component>(
    qry_entity: Query<Entity, With<T>>,
    mut cmds: Commands, // cmdsをmoveするので通常の関数としては使い勝手が悪い！
)
{
    qry_entity.iter().for_each(|id| cmds.entity(id).despawn());
}

////////////////////////////////////////////////////////////////////////////////

// UIを描画するカメラにComponent「IsDefaultUiCamera」を追加する
pub fn select_ui_camera(
    camera2d_entity: Query<Entity, With<Camera2d>>,
    camera3d_entity: Query<Entity, With<Camera3d>>,
    mut cmds: Commands,
) -> Result
{
    // カメラのEntity IDを決定する(優先:Camera2d)
    let id = camera2d_entity
        .single()
        .or_else(|_| camera3d_entity.single())?;

    // UIを描画するCameraにマーカーComponentを追加する
    cmds.entity(id).insert(IsDefaultUiCamera);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// UI Nodeのアウトラインを表示
pub fn toggle_ui_outline_gizmo(
    input: Res<ButtonInput<KeyCode>>,
    opt_ui_debug_options: Option<ResMut<UiDebugOptions>>,
)
{
    if let Some(mut options) = opt_ui_debug_options
        && input.just_pressed(SHOW_HIDE_GIZMO_TOGGLE_KEY)
    {
        options.toggle();
    }
}

////////////////////////////////////////////////////////////////////////////////

// 操作を受付けるgamepadのEntityを保存するResource
#[derive(Resource, Default)]
pub struct TargetGamepad(Option<Entity>);
impl TargetGamepad
{
    pub fn entity(&self) -> Option<Entity> { self.0 }
    pub fn entity_mut(&mut self) -> &mut Option<Entity> { &mut self.0 }
}

// 操作を受付けるgamepadを切り替える
// ※副作用：ResMut<TargetGamepad>が見つからない場合、登録する
pub fn detect_gamepad_connection(
    mut qry_gamepads: Query<(Entity, &Name), With<Gamepad>>,
    opt_gamepad: Option<ResMut<TargetGamepad>>,
    mut cmds: Commands,
)
{
    // gamepadの接続状態を調べてResourceを更新するクロージャ
    let mut update_gamepad_connection = |gamepad: &mut TargetGamepad| {
        // gamepadの接続が保存されているなら
        if let Some(entity) = gamepad.entity()
        {
            // その接続が切断されたか？
            if !qry_gamepads.contains(entity)
            {
                // 新たに接続を保存しようとする（結果はNoneかもしれない）
                let (opt_a, _opt_b) = qry_gamepads.iter_mut().next().unzip();
                *gamepad.entity_mut() = opt_a;

                #[cfg(debug_assertions)]
                dbg!(&_opt_b, gamepad.entity()); // Some⇒Some、Some⇒None
            }
        }
        else if !qry_gamepads.is_empty()
        {
            // 新たに接続を保存する
            let (opt_a, _opt_b) = qry_gamepads.iter_mut().next().unzip();
            *gamepad.entity_mut() = opt_a;

            #[cfg(debug_assertions)]
            dbg!(&_opt_b, gamepad.entity()); // None⇒Some
        }
    };

    // Resourceが登録済みか？
    if let Some(mut gamepad) = opt_gamepad
    {
        // 既存のResourceを使用する
        update_gamepad_connection(&mut gamepad);
    }
    else
    {
        // Resourceがない(関数実行一回目)ならResourceを登録する
        let mut gamepad = TargetGamepad::default();
        update_gamepad_connection(&mut gamepad);
        cmds.insert_resource(gamepad);
    }
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたComponentを可視化する
pub fn show_component<T: Component>(mut query: Query<&mut Visibility, With<T>>)
{
    query.iter_mut().for_each(|mut v| *v = Visibility::Visible);
}

//QueryしたComponentを不可視にする
pub fn hide_component<T: Component>(mut query: Query<&mut Visibility, With<T>>)
{
    query.iter_mut().for_each(|mut v| *v = Visibility::Hidden);
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
