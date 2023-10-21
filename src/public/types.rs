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
    {   ( self.as_vec2() + 0.5 ) * PIXELS_PER_GRID
    }

    //平面座標(IVec2)から画面第一象限の座標(Vec2)を算出する
    //アンカーはグリッドの左下
    fn to_screen_pixels( &self ) -> Vec2
    {   self.as_vec2() * PIXELS_PER_GRID
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
    // pub fn is_stageclear( &self ) -> bool { *self == MyState::StageClear }
    // pub fn is_demoplay  ( &self ) -> bool { *self == MyState::TitleDemo || *self == MyState::DemoLoop }
}

//Stateの遷移に使うマーカー(not Resource)
#[derive( Default )] pub struct StageStart;
// #[derive( Default )] pub struct TitleDemo;
// #[derive( Default )] pub struct MainLoop;

//Stateの遷移に使うResouce
#[derive( Resource )] pub struct AfterLoadAssetsTo<T: States> ( pub T );
#[derive( Resource )] pub struct AfterInitAppTo   <T: States> ( pub T );
#[derive( Resource )] pub struct TitleDemoExist   <T: States> ( pub T );

//Stateの遷移に使うTrait
pub trait GotoState { fn next( &self ) -> MyState; }

//Traitの実装
impl GotoState for StageStart                  { fn next( &self ) -> MyState { MyState::StageStart } }
// impl GotoState for TitleDemo                  { fn next( &self ) -> MyState { MyState::TitleDemo } }
// impl GotoState for MainLoop                   { fn next( &self ) -> MyState { MyState::MainLoop } }
impl GotoState for AfterLoadAssetsTo<MyState> { fn next( &self ) -> MyState { self.0 } }
impl GotoState for AfterInitAppTo   <MyState> { fn next( &self ) -> MyState { self.0 } }
impl GotoState for TitleDemoExist   <MyState> { fn next( &self ) -> MyState { self.0 } }

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
    // pub fn get_mut( &mut self ) -> &mut i32 { &mut self.0 }
}

#[derive( Resource, Default )] pub struct HiScore ( i32 );
impl HiScore
{   pub fn get( &self ) -> i32 { self.0 }
    // pub fn get_mut( &mut self ) -> &mut i32 { &mut self.0 }
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

////////////////////////////////////////////////////////////////////////////////

//text UIのメッセージセクションの型
pub type MessageSect<'a> =
(   &'a str, //表示文字列
    &'a str, //フォントのAssets
    f32,     //フォントのサイズ
    Color,   //フォントの色
);

////////////////////////////////////////////////////////////////////////////////

//操作を受け付けるgamepadを保存するResource
#[derive( Resource, Default )]
pub struct EnabledGamepadId ( pub Option<Gamepad> );

////////////////////////////////////////////////////////////////////////////////

//四方を表す列挙型
#[derive( Default, Clone, Copy, PartialEq, Eq, Hash, Debug )]
pub enum News { #[default] North, East, West, South }

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

////////////////////////////////////////////////////////////////////////////////

//End of code.



// //IVec2 += News
// impl AddAssign<News> for IVec2
// {   fn add_assign( &mut self, news: News )
//     {   match news
//         {   News::North => { self.y -= 1; }
//             News::East  => { self.x += 1; }
//             News::West  => { self.x -= 1; }
//             News::South => { self.y += 1; }
//         }
//     }
// }

// impl News
// {   //四方に対応するXZ平面上の角度（四元数）を返す（Y軸回転）
//     #[allow(clippy::wrong_self_convention)]
//     pub fn to_quat_y( &self ) -> Quat
//     {   match self
//         {   News::North => Quat::from_rotation_y( PI * 0.0 ),
//             News::East  => Quat::from_rotation_y( PI * 1.5 ),
//             News::West  => Quat::from_rotation_y( PI * 0.5 ),
//             News::South => Quat::from_rotation_y( PI * 1.0 ),
//         }
//     }

//     //四方に対応するXY平面上の角度（四元数）を返す（Z軸回転）
//     #[allow(clippy::wrong_self_convention)]
//     pub fn to_quat_z( &self ) -> Quat
//     {   match self
//         {   News::North => Quat::from_rotation_z( PI * 0.0 ),
//             News::East  => Quat::from_rotation_z( PI * 1.5 ),
//             News::West  => Quat::from_rotation_z( PI * 0.5 ),
//             News::South => Quat::from_rotation_z( PI * 1.0 ),
//         }
//     }

//     //時計回りで方角を得る
//     pub fn turn_right( &self ) -> Self
//     {   match self
//         {   News::North => News::East,
//             News::East  => News::South,
//             News::West  => News::North,
//             News::South => News::West,
//         }
//     }

//     //反時計回りで方角を得る
//     pub fn turn_left( &self ) -> Self
//     {   match self
//         {   News::North => News::West,
//             News::East  => News::North,
//             News::West  => News::South,
//             News::South => News::East,
//         }
//     }

//     //背面の方角を得る
//     pub fn back( &self ) -> Self
//     {   match self
//         {   News::North => News::South,
//             News::East  => News::West,
//             News::West  => News::East,
//             News::South => News::North,
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////





// ////////////////////////////////////////////////////////////////////////////////

// //glamの型に別名を付ける
// pub type Grid = IVec2;
// pub type Px2d =  Vec2;

// pub trait GridTrait
// {   fn px2d_screen( &self ) -> Px2d;
//     fn px2d_map   ( &self ) -> Px2d;
// }
// impl GridTrait for Grid
// {   //Gridからスクリーン座標(Px2d)を算出する
//     fn px2d_screen( &self ) -> Px2d
//     {   let neg_half_w = SCREEN_PIXELS_WIDTH  / -2.0;
//         let half_h     = SCREEN_PIXELS_HEIGHT /  2.0;
//         let half_grid  = PIXELS_PER_GRID      /  2.0;

//         let x = neg_half_w + PIXELS_PER_GRID * self.x as f32 + half_grid;
//         let y = half_h     - PIXELS_PER_GRID * self.y as f32 - half_grid;

//         Px2d::new( x, y )
//     }

//     //Gridからマップの原点座標を加味したスクリーン座標(Px2d)を算出する
//     fn px2d_map( &self ) -> Px2d
//     {   ( *self + MAP_ORIGIN_GRID ).px2d_screen()
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //System間の通知用イベント
// #[derive( Event )]
// pub struct EventClear;

// #[derive( Event )]
// pub struct EventOver;

// ////////////////////////////////////////////////////////////////////////////////


// //text UIのメッセージセクションの型
// pub type MessageSect<'a> =
// (   &'a str, //表示文字列
//     &'a str, //フォントのファイル名
//     f32,     //フォンtのピクセル数（PIXELS_PER_GRIDＸ0.7 等）
//     Color,   //文字の色（Bevy::Color）
// );

// //text UIのComponent
// #[derive( Component )]
// pub struct TextUiTitle ( pub MyState );

// #[derive( Component )]
// pub struct TextUiStart ( pub i32, pub MyState, pub usize, pub fn ( i32 ) -> String );

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
// impl WithCountDown for TextUiStart
// {   fn initial_value( &self ) -> i32     { self.0 }
//     fn next_state   ( &self ) -> MyState { self.1 }
//     fn placeholder  ( &self ) -> usize   { self.2 }
//     fn cd_string    ( &self, n: i32 ) -> String { self.3( n ) }
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

// //カウントダウンタイマー用のResource
// #[derive( Resource )]
// pub struct CountDown
// {   pub count: i32,   //カウントダウンの初期値
//     pub timer: Timer, //カウントダウン用タイマー
// }
// impl Default for CountDown
// {   fn default() -> Self
//     {   Self
//         {   count: 0,
//             timer: Timer::from_seconds( 1.0, TimerMode::Once ),
//         }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //十字ボタンの入力状態を保存するResource
// #[derive( Resource )]
// pub struct CrossButton ( pub Vec::<DxDy> );
// impl Default for CrossButton
// {   fn default() -> Self
//     {   Self ( Vec::with_capacity( 2 ) ) //十字ボタンは最大2要素
//     }
// }
// impl CrossButton
// {   pub fn sides( &self ) -> &[ DxDy ] { &self.0 }
//     pub fn is_empty( &self ) -> bool { self.0.is_empty() }
//     pub fn push( &mut self, dxdy: DxDy ) { self.0.push( dxdy ) }
//     pub fn clear( &mut self ) { self.0.clear() }
// }

// //判定用メソッド（traitはオーファンルール対策）
// pub trait Cotains
// {   fn contains_right( &self ) -> bool;
//     fn contains_left ( &self ) -> bool;
//     fn contains_down ( &self ) -> bool;
//     fn contains_up   ( &self ) -> bool;
// }
// impl Cotains for HashSet<GamepadButtonType>
// {   fn contains_right( &self ) -> bool { self.contains( &GamepadButtonType::DPadRight ) }
//     fn contains_left ( &self ) -> bool { self.contains( &GamepadButtonType::DPadLeft  ) }
//     fn contains_down ( &self ) -> bool { self.contains( &GamepadButtonType::DPadDown  ) }
//     fn contains_up   ( &self ) -> bool { self.contains( &GamepadButtonType::DPadUp    ) }
// }

// ////////////////////////////////////////////////////////////////////////////////

// //End of code.