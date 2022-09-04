use super::*;

//自機の構造体
#[derive( Component )]
pub struct Player
{   pub grid    : Grid,     //移動中は移動元の座標、停止中はその場の座標
    pub next    : Grid,     //移動中は移動先の座標、停止中はその場の座標
    pub side    : DxDy,     //移動向き
    pub wait    : Timer,    //移動ウエイト
    pub stop    : bool,     //移動停止フラグ
    pub speedup : f32,      //スピードアップ係数
    pub px_start: Pixel,    //移動した微小区間の始点
    pub px_end  : Pixel,    //移動した微小区間の終点
}

//自機の構造体の初期化
impl Default for Player
{   fn default() -> Self
    {   Self
        {   grid    : Grid::default(),
            next    : Grid::default(),
            side    : DxDy::Up,
            wait    : Timer::from_seconds( PLAYER_WAIT, false ),
            stop    : true,
            speedup : 1.0,
            px_start: Pixel::default(),
            px_end  : Pixel::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//追手の構造体
#[derive( Component )]
pub struct Chaser
{   pub grid      : Grid,               //移動中は移動元の座標、停止中はその場の座標
    pub next      : Grid,               //移動中は移動先の座標、停止中はその場の座標
    pub side      : DxDy,               //移動向き
    pub wait      : Timer,              //移動ウエイト
    pub stop      : bool,               //移動停止フラグ
    pub speedup   : f32,                //スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub px_start  : Pixel,              //移動した微小区間の始点
    pub px_end    : Pixel,              //移動した微小区間の終点
    pub color     : Color,              //表示色
    pub fn_chasing: Option<FnChasing>,  //追手の移動方向を決める関数のポインタ
}

//追手の構造体の初期化
impl Default for Chaser
{   fn default() -> Self
    {   Self
        {   grid      : Grid::default(),
            next      : Grid::default(),
            side      : DxDy::Up,
            wait      : Timer::from_seconds( CHASER_WAIT, false ),
            stop      : true,
            speedup   : 1.0,
            px_start  : Pixel::default(),
            px_end    : Pixel::default(),
            color     : Color::NONE,
            fn_chasing: None,
        }
    }
}

//関数ポインタ型(追手の移動方向を決める関数)
pub type FnChasing = fn( &mut Chaser, &Player, &[ DxDy ] ) -> DxDy;

//End of code.