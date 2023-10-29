use super::*;

//internal submodules
mod map;
pub use map::*;

////////////////////////////////////////////////////////////////////////////////

//画面デザイン(枠)
pub struct ScreenFrame<'a>
{   pub design  : Vec<&'a str>,
}

////////////////////////////////////////////////////////////////////////////////

//glamの型にメソッドを追加する準備
pub trait GridToPixel
{   fn to_sprite_pixels( &self ) -> Vec2;
    fn to_screen_pixels( &self ) -> Vec2;
}

//glamの型にメソッドを追加する
impl GridToPixel for IVec2
{   //平面座標(IVec2)から画面第一象限の座標(Vec2)を算出する
    //アンカーはグリッドの中央
    fn to_sprite_pixels( &self ) -> Vec2
    {   ( self.as_vec2() + 0.5 ) * PIXELS_PER_GRID * Vec2::new( 1.0, -1.0 )
    }

    //平面座標(IVec2)から画面第一象限の座標(Vec2)を算出する
    //アンカーはグリッドの左下
    fn to_screen_pixels( &self ) -> Vec2
    {   self.as_vec2() * PIXELS_PER_GRID * Vec2::new( 1.0, -1.0 )
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームの状態
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
pub enum MyState
{   #[default] LoadAssets, InitApp,
    GameStart, StageStart, MainLoop, StageClear, GameOver,
    TitleDemo, DemoLoop,
    Pause, Debug,
}

impl MyState
{   pub fn is_pause     ( &self ) -> bool { *self == MyState::Pause      }
    pub fn is_demoplay  ( &self ) -> bool { *self == MyState::TitleDemo || *self == MyState::DemoLoop }
    // pub fn is_stageclear( &self ) -> bool { *self == MyState::StageClear }
}

//Stateの遷移に使うTrait
pub trait GotoState { fn next( &self ) -> MyState; }

//Stateの遷移に使うマーカー(not Resource)
#[derive( Default )] pub struct StageStart;
#[derive( Default )] pub struct MainLoop;

impl GotoState for StageStart { fn next( &self ) -> MyState { MyState::StageStart } }
impl GotoState for MainLoop   { fn next( &self ) -> MyState { MyState::MainLoop   } }

//Stateの遷移に使うマーカー(Resouce)
#[derive( Resource )] pub struct AfterLoadAssetsTo<T: States> ( pub T );
#[derive( Resource )] pub struct AfterInitAppTo   <T: States> ( pub T );
#[derive( Resource )] pub struct TitleDemoExist   <T: States> ( pub T );

impl GotoState for AfterLoadAssetsTo<MyState> { fn next( &self ) -> MyState { self.0 } }
impl GotoState for AfterInitAppTo   <MyState> { fn next( &self ) -> MyState { self.0 } }
impl GotoState for TitleDemoExist   <MyState> { fn next( &self ) -> MyState { self.0 } }

////////////////////////////////////////////////////////////////////////////////

//System間の通知用イベント
#[derive( Event )] pub struct EventClear;
#[derive( Event )] pub struct EventOver;

////////////////////////////////////////////////////////////////////////////////

//ゲームの記録用Resource

#[derive( Resource, Default )] pub struct Stage ( i32 );
impl Stage
{   pub fn get( &self ) -> i32 { self.0 }
    pub fn get_mut( &mut self ) -> &mut i32 { &mut self.0 }
}

#[derive( Resource, Default )] pub struct Score ( i32 );
impl Score
{   pub fn get( &self ) -> i32 { self.0 }
    pub fn get_mut( &mut self ) -> &mut i32 { &mut self.0 }
}

#[derive( Resource, Default )] pub struct HiScore ( i32 );
impl HiScore
{   pub fn get( &self ) -> i32 { self.0 }
    pub fn get_mut( &mut self ) -> &mut i32 { &mut self.0 }
}

// #[derive( Resource )]
// pub struct Record
// {   pub score         : i32,        //スコア
//     pub hi_score      : i32,        //ハイスコア
//     pub stage         : i32,        //ステージ数
//     pub count_down    : i32,        //カウントダウンタイマーの初期値
//     pub cd_timer      : Timer,      //カウントダウンタイマー用タイマー
//     pub is_stage_clear: bool,       //ステージクリアすると真、それ以外は偽
//     pub demo          : DemoRecord, //demo用の記録
// }

// //demo用
// #[derive( Default )]
// pub struct DemoRecord
// {   pub stage   : i32,  //ステージ数
//     pub hi_score: i32,  //ハイスコア
// }

// impl Default for Record
// {   fn default() -> Self
//     {   Self
//         {   score         : 0,
//             hi_score      : 0,
//             stage         : 0,
//             count_down    : 0,
//             cd_timer      : Timer::from_seconds( 1.0, TimerMode::Once ),
//             is_stage_clear: false,
//             demo          : DemoRecord::default(),
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////

//中央の隠しフレームのComponent
#[derive( Component )] pub struct HiddenFrameMiddle;

//ヘッダーのComponent
#[derive( Component )] pub struct UiStage;
#[derive( Component )] pub struct UiScore;
#[derive( Component )] pub struct UiHiScore;

//TextUIのメッセージセクションの型
pub type MessageSect<'a> =
(   &'a str, //表示文字列
    &'a str, //フォントのAssets
    f32,     //フォントのサイズ
    Color,   //フォントの色
);

//マーカーtrait
pub trait TextUI
{   fn message( &self ) -> & [ MessageSect ];
}
pub trait CountDown
{   fn initial_count( &self ) -> i32;
    fn next_state( &self ) -> MyState;
    fn placeholder( &self ) -> usize;
    fn to_string( &self, n: i32 ) -> String;
}

//TextUIのComponent
#[derive( Component, Clone, Copy )] pub struct UiStart<'a>
{   count      : i32,
    next_state : MyState,
    placeholder: usize,
    string     : fn ( i32 ) -> String,
    message    : &'a [ MessageSect<'a> ],
}
impl<'a> TextUI for UiStart<'a>
{   fn message( &self ) -> & [ MessageSect ] { self.message }
}
impl<'a> CountDown for UiStart<'a>
{   fn initial_count( &self ) -> i32 { self.count + 1 }
    fn next_state( &self ) -> MyState { self.next_state }
    fn placeholder( &self ) -> usize { self.placeholder }
    fn to_string( &self, n: i32 ) -> String { ( self.string )( n ) }
}
impl<'a> Default for UiStart<'a>
{   fn default() -> Self
    {   Self
        {   count      : 3,
            next_state : MyState::MainLoop,
            placeholder: 4,
            string     : |n| { if n == 0 { "Go!!".to_string() } else { n.to_string() } },
            message    : UI_START,
        }
    }
}

//カウントダウン用のResource
#[derive( Resource )]
pub struct CountDownTimer
{   pub counter: i32,   //カウンター
    pub timer  : Timer, //１秒タイマー
}
impl Default for CountDownTimer
{   fn default() -> Self
    {   Self
        {   counter: 0,
            timer  : Timer::from_seconds( 1.0, TimerMode::Once ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//操作を受け付けるgamepadを保存するResource
#[derive( Resource, Default )]
pub struct ConnectedGamepad ( Option<Gamepad> );

pub trait TraitGamepad
{   fn id( &self ) -> Option<Gamepad>;
    fn id_mut( &mut self ) -> &mut Option<Gamepad>;
}

impl TraitGamepad for ConnectedGamepad
{   fn id( &self ) -> Option<Gamepad> { self.0 }
    fn id_mut( &mut self ) -> &mut Option<Gamepad> { &mut self.0 }
}

////////////////////////////////////////////////////////////////////////////////

//四方を表す列挙型
#[derive( Default, Clone, Copy, PartialEq, Eq, Hash, Debug )]
pub enum News { #[default] North, East, West, South }

impl News
{   //時計回りで方角を得る
    pub fn turn_right( &self ) -> Self
    {   match self
        {   News::North => News::East,
            News::East  => News::South,
            News::West  => News::North,
            News::South => News::West,
        }
    }

    //反時計回りで方角を得る
    pub fn turn_left( &self ) -> Self
    {   match self
        {   News::North => News::West,
            News::East  => News::North,
            News::West  => News::South,
            News::South => News::East,
        }
    }

    //背面の方角を得る
    pub fn back_side( &self ) -> Self
    {   match self
        {   News::North => News::South,
            News::East  => News::West,
            News::West  => News::East,
            News::South => News::North,
        }
    }
}

//IVec2 = IVec2 + News
impl Add<News> for IVec2
{   type Output = IVec2;
    fn add( mut self, news: News ) -> IVec2
    {   match news
        {   News::North => { self.y -= 1; }
            News::East  => { self.x += 1; }
            News::West  => { self.x -= 1; }
            News::South => { self.y += 1; }
        }
        self
    }
}

//IVec2 = IVec2 + &News
impl Add<&News> for IVec2
{   type Output = IVec2;
    fn add( mut self, news: &News ) -> IVec2
    {   match news
        {   News::North => { self.y -= 1; }
            News::South => { self.y += 1; }
            News::East  => { self.x += 1; }
            News::West  => { self.x -= 1; }
        }
        self
    }
}

//IVec2 += News
impl AddAssign<News> for IVec2
{   fn add_assign( &mut self, news: News )
    {   match news
        {   News::North => { self.y -= 1; }
            News::East  => { self.x += 1; }
            News::West  => { self.x -= 1; }
            News::South => { self.y += 1; }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//プレイヤーのComponent
#[derive( Component )]
pub struct Player
{   pub grid        : IVec2,             //移動中は移動元の座標、停止中はその場の座標
    pub next        : IVec2,             //移動中は移動先の座標、停止中はその場の座標
    pub side        : News,              //移動向き
    pub wait        : Timer,             //移動ウエイト
    pub stop        : bool,              //移動停止フラグ
    pub speedup     : f32,               //スピードアップ係数
    pub px_start    : Vec2,              //移動した微小区間の始点
    pub px_end      : Vec2,              //移動した微小区間の終点
    pub o_fn_runaway: Option<FnRunAway>, //(demoplay)自機の移動方向を決める関数のポインタ
}

impl Default for Player
{   fn default() -> Self
    {   Self
        {   grid        : IVec2::default(),
            next        : IVec2::default(),
            side        : News::default(),
            wait        : Timer::from_seconds( PLAYER_WAIT, TimerMode::Once ),
            stop        : true,
            speedup     : 1.0,
            px_start    : Vec2::default(),
            px_end      : Vec2::default(),
            o_fn_runaway: None,
        }
    }
}

//関数ポインタ型((demoplay)自機の移動方向を決める関数)
type FnRunAway = fn( &Player, Query<&Chaser>, Res<Map>, &[ News ] ) -> News;

////////////////////////////////////////////////////////////////////////////////

//チェイサーのComponent
#[derive( Component )]
pub struct Chaser
{   pub grid      : IVec2,              //移動中は移動元の座標、停止中はその場の座標
    pub next      : IVec2,              //移動中は移動先の座標、停止中はその場の座標
    pub side      : News,               //移動向き
    pub wait      : Timer,              //移動ウエイト
    pub stop      : bool,               //移動停止フラグ
    pub speedup   : f32,                //スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub px_start  : Vec2,               //移動した微小区間の始点
    pub px_end    : Vec2,               //移動した微小区間の終点
    pub color     : Color,              //表示色
    pub fn_chasing: Option<FnChasing>,  //追手の移動方向を決める関数のポインタ
}

impl Default for Chaser
{   fn default() -> Self
    {   Self
        {   grid      : IVec2::default(),
            next      : IVec2::default(),
            side      : News::default(),
            wait      : Timer::from_seconds( CHASER_WAIT, TimerMode::Once ),
            stop      : true,
            speedup   : 1.0,
            px_start  : Vec2::default(),
            px_end    : Vec2::default(),
            color     : Color::NONE,
            fn_chasing: None,
        }
    }
}

//関数ポインタ型(追手の移動方向を決める関数)
pub type FnChasing = fn( &mut Chaser, &Player, &[ News ] ) -> News;

////////////////////////////////////////////////////////////////////////////////

//End of code.

// ////////////////////////////////////////////////////////////////////////////////

// //text UIのComponent
// #[derive( Component )]
// pub struct TextUiTitle ( pub MyState );

// #[derive( Component )]
// pub struct TextUiOver  ( pub i32, pub MyState, pub usize, pub fn ( i32 ) -> String, pub MyState );

// #[derive( Component )]
// pub struct TextUiClear ( pub i32, pub MyState, pub usize, pub fn ( i32 ) -> String );

// //カウントダウン付きtext UIでトレイト境界を使う準備
// pub trait WithCountDown
// {   fn initial_value( &self ) -> i32;
//     fn next_state   ( &self ) -> MyState;
//     fn placeholder  ( &self ) -> usize;
//     fn cd_string    ( &self, n: i32 ) -> String;
// }
// impl WithCountDown for TextUiClear
// {   fn initial_value( &self ) -> i32     { self.0 }
//     fn next_state   ( &self ) -> MyState { self.1 }
//     fn placeholder  ( &self ) -> usize   { self.2 }
//     fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
// }
// impl WithCountDown for TextUiOver
// {   fn initial_value( &self ) -> i32     { self.0 }
//     fn next_state   ( &self ) -> MyState { self.1 }
//     fn placeholder  ( &self ) -> usize   { self.2 }
//     fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
// }

// //キー入力でstateを変更するtext UIでトレイト境界を使う準備
// pub trait WithHitAnyKey
// {   fn next_state( &self ) -> MyState;
// }
// impl WithHitAnyKey for TextUiTitle
// {   fn next_state( &self ) -> MyState { self.0 }
// }
// impl WithHitAnyKey for TextUiOver
// {   fn next_state( &self ) -> MyState { self.4 }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //End of code.