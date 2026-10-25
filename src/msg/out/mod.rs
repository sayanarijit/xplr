use serde::{Deserialize, Serialize};

use crate::app::{Command, Task};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MsgOut {
    ExplorePwdAsync,
    ExploreParentsAsync,
    Refresh,
    ClearScreen,
    Debug(String),
    Call(Command),
    Call0(Command),
    CallSilently(Command),
    CallSilently0(Command),
    CallLua(String),
    CallLuaSilently(String),
    LuaEval(String),
    LuaEvalSilently(String),
    EnableMouse,
    DisableMouse,
    ToggleMouse,
    StartFifo(String),
    StopFifo,
    ToggleFifo(String),
    ScrollUp,
    ScrollDown,
    ScrollUpHalf,
    ScrollDownHalf,
    Quit,
    PrintPwdAndQuit,
    PrintFocusPathAndQuit,
    PrintSelectionAndQuit,
    PrintResultAndQuit,
    PrintAppStateAndQuit,
    Enqueue(Task),
}

impl MsgOut {
    pub fn is_read_only(&self) -> bool {
        !matches!(
            self,
            Self::Call(_)
                | Self::Call0(_)
                | Self::CallSilently(_)
                | Self::CallSilently0(_)
                | Self::CallLua(_)
                | Self::CallLuaSilently(_)
                | Self::LuaEval(_)
                | Self::LuaEvalSilently(_)
        )
    }
}
