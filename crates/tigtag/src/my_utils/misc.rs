use super::*;

////////////////////////////////////////////////////////////////////////////////

// .run_if( condition )用の定数
pub const DEBUG: fn() -> bool = || cfg!(debug_assertions);
pub const WASM: fn() -> bool = || cfg!(target_arch = "wasm32");

////////////////////////////////////////////////////////////////////////////////

// gamepadのEntityを保存するResource
#[derive(Resource, Default)]
pub struct TargetGamepad(Option<Entity>);

// アクセス用メソッド
impl TargetGamepad
{
    pub fn entity(&self) -> Option<Entity> { self.0 }
    pub fn entity_mut(&mut self) -> &mut Option<Entity> { &mut self.0 }
}

// gamepadの接続を検出して必要なら切り替える
pub fn watch_gamepad_connections(
    option_target_gamepad: Option<ResMut<TargetGamepad>>,
    mut query_gamepads: Query<(Entity, &Name), With<Gamepad>>,
    mut cmds: Commands,
)
{
    // gamepadの接続状態を調べてResourceを更新する（クロージャ）
    let mut check_gamepad = |target_gamepad: &mut TargetGamepad| {
        // gamepadのEntityが保存されているなら
        if let Some(entity) = target_gamepad.entity()
        {
            // そのEntityが存在しないなら（切断）
            if !query_gamepads.contains(entity)
            {
                // Entityを探す（結果はNoneかもしれない）
                let (entity, _name) = query_gamepads.iter_mut().next().unzip();
                *target_gamepad.entity_mut() = entity;

                #[cfg(debug_assertions)]
                dbg!(&_name, target_gamepad.entity()); // Some⇒Some、Some⇒None
            }
        }
        else if !query_gamepads.is_empty()
        // 現在gamepadの接続があるなら
        {
            // Entityを探す（結果は必ずSome）
            let (entity, _name) = query_gamepads.iter_mut().next().unzip();
            *target_gamepad.entity_mut() = entity;

            #[cfg(debug_assertions)]
            dbg!(&_name, target_gamepad.entity()); // None⇒Some
        }
    };

    // Resourceが登録済みなら
    if let Some(mut target_gamepad) = option_target_gamepad
    {
        check_gamepad(&mut target_gamepad);
    }
    else
    {
        // Resourceが未登録なら、初期化した後に登録する
        let mut target_gamepad = TargetGamepad::default();
        check_gamepad(&mut target_gamepad);
        cmds.insert_resource(target_gamepad);
    }
}

////////////////////////////////////////////////////////////////////////////////

// システムの実行順を制御するためのSystemSet
#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub enum SystemOrderHitAnyKey
{
    Before, // HitAnyKeyの前に実行するSystem
    Marker,
    After, // HitAnyKeyの後に実行するSystem
}

//------------------------------------------------------------------------------

// Hit Any Keyの入力フィルターを保存するResource（Todo: ゲームパッドボタンも必要）
// #[derive(Resource)]
// pub struct MaskHitAnyKeyInput
// {
//     pub keys: FxHashSet<&'static KeyCode>,
//     pub buttons: FxHashSet<&'static GamepadButton>,
// }

// 何かしら入力があったことを通知するイベント
// #[derive(Event)]
// pub struct AnyButtonPressed;

// 何かしら入力によりEventHitAnyKeyをセットする
// pub fn check_hit_any_key(
//     option_masking_input: Option<Res<MaskHitAnyKeyInput>>,
//     input_keycode: Res<ButtonInput<KeyCode>>,
//     option_target_gamepad: Option<Res<TargetGamepad>>,
//     query_gamepads: Query<&Gamepad>,
//     mut event: EventWriter<AnyButtonPressed>,
// ) -> Result
// {
//     // 準備
//     let masking_input = option_masking_input.ok_or("Resource not found.")?;

//     // キー入力を数える（マスクされるキーは除く）
//     let mut is_pressed = input_keycode
//         .get_just_pressed() // .get_pressed()だと[Fn]がすり抜けることがある
//         .filter(|key| !masking_input.keys.contains(key))
//         .count();

//     #[cfg(debug_assertions)]
//     if is_pressed != 0
//     {
//         input_keycode.get_just_pressed().for_each(|key| {
//             dbg!(key);
//         });
//     }

//     // キー入力がなく、ゲームパッドが接続されているなら
//     if is_pressed == 0
//         && let Some(target_gamepad) = option_target_gamepad
//         && let Some(gamepad_entity) = target_gamepad.entity()
//         && let Ok(gamepad) = query_gamepads.get(gamepad_entity)
//     {
//         // ボタン入力を数える（マスクされるボタンは除く）
//         is_pressed = gamepad
//             .get_just_pressed()
//             .filter(|button| !masking_input.buttons.contains(button))
//             .count();

//         #[cfg(debug_assertions)]
//         if is_pressed != 0
//         {
//             gamepad.get_just_pressed().for_each(|button| {
//                 dbg!(button);
//             });
//         }
//     }

//     // キーかボタンの入力があるなら
//     if is_pressed > 0
//     {
//         event.write(AnyButtonPressed);
//     }

//     Ok(())
// }

////////////////////////////////////////////////////////////////////////////////

// スクリーン(第四象限)のピクセル座標(Vec2)へ変換する

// (i32, i32)とIVec2を拡張するトレイト
pub trait I32x2TypeExt
{
    fn to_screen_pixels(&self) -> Vec2;
}

// Y軸は負方向。アンカーがグリッド中央なので補正(0.5)が必要
impl I32x2TypeExt for (i32, i32)
{
    fn to_screen_pixels(&self) -> Vec2
    {
        Vec2::new(self.0 as f32 + 0.5, -self.1 as f32 - 0.5) * PIXELS_PER_GRID
    }
}
// impl I32x2TypeExt for IVec2
// {
//     fn to_screen_pixels(&self) -> Vec2
//     {
//         Vec2::new(self.x as f32 + 0.5, -self.y as f32 - 0.5) * PIXELS_PER_GRID
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// QueryしたEnityを削除する（条件がComponent）
pub fn despawn_component<T: Component>(
    query_entity: Query<Entity, With<T>>,
    mut cmds: Commands, // cmdsをmoveするので通常の関数としては使い勝手が悪い！
) -> Result
{
    query_entity.iter().for_each(|id| cmds.entity(id).despawn());

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 指定されたMyStateへ遷移するSystem
pub fn set_next_state(
    my_state: MyState,
) -> impl FnMut(ResMut<NextState<MyState>>) -> Result
{
    move |mut next_state: ResMut<NextState<MyState>>| -> Result {
        next_state.set(my_state);
        Ok(())
    }
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

// QueryしたComponentを可視化する
pub fn show_component<T: Component>(mut query: Query<&mut Visibility, With<T>>)
{
    query.iter_mut().for_each(|mut v| *v = Visibility::Visible);
}

// QueryしたComponentを不可視にする
// pub fn hide_component<T: Component>(mut query: Query<&mut Visibility, With<T>>)
// {
//     query.iter_mut().for_each(|mut v| *v = Visibility::Hidden);
// }

////////////////////////////////////////////////////////////////////////////////

// 指定のEventを送信する
// pub fn set_event<T: Event + Default>(mut event_writer: EventWriter<T>) -> Result
// {
//     event_writer.write(T::default());

//     Ok(())
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
