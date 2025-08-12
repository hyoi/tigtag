use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        application
            //Resource
            .init_resource::<DemoMapParams>() //デモ用マップ情報
            //plugin
            .add_plugins( footer::Schedule ) //フッター(demo record)
            ;

        // MyState::TitleDemoスケジュール
        //デモプレイ
        application
            .add_systems(
                OnEnter(MyState::TitleDemo),
                (
                    //マップデータ生成
                    map::make_new_stage_data,
                    make_data_for_demo, //デモ用マップ情報を収集
                    //スプライトのspawn
                    (
                        map::spawn_sprite,
                        player::spawn_sprite,
                        chaser::spawn_sprite,
                    ),
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    // スコアリング＆クリア判定
                    detecting_change::scoring_and_stage_clear,
                    // ドット削除のイベント発生時にデモ用マップ情報を更新
                    update_data_for_demo.run_if(on_event::<EventEatDot>),
                    // 衝突判定（ステージクリアならチェックしない）
                    detecting_change::collisions_and_gameover
                        .run_if(not(on_event::<EventStageClear>)),
                    // Stateの条件付き遷移
                    set_next_state::<DemoLoop>.run_if(on_event::<EventStageClear>),
                    set_next_state::<DemoLoop>.run_if(on_event::<EventGameOver>),
                    //表示更新
                    (
                        // スプライトの位置を更新する
                        player::move_sprite,
                        chaser::move_sprite,
                        //debug表示(Gizumo)
                        view_data_for_demo.run_if(DEBUG),
                    ),
                )
                    .chain()
                    .run_if(in_state(MyState::TitleDemo)),
            );

        // MyState::DemoLoopスケジュール
        // デモプレイの繰り返し
        application.add_systems(
            OnEnter(MyState::DemoLoop),
            set_next_state::<TitleDemo>, // 無条件遷移
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

//デモ用のマップ情報を作成する
fn make_data_for_demo(
    opt_map: Option<Res<map::Map>>,
    opt_demo: Option<ResMut<DemoMapParams>>,
)
{
    let Some(map) = opt_map
    else
    {
        return;
    };
    let Some(mut demo) = opt_demo
    else
    {
        return;
    };

    //dotではなく道を数える(マップデータ作成の直後なら必ず道にdotがある)
    map::MAP_GRIDS_Y_RANGE.for_each(|y| {
        *demo.dots_sum_y_mut(y) = {
            map::MAP_GRIDS_X_RANGE
                .filter(|&x| map.is_space(IVec2::new(x, y)))
                .count() as i32
        }
    });
    map::MAP_GRIDS_X_RANGE.for_each(|x| {
        *demo.dots_sum_x_mut(x) = {
            map::MAP_GRIDS_Y_RANGE
                .filter(|&y| map.is_space(IVec2::new(x, y)))
                .count() as i32
        }
    });

    //dotsを内包する最小の矩形の初期値は決め打ちでいい(Mapをそう作っているから)
    *demo.dots_rect_min_mut() = IVec2::new(1, 1);
    *demo.dots_rect_max_mut() =
        IVec2::new(map::MAP_GRIDS_WIDTH - 2, map::MAP_GRIDS_HEIGHT - 2);
}

////////////////////////////////////////////////////////////////////////////////

//ドット削除のイベント発生時に実行され、デモ用マップ情報を更新する
fn update_data_for_demo(
    qry_player: Query<&player::Player>,
    opt_demo: Option<ResMut<DemoMapParams>>,
) -> Result
{
    //準備
    let player = qry_player.single()?;
    let mut demo = opt_demo.ok_or("ResMut<DemoMapParams> not found")?;

    //ドットが削除されたので、プレイヤーから見て列・行方向の合計数を減らす
    *demo.dots_sum_x_mut(player.grid.x) -= 1;
    *demo.dots_sum_y_mut(player.grid.y) -= 1;

    //dotsを内包する最小の矩形左上座標（min）を更新する
    #[allow(const_item_mutation)]
    //.position()が&mutで要素にアクセスするwarningの表示を抑止
    let x = map::MAP_GRIDS_X_RANGE
        .position(|i| demo.dots_sum_x(i) != 0)
        .unwrap_or(map::MAP_GRIDS_WIDTH as usize) as i32;
    #[allow(const_item_mutation)]
    //.position()が&mutで要素にアクセスするwarningの表示を抑止
    let y = map::MAP_GRIDS_Y_RANGE
        .position(|i| demo.dots_sum_y(i) != 0)
        .unwrap_or(map::MAP_GRIDS_HEIGHT as usize) as i32;
    *demo.dots_rect_min_mut() = IVec2::new(x, y);

    //dotsを内包する最小の矩形の右下座標（max）を更新する
    let x = map::MAP_GRIDS_WIDTH
        - 1
        - map::MAP_GRIDS_X_RANGE
            .rev()
            .position(|i| demo.dots_sum_x(i) != 0)
            .unwrap_or(0) as i32;
    let y = map::MAP_GRIDS_HEIGHT
        - 1
        - map::MAP_GRIDS_Y_RANGE
            .rev()
            .position(|i| demo.dots_sum_y(i) != 0)
            .unwrap_or(0) as i32;
    *demo.dots_rect_max_mut() = IVec2::new(x, y);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//demo用情報のdebug表示
fn view_data_for_demo(
    opt_demo: Option<Res<DemoMapParams>>,
    mut gizmos: Gizmos,
) -> Result
{
    //準備
    let demo = opt_demo.ok_or("Res<DemoMapParams> not found.")?;

    //ドットを盛れなく含む矩形を算出する
    let adjuster =
        Vec2::Y * PIXELS_PER_GRID / 2.0 + Vec2::NEG_X * PIXELS_PER_GRID / 2.0;
    let min = demo.dots_rect_min().to_vec2_on_game_map() + adjuster;
    let max = demo.dots_rect_max().to_vec2_on_game_map() + adjuster;
    let width = max.x - min.x + PIXELS_PER_GRID;
    let height = max.y - min.y - PIXELS_PER_GRID;
    let size = Vec2::new(width, height);
    let position = min + size / 2.0;

    //ギズモを描画
    gizmos.rect_2d(Isometry2d::from(position), size, Color::Srgba(css::BLUE));

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//demo時のプレイヤー自走に使うメソッド
impl DemoMapParams
{
    //指定のマスが、残dotsの最小矩形の中か？
    pub fn is_inside_rect(&self, grid: IVec2) -> bool
    {
        let IVec2 { x: x1, y: y1 } = self.dots_rect_min();
        let IVec2 { x: x2, y: y2 } = self.dots_rect_max();

        (x1..=x2).contains(&grid.x) && (y1..=y2).contains(&grid.y)
    }

    //指定のマスから残dotsの最小矩形までの単純距離(dx+dy)を求める
    pub fn how_far_to_rect(&self, grid: IVec2) -> i32
    {
        let IVec2 { x: x1, y: y1 } = self.dots_rect_min();
        let IVec2 { x: x2, y: y2 } = self.dots_rect_max();

        let dx = if grid.x < x1
        {
            x1 - grid.x
        }
        else if grid.x > x2
        {
            grid.x - x2
        }
        else
        {
            0
        };
        let dy = if grid.y < y1
        {
            y1 - grid.y
        }
        else if grid.y > y2
        {
            grid.y - y2
        }
        else
        {
            0
        };

        dx + dy
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
