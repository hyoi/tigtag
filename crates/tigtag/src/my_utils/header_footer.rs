use super::*;

////////////////////////////////////////////////////////////////////////////////

// TextUIのブロックの情報を格納する型
pub struct TextBlock<'a>
{
    pub position: Position,           // Component
    pub align_self: AlignSelf,        // textblock内の寄せ（上中下）
    pub justify_self: JustifySelf,    // textblock内の寄せ（左中右）
    pub bg_color: Color,              // textblockの背景色
    pub textspans: &'a [MessageSpan], // 表示文字列の情報
    pub update_info: Option<(usize, FormatterFn)>,
}

// ヘッダー／フッターのComponent
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Position
{
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

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
    Color,        // フォントの色
);

// 表示情報の更新で使う整形関数のfnポインタ型
pub type FormatterFn = fn(&dyn std::fmt::Display) -> String;

// 表示情報の更新用
#[derive(Component)]
pub struct UpdateInfo(pub Option<(usize, FormatterFn)>);

////////////////////////////////////////////////////////////////////////////////

// ヘッダー／フッターのレイアウトノードのComponet
#[derive(Component)]
pub struct LayoutNode;

// ヘッダー／フッターをspawnする
pub fn spawn(mut cmds: Commands, asset_svr: Res<AssetServer>) -> Result
{
    // 親ノード（GRIDレイアウト(3x3)）
    let mut layout_node = cmds.spawn((
        LayoutNode, //マーカーComponent
        Node {
            width: Val::Px(SCREEN_PIXELS_WIDTH),
            height: Val::Px(SCREEN_PIXELS_HEIGHT),
            // width: Val::Percent(100.0),
            // height: Val::Percent(100.0),
            display: Display::Grid, // CSSグリッドレイアウト
            grid_template_columns: RepeatedGridTrack::fr(3, 1.0), // ３列
            ..default()
        },
    ));

    // 子のTextBlockをspawnする
    HEADER_FOOTER.iter().for_each(|conf| {
        layout_node.add_textblock(conf, &asset_svr);
    });

    // おまけ(蟹スプライト)
    let grid = (SPRITE_KANI_GRID_X, SPRITE_KANI_GRID_Y);
    let vec3 = grid.to_screen_pixels().extend(DEPTH_SPRITE_KANI_DOTOWN);
    cmds.spawn((
        Sprite {
            image: asset_svr.load(ASSETS_SPRITE_KANI_DOTOWN),
            custom_size: Some(CELL_CUSTOM_SIZE * SPRITE_KANI_MAGNIFY),
            color: SPRITE_KANI_ALPHA,
            ..default()
        },
        Transform::from_translation(vec3),
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
        let mut spans = text_block.textspans.iter();
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
                TextColor(*color),
                BackgroundColor(text_block.bg_color),
                Node {
                    grid_row: GridPlacement::start(row),
                    grid_column: GridPlacement::start(column),
                    align_self: text_block.align_self,
                    justify_self: text_block.justify_self,
                    ..default()
                },
                text_block.position, // マーカーComponent
                UpdateInfo(text_block.update_info),
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

// 全画面時にヘッダー／フッターの位置を構成して画面中央に寄せる
pub fn adjust_header_footer_layout(
    window: Single<&Window>,
    mut headder_footer_layout_node: Single<&mut Node, With<LayoutNode>>,
    option_scale_factor: Option<Res<appctrl_input::ScaleFactor>>,
) -> Result
{
    // 準備
    let res_scale_factor = option_scale_factor.ok_or("Resource not found.")?;

    // カメラにviewportがセットされているなら
    if let appctrl_input::ScaleFactor(Some((_, ui_adjuster, _))) = *res_scale_factor
    {
        // window.modeが全画面なら
        if matches!(window.mode, WindowMode::BorderlessFullscreen(_))
        {
            // UIの表示位置（top、left）を調整する
            headder_footer_layout_node.left = Val::Px(ui_adjuster.x);
            headder_footer_layout_node.top = Val::Px(ui_adjuster.y);
        }
    }
    // 全画面以外の時、調整値が設定されていたなら
    else if headder_footer_layout_node.left != Val::Auto
        || headder_footer_layout_node.top != Val::Auto
    {
        // 調整値をクリアする
        headder_footer_layout_node.left = Val::Auto;
        headder_footer_layout_node.top = Val::Auto;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
