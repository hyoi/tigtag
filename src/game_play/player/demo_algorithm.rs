use super::*;

//demoplay用の自機の移動方向を決める関数
//＜前提条件＞
//　1.本関数は、三叉路または十字路で呼び出される
//　2.sidesは、後戻り方向を含まない(自機が後戻りしない仕様のため)
pub fn which_way_player_goes
(   player: &Player,
    q_chasers: Query<&Chaser>,
    map: &Map,
    sides: &[ DxDy ]
) -> DxDy
{   const RIGHT: usize = 0;
    const LEFT : usize = 1;
    const DOWN : usize = 2;
    const UP   : usize = 3;
    let mut rough_value: [ Option<i32>; 4 ] = [ None; 4 ];

    //自機と追手の距離をざっくり把握する
    #[allow( clippy::comparison_chain )]
    q_chasers.for_each
    (   | chaser |
        {   let mut dx = chaser.next.x - player.next.x;
            if dx > 0
            {   rough_value[ RIGHT ] = Some ( rough_value[ RIGHT ].map_or( dx, | val | val.min( dx ) ) );
            }
            else if dx < 0
            {   dx = dx.abs();
                rough_value[ LEFT  ] = Some ( rough_value[ LEFT  ].map_or( dx, | val | val.min( dx ) ) );
            }

            let mut dy = chaser.next.y - player.next.y;
            if dy > 0
            {   rough_value[ DOWN  ] = Some ( rough_value[ DOWN  ].map_or( dy, | val | val.min( dy ) ) );
            }
            else if dy < 0
            {   dy = dy.abs();
                rough_value[ UP    ] = Some ( rough_value[ UP    ].map_or( dy, | val | val.min( dy ) ) );
            }
        }
    );

    //進める方向(sides)のみ、ざっくり距離とのタプルを作る
    let mut work = Vec::with_capacity( 8 ); //.append()に備えて
    let mut zero = Vec::with_capacity( 4 );
    sides.iter().for_each
    (   | dxdy |
        {   let dot = map.land_values( player.next + dxdy );
            let side = match dxdy
            {   DxDy::Right => RIGHT,
                DxDy::Left  => LEFT,
                DxDy::Down  => DOWN,
                DxDy::Up    => UP,
            };
            match rough_value[ side ]
            {   Some ( x ) => work.push( ( x, dxdy, dot ) ),
                None       => zero.push( ( 0, dxdy, dot ) ),
            }
        }
    );

    //降順にソートして距離の最大値を見つけ、閾値(5?)以上の値のvecを作る
    if ! work.is_empty()
    {   work.sort_by( | a, b | b.0.cmp( &a.0 ) );
        let max_val = work[ 0 ].0;
        if max_val >= 5 //自壁道壁道敵
        {   work.retain( | x | x.0 >= max_val );
        }
        else
        {   work.clear();
        }
    }
    work.append( &mut zero );

    //dotの価値の降順にソートして最大値を見つけ、最大値のみのvecを作る
    if work.len() >= 2
    {   work.sort_by( | a, b | b.2.cmp( &a.2 ) );
        let max_val = work[ 0 ].2;
        work.retain( | x | x.2 >= max_val );
    }

    //候補が複数なら乱数で決める
    let mut rng = rand::thread_rng();
    if ! work.is_empty()
    {   return *work[ rng.gen_range( 0..work.len() ) ].1
    }

    //コードの安全装置
    sides[ rng.gen_range( 0..sides.len() ) ]
}

//End of code.