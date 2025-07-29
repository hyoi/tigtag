use super::*;

////////////////////////////////////////////////////////////////////////////////

// ヘッダー情報の位置と更新対象のtext spanのindex
pub enum PlaceHolder
{
    Stage(header_footer::Position, usize),
    Score(header_footer::Position, usize),
    HiScore(header_footer::Position, usize),
}
pub use PlaceHolder::*; //enum PlaceHolderのバリアントを剥き身で公開する

// 登録用Resource
#[derive(Resource)]
pub struct HeaderInfo(pub &'static [PlaceHolder]);
pub trait HeaderInfoTrait
{
    fn list(&self) -> &'static [PlaceHolder];
}
impl HeaderInfoTrait for HeaderInfo
{
    fn list(&self) -> &'static [PlaceHolder] { self.0 }
}

////////////////////////////////////////////////////////////////////////////////

// STAGEの表示を更新する
pub fn update_header<T>(
    opt_place_holder: Option<Res<T>>,
    qry_text_block: Query<(Entity, &header_footer::Position)>,
    mut text_writer: TextUiWriter,
    opt_record: Option<Res<Record>>,
) -> Result
where
    T: Resource + HeaderInfoTrait,
{
    // 準備
    let place_holder = opt_place_holder.ok_or("Res<{T}> not found.")?;
    let record = opt_record.ok_or("Res<Record> not found.")?;

    for x in place_holder.list()
    {
        let (p, index, value) = match x
        {
            PlaceHolder::Stage(p, i) => (p, i, format!("{:02}", record.stage())),
            PlaceHolder::Score(p, i) => (p, i, format!("{:05}", record.score())),
            PlaceHolder::HiScore(p, i) =>
                (p, i, format!("{:05}", record.hi_score())),
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
