use super::*;

//(demoplay)自機の移動方向を決める関数
pub fn which_way_player_goes
(   player: &Player,
    q_chasers: Query<&Chaser>,
    _map: &Map,
    sides: &[ DxDy ]
) -> DxDy
{   let ( mut right, mut left, mut down, mut up ) = ( 0, 0, 0, 0 );
    let mut rng = rand::thread_rng();

    q_chasers.for_each
    (   | chaser |
        {   let dx = chaser.next.x - player.next.x;
            if dx > 0
            {   right = if right == 0 { dx } else { right.min( dx ) };
            }
            else if dx < 0
            {   left = if left == 0 { dx.abs() } else { left.min( dx.abs() ) };
            }

            let dy = chaser.next.y - player.next.y;
            if dy > 0
            {   down = if down == 0 { dy } else { down.min( dy ) };
            }
            else if dy < 0
            {   up = if up == 0 { dy.abs() } else { up.min( dy.abs() ) };
            }
        }
    );

    if right == 0 && left == 0 //暗に down != 0 && up != 0
    {   //R or L
        if sides.contains( &DxDy::Right )
        {   if sides.contains( &DxDy::Left )
            {   return [ DxDy::Right, DxDy::Left ][ rng.gen_range( 0..=1 ) ]
            }
            else
            {   return DxDy::Right
            }
        }
        else if sides.contains( &DxDy::Left )
        {   return DxDy::Left
        }
    }
    else if down == 0 && up == 0 //暗に right != 0 && left != 0
    {   //D or U
        if sides.contains( &DxDy::Down )
        {   if sides.contains( &DxDy::Up )
            {   return [ DxDy::Down, DxDy::Up ][ rng.gen_range( 0..=1 ) ]
            }
            else
            {   return DxDy::Down
            }
        }
        else if sides.contains( &DxDy::Up )
        {   return DxDy::Up
        }
    }
    else if right == 0 //left != 0
    {   //R or ( D or U or null )
    }
    else if left == 0 //right != 0
    {   //L or ( D or U or null )
    }
    else if down == 0 //up != 0
    {   //D or ( R or L or null )
    }
    else if up == 0 //down != 0
    {   //U or ( R or L or null )
    }
    else if right >= left && down >= up
    {   //R > D ==> R or R < D ==> D
    }
    else if right >= left && down < up
    {
    }
    else if right < left && down >= up
    {
    }
    else if right < left && down < up
    {
    }

    sides[ rng.gen_range( 0..sides.len() ) ]
}

//End of code.