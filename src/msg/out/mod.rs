use serde::{Deserialize, Serialize};

use crate::app::{Command, Task};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MsgOut {
    ExplorePwdAsync,
    ExploreParentsAsync,
    Refresh,
    ClearScreen,
    Quit,
    Debug(String),
    Call(Command),
    CallSilently(Command),
    CallLua(String),
    CallLuaSilently(String),
    LuaEval(String),
    LuaEvalSilently(String),
    Enque(Task),
    EnableMouse,
    DisableMouse,
    ToggleMouse,
    StartFifo(String),
    StopFifo,
    ToggleFifo(String),
    PrintPwdAndQuit,
    PrintFocusPathAndQuit,
    PrintSelectionAndQuit,
    PrintResultAndQuit,
    PrintAppStateAndQuit,
}
