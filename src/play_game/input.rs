use super::*;

////////////////////////////////////////////////////////////////////////////////

//十字方向の入力状態を保存するResource
#[derive( Resource )]
pub struct CrossDirection ( pub Vec::<News> );

impl Default for CrossDirection
{   fn default() -> Self
    {   Self ( Vec::with_capacity( 2 ) ) //十字方向の最大要素数は２
    }
}

impl CrossDirection
{   pub fn direction( &self ) -> &[ News ] { &self.0 }
    pub fn clear( &mut self ) { self.0.clear() }
    pub fn push( &mut self, dxdy: News ) { self.0.push( dxdy ) }
}

////////////////////////////////////////////////////////////////////////////////

//十字方向の入力状態を取得する
pub fn catch_player_operation
(   qry_player: Query<&Player>,
    opt_cross: Option<ResMut<CrossDirection>>,
    opt_gamepad: Option<Res<TargetGamepad>>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };

    //初期化
    let Some ( mut cross ) = opt_cross else { return };
    cross.clear();

    //十字ボタン@gamepadを取得する
    let mut pressed_cross = HashSet::new();
    if let Some ( id ) = opt_gamepad.and_then( | gamepad | gamepad.id() )
    {   pressed_cross = buttons.get_pressed()
        .filter_map
        (   | x |
            match x.button_type
            {   GamepadButtonType::DPadUp    if x.gamepad == id => Some ( News::North ),
                GamepadButtonType::DPadRight if x.gamepad == id => Some ( News::East  ),
                GamepadButtonType::DPadLeft  if x.gamepad == id => Some ( News::West  ),
                GamepadButtonType::DPadDown  if x.gamepad == id => Some ( News::South ),
                _ => None,
            }
        )
        .collect();
    }

    //要素数０ならカーソルキーを調べる
    if pressed_cross.is_empty()
    {   pressed_cross = keys.get_pressed()
        .filter_map
        (   | keycode |
            match keycode
            {   KeyCode::Up    => Some ( News::North ),
                KeyCode::Right => Some ( News::East  ),
                KeyCode::Left  => Some ( News::West  ),
                KeyCode::Down  => Some ( News::South ),
                _ => None,
            }
        )
        .collect();
    }

    //上と下、右と左があれば相殺する
    if pressed_cross.contains( &News::North ) && pressed_cross.contains( &News::South )
    {   pressed_cross.remove( &News::North );
        pressed_cross.remove( &News::South );
    }
    if pressed_cross.contains( &News::East ) && pressed_cross.contains( &News::West )
    {   pressed_cross.remove( &News::East );
        pressed_cross.remove( &News::West );
    }

    //要素数０なら
    if pressed_cross.is_empty() { return }

    //要素数が１なら
    if pressed_cross.len() == 1
    {   cross.push( *pressed_cross.iter().next().unwrap() );
        return;
    }

    //要素数が２なら
    if player.is_stop
    {   //停止中なら
        if pressed_cross.remove( &player.direction ) //前進が入力されている場合
        {   cross.push( player.direction ); //前進
            cross.push( *pressed_cross.iter().next().unwrap() ); //右折／左折
            return;
        }

        //後進と右折／左折のどちらか
        let back = player.direction.back_side();
        pressed_cross.remove( &back );
        cross.push( back ); //後進
        cross.push( *pressed_cross.iter().next().unwrap() );
    }
    else
    {   //移動中なら
        if pressed_cross.remove( &player.direction ) //前進が入力されている場合
        {   cross.push( *pressed_cross.iter().next().unwrap() ); //右折／左折
            cross.push( player.direction ); //前進
            return;
        }

        //後進と右折／左折のどちらか
        let back = player.direction.back_side();
        pressed_cross.remove( &back );
        cross.push( *pressed_cross.iter().next().unwrap() ); //右折／左折
        cross.push( back ); //後進
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.