use super::*;

////////////////////////////////////////////////////////////////////////////////

// スプライトアニメーションの情報
pub struct SpriteAnimationParams
{
    pub timer: Timer,                               // タイマー
    pub num_patterns: u32,                          // フレーム数
    pub sprite_sheet_offsets: FxHashMap<News, u32>, // 先頭位置(offset値)
}

// スプライトシートの情報１
impl Default for SpriteAnimationParams
{
    fn default() -> Self
    {
        const NUM_PATTERNS: u32 = 4; // パターン数(TextureAtlasLayoutのcolumns)
        Self {
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            num_patterns: NUM_PATTERNS,
            sprite_sheet_offsets: FxHashMap::from_iter([
                (News::North, 0),
                (News::East, NUM_PATTERNS),
                (News::West, NUM_PATTERNS * 2),
                (News::South, NUM_PATTERNS * 3),
            ]),
        }
    }
}

// スプライトシートの情報２
pub struct MySpriteSheetLayout(pub TextureAtlasLayout);
impl Default for MySpriteSheetLayout
{
    fn default() -> Self
    {
        Self(TextureAtlasLayout::from_grid(
            UVec2::new(8, 8), // １セルの縦横px
            4,                // columns（アニメのパターン数）
            4,                // rows（上下左右の向きで４つ）
            None,             // padding
            None,             // offset
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////

// スプライトシートアニメーション用トレイト
pub trait SpriteAnimation
{
    fn sprite_sheet_offset(&self, news: News) -> u32;
    fn direction(&self) -> News;
    fn anime_timer_mut(&mut self) -> &mut Timer;
    fn num_patterns(&self) -> u32;
}

// Playerのトレイト実装
impl SpriteAnimation for player::Player
{
    fn sprite_sheet_offset(&self, news: News) -> u32
    {
        *self.anime.sprite_sheet_offsets.get(&news).unwrap()
    }
    fn direction(&self) -> News { self.direction }
    fn anime_timer_mut(&mut self) -> &mut Timer { &mut self.anime.timer }
    fn num_patterns(&self) -> u32 { self.anime.num_patterns }
}

// Chaserのトレイト実装
impl SpriteAnimation for chaser::Chaser
{
    fn sprite_sheet_offset(&self, news: News) -> u32
    {
        *self.anime.sprite_sheet_offsets.get(&news).unwrap()
    }
    fn direction(&self) -> News { self.direction }
    fn anime_timer_mut(&mut self) -> &mut Timer { &mut self.anime.timer }
    fn num_patterns(&self) -> u32 { self.anime.num_patterns }
}

////////////////////////////////////////////////////////////////////////////////

// スプライトをアニメーションさせる
pub fn animate_sprites<T>(
    mut query_target: Query<(&mut Sprite, &mut T)>,
    time: Res<Time>,
) where
    T: Component<Mutability = Mutable> + SpriteAnimation,
{
    for (mut sprite, mut character) in &mut query_target
    {
        // アニメーションのタイマー
        let delta = time.delta();
        let finished = character.anime_timer_mut().tick(delta).just_finished();

        if finished && let Some(texture_atlas) = &mut sprite.texture_atlas
        {
            // アニメのパターンを1つ進める
            let index = &mut texture_atlas.index;
            *index += 1;

            // スプライトシートの情報を取り出す
            let news = character.direction();
            let offset = character.sprite_sheet_offset(news) as usize;
            let count = character.num_patterns() as usize;

            // アニメのパターンを周期的に繰り返す
            *index = offset + (*index - offset) % count;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
