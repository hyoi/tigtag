use super::*;

////////////////////////////////////////////////////////////////////////////////

// マップのデータを作る
pub fn make_new_stage_data(
    option_record: Option<ResMut<Record>>,
    option_map: Option<ResMut<Map>>,
) -> Result
{
    // 必須のResource
    let mut record = option_record.ok_or("Resource not found.")?;
    let mut map = option_map.ok_or("Resource not found.")?;

    // 二次元配列の矩形領域を指定の値によって埋める無名関数
    let origin_bottom_right =
        IVec2::from((MAP_WIDTH_IN_CELLS, MAP_HEIGHT_IN_CELLS)) - IVec2::ONE;
    let mut box_fill = |pt1: IVec2, is_wall| {
        let pt2 = origin_bottom_right - pt1;
        if is_wall
        {
            (pt1.y..=pt2.y).for_each(|y| {
                (pt1.x..=pt2.x).for_each(|x| {
                    map.set_wall(IVec2::new(x, y));
                });
            });
        }
        else
        {
            (pt1.y..=pt2.y).for_each(|y| {
                (pt1.x..=pt2.x).for_each(|x| {
                    map.set_path(IVec2::new(x, y));
                });
            });
        }
    };

    // 準備
    let half_w = MAP_WIDTH_IN_CELLS / 2;
    let half_h = MAP_HEIGHT_IN_CELLS / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };

    // 基本的な回廊
    (0..=short_side).for_each(|x| {
        box_fill(IVec2::new(x, x), x % 2 == 0);
    });

    // 十字の通路
    box_fill(IVec2::new(1, half_h), false);
    box_fill(IVec2::new(half_w, 1), false);

    // 十字通路の中央に壁を作る
    if short_side % 2 == 0
    {
        if half_w >= half_h
        {
            if MAP_HEIGHT_IN_CELLS % 2 != 0
            {
                box_fill(IVec2::new(short_side, short_side), true);
            }
        }
        else if MAP_WIDTH_IN_CELLS % 2 != 0
        {
            box_fill(IVec2::new(short_side, short_side), true);
        }
    }

    // ランダムに壁を通路に置き換える
    let n = MAP_WIDTH_IN_CELLS * MAP_HEIGHT_IN_CELLS / 10; // 例: 40☓25／10＝100
    (0..n).for_each(|_| {
        let x = map.rng.random_range(2..MAP_WIDTH_IN_CELLS - 2);
        let y = map.rng.random_range(2..MAP_HEIGHT_IN_CELLS - 2);
        map.set_path(IVec2::new(x, y));
    });

    // 付随する情報の初期化
    *record.stage_mut() += 1;
    map.init_path_bits(); // 全グリッドに対し、四方の壁・通の状態をセットする

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// スプライトのComponent（壁とドット）
#[derive(Component)]
pub struct SpriteWall;
#[derive(Component)]
pub struct SpriteDot;

// 壁とドットのセットの型（Query用）
type WithWallAndDotSprite = Or<(With<SpriteWall>, With<SpriteDot>)>;

// スプライトをspawnしてマップを表示する
pub fn spawn_sprite(
    option_map: Option<ResMut<Map>>,
    query_entity: Query<Entity, WithWallAndDotSprite>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result
{
    // 準備
    let mut map = option_map.ok_or("Resource not found.")?; // 必須のResource
    query_entity.iter().for_each(|id| cmds.entity(id).despawn()); // 既存スプライトがあれば削除する
    map.remaining_dots = 0; // カウンターのゼロクリア

    // 壁とドットのスプライトを配置する
    MAP_CELLS_Y_RANGE.for_each(|y| {
        MAP_CELLS_X_RANGE.for_each(|x| {
            let grid = IVec2::new(x, y);
            let vec2 = grid.to_screen_pixels_map_adjusted();

            // 壁のスプライト
            if map.is_wall(grid)
            {
                // コンパイルスイッチが指定されていたなら
                let sprite = if SPRITE_OFF()
                {
                    // 単色正方形メッシュ
                    Sprite {
                        custom_size: Some(CELL_CUSTOM_SIZE * 0.9),
                        color: css::MAROON.into(),
                        ..default()
                    }
                }
                else
                {
                    // スプライト画像
                    Sprite {
                        custom_size: Some(CELL_CUSTOM_SIZE),
                        image: asset_svr.load(ASSETS_SPRITE_BRICK_WALL),
                        ..default()
                    }
                };
                let id = cmds
                    .spawn((
                        sprite,
                        Transform::from_translation(
                            vec2.extend(DEPTH_SPRITE_BRICK_WALL),
                        ),
                        SpriteWall, // マーカー
                    ))
                    .id();

                // debug用のText
                if misc::DEBUG()
                {
                    cmds.entity(id).insert(
                        // 座標の表示はSpriteの子のText2d
                        children![(
                            Text2d::new(format!("{x:02}\n{y:02}")),
                            TextFont {
                                font_size: PIXELS_PER_GRID * 0.4,
                                ..default()
                            },
                            TextColor(css::YELLOW.into()),
                            TextLayout {
                                justify: Justify::Center,
                                ..default()
                            },
                            Transform::from_translation(Vec3::Z),
                        )],
                    );
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
                *map.option_entity_mut(grid) = Some(id); // idを保存(プレー中にdespawnするため)
                map.remaining_dots += 1; // ドットを数える
            }
        })
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
