use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        //----------------------------------------------------------------------
        // 各種登録
        application
            .init_resource::<player::DemoMapParams>()   // マップのドット配置情報
            .init_resource::<player::DemoAutoDriveFn>() // 自走プレイヤーの関数ポインタ
            // .add_plugins( footer::Schedule ) //フッター(demo record)
            ;

        //----------------------------------------------------------------------
        // 既存のフッターを改造する（MyState::Initialize）
        application.add_systems(
            OnExit(MyState::Initialize),
            extend_footer::add_text_spans.after(header_footer::spawn),
        );

        //----------------------------------------------------------------------
        // デモプレイ（MyState::TitleDemo）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::TitleDemo),
                (
                    // ステージ初期化
                    map::make_new_stage_data, // マップデータ
                    make_data_for_demo // マップのドット配置情報を初期化
                        .after(map::make_new_stage_data),
                    (
                        map::spawn_sprite,    // マップスプライト
                        player::spawn_sprite, // プレーヤースプライト
                        chaser::spawn_sprite, // チェイサースプライト
                    )
                        .after(map::make_new_stage_data),
                ),
            )
            // ループ処理
            .add_systems(
                Update,
                (
                    (
                        // スプライトの位置を更新する
                        player::move_sprite,
                        chaser::move_sprite,
                    ),
                    // スコアリング＆クリア判定
                    detecting_change::scoring_and_stage_clear,
                    set_next_state::<DemoLoop> //
                        .run_if(on_event::<DotsAllEaten>),
                    // demo record表示の更新
                    extend_footer::update_demo_record,
                    // ドット削除のイベント発生時にデモ用マップ情報を更新
                    update_data_for_demo.run_if(on_event::<DotEaten>),
                    view_data_for_demo.run_if(misc::DEBUG), // debug表示(Gizumo)
                    // 衝突判定
                    detecting_change::collisions_and_gameover
                        .run_if(not(on_event::<DotsAllEaten>)), // DotsAllEaten ➡ スキップ
                    (
                        // scoreとstageをゼロクリアする
                        detecting_change::initialize_score_stage,
                        set_next_state::<DemoLoop>,
                    )
                        .run_if(on_event::<PlayerCaught>),
                )
                    .chain()
                    .run_if(in_state(MyState::TitleDemo)),
            );

        //----------------------------------------------------------------------
        // デモプレイの繰り返し（MyState::DemoLoop）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::DemoLoop),
                set_next_state::<TitleDemo>, // 無条件遷移
            );
    }
}

////////////////////////////////////////////////////////////////////////////////

// demo用にマップのドット配置情報を初期化する
fn make_data_for_demo(
    option_demo: Option<ResMut<player::DemoMapParams>>,
    option_map: Option<Res<map::Map>>,
) -> Result
{
    let mut demo = option_demo.ok_or("Resource not found.")?;
    let map = option_map.ok_or("Resource not found.")?;

    // dotではなく道を数える(マップデータ作成の直後なら必ず道にdotがある)
    map::MAP_CELLS_Y_RANGE.for_each(|y| {
        *demo.dots_sum_y_mut(y) = {
            map::MAP_CELLS_X_RANGE
                .filter(|&x| map.is_space(IVec2::new(x, y)))
                .count() as i32
        }
    });
    map::MAP_CELLS_X_RANGE.for_each(|x| {
        *demo.dots_sum_x_mut(x) = {
            map::MAP_CELLS_Y_RANGE
                .filter(|&y| map.is_space(IVec2::new(x, y)))
                .count() as i32
        }
    });

    // dotsを内包する最小の矩形の初期値は決め打ちでいい(Mapをそう作っているから)
    *demo.dots_rect_min_mut() = IVec2::new(1, 1);
    *demo.dots_rect_max_mut() =
        IVec2::new(map::MAP_WIDTH_IN_CELLS - 2, map::MAP_HEIGHT_IN_CELLS - 2);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// ドット削除のイベント発生時に実行され、デモ用マップ情報を更新する
fn update_data_for_demo(
    query_player: Query<&player::Player>,
    option_demo: Option<ResMut<player::DemoMapParams>>,
) -> Result
{
    // 準備
    let player = query_player.single()?;
    let mut demo = option_demo.ok_or("ResMut<DemoMapParams> not found")?;

    // ドットが削除されたので、プレイヤーから見て列・行方向の合計数を減らす
    *demo.dots_sum_x_mut(player.cell.x) -= 1;
    *demo.dots_sum_y_mut(player.cell.y) -= 1;

    // dotsを内包する最小の矩形左上座標（min）を更新する
    #[allow(const_item_mutation)]
    // .position()が&mutで要素にアクセスするwarningの表示を抑止
    let x = map::MAP_CELLS_X_RANGE
        .position(|i| demo.dots_sum_x(i) != 0)
        .unwrap_or(map::MAP_WIDTH_IN_CELLS as usize) as i32;
    #[allow(const_item_mutation)]
    // .position()が&mutで要素にアクセスするwarningの表示を抑止
    let y = map::MAP_CELLS_Y_RANGE
        .position(|i| demo.dots_sum_y(i) != 0)
        .unwrap_or(map::MAP_HEIGHT_IN_CELLS as usize) as i32;
    *demo.dots_rect_min_mut() = IVec2::new(x, y);

    // dotsを内包する最小の矩形の右下座標（max）を更新する
    let x = map::MAP_WIDTH_IN_CELLS
        - 1
        - map::MAP_CELLS_X_RANGE
            .rev()
            .position(|i| demo.dots_sum_x(i) != 0)
            .unwrap_or(0) as i32;
    let y = map::MAP_HEIGHT_IN_CELLS
        - 1
        - map::MAP_CELLS_Y_RANGE
            .rev()
            .position(|i| demo.dots_sum_y(i) != 0)
            .unwrap_or(0) as i32;
    *demo.dots_rect_max_mut() = IVec2::new(x, y);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// demo用情報のdebug表示
fn view_data_for_demo(
    option_demo: Option<Res<player::DemoMapParams>>,
    mut gizmos: Gizmos,
) -> Result
{
    // 準備
    let demo = option_demo.ok_or("Resource not found.")?;

    // ドットを盛れなく含む矩形を算出する
    let adjuster =
        Vec2::Y * PIXELS_PER_GRID / 2.0 + Vec2::NEG_X * PIXELS_PER_GRID / 2.0;
    let min = demo.dots_rect_min().to_screen_pixels_map_adjusted() + adjuster;
    let max = demo.dots_rect_max().to_screen_pixels_map_adjusted() + adjuster;
    let width = max.x - min.x + PIXELS_PER_GRID;
    let height = max.y - min.y - PIXELS_PER_GRID;
    let size = Vec2::new(width, height);
    let position = min + size / 2.0;

    // ギズモを描画
    gizmos.rect_2d(Isometry2d::from(position), size, Color::Srgba(css::BLUE));

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
