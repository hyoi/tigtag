use super::*;

//プラグインの設定
pub struct CrossButton;
impl Plugin for CrossButton
{   fn build( &self, app: &mut App )
    {   app
        .init_resource::<GamepadCrossButton>()    //十字キーの入力状態を保存するResource
        .add_systems( Update, read_cross_button ) //十字キーの入力読み取り
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//十字キーの入力状態を保存するResource
#[derive( Resource )]
pub struct GamepadCrossButton ( Vec::<DxDy> );

impl Default for GamepadCrossButton
{   fn default() -> Self
    {   Self ( Vec::with_capacity( 4 ) ) //最大4要素
    }
}

//タプルの「.0」を隠すためのメソッド
impl GamepadCrossButton
{   pub fn is_empty( &self ) -> bool { self.0.is_empty() }
    pub fn sides_list( &self ) -> &[ DxDy ] { &self.0 }
    fn push( &mut self, dxdy: DxDy ) { self.0.push( dxdy ) }
    fn clear( &mut self ) { self.0.clear() }
}

//判定用メソッド（traitはオーファンルール対策）
trait JudgeCotains
{   fn contains_right( &self ) -> bool;
    fn contains_left ( &self ) -> bool;
    fn contains_down ( &self ) -> bool;
    fn contains_up   ( &self ) -> bool;
}
impl JudgeCotains for HashSet<GamepadButtonType>
{   fn contains_right( &self ) -> bool { self.contains( &GamepadButtonType::DPadRight ) }
    fn contains_left ( &self ) -> bool { self.contains( &GamepadButtonType::DPadLeft  ) }
    fn contains_down ( &self ) -> bool { self.contains( &GamepadButtonType::DPadDown  ) }
    fn contains_up   ( &self ) -> bool { self.contains( &GamepadButtonType::DPadUp    ) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//十字キーの入力読み取り
fn read_cross_button
(   q_player : Query< &Player >,
    o_cross_button: Option<ResMut<GamepadCrossButton>>,
    button_inputs: Res<Input<GamepadButton>>,
)
{   //安全装置
    let Ok ( player ) = q_player.get_single() else { return };      //無ければreturn
    let Some ( mut cross_button ) = o_cross_button else { return }; //無ければreturn
    cross_button.clear();

    //ゲームパッド(0番)の十字ボタン入力をハッシュに取得する
    let pressed_btns: HashSet<_> =
    {   button_inputs
        .get_pressed()
        .filter
        (   | x |
            x.gamepad.id == 0  //Todo: pad 0番決め打ちでいいいのか？
            &&
            CROSS_BUTTON_SET.contains( &x.button_type )
        )
        .map( |x| x.button_type )
        .collect()
    };

    //要素数が２未満なら
    if pressed_btns.is_empty() { return }
    if pressed_btns.len() == 1
    {   match pressed_btns.iter().next().unwrap()
        {   GamepadButtonType::DPadRight => cross_button.push( DxDy::Right ),
            GamepadButtonType::DPadLeft  => cross_button.push( DxDy::Left  ),
            GamepadButtonType::DPadDown  => cross_button.push( DxDy::Down  ),
            GamepadButtonType::DPadUp    => cross_button.push( DxDy::Up    ),
            _ => unimplemented!(),
        }
        return
    }

    //要素数が２つなら（十字キーの入力は最大２要素）
    let right_left = if pressed_btns.contains_right() { DxDy::Right } else { DxDy::Left };
    let down_up    = if pressed_btns.contains_down()  { DxDy::Down  } else { DxDy::Up   };  
    if player.stop //停止中なら
    {   match player.side
        {   DxDy::Right =>
            {   cross_button.push( right_left );
                cross_button.push( down_up    );
            },
            DxDy::Left =>
            {   cross_button.push( right_left );
                cross_button.push( down_up    );
            },
            DxDy::Down =>
            {   cross_button.push( down_up    );
                cross_button.push( right_left );
            },
            DxDy::Up =>
            {   cross_button.push( down_up    );
                cross_button.push( right_left );
            },
        }
    }
    else
    {   //移動中なら曲がりやすい順番にする
        match player.side
        {   DxDy::Right =>
            {   if pressed_btns.contains_right()
                {   cross_button.push( down_up     );
                    cross_button.push( DxDy::Right );
                }
                else
                {   cross_button.push( DxDy::Left );
                    cross_button.push( down_up    );
                }
            },
            DxDy::Left =>
            {   if pressed_btns.contains_left()
                {   cross_button.push( down_up    );
                    cross_button.push( DxDy::Left );
                }
                else
                {   cross_button.push( DxDy::Right );
                    cross_button.push( down_up     );
                }
            },
            DxDy::Down =>
            {   if pressed_btns.contains_down()
                {   cross_button.push( right_left );
                    cross_button.push( DxDy::Down );
                }
                else
                {   cross_button.push( DxDy::Up   );
                    cross_button.push( right_left );
                }
            },
            DxDy::Up =>
            {   if pressed_btns.contains_up()
                {   cross_button.push( right_left );
                    cross_button.push( DxDy::Up   );
                }
                else
                {   cross_button.push( DxDy::Down );
                    cross_button.push( right_left );
                }
            },
        }
    }
}

//End of code.