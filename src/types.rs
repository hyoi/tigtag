use super::*;

//Resource Score
pub struct Record
{	pub score	  : usize,
	pub high_score: usize,
	pub stage	  : usize,
}
impl Default for Record
{	fn default() -> Self
	{	Self
		{	score	  : 0,
			high_score: 0,
			stage	  : 1,
		}
	}
}

//Resource Map
pub struct MapInfo
{	pub array: [ [ MapObj; MAP_HEIGHT ]; MAP_WIDTH ],
	pub count_dots: usize,
}
impl Default for MapInfo
{	fn default() -> Self
	{	Self
		{	array: [ [ MapObj::Space; MAP_HEIGHT ]; MAP_WIDTH ],
			count_dots: 0,
		}
	}
}
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	Space,
	Dot ( Option<Entity> ),
	Wall,
}

//End of code.