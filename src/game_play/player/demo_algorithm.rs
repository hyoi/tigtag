use std::collections::VecDeque;

use super::*;

//demoplay用の自機の移動方向を決める関数
pub fn which_way_player_goes
(   player: &Player,
    q_chasers: Query<&Chaser>,
    map: Res<Map>,
    sides: &[ DxDy ],
) -> DxDy
{   let mut rng = rand::thread_rng();

    //同じマスに居るがまだ衝突だと確定していない場合
    for chaser in q_chasers.iter()
    {   //同じマスに居る追手を特定する
        if chaser.next == player.next
        {   //追手とぶつかる向きの判定
            let bad_move =
            {   if chaser.side == player.side
                {   //そのまま進むと追突するので
                    player.side
                }
                else
                {   //自機と追手が違う向きなら
                    match chaser.side
                    {   DxDy::Right => DxDy::Left ,
                        DxDy::Left  => DxDy::Right,
                        DxDy::Down  => DxDy::Up   ,
                        DxDy::Up    => DxDy::Down ,
                    }
                }
            };

            let mut sides = Vec::from( sides );
            sides.retain( | side | *side != bad_move );
            return sides[ rng.gen_range( 0..sides.len() ) ]
        }
    }

    //終点(追手の座標)のリストを作る
    let mut goals = Vec::with_capacity( q_chasers.iter().len() );
    q_chasers.for_each( | chaser | goals.push( chaser.next ) );

    //脇道毎にリスクを評価する
    let mut risk_rating = Vec::with_capacity( 3 ); //最大で十字路(3)
    let mut risk_none   = Vec::with_capacity( 3 ); //最大で十字路(3)
    for side in sides
    {   let byway = player.next + side;
        let risk = check_risk( byway, player, &goals, &map );
        if risk.is_none()
        {   risk_none.push ( ( side, map.land_values( byway ) ) );
        }
        else
        {   risk_rating.push ( ( side, risk ) );
        }
    }

    if risk_none.is_empty()
    {   //リスク値が低い(値が大きい)順にソートして最大値を求める
        risk_rating.sort_by( | a, b | b.1.cmp( &a.1 ) );
        let max_val = risk_rating[ 0 ].1;
        risk_rating.retain( | x | x.1 >= max_val );

        *risk_rating[ rng.gen_range( 0..risk_rating.len() ) ].0
    }
    else
    {   //dotの価値の高い順にソートして最大値を求める
        risk_none.sort_by( | a, b | b.1.cmp( &a.1 ) );
        let max_val = risk_none[ 0 ].1;
        risk_none.retain( | x | x.1 >= max_val );

        *risk_none[ rng.gen_range( 0..risk_none.len() ) ].0
    }
}

impl Map
{   //指定した座標とその四方のドットを数える(結果は0～4)
    fn land_values( &self, center: Grid ) -> i32
    {   //指定の座標にドットはあるか
        let mut count = if self.o_entity( center ).is_some() { 1 } else { 0 };

        //四方にドットはあるか
        for side in self.get_byways_list( center )
        {   if self.o_entity( center + side ).is_some()
            {   count += 1;
            }
        }

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
{   let mut target    = byway;       //脇道の入口の座標
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

                    //先頭から交差点を探し、あればその距離を記録する
                    let mut work: Option<_> = None;
                    for i in 1..path_open[ 0 ].len() //[0]はplayer.nextなので[1]から調べる
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

//End of code.