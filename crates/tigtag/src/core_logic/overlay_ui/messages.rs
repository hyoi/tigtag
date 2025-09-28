use super::*;

////////////////////////////////////////////////////////////////////////////////

// 設定リストから全画面メッセージをspawnする
pub fn spawn(mut cmds: Commands, asset_svr: Res<AssetServer>) -> Result
{
    MessageSettings::default()
        .drain(..)
        .for_each(|boxed_component| {
            boxed_component.spawn_overlay_msg(&mut cmds, &asset_svr)
        });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メッセージの設定を格納する型
#[derive(Deref, DerefMut)]
pub struct MessageSettings(pub Vec<Box<dyn BoxedOverlayMessage>>);

// 全画面メッセージspawn用のトレイト
pub trait BoxedOverlayMessage: Send + Sync + 'static
{
    fn spawn_overlay_msg(
        self: Box<Self>,
        cmds: &mut Commands,
        asset_svr: &Res<AssetServer>,
    );
}

// トレイトの実装
impl<T: Component + Clone + 'static + OverlayMessage> BoxedOverlayMessage for T
{
    fn spawn_overlay_msg(
        self: Box<Self>,
        cmds: &mut Commands,
        asset_svr: &Res<AssetServer>,
    )
    {
        cmds.spawn((
            *self.clone(), // マーカーComponent
            Visibility::Hidden,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex, // CSSフレックスレイアウト
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .add_text_spans(self.text_spans(), asset_svr);
    }
}

////////////////////////////////////////////////////////////////////////////////

// 全画面メッセージComponentのトレイト境界
pub trait OverlayMessage
where
    Self: Component<Mutability = Mutable> + Send + Sync + 'static + Default,
{
    // Structの内部にアクセスするメソッド
    fn text_spans(&self) -> &'static [TextUiSpan];

    // 表示直前にパラメータを初期化するメソッド（デフォルト実装）
    fn init(
    ) -> impl FnMut(Query<&mut Self> /*ResMut<Events<CountDownFinished>>*/) -> Result
    where
        Self: std::marker::Sized,
    {
        move |mut query_params: Query<&mut Self>
              /*mut event_countdown: ResMut<Events<CountDownFinished>>*/| {
            // 準備
            let mut params = query_params.single_mut()?;

            // 初期化
            *params = Self::default();
            // event_countdown.clear(); //[対策]EventCountDownが生きているので（v0.16.1）

            Ok(())
        }
    }
}

// 全画面メッセージの文字情報を格納する型
pub type TextUiSpan = (&'static str, &'static str, f32, Color);

////////////////////////////////////////////////////////////////////////////////

// bevyのEntityCommands型を拡張し、TextSpansを扱いやすくするトレイト
pub trait AddOverlatMessage
{
    fn add_text_spans(
        &mut self,
        text_spans: &[TextUiSpan],
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self;
}

// bevyのEntityCommands型を拡張し、TextSpansを扱いやすくするトレイトの実装
impl AddOverlatMessage for EntityCommands<'_>
{
    fn add_text_spans(
        &mut self,
        text_spans: &[TextUiSpan],
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self
    {
        // 準備
        let parent = self.id();
        let mut spans = text_spans.iter();
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
                    justify: Justify::Center,
                    linebreak: LineBreak::NoWrap,
                },
                TextColor(*color),
                // Visibility::Visible, // ★debug時はVisible
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
