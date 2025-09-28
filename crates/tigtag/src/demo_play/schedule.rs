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
            .init_resource::<core_logic::player::DemoMapParams>() // マップのドット配置情報
            .init_resource::<core_logic::player::DemoAutoDriveFn>() // 自走プレイヤーの関数ポインタ
            ;

        //----------------------------------------------------------------------
        // 既存のフッター(header_footer::spawnで作成)を改造する（MyState::Initialize）
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
                    core_logic::map::make_new_stage_data, // マップデータ
                    make_data_for_demo // マップのドット配置情報を初期化
                        .after(core_logic::map::make_new_stage_data),
                    (
                        core_logic::map::spawn_sprite,    // マップスプライト
                        core_logic::player::spawn_sprite, // プレーヤースプライト
                        core_logic::chaser::spawn_sprite, // チェイサースプライト
                    )
                        .after(core_logic::map::make_new_stage_data),
                ),
            )
            // ループ処理
            .add_systems(
                Update,
                (
                    (
                        // スプライトの位置を更新する
                        core_logic::player::move_sprite, // auto_driveを有効にしないと自走しない
                        core_logic::chaser::move_sprite,
                    ),
                    // スコアリング＆クリア判定
                    core_logic::detecting_change::scoring_and_stage_clear,
                    misc::set_next_state(MyState::DemoLoop)
                        .run_if(on_message::<core_logic::DotsAllEaten>),

                    // demo record表示の更新
                    extend_footer::update_demo_record,

                    // ドット削除のイベント発生時にデモ用マップ情報を更新
                    update_data_for_demo.run_if(on_message::<core_logic::DotEaten>),
                    view_data_for_demo.run_if(misc::DEBUG), // debug表示(Gizumo)

                    // 衝突判定
                    core_logic::detecting_change::collisions_and_gameover
                        // DotsAllEaten ➡ スキップ
                        .run_if(not(on_message::<core_logic::DotsAllEaten>)),
                    (
                        // scoreとstageをゼロクリアする
                        core_logic::detecting_change::initialize_score_stage,
                        misc::set_next_state(MyState::DemoLoop),
                    )
                        .run_if(on_message::<core_logic::PlayerCaught>),
                )
                    .chain()
                    .run_if(in_state(MyState::TitleDemo)),
            )
            ;

        //----------------------------------------------------------------------
        // デモプレイの繰り返し（MyState::DemoLoop）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::DemoLoop),
                // 無条件遷移
                misc::set_next_state(MyState::TitleDemo),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////

// demo用にマップのドット配置情報を初期化する
fn make_data_for_demo(
    option_demo: Option<ResMut<core_logic::player::DemoMapParams>>,
    option_map: Option<Res<core_logic::map::Map>>,
) -> Result
{
    let mut demo = option_demo.ok_or("Resource not found.")?;
    let map = option_map.ok_or("Resource not found.")?;

    // dotではなく道を数える(マップデータ作成の直後なら必ず道にdotがある)
    core_logic::map::MAP_CELLS_Y_RANGE.for_each(|y| {
        *demo.dots_sum_y_mut(y) = {
            core_logic::map::MAP_CELLS_X_RANGE
                .filter(|&x| map.is_space(IVec2::new(x, y)))
                .count() as i32
        }
    });
    core_logic::map::MAP_CELLS_X_RANGE.for_each(|x| {
        *demo.dots_sum_x_mut(x) = {
            core_logic::map::MAP_CELLS_Y_RANGE
                .filter(|&y| map.is_space(IVec2::new(x, y)))
                .count() as i32
        }
    });

    // dotsを内包する最小の矩形の初期値は決め打ちでいい(Mapをそう作っているから)
    *demo.dots_rect_min_mut() = IVec2::new(1, 1);
    *demo.dots_rect_max_mut() = IVec2::new(
        core_logic::map::MAP_WIDTH_IN_CELLS - 2,
        core_logic::map::MAP_HEIGHT_IN_CELLS - 2,
    );

    Ok(())
}

impl core_logic::player::DemoMapParams
{
    pub fn dots_sum_y(&self, y: i32) -> i32 { self.dots_sum_y[y as usize] }
    pub fn dots_sum_y_mut(&mut self, y: i32) -> &mut i32
    {
        &mut self.dots_sum_y[y as usize]
    }
    pub fn dots_sum_x(&self, x: i32) -> i32 { self.dots_sum_x[x as usize] }
    pub fn dots_sum_x_mut(&mut self, x: i32) -> &mut i32
    {
        &mut self.dots_sum_x[x as usize]
    }

    pub fn dots_rect_min(&self) -> IVec2 { self.dots_rect.min }
    pub fn dots_rect_min_mut(&mut self) -> &mut IVec2 { &mut self.dots_rect.min }
    pub fn dots_rect_max(&self) -> IVec2 { self.dots_rect.max }
    pub fn dots_rect_max_mut(&mut self) -> &mut IVec2 { &mut self.dots_rect.max }
}

////////////////////////////////////////////////////////////////////////////////

// ドット削除のイベント発生時に実行され、デモ用マップ情報を更新する
fn update_data_for_demo(
    query_player: Query<&core_logic::player::Player>,
    option_demo: Option<ResMut<core_logic::player::DemoMapParams>>,
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
    let x = core_logic::map::MAP_CELLS_X_RANGE
        .position(|i| demo.dots_sum_x(i) != 0)
        .unwrap_or(core_logic::map::MAP_WIDTH_IN_CELLS as usize) as i32;
    #[allow(const_item_mutation)]
    // .position()が&mutで要素にアクセスするwarningの表示を抑止
    let y = core_logic::map::MAP_CELLS_Y_RANGE
        .position(|i| demo.dots_sum_y(i) != 0)
        .unwrap_or(core_logic::map::MAP_HEIGHT_IN_CELLS as usize) as i32;
    *demo.dots_rect_min_mut() = IVec2::new(x, y);

    // dotsを内包する最小の矩形の右下座標（max）を更新する
    let x = core_logic::map::MAP_WIDTH_IN_CELLS
        - 1
        - core_logic::map::MAP_CELLS_X_RANGE
            .rev()
            .position(|i| demo.dots_sum_x(i) != 0)
            .unwrap_or(0) as i32;
    let y = core_logic::map::MAP_HEIGHT_IN_CELLS
        - 1
        - core_logic::map::MAP_CELLS_Y_RANGE
            .rev()
            .position(|i| demo.dots_sum_y(i) != 0)
            .unwrap_or(0) as i32;
    *demo.dots_rect_max_mut() = IVec2::new(x, y);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

use core_logic::GridToPixelOnMap; // for .to_screen_pixels_map_adjusted()

// demo用情報のdebug表示
fn view_data_for_demo(
    option_demo: Option<Res<core_logic::player::DemoMapParams>>,
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
