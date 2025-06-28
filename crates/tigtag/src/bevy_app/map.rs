use super::*;

////////////////////////////////////////////////////////////////////////////////

// マップのデータを作る
pub fn make_new_stage_data(
    opt_record: Option<ResMut<Record>>,
    opt_map: Option<ResMut<Map>>,
) -> Result
{
    // 必須のResource
    let mut record = opt_record.ok_or("Resource <Record> not found.")?;
    let mut map = opt_map.ok_or("Resource <Map> not found.")?;

    // 準備
    let half_w = MAP_GRIDS_WIDTH / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };

    // 二次元配列の矩形領域を指定の値によって埋める無名関数
    enum Obj
    {
        Wall,
        Passage,
    }
    let mut box_fill = |obj, (mut x1, mut y1), (mut x2, mut y2)| {
        if x1 > x2
        {
            std::mem::swap(&mut x1, &mut x2)
        }
        if y1 > y2
        {
            std::mem::swap(&mut y1, &mut y2)
        }
        for y in y1..=y2
        {
            for x in x1..=x2
            {
                let grid = IVec2::new(x, y);
                match obj
                {
                    Obj::Wall => map.set_wall(grid),
                    Obj::Passage => map.set_path(grid),
                }
            }
        }
    };

    // 基本的な回廊
    for xy in 0..=short_side
    {
        let obj = if xy % 2 == 0 { Obj::Wall } else { Obj::Passage };
        let xy1 = (xy, xy);
        let xy2 = (MAP_GRIDS_WIDTH - 1 - xy, MAP_GRIDS_HEIGHT - 1 - xy);
        box_fill(obj, xy1, xy2);
    }

    // 十字の通路
    let xy1 = (1, half_h);
    let xy2 = (MAP_GRIDS_WIDTH - 2, MAP_GRIDS_HEIGHT - 1 - half_h);
    box_fill(Obj::Passage, xy1, xy2);
    let xy1 = (half_w, 1);
    let xy2 = (MAP_GRIDS_WIDTH - 1 - half_w, MAP_GRIDS_HEIGHT - 2);
    box_fill(Obj::Passage, xy1, xy2);

    // 十字通路の中央に壁を作る
    if short_side % 2 == 0
    {
        if half_w >= half_h
        {
            if MAP_GRIDS_HEIGHT % 2 != 0
            {
                let xy1 = (short_side, short_side);
                let xy2 = (MAP_GRIDS_WIDTH - 1 - short_side, short_side);
                box_fill(Obj::Wall, xy1, xy2);
            }
        }
        else if MAP_GRIDS_WIDTH % 2 != 0
        {
            let xy1 = (short_side, short_side);
            let xy2 = (short_side, MAP_GRIDS_HEIGHT - 1 - short_side);
            box_fill(Obj::Wall, xy1, xy2);
        }
    }

    // ランダムに壁を通路に置き換える
    let n = MAP_GRIDS_WIDTH * MAP_GRIDS_HEIGHT / 10; // 例: 40☓25／10＝100
    for _ in 0..n
    {
        let x = map.rng.random_range(2..MAP_GRIDS_WIDTH - 2);
        let y = map.rng.random_range(2..MAP_GRIDS_HEIGHT - 2);
        map.set_path(IVec2::new(x, y));
    }

    // 付随する情報の初期化
    *record.stage_mut() += 1; // 新マップを作ったらステージ数を＋１する
    map.init_path_bits(); // 全グリッドに対し、四方の壁・通の状態をセットする

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 壁とドットのComponent
#[derive(Component)]
pub struct SpriteWall;
#[derive(Component)]
pub struct SpriteDot;

// スプライトをspawnしてマップを表示する
pub fn spawn_sprite(
    opt_map: Option<ResMut<Map>>,
    qry_entity: Query<Entity, Or<(With<SpriteWall>, With<SpriteDot>)>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result
{
    // 必須のResource
    let mut map = opt_map.ok_or("Resource <Map> not found.")?;

    // 準備
    qry_entity.iter().for_each(|id| cmds.entity(id).despawn()); // 既存スプライトがあれば削除する
    let custom_size = Some(GRID_CUSTOM_SIZE);
    map.remaining_dots = 0; // カウンターのゼロクリア

    // 壁とドットのスプライトを配置する
    for y in MAP_GRIDS_Y_RANGE
    {
        for x in MAP_GRIDS_X_RANGE
        {
            let grid = IVec2::new(x, y);
            let vec2 = grid.to_vec2_on_game_map();

            // 壁のスプライト
            if map.is_wall(grid)
            {
                let transform = Transform::from_translation(
                    vec2.extend(DEPTH_SPRITE_BRICK_WALL),
                );
                let sprite = if SPRITE_OFF()
                {
                    // 単色正方形表示
                    let color = css::MAROON.into();
                    let custom_size = Some(GRID_CUSTOM_SIZE * 0.9);
                    Sprite {
                        custom_size,
                        color,
                        ..default()
                    }
                }
                else
                {
                    // 画像表示
                    let image = asset_svr.load(ASSETS_SPRITE_BRICK_WALL);
                    Sprite {
                        custom_size,
                        image,
                        ..default()
                    }
                };
                let id = cmds.spawn((sprite, transform, SpriteWall)).id();

                // debug用のText
                if DEBUG()
                {
                    cmds.entity(id).insert(children![(
                        // 座標の表示はSpriteの子のText2d
                        Text2d::new(format!("{x:02}\n{y:02}")),
                        TextFont {
                            font_size: PIXELS_PER_GRID * 0.4,
                            ..default()
                        },
                        TextColor(css::YELLOW.into()),
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        Transform::from_translation(Vec3::Z),
                    )]);
                }
            }

            // ドットのスプライト
            if map.is_space(grid)
            {
                let id = cmds
                    .spawn((
                        Mesh2d(meshes.add(Circle::new(SPRITE_DOT_RADIUS))),
                        MeshMaterial2d(materials.add(SPRITE_DOT_COLOR)),
                        Transform::from_translation(vec2.extend(DEPTH_SPRITE_DOT)),
                        SpriteDot, // マーカー
                    ))
                    .id();
                *map.opt_entity_mut(grid) = Some(id); // idを保存(プレー中にdespawnするため)
                map.remaining_dots += 1; // ドットを数える
            }
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
