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
{   fn to_vec2_on_screen( &self ) -> Vec2;
    fn to_vec2_on_map( &self ) -> Vec2;
    fn to_screen_pixels( &self ) -> Vec2;
}

//glamの型にメソッドを追加する
impl GridToPixel for IVec2
{   //平面座標(IVec2)から画面第四象限の座標(Vec2)を算出する
    //アンカーがグリッドの中央になるよう補正する（スプライト等が中央座標で配置されるため）
    fn to_vec2_on_screen( &self ) -> Vec2
    {   ( self.as_vec2() + 0.5 ) * PIXELS_PER_GRID * Vec2::new( 1.0, -1.0 )
    }

    //マップデータからスプライト座標を計算する場合の調整値を加算した座標を返す
    fn to_vec2_on_map( &self ) -> Vec2
    {   self.to_vec2_on_screen() + ADJUSTER_MAP_SPRITES
    }

    //平面座標(IVec2)から画面第四象限の座標(Vec2)を算出する
    fn to_screen_pixels( &self ) -> Vec2
    {   self.as_vec2() * PIXELS_PER_GRID * Vec2::new( 1.0, -1.0 )
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームの状態
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
pub enum MyState
{   #[default] LoadAssets, InitApp,
    GameStart, TitleDemo, DemoLoop, StageStart, MainLoop, StageClear, GameOver,
    Pause,
}

impl MyState
{   pub fn is_pause   ( &self ) -> bool { *self == MyState::Pause }
    pub fn is_demoplay( &self ) -> bool { *self == MyState::TitleDemo || *self == MyState::DemoLoop }
}

//Stateの遷移に使うTrait
pub trait GotoState { fn next( &self ) -> MyState; }

//Stateの遷移に使うマーカー(not Resource)
#[derive( Default )] pub struct GameStart;
#[derive( Default )] pub struct TitleDemo;
#[derive( Default )] pub struct StageStart;
#[derive( Default )] pub struct MainLoop;

impl GotoState for GameStart  { fn next( &self ) -> MyState { MyState::GameStart  } }
impl GotoState for TitleDemo  { fn next( &self ) -> MyState { MyState::TitleDemo  } }
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
#[derive( Event )] pub struct EventEatDot;

////////////////////////////////////////////////////////////////////////////////

//ゲームの成績記録用のResource
#[derive( Resource, Default )]
pub struct Record
{   score   : i32, //スコア
    hi_score: i32, //ハイスコア
    stage   : i32, //ステージ数
    demo    : DemoRecord, //demo用の記録
    is_clear: bool, //ステージクリア時にtrue(※)
}
//※スコアとステージ数を誤って初期化しないよう制御用フラグを設けた

//demo用
#[derive( Default )]
pub struct DemoRecord
{   hi_score: i32, //ハイスコア
    stage   : i32, //ステージ数
}

impl Record
{   pub fn score       ( &    self ) ->      i32 {      self.score    }
    pub fn score_mut   ( &mut self ) -> &mut i32 { &mut self.score    }
    pub fn hi_score    ( &    self ) ->      i32 {      self.hi_score }
    pub fn hi_score_mut( &mut self ) -> &mut i32 { &mut self.hi_score }
    pub fn stage       ( &    self ) ->      i32 {      self.stage    }
    pub fn stage_mut   ( &mut self ) -> &mut i32 { &mut self.stage    }

    pub fn demo_hi_score    ( &    self ) ->      i32 {      self.demo.hi_score }
    pub fn demo_hi_score_mut( &mut self ) -> &mut i32 { &mut self.demo.hi_score }
    pub fn demo_stage       ( &    self ) ->      i32 {      self.demo.stage    }
    pub fn demo_stage_mut   ( &mut self ) -> &mut i32 { &mut self.demo.stage    }

    pub fn is_clear    ( &    self ) ->      bool {      self.is_clear }
    pub fn is_clear_mut( &mut self ) -> &mut bool { &mut self.is_clear }
}

////////////////////////////////////////////////////////////////////////////////

//UIのテキストメッセージセクションの型
pub type MessageSect =
(   &'static str, //表示文字列
    &'static str, //フォントのAssets
    f32,     //フォントのサイズ
    Color,   //フォントの色
);

////////////////////////////////////////////////////////////////////////////////

//操作を受け付けるgamepadを保存するResource
#[derive( Resource, Default )]
pub struct ConnectedGamepad ( Option<Gamepad> );

pub trait TraitGamepad
{   fn id    ( &    self ) ->      Option<Gamepad>;
    fn id_mut( &mut self ) -> &mut Option<Gamepad>;
}

impl TraitGamepad for ConnectedGamepad
{   fn id    ( &    self ) ->      Option<Gamepad> {      self.0 }
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
{   pub grid     : IVec2, //移動中は移動元の座標、停止中はその場の座標
    pub next_grid: IVec2, //移動中は移動先の座標、停止中はその場の座標
    pub direction: News,  //移動向き
    pub timer    : Timer, //移動タイマー
    pub is_stop  : bool,  //移動停止フラグ
    pub speedup  : f32,   //スピードアップ係数
    pub dx_start : Vec2,  //移動した微小区間の始点
    pub dx_end   : Vec2,  //移動した微小区間の終点
    pub opt_fn_autodrive: Option<FnAutoDrive>, //デモ時の自走プレイヤーの移動方向を決める関数
    pub anime_timer: Timer,                           //アニメーションのタイマー
    pub sprite_sheet_frame: usize,                    //アニメーションのフレーム数
    pub sprite_sheet_indexes: HashMap< News, usize >, //アニメーションの先頭位置(offset値)
}

impl Default for Player
{   fn default() -> Self
    {   Self
        {   grid     : IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer    : Timer::from_seconds( PLAYER_TIME_PER_GRID, TimerMode::Once ),
            is_stop  : true,
            speedup  : 1.0,
            dx_start : Vec2::default(),
            dx_end   : Vec2::default(),
            opt_fn_autodrive: None,
            anime_timer: Timer::from_seconds( ANIME_TIMER_PLAYER, TimerMode::Repeating ),
            sprite_sheet_frame: SPRITE_SHEET_COLS_PLAYER,
            sprite_sheet_indexes: ( *SPRITE_SHEET_IDXS_PLAYER ).clone(),
        }
    }
}

//関数ポインタ型(デモ時の自走プレイヤーの移動方向を決める関数)
type FnAutoDrive = fn( &Player, Query<&Chaser>, Res<Map>, Res<DemoMapParams>, &[News] ) -> News;

////////////////////////////////////////////////////////////////////////////////

//チェイサーのComponent
#[derive( Component )]
pub struct Chaser
{   pub grid     : IVec2, //移動中は移動元の座標、停止中はその場の座標
    pub next_grid: IVec2, //移動中は移動先の座標、停止中はその場の座標
    pub direction: News,  //移動向き
    pub timer    : Timer, //移動タイマー
    pub is_stop  : bool,  //移動停止フラグ
    pub speedup  : f32,   //スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub dx_start : Vec2,  //移動した微小区間の始点
    pub dx_end   : Vec2,  //移動した微小区間の終点
    pub opt_fn_chasing: Option<FnChasing>, //チェイサーの移動方向を決める関数
    pub color    : Color, //表示色
    pub anime_timer: Timer,                           //アニメーションのタイマー
    pub sprite_sheet_frame: usize,                    //アニメーションのフレーム数
    pub sprite_sheet_indexes: HashMap< News, usize >, //アニメーションの先頭位置(offset値)
}

impl Default for Chaser
{   fn default() -> Self
    {   Self
        {   grid     : IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::default(),
            timer    : Timer::from_seconds( CHASER_TIME_PER_GRID, TimerMode::Once ),
            is_stop  : true,
            speedup  : 1.0,
            dx_start : Vec2::default(),
            dx_end   : Vec2::default(),
            opt_fn_chasing: None,
            color    : Color::NONE,
            anime_timer: Timer::from_seconds( ANIME_TIMER_CHASER, TimerMode::Repeating ),
            sprite_sheet_frame: SPRITE_SHEET_COLS_CHASER,
            sprite_sheet_indexes: ( *SPRITE_SHEET_IDXS_CHASER ).clone(),
        }
    }
}

//関数ポインタ型(チェイサーの移動方向を決める関数)
pub type FnChasing = fn( &mut Chaser, &Player, &[News] ) -> News;

////////////////////////////////////////////////////////////////////////////////

//スプライトシートでアニメーションするためのトレイト
pub trait CharacterAnimation
{   fn anime_timer_mut( &mut self ) -> &mut Timer;
    fn sprite_sheet_frame( &self ) -> usize;
    fn sprite_sheet_offset( &self, news: News ) -> usize;
    fn direction( &self ) -> News;
}

//Playerのトレイト実装
impl CharacterAnimation for Player
{   fn anime_timer_mut( &mut self ) -> &mut Timer
    {   &mut self.anime_timer
    }
    fn sprite_sheet_frame( &self ) -> usize
    {   self.sprite_sheet_frame
    }
    fn sprite_sheet_offset( &self, news: News ) -> usize
    {   *self.sprite_sheet_indexes.get( &news ).unwrap()
    }
    fn direction( &self ) -> News
    {   self.direction
    }
}

//Chaserのトレイト実装
impl CharacterAnimation for Chaser
{   fn anime_timer_mut( &mut self ) -> &mut Timer
    {   &mut self.anime_timer
    }
    fn sprite_sheet_frame( &self ) -> usize
    {   self.sprite_sheet_frame
    }
    fn sprite_sheet_offset( &self, news: News ) -> usize
    {   *self.sprite_sheet_indexes.get( &news ).unwrap()
    }
    fn direction( &self ) -> News
    {   self.direction
    }
}

////////////////////////////////////////////////////////////////////////////////

//AssetServerにTextureAtlasを作るメソッドを追加
pub trait GenAnimeSprite
{   fn gen_texture_atlas_player( &self ) -> TextureAtlas;
    fn gen_texture_atlas_chaser( &self, asset: &'static str ) -> TextureAtlas;
}
impl GenAnimeSprite for AssetServer
{   fn gen_texture_atlas_player( &self ) -> TextureAtlas
    {   TextureAtlas::from_grid
        (   self.load( ASSETS_SPRITE_SHEET_PLAYER ),
            SPRITE_SHEET_SIZE_PLAYER,
            SPRITE_SHEET_COLS_PLAYER,
            SPRITE_SHEET_ROWS_PLAYER,
            None, None
        )
    }
    fn gen_texture_atlas_chaser( &self, asset: &'static str ) -> TextureAtlas
    {   TextureAtlas::from_grid
        (   self.load( asset ),
            SPRITE_SHEET_SIZE_CHASER,
            SPRITE_SHEET_COLS_CHASER,
            SPRITE_SHEET_ROWS_CHASER,
            None, None
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.