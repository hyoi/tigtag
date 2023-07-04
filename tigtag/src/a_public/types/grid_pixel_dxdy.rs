use super::*;

//glamの型に別名を付ける
//　※bevyがre-exportしているので use glam::{ IVec2, Vec2 }; は不要
pub type Grid  = IVec2; // ≒ struct Grid  { x: i32, y: i32 }
pub type Pixel = Vec2;  // ≒ struct Pixel { x: f32, y: f32 }

//Gridは外部crateの型(の別名)だから直接 impl Grid {…} できない。(オーファンルール)
//なのでトレイトを仲介してメソッドを追加する。
pub trait IntoPixel
{   fn into_pixel_screen( self ) -> Pixel;
    fn into_pixel_map( self ) -> Pixel;
}
impl IntoPixel for Grid
{   //SCREEN全体でGrid座標からPixel座標を算出する
    fn into_pixel_screen( self ) -> Pixel
    {   let x = ( PIXELS_PER_GRID - SCREEN_PIXELS_WIDTH  ) / 2.0 + PIXELS_PER_GRID * self.x as f32;
        let y = ( SCREEN_PIXELS_HEIGHT - PIXELS_PER_GRID ) / 2.0 - PIXELS_PER_GRID * self.y as f32;
        Pixel::new( x, y )
    }

    //MAPの表示位置を考慮してGrid座標からPixel座標へ変換する
    fn into_pixel_map( self ) -> Pixel
    {   self.into_pixel_screen() + *MAP_PIXELS_DISPLAY_OFFSET
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//四方を表す列挙型
#[derive( Debug, Copy, Clone, PartialEq, Eq )]
pub enum DxDy { Up, Down, Right, Left }

////////////////////////////////////////////////////////////////////////////////////////////////////

//GridとDxDyを加算できるよう演算子をオーバーロードする

//Grid = Grid + DxDy
impl Add<DxDy> for Grid
{   type Output = Grid;
    fn add( mut self, dxdy: DxDy ) -> Grid
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
        self
    }
}

//Grid = Grid + &DxDy
impl Add<&DxDy> for Grid
{   type Output = Grid;
    fn add( mut self, dxdy: &DxDy ) -> Grid
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
        self
    }
}

//Grid += DxDy
impl AddAssign<DxDy> for Grid
{   fn add_assign( &mut self, dxdy: DxDy )
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
    }
}

//Grid += &DxDy
impl AddAssign<&DxDy> for Grid
{   fn add_assign( &mut self, dxdy: &DxDy )
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
    }
}

//End of code.

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg( test )]
mod tests
{   #[test]
    fn grid_add_dxdy()
    {   use super::*;

        let grid = Grid::default();
        let mut grid_up    = grid;
        let mut grid_down  = grid;
        let mut grid_right = grid;
        let mut grid_left  = grid;
        let dxdy_up    = DxDy::Up;
        let dxdy_down  = DxDy::Down;
        let dxdy_right = DxDy::Right;
        let dxdy_left  = DxDy::Left;

        //Grid += DxDy
        grid_up    += dxdy_up;
        grid_down  += dxdy_down;
        grid_right += dxdy_right;
        grid_left  += dxdy_left;
        assert_eq!( grid_up   , Grid::new(  0, -1 ) );
        assert_eq!( grid_down , Grid::new(  0,  1 ) );
        assert_eq!( grid_right, Grid::new(  1,  0 ) );
        assert_eq!( grid_left , Grid::new( -1,  0 ) );

        //Grid = Grid + DxDy
        assert_eq!( grid_up   , grid + dxdy_up    );
        assert_eq!( grid_down , grid + dxdy_down  );
        assert_eq!( grid_right, grid + dxdy_right );
        assert_eq!( grid_left , grid + dxdy_left  );

        //Grid += &DxDy
        let ref_dxdy_up    = &dxdy_up;
        let ref_dxdy_down  = &dxdy_down;
        let ref_dxdy_right = &dxdy_right;
        let ref_dxdy_left  = &dxdy_left;
        grid_up    += ref_dxdy_down;
        grid_down  += ref_dxdy_up;
        grid_right += ref_dxdy_left;
        grid_left  += ref_dxdy_right;
        assert_eq!( grid_up   , Grid::new( 0, 0 ) );
        assert_eq!( grid_down , Grid::new( 0, 0 ) );
        assert_eq!( grid_right, Grid::new( 0, 0 ) );
        assert_eq!( grid_left , Grid::new( 0, 0 ) );

        //Grid = Grid + &DxDy
        assert_eq!( grid_up   , grid + dxdy_up    + ref_dxdy_down  );
        assert_eq!( grid_down , grid + dxdy_down  + ref_dxdy_up    );
        assert_eq!( grid_right, grid + dxdy_right + ref_dxdy_left  );
        assert_eq!( grid_left , grid + dxdy_left  + ref_dxdy_right );
    }
}

//End of test code.