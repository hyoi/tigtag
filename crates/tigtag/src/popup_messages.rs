use super::*;

////////////////////////////////////////////////////////////////////////////////

// ポップアップメッセージをspawnするために必要な情報のリスト（Resource）
#[derive(Resource, Deref, DerefMut)]
pub struct PopupMessages ( pub Vec<TextBlock> );

// ポップアップメッセージをspawnするために必要な情報
pub struct TextBlock (
    pub Box<dyn PopupMessage>,  // マーカーComponent
    pub Vec<TextUiSpanSettings> // TextUiのspans
);

// TextUiのspanをspawnする為に必要な情報
pub type TextUiSpanSettings = (&'static str,&'static str,f32,Color);

// TextBlockのCloneトレイトの実装
impl Clone for TextBlock {
    fn clone(&self) -> Self {
        Self ( self.0.clone_boxed(), self.1.clone() )
    }
}

////////////////////////////////////////////////////////////////////////////////

//マーカーComponentをBoxで束ねるためのトレイト境界
pub trait PopupMessage: Send + Sync + 'static
{   fn clone_boxed(&self) -> Box<dyn PopupMessage>;
}

//マーカーComponent
#[derive(Component, Clone)] pub struct StageSatrt;
#[derive(Component, Clone)] pub struct StageClear;
#[derive(Component, Clone)] pub struct GameOver;

impl PopupMessage for StageSatrt
{   fn clone_boxed(&self) -> Box<dyn PopupMessage> { Box::new(self.clone()) }
}
impl PopupMessage for StageClear
{   fn clone_boxed(&self) -> Box<dyn PopupMessage> { Box::new(self.clone()) }
}
impl PopupMessage for GameOver
{   fn clone_boxed(&self) -> Box<dyn PopupMessage> { Box::new(self.clone()) }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
