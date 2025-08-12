use super::*;

////////////////////////////////////////////////////////////////////////////////

// spawnするTextBlockのリストを登録するためのResource
#[derive(Resource, Deref)]
pub struct Settings<'a>(pub &'a [TextBlock<'a>]);

// TextUIのブロックの情報を格納する型
pub struct TextBlock<'a>
{
    pub position: Position,        // Component
    pub align_self: AlignSelf,     // textblock内の寄せ（上中下）
    pub justify_self: JustifySelf, // textblock内の寄せ（左中右）
    pub bg_color: Srgba,           // textblockの背景色
    pub spans: &'a [MessageSpan],  // 表示文字列の情報
}

// シンプル ヘッダー／フッターのComponent
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum Position
{
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}
pub use Position::*; //enum Positionのバリアントを剥き身で公開する
impl Position
{
    fn index_xy(&self) -> (i16, i16)
    {
        match self
        {
            Position::TopLeft => (1, 1),
            Position::TopCenter => (1, 2),
            Position::TopRight => (1, 3),
            Position::BottomLeft => (3, 1),
            Position::BottomCenter => (3, 2),
            Position::BottomRight => (3, 3),
        }
    }
}

// TextUIの文字列の情報を格納する型
pub type MessageSpan = (
    &'static str, // 表示文字列
    &'static str, // フォントのAssets
    f32,          // フォントのサイズ
    Srgba,        // フォントの色
);

////////////////////////////////////////////////////////////////////////////////

// シンプル ヘッダー／フッターをspawnする
pub fn spawn_header_footer(
    opt_textblock_list: Option<Res<Settings<'static>>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
) -> Result
{
    // 準備
    let textblock_list =
        opt_textblock_list.ok_or("Res<Settings<'static>> not found.")?;

    // 親ノードをGRIDレイアウト（３Ｘ３）でspawnする
    let mut layout_node = cmds.spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        // width : Val::Px ( SCREEN_PIXELS_WIDTH  ),
        // height: Val::Px ( SCREEN_PIXELS_HEIGHT ),
        display: Display::Grid, // CSSグリッドレイアウト
        grid_template_columns: RepeatedGridTrack::fr(3, 1.0), // ３列
        ..default()
    },));

    // 子のTextBlockをspawnする
    textblock_list.iter().for_each(|textblock| {
        layout_node.add_textblock(textblock, &asset_svr);
    });

    // おまけ(蟹)
    let custom_size = Some(GRID_CUSTOM_SIZE * SPRITE_KANI_MAGNIFY);
    let color = SPRITE_KANI_COLOR;
    let vec2 = (SPRITE_KANI_GRID_X, SPRITE_KANI_GRID_Y).to_vec2_of_screen();
    let image = asset_svr.load(ASSETS_SPRITE_KANI_DOTOWN);
    cmds.spawn((
        Sprite {
            image,
            custom_size,
            color,
            ..default()
        },
        Transform::from_translation(vec2.extend(DEPTH_SPRITE_KANI_DOTOWN)),
    ));

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// bevyのEntityCommands型を拡張し、TextBlockを扱いやすくする
pub trait AddTextBlock
{
    fn add_textblock(
        &mut self,
        text_block: &TextBlock,
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self;
}

// TextBlock追加メッソド
impl AddTextBlock for EntityCommands<'_>
{
    fn add_textblock(
        &mut self,
        text_block: &TextBlock,
        asset_svr: &Res<AssetServer>,
    ) -> &mut Self
    {
        // 準備
        let parent = self.id();
        let mut spans = text_block.spans.iter();
        let (span, file, size, color) = spans.next().unwrap(); // spansの先頭は特別扱い
        let (row, column) = text_block.position.index_xy();

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
                TextColor(Color::Srgba(*color)),
                BackgroundColor(text_block.bg_color.into()),
                Node {
                    grid_row: GridPlacement::start(row),
                    grid_column: GridPlacement::start(column),
                    align_self: text_block.align_self,
                    justify_self: text_block.justify_self,
                    ..default()
                },
                text_block.position, // マーカーComponent
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
                        TextColor(Color::Srgba(*color)),
                    ));
                }
            });

        self // method-chain
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
