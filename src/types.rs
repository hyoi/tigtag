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

//End of code.