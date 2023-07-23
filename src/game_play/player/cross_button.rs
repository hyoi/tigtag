use super::*;

////////////////////////////////////////////////////////////////////////////////

//十字キーの入力読み取り
pub fn catch_cross_button_pressed
(   q_player : Query<&Player>,
    o_now_gamepad: Option<Res<NowGamepad>>,
    o_cross: Option<ResMut<CrossButton>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   //トラブル除け
    let Ok ( player ) = q_player.get_single() else { return };
    let Some ( now_gamepad ) = o_now_gamepad else { return };
    let Some ( gamepad ) = now_gamepad.0 else { return }; //gamepadが繋がっていない
    let Some ( mut cross ) = o_cross else { return };

    //ゲームパッドの十字ボタン入力をハッシュに取得する
    let pressed_btns: HashSet<_> =
    {   inbtn.get_pressed()
        .filter( | key | key.gamepad == gamepad && CROSS_BUTTONS.contains( &key.button_type ) )
        .map( | key | key.button_type )
        .collect()
    };

    //要素数が0なら
    cross.clear(); //初期化
    if pressed_btns.is_empty() { return }

    //要素数が1なら
    if pressed_btns.len() == 1
    {   match pressed_btns.iter().next().unwrap() //要素数1なので問題なし
        {   GamepadButtonType::DPadRight => cross.push( DxDy::Right ),
            GamepadButtonType::DPadLeft  => cross.push( DxDy::Left  ),
            GamepadButtonType::DPadDown  => cross.push( DxDy::Down  ),
            GamepadButtonType::DPadUp    => cross.push( DxDy::Up    ),
            _ => unimplemented!(),
        }
        return
    }

    //要素数が２つなら
    //・十字キーは対面のキーが押せないので最大２要素
    //・要素数1は処理済み。ここでは左右で1つ、上下で1つの入力を処理する
    let right_left = if pressed_btns.contains_right() { DxDy::Right } else { DxDy::Left };
    let down_up    = if pressed_btns.contains_down()  { DxDy::Down  } else { DxDy::Up   };  

    if player.stop
    {   //停止中なら
        match player.side
        {   DxDy::Right =>
            {   cross.push( right_left );
                cross.push( down_up    );
            },
            DxDy::Left =>
            {   cross.push( right_left );
                cross.push( down_up    );
            },
            DxDy::Down =>
            {   cross.push( down_up    );
                cross.push( right_left );
            },
            DxDy::Up =>
            {   cross.push( down_up    );
                cross.push( right_left );
            },
        }
    }
    else
    {   //移動中なら曲がりやすい順番にする
        match player.side
        {   DxDy::Right =>
            {   if pressed_btns.contains_right()
                {   cross.push( down_up     );
                    cross.push( DxDy::Right );
                }
                else
                {   cross.push( DxDy::Left );
                    cross.push( down_up    );
                }
            },
            DxDy::Left =>
            {   if pressed_btns.contains_left()
                {   cross.push( down_up    );
                    cross.push( DxDy::Left );
                }
                else
                {   cross.push( DxDy::Right );
                    cross.push( down_up     );
                }
            },
            DxDy::Down =>
            {   if pressed_btns.contains_down()
                {   cross.push( right_left );
                    cross.push( DxDy::Down );
                }
                else
                {   cross.push( DxDy::Up   );
                    cross.push( right_left );
                }
            },
            DxDy::Up =>
            {   if pressed_btns.contains_up()
                {   cross.push( right_left );
                    cross.push( DxDy::Up   );
                }
                else
                {   cross.push( DxDy::Down );
                    cross.push( right_left );
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.