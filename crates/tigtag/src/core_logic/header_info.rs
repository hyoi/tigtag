use super::*;

////////////////////////////////////////////////////////////////////////////////

// ヘッダー情報の位置と更新対象のtext spanのindex
pub enum PlaceHolderLabel
{
    Stage(header_footer::Position, usize),
    Score(header_footer::Position, usize),
    HiScore(header_footer::Position, usize),
}
pub use PlaceHolderLabel::*; //enum PlaceHolderLabelのバリアントを剥き身で公開する

// 登録用Resource
#[derive(Resource)]
pub struct PlaceHolder(pub &'static [PlaceHolderLabel]);
pub trait PlaceHolderTrait
{
    fn list(&self) -> &'static [PlaceHolderLabel];
}
impl PlaceHolderTrait for PlaceHolder
{
    fn list(&self) -> &'static [PlaceHolderLabel] { self.0 }
}

////////////////////////////////////////////////////////////////////////////////

// STAGEの表示を更新する
pub fn update<T>(
    opt_place_holder: Option<Res<T>>,
    qry_text_block: Query<(Entity, &header_footer::Position)>,
    mut text_writer: TextUiWriter,
    opt_record: Option<Res<Record>>,
) -> Result
where
    T: Resource + PlaceHolderTrait,
{
    // 準備
    let place_holder = opt_place_holder.ok_or("Res<{T}> not found.")?;
    let record = opt_record.ok_or("Res<Record> not found.")?;

    for x in place_holder.list()
    {
        let (p, index, value) = match x
        {
            Stage(p, i) => (p, i, format!("{:02}", record.stage())),
            Score(p, i) => (p, i, format!("{:05}", record.score())),
            HiScore(p, i) => (p, i, format!("{:05}", record.hi_score())),
        };

        let root_entity = qry_text_block
            .iter()
            .filter(|(_, position)| **position == *p)
            .collect::<Vec<(Entity, &header_footer::Position)>>();

        // 書き換え
        if !root_entity.is_empty()
        {
            let entity = root_entity[0].0;
            let mut text = text_writer
                .get_text(entity, *index)
                .ok_or(format!("No entity with a matching index: {index}"))?;
            *text = value;
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
