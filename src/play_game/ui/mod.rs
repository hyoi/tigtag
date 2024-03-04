use super::*;

//UIの表示効果、振る舞い
pub mod effect;

//ゲームタイトル
pub mod game_title;
pub use game_title::
{   GameTitle,
    GameTitle_Demo,
};

//プレー開始
pub mod stage_start;
pub use stage_start::
{   StageStart,
    StageStartCD,
};

//ステージクリア
pub mod stage_clear;
pub use stage_clear::
{   StageClear,
    StageClearCD,
};

//ゲームオーバー
pub mod game_over;
pub use game_over::
{   GameOver,
    GameOverCD,
    GameOver_Replay,
};

//End of code.