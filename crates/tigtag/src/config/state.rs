use super::*;

////////////////////////////////////////////////////////////////////////////////

// ゲームの状態
#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, States, MyState)]
pub enum MyState
{
    LoadAssets,
    Initialize,
    TitleDemo, DemoLoop,
    StageStart, MainLoop, StageClear, GameOver,
    Pause,
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
