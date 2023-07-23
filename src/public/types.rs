use super::*;

//internal submodules
mod dxdy;
mod map;
mod player_chaser;

//re-export
pub use dxdy::*;
pub use map::*;
pub use player_chaser::*;

////////////////////////////////////////////////////////////////////////////////

//ゲームの状態
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
pub enum MyState
{   #[default] InitApp,
    Title, DemoLoop,
    StageStart, MainLoop, StageClear, GameOver,
    Pause, Debug,
}
impl MyState
{   pub fn is_stageclear( &self ) -> bool { *self == MyState::StageClear }
    pub fn is_pause     ( &self ) -> bool { *self == MyState::Pause      }
    pub fn is_demoplay  ( &self ) -> bool { *self == MyState::Title || *self == MyState::DemoLoop }
}

////////////////////////////////////////////////////////////////////////////////

//InitAppから遷移する先のState
#[derive( Resource )]
pub struct AfterInitApp<T: States> ( pub T );

////////////////////////////////////////////////////////////////////////////////

//glamの型に別名を付ける
pub type Grid = IVec2;
pub type Px2d =  Vec2;

pub trait GridTrait
{   fn px2d_screen( &self ) -> Px2d;
    fn px2d_map   ( &self ) -> Px2d;
}
impl GridTrait for Grid
{   //Gridからスクリーン座標(Px2d)を算出する
    fn px2d_screen( &self ) -> Px2d
    {   let neg_half_w = SCREEN_PIXELS_WIDTH  / -2.0;
        let half_h     = SCREEN_PIXELS_HEIGHT /  2.0;
        let half_grid  = PIXELS_PER_GRID      /  2.0;

        let x = neg_half_w + PIXELS_PER_GRID * self.x as f32 + half_grid;
        let y = half_h     - PIXELS_PER_GRID * self.y as f32 - half_grid;

        Px2d::new( x, y )
    }

    //Gridからマップの原点座標を加味したスクリーン座標(Px2d)を算出する
    fn px2d_map( &self ) -> Px2d
    {   ( *self + MAP_ORIGIN_GRID ).px2d_screen()
    }
}

////////////////////////////////////////////////////////////////////////////////

//System間の通知用イベント
#[derive( Event )]
pub struct EventClear;

#[derive( Event )]
pub struct EventOver;

////////////////////////////////////////////////////////////////////////////////

//ゲームの記録用Resource
#[derive( Resource )]
pub struct Record
{   pub stage   : i32,        //ステージ数
    pub score   : i32,        //スコア
    pub hi_score: i32,        //ハイスコア
    pub count   : i32,        //カウントダウンタイマーの初期値
    pub timer   : Timer,      //カウントダウンタイマー用タイマー
    pub is_clear: bool,       //ステージクリアすると真、それ以外は偽
    pub demo    : DemoRecord, //demo用の記録
}
impl Default for Record
{   fn default() -> Self
    {   Self
        {   stage   : 0,
            score   : 0,
            hi_score: 0,
            count   : 0,
            timer   : Timer::from_seconds( 1.0, TimerMode::Once ),
            is_clear: false,
            demo    : DemoRecord::default(),
        }
    }
}

//demo用の記録
#[derive( Default )]
pub struct DemoRecord
{   pub stage     : i32,  //ステージ数
    pub hi_score  : i32,  //ハイスコア
}

////////////////////////////////////////////////////////////////////////////////

//text UIのメッセージセクションの型
pub type MessageSect<'a> =
(   &'a str, //表示文字列
    &'a str, //フォントのファイル名
    f32,     //フォンtのピクセル数（PIXELS_PER_GRIDＸ0.7 等）
    Color,   //文字の色（Bevy::Color）
);

//text UIのComponent
#[derive( Component )]
pub struct TextUiTitle ( pub MyState );

#[derive( Component )]
pub struct TextUiStart ( pub i32, pub MyState, pub usize, pub fn ( i32 ) -> String );

#[derive( Component )]
pub struct TextUiOver  ( pub i32, pub MyState, pub usize, pub fn ( i32 ) -> String, pub MyState );

#[derive( Component )]
pub struct TextUiClear ( pub i32, pub MyState, pub usize, pub fn ( i32 ) -> String );

#[derive( Component )]
pub struct HeaderLeft;

#[derive( Component )]
pub struct HeaderCenter;

#[derive( Component )]
pub struct HeaderRight;

#[derive( Component )]
pub struct FooterLeft;

#[derive( Component )]
pub struct FooterCenter;

#[derive( Component )]
pub struct FooterRight;

//カウントダウン付きtext UIでトレイト境界を使う準備
pub trait WithCountDown
{   fn initial_value( &self ) -> i32;
    fn next_state   ( &self ) -> MyState;
    fn placeholder  ( &self ) -> usize;
    fn cd_string    ( &self, n: i32 ) -> String;
}
impl WithCountDown for TextUiStart
{   fn initial_value( &self ) -> i32     { self.0 }
    fn next_state   ( &self ) -> MyState { self.1 }
    fn placeholder  ( &self ) -> usize   { self.2 }
    fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
}
impl WithCountDown for TextUiClear
{   fn initial_value( &self ) -> i32     { self.0 }
    fn next_state   ( &self ) -> MyState { self.1 }
    fn placeholder  ( &self ) -> usize   { self.2 }
    fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
}
impl WithCountDown for TextUiOver
{   fn initial_value( &self ) -> i32     { self.0 }
    fn next_state   ( &self ) -> MyState { self.1 }
    fn placeholder  ( &self ) -> usize   { self.2 }
    fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
}

//キー入力でstateを変更するtext UIでトレイト境界を使う準備
pub trait WithHitAnyKey
{   fn next_state( &self ) -> MyState;
}
impl WithHitAnyKey for TextUiTitle
{   fn next_state( &self ) -> MyState { self.0 }
}
impl WithHitAnyKey for TextUiOver
{   fn next_state( &self ) -> MyState { self.4 }
}

//カウントダウンタイマー用のResource
#[derive( Resource )]
pub struct CountDown
{   pub count: i32,   //カウントダウンの初期値
    pub timer: Timer, //カウントダウン用タイマー
}
impl Default for CountDown
{   fn default() -> Self
    {   Self
        {   count: 0,
            timer: Timer::from_seconds( 1.0, TimerMode::Once ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//操作を受け付けるgamepadを保存するResource
#[derive( Resource, Default )]
pub struct NowGamepad ( pub Option<Gamepad> );

//十字ボタンの入力状態を保存するResource
#[derive( Resource )]
pub struct CrossButton ( pub Vec::<DxDy> );
impl Default for CrossButton
{   fn default() -> Self
    {   Self ( Vec::with_capacity( 2 ) ) //十字ボタンは最大2要素
    }
}
impl CrossButton
{   pub fn sides( &self ) -> &[ DxDy ] { &self.0 }
    pub fn is_empty( &self ) -> bool { self.0.is_empty() }
    pub fn push( &mut self, dxdy: DxDy ) { self.0.push( dxdy ) }
    pub fn clear( &mut self ) { self.0.clear() }
}

//判定用メソッド（traitはオーファンルール対策）
pub trait Cotains
{   fn contains_right( &self ) -> bool;
    fn contains_left ( &self ) -> bool;
    fn contains_down ( &self ) -> bool;
    fn contains_up   ( &self ) -> bool;
}
impl Cotains for HashSet<GamepadButtonType>
{   fn contains_right( &self ) -> bool { self.contains( &GamepadButtonType::DPadRight ) }
    fn contains_left ( &self ) -> bool { self.contains( &GamepadButtonType::DPadLeft  ) }
    fn contains_down ( &self ) -> bool { self.contains( &GamepadButtonType::DPadDown  ) }
    fn contains_up   ( &self ) -> bool { self.contains( &GamepadButtonType::DPadUp    ) }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.