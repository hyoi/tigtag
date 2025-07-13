use super::*;

////////////////////////////////////////////////////////////////////////////////

// スプライトをアニメーションさせる
pub fn animate_sprites<T>(
    mut qry_target: Query<(&mut Sprite, &mut T)>,
    time: Res<Time>,
) where
    T: Component<Mutability = Mutable> + SpriteAnimation,
{
    for (mut sprite, mut character) in &mut qry_target
    {
        //アニメーションのタイマー
        let delta = time.delta();
        let finished = character.anime_timer_mut().tick(delta).just_finished();

        if finished && let Some(texture_atlas) = &mut sprite.texture_atlas
        {
            //アニメのパターンを1つ進める
            let index = &mut texture_atlas.index;
            *index += 1;

            //スプライトシートの情報を取り出す
            let news = character.direction();
            let offset = character.sprite_sheet_offset(news) as usize;
            let count = character.num_patterns() as usize;

            //アニメのパターンを周期的に繰り返す
            *index = offset + (*index - offset) % count;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// スプライトシートアニメーション用トレイト
pub trait SpriteAnimation
{
    fn anime_timer_mut(&mut self) -> &mut Timer;
    fn num_patterns(&self) -> u32;
    fn sprite_sheet_offset(&self, news: News) -> u32;
    fn direction(&self) -> News;
}

////////////////////////////////////////////////////////////////////////////////

// Playerのトレイト実装
impl SpriteAnimation for player::Player
{
    fn anime_timer_mut(&mut self) -> &mut Timer { &mut self.anime.timer }
    fn num_patterns(&self) -> u32 { self.anime.num_patterns }
    fn sprite_sheet_offset(&self, news: News) -> u32
    {
        *self.anime.sprite_sheet_offsets.get(&news).unwrap()
    }
    fn direction(&self) -> News { self.direction }
}

// Chaserのトレイト実装
impl SpriteAnimation for chaser::Chaser
{
    fn anime_timer_mut(&mut self) -> &mut Timer { &mut self.anime_timer }
    fn num_patterns(&self) -> u32 { self.sprite_sheet_frame }
    fn sprite_sheet_offset(&self, news: News) -> u32
    {
        *self.sprite_sheet_indexes.get(&news).unwrap()
    }
    fn direction(&self) -> News { self.direction }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
