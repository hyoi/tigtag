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
    fn spawn_textui
    (   self: Box<Self>,
        cmds: &mut Commands,
        asset_svr: &Res<AssetServer>,
        vec_text_spans: Vec<TextUiSpanSettings>,
    );
}

//マーカーComponent
#[derive(Component, Clone)] pub struct StageSatrt;
#[derive(Component, Clone)] pub struct StageClear;
#[derive(Component, Clone)] pub struct GameOver;

//トレイト境界をジェネリクス(T: Component)対象に実装する
impl<T: Component + Clone + 'static> PopupMessage for T
{   fn clone_boxed(&self) -> Box<dyn PopupMessage> { Box::new(self.clone()) }
    fn spawn_textui
    (   self: Box<Self>,
        cmds: &mut Commands,
        asset_svr: &Res<AssetServer>,
        vec_text_spans: Vec<TextUiSpanSettings>,
    )
    {
        cmds.spawn((
            *self, // マーカーComponent
            Visibility::Hidden,
            Node
            {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex, // CSSフレックスレイアウト
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .add_popup_message(vec_text_spans, asset_svr)
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

// リストを基にポップアップメッセージをspawnするSystem
pub fn spawn<T: Resource + Deref<Target = Vec<TextBlock>> + DerefMut>(
    mut settings: ResMut<T>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{
    settings.drain(..).for_each(
        |TextBlock ( boxed_trait, vec_text_spans )| {
            boxed_trait.spawn_textui( &mut cmds, &asset_svr, vec_text_spans )
        },
    );
}

////////////////////////////////////////////////////////////////////////////////

// bevyのEntityCommands型を拡張し、TextBlockを扱いやすくする
pub trait AddPopupMessage
{
    fn add_popup_message(
        &mut self,
        text_block: Vec<TextUiSpanSettings>,
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self;
}

// TextBlock追加メッソド
impl AddPopupMessage for EntityCommands<'_>
{
    fn add_popup_message(
        &mut self,
        text_block: Vec<TextUiSpanSettings>,
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self
    {
        // 準備
        let parent = self.id();
        let mut spans = text_block.iter();
        let (span, file, size, color) = spans.next().unwrap(); // spansの先頭は特別扱い

        // テキストブロックをspawnする
        self.commands_mut()
            .spawn((
                ChildOf(parent),  // 親Entity
                Text::new(*span), // 先頭はTextをspawn
                TextFont {
                    font: asset_svr.load(*file),
                    font_size: *size,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    linebreak: LineBreak::NoWrap,
                },
                TextColor(*color),
            ))
            .with_children(|cmds| {
                for (span, file, size, color) in spans
                {
                    cmds.spawn((
                        TextSpan::new(*span), // 先頭以外はTextSpanをspawn
                        TextFont {
                            font: asset_svr.load(*file),
                            font_size: *size,
                            ..default()
                        },
                        TextColor(*color),
                    ));
                }
            });

        self // method-chain
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
