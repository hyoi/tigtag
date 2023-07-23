use super::*;

////////////////////////////////////////////////////////////////////////////////

//demoplay用の自機の移動方向を決める関数
pub fn which_way_player_goes
(   player: &Player,
    q_chasers: Query<&Chaser>,
    map: Res<Map>,
    org_sides: &[ DxDy ], //交差点が前提なので、要素数は2(三叉路)～3(十字路)
) -> DxDy
{   let mut sides = Vec::from( org_sides );
    let mut goals = Vec::with_capacity( q_chasers.iter().len() );
    q_chasers.for_each
    (   | chaser |
        if chaser.next != player.next
        {   //追手が遠くにいる場合、終点(追手の座標)のリストを作る
            goals.push( chaser.next );
        }
        else
        {   //衝突寸前で緊急回避が必要な場合
            let bad_move =
            {   if chaser.side == player.side
                {   //進行方向が同じなら追突を避ける
                    player.side
                }
                else
                {   //追手に対し正面衝突する方向を避ける
                    match chaser.side
                    {   DxDy::Right => DxDy::Left ,
                        DxDy::Left  => DxDy::Right,
                        DxDy::Down  => DxDy::Up   ,
                        DxDy::Up    => DxDy::Down ,
                    }
                }
            };
            //bad_moveを除く
            sides.retain( | side | *side != bad_move );
        }
    );

    //sidesが空なら運任せ
    if sides.is_empty()
    {   let mut rng = rand::thread_rng();
        return org_sides[ rng.gen_range( 0..org_sides.len() ) ]
    }

    //sidesの要素数が１なら
    if sides.len() == 1
    {   return sides[ 0 ]
    }

    //sidesの要素数が２以上なら、リスクを評価する
    let mut risk_rating = Vec::with_capacity( 3 ); //最大で十字路(3)
    let mut risk_none   = Vec::with_capacity( 3 ); //最大で十字路(3)
    for side in sides
    {   let byway = player.next + side; //わき道の座標
        let risk = check_risk( byway, player, &goals, &map );

        if let Some ( risk ) = risk
        {   //リスクがある場合
            risk_rating.push ( ( side, risk as i32 ) );
        }
        else
        {   //リスクがないと判定されたら、隣接するマス目四方のドットを数える（高得点を狙うのに使う）
            risk_none.push ( ( side, map.count_dots_4sides( byway ) ) );
        }
    }

    //リスクなしの道があるか？によって対象を変える
    let ptr_sides =
    {   if risk_none.is_empty()
        {   //全ての道にリスクがあるなら
            &mut risk_rating
        }
        else
        {   //リスクなしの道があるなら
            &mut risk_none
        }
    };

    //進む道を決める
    if ptr_sides.len() == 1
    {   //道が１つだけの場合
        ptr_sides[ 0 ].0
    }
    else
    {   //道が複数ある場合(２～３)
        ptr_sides.sort_by( | a, b | b.1.cmp( &a.1 ) ); //大きい順にソート
        let max_val = ptr_sides[ 0 ].1;                //先頭の最大値
        ptr_sides.retain( | x | x.1 >= max_val );      //最大値だけのリストにする

        if ptr_sides.len() == 1
        {   //道が１つだけの場合
            ptr_sides[ 0 ].0
        }
        else
        {   //道が複数ある場合
            if ! map.demo.is_inside_rect( player.next )
            {   //自機が残dotsを含む最小の矩形の外にいる場合
                if let Some ( side ) = heuristic_dots_rect( player.next, ptr_sides, map )
                {   return side
                }
            }

            //自機が残dotsを含む最小の矩形の中にいる場合、運頼み
            let mut rng = rand::thread_rng();
            ptr_sides[ rng.gen_range( 0..ptr_sides.len() ) ].0
        }
    }
}

//自機が残dotsを含む最小の矩形の外にいる場合のheuristic関数
fn heuristic_dots_rect
(   grid: Grid,
    sides: &[ ( DxDy, i32 ) ],
    map: Res<Map>,
) -> Option<DxDy>
{   //脇道ごとにdots_rectまでの単純距離(dx+dy)を求める
    let mut vec = Vec::with_capacity( 3 );
    for &( dxdy, _ ) in sides
    {   let side = grid + dxdy;
        let count = map.demo.how_far_to_rect( side );
        vec.push( ( dxdy, count ) );
    }

    //単純距離が最短の脇道を探す
    vec.sort_by( | a, b | a.1.cmp( &b.1 ) ); //小さい順にソート
    let min_val = vec[ 0 ].1;                //先頭の最小値
    vec.retain( | x | x.1 <= min_val );      //最小値だけのリストにする

    //脇道が1つだけならそれを、そうでないならNoneを返す
    if vec.len() == 1 { Some ( vec[ 0 ].0 ) } else { None }
}

impl Map
{   //指定した座標とその四方のドットを数える(結果は0～4)
    fn count_dots_4sides( &self, center: Grid ) -> i32
    {   //指定の座標にドットはあるか
        let mut count = i32::from( self.o_entity( center ).is_some() ); //true:1,false:0

        //四方にドットはあるか
        self.get_byways_list( center ).iter().for_each
        (   | side |
            count += i32::from( self.o_entity( center + side ).is_some() ) //true:1,false:0
        );

        count
    }
}

//リスクを評価する
fn check_risk
(   byway: Grid,
    player: &Player,
    goals: &[ Grid ],
    map: &Res<Map>,
) -> Option<usize>
{   //goalsが空の場合(全ての追手が目前にいる場合)、わき道はリスクがない
    if goals.is_empty() { return None }

    let mut target    = byway;       //脇道の入口の座標
    let mut previous  = player.next; //戻り路の座標
    let mut path_open = VecDeque::from( [ Vec::from( [ previous, target ] ) ] );
    let mut crossing: Option<_> = None;

    let mut risk: Option<usize> = None;
    loop //枝道ぶんを調べる
    {   loop //path_open[ 0 ]を調べる
        {   //終点に到達したら
            if goals.contains( &target )
            {   //最短距離を更新する
                if risk.is_none() || risk.unwrap() > path_open[ 0 ].len()
                {   risk = Some ( path_open[ 0 ].len() );

                    //交差点を探し、あればその距離を記録する
                    let mut work: Option<_> = None;
                    for i in 2..path_open[ 0 ].len() //２×２領域で回り続けないよう、[2]から調べる
                    {   let count_byways = map.get_byways_list( path_open[ 0 ][ i ] ).len();
                        if count_byways >= 3
                        {   work = Some ( i );
                            break
                        }
                    }
                    crossing = work;
                }

                break
            }

            //既に見つかっている最短経路より長くなるなら、それ以上調べない
            if let Some ( shortest ) = risk
            {   if path_open[ 0 ].len() >= shortest { break }
            }

            //交差点か調べる
            let mut sides = map.get_byways_list( target );      //脇道のリスト
            sides.retain( | side | target + side != previous ); //戻り路を排除
            let count = sides.len(); //一本道(1)、Ｔ字路(2)、十字路(3) ※行止り(0)はない

            //ざっくり追手に近いと推測される順に並べ替える
            sides.sort_by_key( | side | heuristic( target + side, goals ) );

            //Ｔ字路(2)、十字路(3)の場合
            for side in sides.iter().take( count ).skip( 1 ) //一本道(1)はループに入らない
            {   let byway = target + side;

                //それまで通った道に到達したら（蛇が胴体にかみついた）
                if path_open[ 0 ].contains( &byway ) { continue }

                //それまでの経路(path_open[ 0 ])を複製した後、脇道を追加
                path_open.push_back( path_open[ 0 ].clone() );
                let last_index = path_open.len() - 1;
                path_open[ last_index ].push( byway );
            }

            //それまで通った道に到達した場合、それ以上調べない（蛇が胴体にかみついた）
            let byway = target + sides[ 0 ];
            if path_open[ 0 ].contains( &byway ) { break }
            path_open[ 0 ].push( byway );

            //一歩進む
            previous = target;
            target   = byway;
        }
        //loop end

        //path_open[ 0 ]を調べ終わったので削除して、空になればチェック完了
        path_open.pop_front();
        if path_open.is_empty() { break }

        //脇道がまだ残っているので、調べるためtargetとpreviousを準備する
        let last_index1 = path_open[ 0 ].len() - 1;
        let last_index2 = path_open[ 0 ].len() - 2;
        target   = path_open[ 0 ][ last_index1 ];
        previous = path_open[ 0 ][ last_index2 ];
    }
    //loop end

    //十分手前に交差点があれば(曲がって逃げられるから)リスクなしと判定する
    if crossing.is_none() || crossing.unwrap() * 2 >= risk.unwrap() - 1
    {   risk
    }
    else
    {   None
    }
}

//ざっくり追手との距離を測って最小値を返す
fn heuristic( byway: Grid, goals: &[ Grid ] ) -> i32
{   let mut shortest = MAP_GRIDS_WIDTH + MAP_GRIDS_HEIGHT;
    for goal in goals
    {   shortest = shortest.min( ( byway.x - goal.x ).abs() + ( byway.y - goal.y ).abs() );
    }
    shortest
}

////////////////////////////////////////////////////////////////////////////////

//End of code.