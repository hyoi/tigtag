use super::*;

//submodules
mod grid_pixel_dxdy;
mod map;
mod player_chaser;

//re-export
pub use grid_pixel_dxdy::*;
pub use map::*;
pub use player_chaser::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームの記録のResource
pub struct Record
{   pub stage   : i32,      //ステージ数
    pub score   : i32,      //スコア
    pub hi_score: i32,      //ハイスコア
    pub count   : i32,      //カウントダウンタイマーの初期値
    pub timer   : Timer,    //カウントダウンタイマー用タイマー
}
impl Default for Record
{   fn default() -> Self
    {   Self
        {   stage    : 0,
            score    : 0,
            hi_score : 0,
            count: 0,
            timer: Timer::from_seconds( 1.0, false ),
        }
    }
}

//demoの記録を残すResource
#[derive(Default)]
pub struct DemoRecord
{   pub stage   : i32,      //ステージ数
    pub hi_score: i32,      //ハイスコア
    pub clear_flag: bool,   //demoでステージクリアすると真、それ以外は偽
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームの状態
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug )]
pub enum GameState
{   InitApp,
    TitleDemo, DemoLoop,
    GameStart, StageStart, MainLoop, StageClear, GameOver,
    Pause,
}
#[allow( dead_code )]
impl GameState
{   pub fn is_stageclear( &self ) -> bool { *self == GameState::StageClear }
    pub fn is_pause     ( &self ) -> bool { *self == GameState::Pause      }
    pub fn is_demoplay  ( &self ) -> bool { *self == GameState::TitleDemo || *self == GameState::DemoLoop }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//System Set内で実行順を制御するためのLabel
#[derive( SystemLabel, Clone )]
pub enum Mark
{   MakeMapNewData,   //マップデータ作成処理の目印
    DetectCollisions, //衝突判定処理の目印
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//System間の通知用イベント
pub struct EventClear;
pub struct EventOver;

////////////////////////////////////////////////////////////////////////////////////////////////////

//text UIのメッセージセクションの型
pub type MessageSect<'a> =
(   &'a str, //表示文字列
    &'a str, //フォントのファイル名
    f32,     //フォンtのピクセル数（PIXELS_PER_GRIDＸ0.7 等）
    Color,   //文字の色（Bevy::Color）
);

//text UIのComponent
#[derive( Component )] pub struct TextUiTitle ( pub KeyCode, pub GameState );
#[derive( Component )] pub struct TextUiStart ( pub i32, pub GameState, pub usize, pub fn ( i32 ) -> String );
#[derive( Component )] pub struct TextUiOver  ( pub i32, pub GameState, pub usize, pub fn ( i32 ) -> String, pub KeyCode, pub GameState );
#[derive( Component )] pub struct TextUiClear ( pub i32, pub GameState, pub usize, pub fn ( i32 ) -> String );
#[derive( Component )] pub struct TextUiPause;

#[derive( Component )] pub struct HeaderLeft;
#[derive( Component )] pub struct HeaderCenter;
#[derive( Component )] pub struct HeaderRight;

#[derive( Component )] pub struct FooterLeft;
#[derive( Component )] pub struct FooterCenter;
#[derive( Component )] pub struct FooterRight;

//カウントダウン付きtext UI用Systemでトレイト境界を使う準備
pub trait TextUiWithCountDown
{   fn initial_value( &self ) -> i32;
    fn next_state   ( &self ) -> GameState;
    fn placeholder  ( &self ) -> usize;
    fn cd_string    ( &self, n: i32 ) -> String;
}
impl TextUiWithCountDown for TextUiStart
{   fn initial_value( &self ) -> i32       { self.0 }
    fn next_state   ( &self ) -> GameState { self.1 }
    fn placeholder  ( &self ) -> usize     { self.2 }
    fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
}
impl TextUiWithCountDown for TextUiClear
{   fn initial_value( &self ) -> i32       { self.0 }
    fn next_state   ( &self ) -> GameState { self.1 }
    fn placeholder  ( &self ) -> usize     { self.2 }
    fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
}
impl TextUiWithCountDown for TextUiOver
{   fn initial_value( &self ) -> i32       { self.0 }
    fn next_state   ( &self ) -> GameState { self.1 }
    fn placeholder  ( &self ) -> usize     { self.2 }
    fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
}

//キー入力でstateを変更するSystemでトレイト境界を使う準備
pub trait TextUiWithHitKey
{   fn key_code  ( &self ) -> KeyCode;
    fn next_state( &self ) -> GameState;
}
impl TextUiWithHitKey for TextUiTitle
{   fn key_code  ( &self ) -> KeyCode   { self.0 }
    fn next_state( &self ) -> GameState { self.1 }
}
impl TextUiWithHitKey for TextUiOver
{   fn key_code  ( &self ) -> KeyCode   { self.4 }
    fn next_state( &self ) -> GameState { self.5 }
}

//カウントダウンタイマー用のResource
pub struct CountDown
{   pub count: i32,   //カウントダウンタイマーの初期値
    pub timer: Timer, //カウントダウンタイマー用タイマー
}
impl Default for CountDown
{   fn default() -> Self
    {   Self
        {   count: 0,
            timer: Timer::from_seconds( 1.0, false ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ロードしたAssetsのハンドルの保存先
pub struct LoadedAssets { pub preload: Vec<HandleUntyped> }

//ローディングアニメ用スプライトのComponent
#[derive( Component )] pub struct SpriteTile ( pub Grid );

//マーカーResource
pub struct MarkAfterFetchAssets ( pub GameState );

//開発用スプライトのComponent
#[derive( Component )] pub struct PathFinder;

//End of code.