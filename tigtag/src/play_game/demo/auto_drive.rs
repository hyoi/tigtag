use super::*;

////////////////////////////////////////////////////////////////////////////////

//デモ時の自走プレイヤーの移動方向を決める関数
pub fn choice_way
(   player: &player::Player,
    qry_chasers: Query<&chasers::Chaser>,
    map: Res<map::Map>,
    demo: Res<schedule::DemoMapParams>,
    org_sides: &[ News ], //交差点が前提なので、要素数は2(三叉路)～3(十字路)
) -> News
{   let mut sides = Vec::from( org_sides );
    let mut chasers = Vec::with_capacity( qry_chasers.iter().len() );
    qry_chasers.iter().for_each
    (   | chaser |
        if chaser.next_grid == player.next_grid
        {   //衝突寸前で緊急回避が必要な場合
            let bad_move =
            {   if chaser.direction == player.direction
                {   //進行方向が同じなら追突を避ける
                    player.direction
                }
                else
                {   //追手に対し正面衝突する方向を避ける
                    match chaser.direction
                    {   News::East  => News::West ,
                        News::West  => News::East ,
                        News::South => News::North,
                        News::North => News::South,
                    }
                }
            };
            sides.retain( | side | *side != bad_move ); //bad_moveを除く
        }
        else
        {   //ぶつからないなら(遠くにいるなら)、リスク評価用に追手の座標リストを作る
            chasers.push( chaser.next_grid );
        }
    );

    //緊急回避でbad_moveを除いた結果sidesが空なら運任せ
    if sides.is_empty()
    {   let mut rng = rand::thread_rng();
        return org_sides[ rng.gen_range( 0..org_sides.len() ) ]
    }

    //sidesの要素数が１ならそれで決まり
    if sides.len() == 1 { return sides[ 0 ] }

    //sidesの要素数が２以上なら、リスクを評価する
    let mut risk_rating = Vec::with_capacity( 3 ); //最大で十字路(3)
    let mut risk_none   = Vec::with_capacity( 3 ); //最大で十字路(3)
    for side in sides
    {   let byway = player.next_grid + side; //脇道の入口の座標
        let risk = check_byway_risk( byway, player.next_grid, &chasers, &map );

        if let Some ( risk ) = risk
        {   //リスクがある場合
            risk_rating.push ( ( side, risk as i32 ) );
        }
        else
        {   //リスクがないと判定されたら、隣接するマス目四方のドットを数える（高得点を狙うのに使う）
            risk_none.push ( ( side, map.count_dots_4sides( byway ) ) );
        }
    }

    //リスクなしの道があるか？によって操作対象のVecを変える
    let ptr_sides = if risk_none.is_empty() { &mut risk_rating } else { &mut risk_none };

    //進む道を決める
    if ptr_sides.len() == 1
    {   //道が１つだけの場合
        ptr_sides[ 0 ].0
    }
    else
    {   //道が複数ある場合(２～３)
        ptr_sides.sort_by( | a, b | b.1.cmp( &a.1 ) ); //大きい順(＝安全な順or高得点な順)にソート
        let max_val = ptr_sides[ 0 ].1;                //先頭の最大値
        ptr_sides.retain( | x | x.1 >= max_val );      //最大値だけのリストにする

        if ptr_sides.len() == 1
        {   //道が１つだけの場合
            ptr_sides[ 0 ].0
        }
        else
        {   //道が複数ある場合
            if ! demo.is_inside_rect( player.next_grid )
            {   //自機が残dotsを含む最小の矩形の外にいる場合
                if let Some ( side ) = heuristic_dots_rect( player.next_grid, ptr_sides, demo )
                {   return side
                }
            }

            //プレイヤーが残dotsを含む最小の矩形の中にいる場合、乱数で決める
            let mut rng = rand::thread_rng();
            ptr_sides[ rng.gen_range( 0..ptr_sides.len() ) ].0
        }
    }
}

//プレイヤーが残dotsを含む最小の矩形の外にいる場合のheuristic関数
fn heuristic_dots_rect
(   grid: IVec2,
    sides: &[ ( News, i32 ) ],
    demo: Res<schedule::DemoMapParams>,
) -> Option<News>
{   //脇道ごとにdots_rectまでの単純距離(dx+dy)を求める
    let mut vec = Vec::with_capacity( 3 );
    for &( dxdy, _ ) in sides
    {   let side = grid + dxdy;
        let count = demo.how_far_to_rect( side );
        vec.push( ( dxdy, count ) );
    }

    //単純距離が最短の脇道を探す
    vec.sort_by( | a, b | a.1.cmp( &b.1 ) ); //小さい順にソート
    let min_val = vec[ 0 ].1;                //先頭の最小値
    vec.retain( | x | x.1 <= min_val );      //最小値だけのリストにする

    //脇道が1つだけならそれを、そうでないならNoneを返す
    if vec.len() == 1 { Some ( vec[ 0 ].0 ) } else { None }
}

impl map::Map
{   //指定した座標とその四方のドットを数える(結果は0～4)
    fn count_dots_4sides( &self, center: IVec2 ) -> i32
    {   //指定の座標にドットはあるか
        let mut count = i32::from( self.opt_entity( center ).is_some() ); //true:1,false:0

        //四方にドットはあるか
        self.get_side_spaces_list( center ).iter().for_each
        (   | side |
            count += i32::from( self.opt_entity( center + side ).is_some() ) //true:1,false:0
        );

        count
    }
}

//脇道を走査してリスクを評価する
fn check_byway_risk
(   mut target  : IVec2, //初期値：player.next_grid + side
    mut previous: IVec2, //初期値：player.next_grid
    chasers: &[ IVec2 ], //chaser.next_gridのリスト
    map: &Res<map::Map>,
) -> Option<usize>
{   //chasersが空の場合(全ての追手が衝突寸前)、脇道にはリスクがない
    if chasers.is_empty() { return None }

    let mut paths = VecDeque::from( [ Vec::from( [ previous, target ] ) ] );
    let mut risk = None;
    let mut crossing = None;

    'Outside: loop //全pathsを調べるループ
    {   'Inside: loop //paths[ 0 ]を調べるループ
        {   let path_0_len = paths[ 0 ].len();

            //chaserのどれかにぶつかれば、そのpathの調査は終わり
            if chasers.contains( &target )
            {   if risk.is_none() || path_0_len < risk.unwrap()
                {   //最短距離を更新する
                    risk = Some ( path_0_len );

                    //交差点を探し、あればその距離を記録する
                    let mut work = None;
                    for i in 2..path_0_len //２×２領域で回り続けないよう、[2]から調べる
                    {   let count_byways = map.get_side_spaces_list( paths[ 0 ][ i ] ).len();
                        if count_byways >= 3
                        {   work = Some ( i );
                            break
                        }
                    }
                    crossing = work;
                }

                break 'Inside;
            }

            //既に見つかっている最短経路より長くなるなら、それ以上調べない
            if risk.is_some_and( |x| path_0_len >= x ) { break 'Inside }

            //targetが交差点か調べる
            let mut sides = map.get_side_spaces_list( target ); //脇道のリスト
            sides.retain( | side | target + side != previous ); //戻り路を排除

            //ざっくりチェイサーに近い順に交差点の入り口を並べ替える
            sides.sort_by_key( | side | heuristic( target + side, chasers ) );

            //Ｔ字路(2)か十字路(3)の場合、分岐PATHを作る
            for side in sides.iter().skip( 1 ) //一本道(1)はループに入らない
            {   let byway = target + side;

                //それまで通った道に入り込まない（蛇の頭が自分の胴体にかみついた）
                if paths[ 0 ].contains( &byway ) { continue }

                //それまでの経路(paths[ 0 ])を複製し、脇道を追加
                paths.push_back( paths[ 0 ].clone() );
                let last_index = paths.len() - 1;
                paths[ last_index ].push( byway );
            }

            //それまで通った道に到達したらそれ以上調べない（蛇の頭が自分の胴体にかみついた）
            let byway = target + sides[ 0 ];
            if paths[ 0 ].contains( &byway ) { break 'Inside }

            //一歩進む
            paths[ 0 ].push( byway );
            previous = target;
            target   = byway;
        }

        //paths[ 0 ]を調べ終わったので削除してpathsが空になればチェック完了
        paths.pop_front();
        if paths.is_empty() { break 'Outside }

        //次のpath[ 0 ]を調べる準備
        target   = paths[ 0 ][ paths[ 0 ].len() - 1 ];
        previous = paths[ 0 ][ paths[ 0 ].len() - 2 ];
    }

    //チェイサーとの間に交差点がないか、あっても中央よりチェイサー側にある場合はリスクあり
    if crossing.is_none() || crossing.unwrap() * 2 >= risk.unwrap() - 1
    {   risk
    }
    else
    {   //チェイサーとの間で中央より手前に交差点があればリスクなし(曲がって逃げられるから)
        None
    }
}

//ざっくりチェイサーとの距離を測って最小値を返す
fn heuristic( target: IVec2, chasers: &[ IVec2 ] ) -> i32
{   let mut shortest = map::MAP_GRIDS_WIDTH + map::MAP_GRIDS_HEIGHT;
    for chaser in chasers
    {   let w_and_h = ( target.x - chaser.x ).abs()
                    + ( target.y - chaser.y ).abs();
        shortest = shortest.min( w_and_h );
    }
    shortest
}

////////////////////////////////////////////////////////////////////////////////

//End of code.