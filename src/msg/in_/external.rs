use crate::app::Node;
use crate::input::InputOperation;
use crate::search::PathItem;
use crate::search::RankCriteria;
use crate::search::SearchAlgorithm;
use indexmap::IndexSet;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, sync::Arc};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExternalMsg {
    /// ### Exploring ----------------------------------------------------------
    ///
    /// Explore the present working directory and register the filtered nodes.
    /// This operation is expensive. So, try to avoid using it too often.
    ///
    /// Example:
    ///
    /// - Lua: `"ExplorePwd"`
    /// - YAML: `ExplorePwd`
    ExplorePwd,

    /// Explore the present working directory and register the filtered nodes
    /// asynchronously. This operation happens asynchronously. That means, the
    /// xplr directory buffers won't be updated immediately. Hence, it needs to
    /// be used with care and probably with special checks in place. To explore
    /// $PWD synchronously, use `ExplorePwd` instead.
    ///
    /// Example:
    ///
    /// - Lua: `"ExplorePwdAsync"`
    /// - YAML: `ExplorePwdAsync`
    ExplorePwdAsync,

    /// Explore the present working directory along with its parents and
    /// register the filtered nodes. This operation happens asynchronously.
    /// That means, the xplr directory buffers won't be updated immediately.
    /// Hence, it needs to be used with care and probably with special checks
    /// in place. To explore just the `$PWD` synchronously, use `ExplorePwd`
    /// instead.
    ///
    /// Example:
    ///
    /// - Lua: `"ExploreParentsAsync"`
    /// - YAML: `ExploreParentsAsync`
    ExploreParentsAsync,

    /// Tries to complete the path in the input buffer.
    ///
    /// Example:
    /// - Lua: `"TryCompletePath"`
    /// - YAML: `TryCompletePath`
    TryCompletePath,

    /// ### Screen -------------------------------------------------------------
    ///
    /// Clear the screen.
    ///
    /// Example:
    ///
    /// - Lua: `"ClearScreen"`
    /// - YAML: `ClearScreen`
    ClearScreen,

    /// Refresh the screen.
    /// But it will not re-explore the directory if the working directory is
    /// the same. If there is some change in the working directory and you want
    /// to re-explore it, use the `Explore` message instead.
    /// Also, it will not clear the screen. Use `ClearScreen` for that.
    ///
    /// Example:
    ///
    /// - Lua: `"Refresh"`
    /// - YAML: `Refresh`
    Refresh,

    /// ### Navigation ---------------------------------------------------------
    ///
    /// Focus next node.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusNext"`
    /// - YAML: `FocusNext`
    FocusNext,

    /// Focus on the next selected node.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusNextSelection"`
    /// - YAML: `FocusNextSelection`
    FocusNextSelection,

    /// Focus on the `n`th node relative to the current focus where `n` is a
    /// given value.
    ///
    /// Type: { FocusNextByRelativeIndex = int }
    ///
    /// Example:
    ///
    /// - Lua: `{ FocusNextByRelativeIndex = 2 }`
    /// - YAML: `FocusNextByRelativeIndex: 2`
    FocusNextByRelativeIndex(usize),

    /// Focus on the `n`th node relative to the current focus where `n` is read
    /// from the input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusNextByRelativeIndexFromInput"`
    /// - YAML: `FocusNextByRelativeIndexFromInput`
    FocusNextByRelativeIndexFromInput,

    /// Focus on the previous item.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusPrevious"`
    /// - YAML: `FocusPrevious`
    FocusPrevious,

    /// Focus on the previous selection item.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusPreviousSelection"`
    /// - YAML: `FocusPreviousSelection`
    FocusPreviousSelection,

    /// Focus on the `-n`th node relative to the current focus where `n` is a
    /// given value.
    ///
    /// Type: { FocusPreviousByRelativeIndex = int }
    ///
    /// Example:
    ///
    /// - Lua: `{ FocusPreviousByRelativeIndex = 2 }`
    /// - YAML: `FocusPreviousByRelativeIndex: 2`
    FocusPreviousByRelativeIndex(usize),

    /// Focus on the `-n`th node relative to the current focus where `n` is
    /// read from the input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusPreviousByRelativeIndexFromInput"`
    /// - YAML: `FocusPreviousByRelativeIndexFromInput`
    FocusPreviousByRelativeIndexFromInput,

    /// Focus on the first node.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusFirst"`
    /// - YAML: `FocusFirst`
    ///
    FocusFirst,

    /// Focus on the last node.
    ///
    /// Example:
    /// - Lua:  `"FocusLast"`
    /// - YAML: `FocusLast`
    FocusLast,

    /// Focus on the given path.
    ///
    /// Type: { FocusPath = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ FocusPath = "/path/to/file" }`
    /// - YAML: `FocusPath: /path/to/file`
    FocusPath(String),

    /// Focus on the path read from input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusPathFromInput"`
    /// - YAML: `FocusPathFromInput`
    FocusPathFromInput,

    /// Focus on the absolute `n`th node where `n` is a given value.
    ///
    /// Type: { FocusByIndex = int }
    ///
    /// Example:
    ///
    /// - Lua: `{ FocusByIndex = 2 }`
    /// - YAML: `FocusByIndex: 2`
    FocusByIndex(usize),

    /// Focus on the absolute `n`th node where `n` is read from the input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"FocusByIndexFromInput"`
    /// - YAML: `FocusByIndexFromInput`
    FocusByIndexFromInput,

    /// Focus on the file by name from the present working directory.
    ///
    /// Type: { FocusByFileName = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ FocusByFileName = "filename.ext" }`
    /// - YAML: `FocusByFileName: filename.ext`
    FocusByFileName(String),

    /// Scroll up by terminal height.
    ///
    /// Example:
    ///
    /// - Lua: `"ScrollUp"`
    /// - YAML: `ScrollUp`
    ScrollUp,

    /// Scroll down by terminal height.
    ///
    /// Example:
    ///
    /// - Lua: `"ScrollDown"`
    /// - YAML: `ScrollDown`
    ScrollDown,

    /// Scroll up by half of terminal height.
    ///
    /// Example:
    ///
    /// - Lua: `"ScrollUpHalf"`
    /// - YAML: `ScrollUpHalf`
    ScrollUpHalf,

    /// Scroll down by half of terminal height.
    ///
    /// Example:
    ///
    /// - Lua: `"ScrollDownHalf"`
    /// - YAML: `ScrollDownHalf`
    ScrollDownHalf,

    /// Change the present working directory ($PWD)
    ///
    /// Type: { ChangeDirectory = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ ChangeDirectory = "/path/to/directory" }`
    /// - YAML: `ChangeDirectory: /path/to/directory`
    ChangeDirectory(String),

    /// Enter into the currently focused path if it's a directory.
    ///
    /// Example:
    ///
    /// - Lua: `"Enter"`
    /// - YAML: `Enter`
    Enter,

    /// Go back to the parent directory.
    ///  
    /// Example:
    ///
    /// - Lua: `"Back"`
    /// - YAML: `Back`
    Back,

    /// Go to the last path visited.
    ///
    /// Example:
    ///
    /// - Lua: `"LastVisitedPath"`
    /// - YAML: `LastVisitedPath`
    LastVisitedPath,

    /// Go to the next path visited.
    ///
    /// Example:
    ///
    /// - Lua: `"NextVisitedPath"`
    /// - YAML: `NextVisitedPath`
    NextVisitedPath,

    /// Go to the previous deep level branch.
    ///
    /// Example:
    ///
    /// - Lua: `"PreviousVisitedDeepBranch"`
    /// - YAML: `PreviousVisitedDeepBranch`
    PreviousVisitedDeepBranch,

    /// Go to the next deep level branch.
    ///
    /// Example:
    ///
    /// - Lua: `"NextVisitedDeepBranch"`
    /// - YAML: `NextVisitedDeepBranch`
    NextVisitedDeepBranch,

    /// Follow the symlink under focus to its actual location.
    ///
    /// Example:
    ///
    /// - Lua: `"FollowSymlink"`
    /// - YAML: `FollowSymlink`
    FollowSymlink,

    /// ### Virtual Root ------------------------------------------------------
    ///
    /// Sets the virtual root for isolating xplr navigation, similar to
    /// `--vroot`, but temporary (can be reset back to initial value).
    /// If the $PWD is outside the vroot, xplr will automatically enter vroot.
    ///
    /// Type: { SetVroot = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SetVroot = "/tmp" }`
    /// - YAML: `SetVroot: /tmp`
    SetVroot(String),

    /// Unset the virtual root temporarily (can be reset back to the initial
    /// value).
    ///
    /// Example:
    ///
    /// - Lua: `"UnsetVroot"`
    /// - YAML: `UnsetVroot`
    UnsetVroot,

    /// Toggle virtual root between unset, initial value and $PWD.
    ///
    /// Example:
    ///
    /// - Lua: `"ToggleVroot"`
    /// - YAML: `ToggleVroot`
    ToggleVroot,

    /// Reset the virtual root back to the initial value.
    ///
    /// Example:
    ///
    /// - Lua: `"ResetVroot"`
    /// - YAML: `ResetVroot`
    ResetVroot,

    /// ### Reading Input -----------------------------------------------------
    ///
    /// Set the input prompt temporarily, until the input buffer is reset.
    ///
    /// Type: { SetInputPrompt = string }
    ///
    /// Example:
    ///
    /// - Lua: `{ SetInputPrompt = "→" }`
    /// - YAML: `SetInputPrompt: →`
    SetInputPrompt(String),

    /// Update the input buffer using cursor based operations.
    ///
    /// Type: { UpdateInputBuffer = [Input Operation](https://xplr.dev/en/input-operation) }
    ///
    /// Example:
    ///
    /// - Lua: `{ UpdateInputBuffer = "GoToPreviousWord" }`
    /// - YAML: `UpdateInputBuffer: GoToPreviousWord`
    UpdateInputBuffer(InputOperation),

    /// Update the input buffer from the key read from keyboard input.
    ///
    /// Example:
    ///
    /// - Lua: `"UpdateInputBufferFromKey"`
    /// - YAML: `UpdateInputBufferFromKey`
    UpdateInputBufferFromKey,

    /// Append/buffer the given string into the input buffer.
    ///
    /// Type: { BufferInput = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ BufferInput = "foo" }`
    /// - YAML: `BufferInput: foo`
    BufferInput(String),

    /// Append/buffer the character read from a keyboard input into the
    /// input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"BufferInputFromKey"`
    /// - YAML: `BufferInputFromKey`
    BufferInputFromKey,

    /// Set/rewrite the input buffer with the given string.
    /// When the input buffer is not-null (even if empty string)
    /// it will show in the UI.
    ///
    /// Type: { SetInputBuffer = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SetInputBuffer = "foo" }`
    /// - YAML: `SetInputBuffer: foo`
    SetInputBuffer(String),

    /// Remove input buffer's last character.
    ///
    ///  Example:
    ///
    ///  - Lua: `"RemoveInputBufferLastCharacter"`
    ///  - YAML: `RemoveInputBufferLastCharacter`
    RemoveInputBufferLastCharacter,

    /// Remove input buffer's last word.
    ///
    /// Example:
    ///
    /// - Lua: `"RemoveInputBufferLastWord"`
    /// - YAML: `RemoveInputBufferLastWord`
    RemoveInputBufferLastWord,

    /// Reset the input buffer back to null. It will not show in the UI.
    ///
    /// Example:
    ///
    /// - Lua: `"ResetInputBuffer"`
    /// - YAML: `ResetInputBuffer`
    ResetInputBuffer,

    /// ### Switching Mode -----------------------------------------------------
    ///
    /// Switch input [mode](https://xplr.dev/en/modes).
    ///
    /// Type : { SwitchMode = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchMode = "default" }`
    /// - YAML: SwitchMode: default
    ///
    /// > **NOTE:** To be specific about which mode to switch to, use
    /// > `SwitchModeBuiltin` or `SwitchModeCustom` instead.
    SwitchMode(String),

    /// Switch input [mode](https://xplr.dev/en/modes).
    /// It keeps the input buffer.
    ///
    /// Type: { SwitchModeKeepingInputBuffer = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchModeKeepingInputBuffer = "default" }`
    /// - YAML: `SwitchModeKeepingInputBuffer: default`
    ///
    /// > **NOTE:** To be specific about which mode to switch to, use
    /// > `SwitchModeBuiltinKeepingInputBuffer` or
    /// > `SwitchModeCustomKeepingInputBuffer` instead.
    SwitchModeKeepingInputBuffer(String),

    /// Switch to a [builtin mode](https://xplr.dev/en/modes#builtin).
    /// It clears the input buffer.
    ///
    /// Type: { SwitchModeBuiltin = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchModeBuiltin = "default" }`
    /// - YAML: `SwitchModeBuiltin: default`
    SwitchModeBuiltin(String),

    /// Switch to a [builtin mode](https://xplr.dev/en/modes#builtin).
    /// It keeps the input buffer.
    ///
    /// Type: { SwitchModeBuiltinKeepingInputBuffer = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchModeBuiltinKeepingInputBuffer = "default" }`
    /// - YAML: `SwitchModeBuiltinKeepingInputBuffer: default`
    SwitchModeBuiltinKeepingInputBuffer(String),

    /// Switch to a [custom mode](https://xplr.dev/en/modes#custom).
    /// It clears the input buffer.
    ///
    /// Type: { SwitchModeCustom = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchModeCustom = "my_custom_mode" }`
    /// - YAML: `SwitchModeCustom: my_custom_mode`
    SwitchModeCustom(String),

    /// Switch to a [custom mode](https://xplr.dev/en/modes#custom).
    /// It keeps the input buffer.
    ///
    /// Type: { SwitchModeCustomKeepingInputBuffer = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchModeCustomKeepingInputBuffer = "my_custom_mode" }`
    /// - YAML: `SwitchModeCustomKeepingInputBuffer: my_custom_mode`
    SwitchModeCustomKeepingInputBuffer(String),

    /// Pop the last mode from the history and switch to it.
    /// It clears the input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"PopMode"`
    /// - YAML: `PopMode`
    PopMode,

    /// Pop the last mode from the history and switch to it.
    /// It keeps the input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `PopModeKeepingInputBuffer`
    /// - YAML: `PopModeKeepingInputBuffer`
    PopModeKeepingInputBuffer,

    /// ### Switching Layout ---------------------------------------------------
    ///
    /// Switch [layout](https://xplr.dev/en/layouts).
    ///
    /// Type: { SwitchLayout = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchLayout = "default" }`
    /// - YAML: `SwitchLayout: default`
    ///
    /// > **NOTE:** To be specific about which layout to switch to, use `SwitchLayoutBuiltin` or
    /// > `SwitchLayoutCustom` instead.
    SwitchLayout(String),

    /// Switch to a [builtin layout](https://xplr.dev/en/layouts#builtin).
    ///
    /// Type: { SwitchLayoutBuiltin = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchLayoutBuiltin = "default" }`
    /// - YAML: `SwitchLayoutBuiltin: default`
    SwitchLayoutBuiltin(String),

    /// Switch to a [custom layout](https://xplr.dev/en/layouts#custom).
    ///
    /// Type: { SwitchLayoutCustom = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SwitchLayoutCustom = "my_custom_layout" }`
    /// - YAML: `SwitchLayoutCustom: my_custom_layout`
    SwitchLayoutCustom(String),

    /// ### Executing Commands ------------------------------------------------
    ///
    /// Like `Call0`, but it uses `\n` as the delimiter in input/output pipes,
    /// hence it cannot handle files with `\n` in the name.
    /// You may want to use `Call0` instead.
    Call(Command),

    /// Call a shell command with the given arguments.
    /// Note that the arguments will be shell-escaped.
    /// So to read the variables, the `-c` option of the shell
    /// can be used.
    /// You may need to pass `ExplorePwd` depending on the expectation.
    ///
    /// Type: { Call0 = { command = "string", args = { "list", "of", "string" } }
    ///
    /// Example:
    ///
    /// - Lua: `{ Call0 = { command = "bash", args = { "-c", "read -p test" } } }`
    /// - YAML: `Call0: { command: bash, args: ["-c", "read -p test"] }`
    Call0(Command),

    /// Like `CallSilently0`, but it uses `\n` as the delimiter in input/output
    /// pipes, hence it cannot handle files with `\n` in the name.
    /// You may want to use `CallSilently0` instead.
    CallSilently(Command),

    /// Like `Call0` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// Type: { CallSilently0 = { command = "string", args = {"list", "of", "string"} } }
    ///
    /// Example:
    ///
    /// - Lua: `{ CallSilently0 = { command = "tput", args = { "bell" } } }`
    /// - YAML: `CallSilently0: { command: tput, args: ["bell"] }`
    CallSilently0(Command),

    /// Like `BashExec0`, but it uses `\n` as the delimiter in input/output
    /// pipes, hence it cannot handle files with `\n` in the name.
    /// You may want to use `BashExec0` instead.
    BashExec(String),

    /// An alias to `Call: {command: bash, args: ["-c", "{string}"], silent: false}`
    /// where `{string}` is the given value.
    ///
    /// Type: { BashExec0 = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ BashExec0 = "read -p test" }`
    /// - YAML: `BashExec0: "read -p test"`
    BashExec0(String),

    /// Like `BashExecSilently0`, but it uses `\n` as the delimiter in
    /// input/output pipes, hence it cannot handle files with `\n` in the name.
    /// You may want to use `BashExecSilently0` instead.
    BashExecSilently(String),

    /// Like `BashExec0` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// Type: { BashExecSilently0 = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ BashExecSilently0 = "tput bell" }`
    /// - YAML: `BashExecSilently0: "tput bell"`
    BashExecSilently0(String),

    /// ### Calling Lua Functions ----------------------------------------------
    ///
    /// Call a Lua function.
    ///
    /// A [Lua Context](https://xplr.dev/en/lua-function-calls#lua-context)
    /// object will be passed to the function as argument.
    /// The function can optionally return a list of messages for xplr to
    /// handle after the executing the function.
    ///
    /// Type: { CallLua = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ CallLua = "custom.some_custom_funtion" }`
    /// - YAML: `CallLua: custom.some_custom_funtion`
    CallLua(String),

    /// Like `CallLua` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// Type: { CallLuaSilently = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ CallLuaSilently = "custom.some_custom_function" }`
    /// - YAML: `CallLuaSilently: custom.some_custom_function`
    CallLuaSilently(String),

    /// Execute Lua code without needing to define a function.
    ///
    /// If the `string` is a callable, xplr will try to call it with with the
    /// [Lua Context](https://xplr.dev/en/lua-function-calls#lua-context)
    /// argument.
    ///
    /// Type: { LuaEval = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ LuaEval = [[return { { LogInfo = io.read() } }]] }`
    /// - Lua: `{ LuaEval = [[function(app) return { { LogInfo = app.pwd } } end]] }`
    /// - YAML: `LuaEval: "return { { LogInfo = io.read() } }"`
    /// - YAML: `LuaEval: "function(app) return { { LogInfo = app.pwd } } end"`
    LuaEval(String),

    /// Like `LuaEval` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// Type: { LuaEvalSilently = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ LuaEvalSilently = [[return { { LogInfo = "foo" } }]] }`
    /// - YAML: `LuaEvalSilently: "return { { LogInfo = 'foo' } }"`
    LuaEvalSilently(String),

    /// ### Select Operations --------------------------------------------------
    ///
    /// Select the focused node.
    ///
    /// Example:
    ///
    /// - Lua: `"Select"`
    /// - YAML: `Select`
    Select,

    /// Select all the visible nodes.
    ///
    /// Example:
    ///
    /// - Lua: `"SelectAll"`
    /// - YAML: `SelectAll`
    SelectAll,

    /// Select the given path.
    ///
    /// Type: { SelectPath = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SelectPath = "/path/to/file" }`
    /// - YAML: `SelectPath: /path/to/file`
    SelectPath(String),

    /// Unselect the focused node.
    ///
    /// Example:
    ///
    /// - Lua: `"UnSelect"`
    /// - YAML: `UnSelect`
    UnSelect,

    /// Unselect all the visible nodes.
    ///
    /// Example:
    ///
    /// - Lua: `"UnSelectAll"`
    /// - YAML: `UnSelectAll`
    UnSelectAll,

    /// UnSelect the given path.
    ///
    /// Type: { UnSelectPath = "string)" }
    ///
    /// Example:
    ///
    /// - Lua: `{ UnSelectPath = "/path/to/file" }`
    /// - YAML: `UnSelectPath: /path/to/file`
    UnSelectPath(String),

    /// Toggle selection on the focused node.
    ///
    /// Example:
    ///
    /// - Lua: `"ToggleSelection"`
    /// - YAML `ToggleSelection`
    ToggleSelection,

    /// Toggle between select all and unselect all.
    /// Example:
    ///
    /// - Lua: `"ToggleSelectAll"`
    /// - YAML: `ToggleSelectAll`
    ToggleSelectAll,

    /// Toggle selection by file path.
    ///
    /// Type: { ToggleSelectionByPath = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ ToggleSelectionByPath = "/path/to/file" }`
    /// - YAML: `ToggleSelectionByPath: /path/to/file`
    ToggleSelectionByPath(String),

    /// Clear the selection.
    ///
    /// Example:
    ///
    /// - Lua: `"ClearSelection"`
    /// - YAML: `ClearSelection`
    ClearSelection,

    /// ### Filter Operations --------------------------------------------------
    ///
    /// Add a [filter](https://xplr.dev/en/filtering#filter) to exclude nodes
    /// while exploring directories.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// Filters get automatically cleared when changing directories.
    ///
    /// Type: { AddNodeFilter = { filter = [Filter](https://xplr.dev/en/filtering#filter), input = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ AddNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" } }`
    /// - YAML: `AddNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`
    AddNodeFilter(NodeFilterApplicable),

    /// Remove an existing [filter](https://xplr.dev/en/filtering#filter).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { RemoveNodeFilter = { filter = [Filter](https://xplr.dev/en/filtering), input = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ RemoveNodeFilter: { filter: "RelativePathDoesStartWith", input: "foo" } }`
    /// - YAML: `RemoveNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`
    RemoveNodeFilter(NodeFilterApplicable),

    /// Remove a [filter](https://xplr.dev/en/filtering#filter) if it exists,
    /// else, add a it.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { ToggleNodeFilter = { filter = [Filter](https://xplr.dev/en/filtering), input = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ ToggleNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" } }`
    /// - YAML: `ToggleNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`
    ToggleNodeFilter(NodeFilterApplicable),

    /// Add a node [filter](https://xplr.dev/en/filtering#filter) reading the
    /// input from the buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { AddNodeFilterFromInput = [Filter](https://xplr.dev/en/filtering) }
    ///
    /// Example:
    ///
    /// - Lua: `{ AddNodeFilterFromInput = "RelativePathDoesStartWith" }`
    /// - YAML: `AddNodeFilterFromInput: RelativePathDoesStartWith`
    AddNodeFilterFromInput(NodeFilter),

    /// Remove a node [filter](https://xplr.dev/en/filtering#filter) reading
    /// the input from the buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { RemoveNodeFilterFromInput = [Filter](https://xplr.dev/en/filtering) }
    ///
    /// Example:
    ///
    /// - Lua: `{ RemoveNodeFilterFromInput = "RelativePathDoesStartWith" }`
    /// - YAML: `RemoveNodeFilterFromInput: RelativePathDoesStartWith`
    RemoveNodeFilterFromInput(NodeFilter),

    /// Remove the last node [filter](https://xplr.dev/en/filtering).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"RemoveLastNodeFilter"`
    /// - YAML: `RemoveLastNodeFilter`
    RemoveLastNodeFilter,

    /// Reset the node [filters](https://xplr.dev/en/filtering) back to the
    /// default configuration.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"ResetNodeFilters"`
    /// - YAML: `ResetNodeFilters`
    ResetNodeFilters,

    /// Clear all the node [filters](https://xplr.dev/en/filtering).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"ClearNodeFilters"`
    /// - YAML: `ClearNodeFilters`
    ClearNodeFilters,

    /// ### Sort Operations ----------------------------------------------------
    ///
    /// Add a [sorter](https://xplr.dev/en/sorting#sorter) to sort nodes while
    /// exploring directories.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { AddNodeSorter = { sorter = [Sorter](https://xplr.dev/en/sorting#sorter), reverse = bool } }
    ///
    /// Example:
    ///
    /// - Lua: `{ AddNodeSorter = { sorter = "ByRelativePath", reverse = false } }`
    /// - YAML: `AddNodeSorter: { sorter: ByRelativePath, reverse: false }`
    AddNodeSorter(NodeSorterApplicable),

    /// Remove an existing [sorter](https://xplr.dev/en/sorting#sorter).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { RemoveNodeSorter = [Sorter](https://xplr.dev/en/sorting#sorter) }
    ///
    /// Example:
    ///
    /// - Lua: `{ RemoveNodeSorter = "ByRelativePath" }`
    /// - YAML: `RemoveNodeSorter: ByRelativePath`
    RemoveNodeSorter(NodeSorter),

    /// Reverse a node [sorter](https://xplr.dev/en/sorting#sorter).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { ReverseNodeSorter = [Sorter](https://xplr.dev/en/sorting#sorter) }
    ///
    /// Example:
    ///
    /// - Lua: `{ ReverseNodeSorter = "ByRelativePath" }`
    /// - YAML: `ReverseNodeSorter: ByRelativePath`
    ReverseNodeSorter(NodeSorter),

    /// Remove a [sorter](https://xplr.dev/en/sorting#sorter) if it exists,
    /// else, add a it.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Type: { ToggleNodeSorter = { sorter = [Sorter](https://xplr.dev/en/sorting#sorter), reverse = bool } }
    ///
    /// Example:
    ///
    /// - Lua: `{ ToggleSorterSorter: { sorter = "ByRelativePath", reverse = false } }`
    /// - YAML: `ToggleSorterSorter: {sorter: ByRelativePath, reverse: false }`
    ToggleNodeSorter(NodeSorterApplicable),

    /// Reverse the node [sorters](https://xplr.dev/en/sorting#sorter).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"ReverseNodeSorters"`
    /// - YAML: `ReverseNodeSorters`
    ReverseNodeSorters,

    /// Remove the last node [sorter](https://xplr.dev/en/sorting#sorter).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"RemoveLastNodeSorter"`
    /// - YAML: `RemoveLastNodeSorter`
    RemoveLastNodeSorter,

    /// Reset the node [sorters](https://xplr.dev/en/sorting#sorter) back to
    /// the default configuration.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"ResetNodeSorters"`
    /// - YAML: `ResetNodeSorters`
    ResetNodeSorters,

    /// Clear all the node [sorters](https://xplr.dev/en/sorting#sorter).
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"ClearNodeSorters"`
    /// - YAML: `ClearNodeSorters`
    ClearNodeSorters,

    /// ### Search Operations --------------------------------------------------
    ///
    /// Search files using the current or default (fuzzy) search algorithm.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Type: { Search = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ Search = "pattern" }`
    /// - YAML: `Search: pattern`
    Search(String),

    /// Calls `Search` with the input taken from the input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"SearchFromInput"`
    /// - YAML: `SearchFromInput`
    SearchFromInput,

    /// Search files using fuzzy match algorithm.
    /// It keeps the filters, but overrides the sorters.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Type: { SearchFuzzy = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SearchFuzzy = "pattern" }`
    /// - YAML: `SearchFuzzy: pattern`
    SearchFuzzy(String),

    /// Calls `SearchFuzzy` with the input taken from the input buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Example:
    ///
    /// - Lua: `"SearchFuzzyFromInput"`
    /// - YAML: `SearchFuzzyFromInput`
    SearchFuzzyFromInput,

    /// Like `SearchFuzzy`, but doesn't not perform rank based sorting.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Type: { SearchFuzzyUnordered = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SearchFuzzyUnordered = "pattern" }`
    /// - YAML: `SearchFuzzyUnordered: pattern`
    SearchFuzzyUnordered(String),

    /// Calls `SearchFuzzyUnordered` with the input taken from the input buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Example:
    ///
    /// - Lua: `"SearchFuzzyUnorderedFromInput"`
    /// - YAML: `SearchFuzzyUnorderedFromInput`
    SearchFuzzyUnorderedFromInput,

    /// Search files using regex match algorithm.
    /// It keeps the filters, but overrides the sorters.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Type: { SearchRegex = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SearchRegex = "pattern" }`
    /// - YAML: `SearchRegex: pattern`
    SearchRegex(String),

    /// Calls `SearchRegex` with the input taken from the input buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Example:
    ///
    /// - Lua: `"SearchRegexFromInput"`
    /// - YAML: `SearchRegexFromInput`
    SearchRegexFromInput,

    /// Like `SearchRegex`, but doesn't not perform rank based sorting.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Type: { SearchRegexUnordered = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ SearchRegexUnordered = "pattern" }`
    /// - YAML: `SearchRegexUnordered: pattern`
    SearchRegexUnordered(String),

    /// Calls `SearchRegexUnordered` with the input taken from the input buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    /// It gets reset automatically when changing directory.
    ///
    /// Example:
    ///
    /// - Lua: `"SearchRegexUnorderedFromInput"`
    /// - YAML: `SearchRegexUnorderedFromInput`
    SearchRegexUnorderedFromInput,

    /// Toggles between different search algorithms, without changing the input
    /// buffer
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"ToggleSearchAlgorithm"`
    /// - YAML: `ToggleSearchAlgorithm`
    ToggleSearchAlgorithm,

    /// Enables ranked search without changing the input buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"EnableOrderedSearch"`
    /// - YAML: `EnableSearchOrder`
    EnableSearchOrder,

    /// Disabled ranked search without changing the input buffer.
    /// You need to call `ExplorePwd` or `ExplorePwdAsync` explicitly.
    ///
    /// Example:
    ///
    /// - Lua: `"DisableSearchOrder"`
    /// - YAML: `DisableSearchOrder`
    DisableSearchOrder,

    /// Toggles ranked search without changing the input buffer.
    ///
    /// Example:
    ///
    /// - Lua: `"ToggleSearchOrder"`
    /// - YAML: `ToggleSearchOrder`
    ToggleSearchOrder,

    /// Accepts the search by keeping the latest focus while in search mode.
    /// Automatically calls `ExplorePwd`.
    ///
    /// Example:
    ///
    /// - Lua: `"AcceptSearch"`
    /// - YAML: `AcceptSearch`
    AcceptSearch,

    /// Cancels the search by discarding the latest focus and recovering
    /// the focus before search.
    /// Automatically calls `ExplorePwd`.
    ///
    /// Example:
    ///
    /// - Lua: `"CancelSearch"`
    /// - YAML: `CancelSearch`
    CancelSearch,

    /// ### Mouse Operations ---------------------------------------------------
    ///
    /// Enable mouse
    ///
    /// Example:
    ///
    /// - Lua: `"EnableMouse"`
    /// - YAML: `EnableMouse`
    EnableMouse,

    /// Disable mouse
    ///
    /// Example:
    ///
    /// - Lua: `"DisableMouse"`
    /// - YAML: `DisableMouse`
    DisableMouse,

    /// Toggle mouse
    ///
    /// Example:
    ///
    /// - Lua: `"ToggleMouse"`
    /// - YAML: `ToggleMouse`
    ToggleMouse,

    /// ### Fifo Operations ----------------------------------------------------
    ///
    /// Start piping the focused path to the given fifo path
    ///
    /// Type: { StartFifo = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ StartFifo = "/tmp/xplr.fifo }`
    /// - YAML: `StartFifo: /tmp/xplr.fifo`
    StartFifo(String),

    /// Close the active fifo and stop piping.
    ///
    /// Example:
    ///
    /// - Lua: `"StopFifo"`
    /// - YAML: `StopFifo`
    StopFifo,

    /// Toggle between {Start|Stop}Fifo
    ///
    /// Type: { ToggleFifo = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ ToggleFifo = "/path/to/fifo" }`
    /// - YAML: `ToggleFifo: /path/to/fifo`
    ToggleFifo(String),

    /// ### Logging ------------------------------------------------------------
    ///
    /// Log information message.
    ///
    /// Type: { LogInfo = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ LogInfo = "launching satellite" }`
    /// - YAML: `LogInfo: launching satellite`
    LogInfo(String),

    /// Log a success message.
    ///
    /// Type: { LogSuccess = "String" }
    ///
    /// Example:
    ///
    /// - Lua: `{ LogSuccess = "satellite reached destination" }`
    /// - YAML: `LogSuccess: satellite reached destination`
    LogSuccess(String),

    /// Log an warning message.
    ///
    /// Type: { LogWarning = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ LogWarning = "satellite is heating" }`
    /// - YAML: `LogWarning: satellite is heating`
    LogWarning(String),

    /// Log an error message.
    ///
    /// Type: { LogError = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ LogError = "satellite crashed" }`
    /// - YAML: `LogError: satellite crashed`
    LogError(String),

    /// ### Debugging ----------------------------------------------------------
    ///
    /// Write the application state to a file, without quitting. Also helpful
    /// for debugging.
    ///
    /// Type: { Debug = "string" }
    ///
    /// Example:
    ///
    /// - Lua: `{ Debug = "/path/to/file" }`
    /// - YAML: `Debug: /path/to/file`
    Debug(String),

    /// ### Quit Options -------------------------------------------------------
    ///
    /// Example:
    ///
    /// - Lua: `"Quit"`
    /// - YAML: `Quit`
    ///
    /// Quit with returncode zero (success).
    Quit,

    /// Print $PWD and quit.
    ///
    /// Example:
    ///
    /// - Lua: `"PrintPwdAndQuit"`
    /// - YAML: `PrintPwdAndQuit`
    PrintPwdAndQuit,

    /// Print the path under focus and quit. It can be empty string if there's
    /// nothing to focus.
    ///
    /// Example:
    ///
    /// - Lua: `"PrintFocusPathAndQuit"`
    /// - YAML: `PrintFocusPathAndQuit`
    PrintFocusPathAndQuit,

    /// Print the selected paths and quit. It can be empty is no path is
    /// selected.
    ///
    /// Example:
    ///
    /// - Lua: `"PrintSelectionAndQuit"`
    /// - YAML: `PrintSelectionAndQuit`
    PrintSelectionAndQuit,

    /// Print the selected paths if it's not empty, else, print the focused
    /// node's path.
    ///
    /// Example:
    ///
    /// - Lua: `"PrintResultAndQuit"`
    /// - YAML: `PrintResultAndQuit`
    PrintResultAndQuit,

    /// Print the state of application in YAML format. Helpful for debugging or
    /// generating the default configuration file.
    ///
    /// Example:
    ///
    /// - Lua: `"PrintAppStateAndQuit"`
    /// - YAML: `PrintAppStateAndQuit`
    PrintAppStateAndQuit,

    /// Terminate the application with a non-zero return code.
    ///
    /// Example:
    ///
    /// - Lua: `"Terminate"`
    /// - YAML: `Terminate`
    Terminate,
}

impl ExternalMsg {
    pub fn is_read_only(&self) -> bool {
        !matches!(
            self,
            Self::Call(_)
                | Self::Call0(_)
                | Self::CallSilently(_)
                | Self::CallSilently0(_)
                | Self::BashExec(_)
                | Self::BashExec0(_)
                | Self::BashExecSilently(_)
                | Self::BashExecSilently0(_)
                | Self::CallLua(_)
                | Self::CallLuaSilently(_)
                | Self::LuaEval(_)
                | Self::LuaEvalSilently(_)
        )
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum NodeSorter {
    ByRelativePath,
    ByIRelativePath,
    ByExtension,
    ByIsDir,
    ByIsFile,
    ByIsSymlink,
    ByIsBroken,
    ByIsReadonly,
    ByMimeEssence,
    BySize,
    ByCreated,
    ByLastModified,

    ByCanonicalAbsolutePath,
    ByICanonicalAbsolutePath,
    ByCanonicalExtension,
    ByCanonicalIsDir,
    ByCanonicalIsFile,
    ByCanonicalIsReadonly,
    ByCanonicalMimeEssence,
    ByCanonicalSize,
    ByCanonicalCreated,
    ByCanonicalLastModified,

    BySymlinkAbsolutePath,
    ByISymlinkAbsolutePath,
    BySymlinkExtension,
    BySymlinkIsDir,
    BySymlinkIsFile,
    BySymlinkIsReadonly,
    BySymlinkMimeEssence,
    BySymlinkSize,
    BySymlinkCreated,
    BySymlinkLastModified,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeSorterApplicable {
    pub sorter: NodeSorter,
    #[serde(default)]
    pub reverse: bool,
}

impl PartialEq for NodeSorterApplicable {
    fn eq(&self, other: &NodeSorterApplicable) -> bool {
        self.sorter == other.sorter
    }
}

impl std::hash::Hash for NodeSorterApplicable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sorter.hash(state);
    }
}

impl NodeSorterApplicable {
    pub fn reversed(mut self) -> Self {
        self.reverse = !self.reverse;
        self
    }

    fn apply(&self, a: &Node, b: &Node) -> Ordering {
        let order = match self.sorter {
            NodeSorter::ByRelativePath => {
                natord::compare(&a.relative_path, &b.relative_path)
            }
            NodeSorter::ByIRelativePath => {
                natord::compare_ignore_case(&a.relative_path, &b.relative_path)
            }
            NodeSorter::ByExtension => a.extension.cmp(&b.extension),
            NodeSorter::ByIsDir => a.is_dir.cmp(&b.is_dir),
            NodeSorter::ByIsFile => a.is_file.cmp(&b.is_file),
            NodeSorter::ByIsSymlink => a.is_symlink.cmp(&b.is_symlink),
            NodeSorter::ByIsBroken => a.is_broken.cmp(&b.is_broken),
            NodeSorter::ByIsReadonly => a.is_readonly.cmp(&b.is_readonly),
            NodeSorter::ByMimeEssence => a.mime_essence.cmp(&b.mime_essence),
            NodeSorter::BySize => a.size.cmp(&b.size),
            NodeSorter::ByCreated => a.created.cmp(&b.created),
            NodeSorter::ByLastModified => a.last_modified.cmp(&b.last_modified),

            NodeSorter::ByCanonicalAbsolutePath => natord::compare(
                &a.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            NodeSorter::ByICanonicalAbsolutePath => natord::compare_ignore_case(
                &a.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            NodeSorter::ByCanonicalExtension => a
                .canonical
                .as_ref()
                .map(|s| &s.extension)
                .cmp(&b.canonical.as_ref().map(|s| &s.extension)),

            NodeSorter::ByCanonicalIsDir => a
                .canonical
                .as_ref()
                .map(|s| &s.is_dir)
                .cmp(&b.canonical.as_ref().map(|s| &s.is_dir)),

            NodeSorter::ByCanonicalIsFile => a
                .canonical
                .as_ref()
                .map(|s| &s.is_file)
                .cmp(&b.canonical.as_ref().map(|s| &s.is_file)),

            NodeSorter::ByCanonicalIsReadonly => a
                .canonical
                .as_ref()
                .map(|s| &s.is_readonly)
                .cmp(&b.canonical.as_ref().map(|s| &s.is_readonly)),

            NodeSorter::ByCanonicalMimeEssence => a
                .canonical
                .as_ref()
                .map(|s| &s.mime_essence)
                .cmp(&b.canonical.as_ref().map(|s| &s.mime_essence)),

            NodeSorter::ByCanonicalSize => a
                .canonical
                .as_ref()
                .map(|s| &s.size)
                .cmp(&b.canonical.as_ref().map(|s| &s.size)),

            NodeSorter::ByCanonicalCreated => a
                .canonical
                .as_ref()
                .map(|s| &s.created)
                .cmp(&b.canonical.as_ref().map(|s| &s.created)),

            NodeSorter::ByCanonicalLastModified => a
                .canonical
                .as_ref()
                .map(|s| &s.last_modified)
                .cmp(&b.canonical.as_ref().map(|s| &s.last_modified)),

            NodeSorter::BySymlinkAbsolutePath => natord::compare(
                &a.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            NodeSorter::ByISymlinkAbsolutePath => natord::compare_ignore_case(
                &a.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            NodeSorter::BySymlinkExtension => a
                .symlink
                .as_ref()
                .map(|s| &s.extension)
                .cmp(&b.symlink.as_ref().map(|s| &s.extension)),

            NodeSorter::BySymlinkIsDir => a
                .symlink
                .as_ref()
                .map(|s| &s.is_dir)
                .cmp(&b.symlink.as_ref().map(|s| &s.is_dir)),

            NodeSorter::BySymlinkIsFile => a
                .symlink
                .as_ref()
                .map(|s| &s.is_file)
                .cmp(&b.symlink.as_ref().map(|s| &s.is_file)),

            NodeSorter::BySymlinkIsReadonly => a
                .symlink
                .as_ref()
                .map(|s| &s.is_readonly)
                .cmp(&b.symlink.as_ref().map(|s| &s.is_readonly)),

            NodeSorter::BySymlinkMimeEssence => a
                .symlink
                .as_ref()
                .map(|s| &s.mime_essence)
                .cmp(&b.symlink.as_ref().map(|s| &s.mime_essence)),

            NodeSorter::BySymlinkSize => a
                .symlink
                .as_ref()
                .map(|s| &s.size)
                .cmp(&b.symlink.as_ref().map(|s| &s.size)),

            NodeSorter::BySymlinkCreated => a
                .symlink
                .as_ref()
                .map(|s| &s.created)
                .cmp(&b.symlink.as_ref().map(|s| &s.created)),

            NodeSorter::BySymlinkLastModified => a
                .symlink
                .as_ref()
                .map(|s| &s.last_modified)
                .cmp(&b.symlink.as_ref().map(|s| &s.last_modified)),
        };
        if self.reverse {
            order.reverse()
        } else {
            order
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum NodeFilter {
    RelativePathIs,
    RelativePathIsNot,

    IRelativePathIs,
    IRelativePathIsNot,

    RelativePathDoesStartWith,
    RelativePathDoesNotStartWith,

    IRelativePathDoesStartWith,
    IRelativePathDoesNotStartWith,

    RelativePathDoesContain,
    RelativePathDoesNotContain,

    IRelativePathDoesContain,
    IRelativePathDoesNotContain,

    RelativePathDoesEndWith,
    RelativePathDoesNotEndWith,

    IRelativePathDoesEndWith,
    IRelativePathDoesNotEndWith,

    RelativePathDoesMatchRegex,
    RelativePathDoesNotMatchRegex,

    IRelativePathDoesMatchRegex,
    IRelativePathDoesNotMatchRegex,

    AbsolutePathIs,
    AbsolutePathIsNot,

    IAbsolutePathIs,
    IAbsolutePathIsNot,

    AbsolutePathDoesStartWith,
    AbsolutePathDoesNotStartWith,

    IAbsolutePathDoesStartWith,
    IAbsolutePathDoesNotStartWith,

    AbsolutePathDoesContain,
    AbsolutePathDoesNotContain,

    IAbsolutePathDoesContain,
    IAbsolutePathDoesNotContain,

    AbsolutePathDoesEndWith,
    AbsolutePathDoesNotEndWith,

    IAbsolutePathDoesEndWith,
    IAbsolutePathDoesNotEndWith,

    AbsolutePathDoesMatchRegex,
    AbsolutePathDoesNotMatchRegex,

    IAbsolutePathDoesMatchRegex,
    IAbsolutePathDoesNotMatchRegex,
}

impl NodeFilter {
    fn apply(&self, node: &Node, input: &str, regex: Option<&Regex>) -> bool {
        match self {
            Self::RelativePathIs => node.relative_path.eq(input),
            Self::IRelativePathIs => node.relative_path.eq_ignore_ascii_case(input),

            Self::RelativePathIsNot => !node.relative_path.eq(input),
            Self::IRelativePathIsNot => !node.relative_path.eq_ignore_ascii_case(input),

            Self::RelativePathDoesStartWith => node.relative_path.starts_with(input),
            Self::IRelativePathDoesStartWith => node
                .relative_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::RelativePathDoesNotStartWith => !node.relative_path.starts_with(input),
            Self::IRelativePathDoesNotStartWith => !node
                .relative_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::RelativePathDoesContain => node.relative_path.contains(input),
            Self::IRelativePathDoesContain => node
                .relative_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::RelativePathDoesNotContain => !node.relative_path.contains(input),
            Self::IRelativePathDoesNotContain => !node
                .relative_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::RelativePathDoesEndWith => node.relative_path.ends_with(input),
            Self::IRelativePathDoesEndWith => node
                .relative_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),

            Self::RelativePathDoesNotEndWith => !node.relative_path.ends_with(input),
            Self::IRelativePathDoesNotEndWith => !node
                .relative_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),

            Self::RelativePathDoesMatchRegex => regex
                .map(|r| r.is_match(&node.relative_path))
                .unwrap_or(false),
            Self::IRelativePathDoesMatchRegex => regex
                .map(|r| r.is_match(&node.relative_path.to_lowercase()))
                .unwrap_or(false),

            Self::RelativePathDoesNotMatchRegex => !regex
                .map(|r| r.is_match(&node.relative_path))
                .unwrap_or(false),
            Self::IRelativePathDoesNotMatchRegex => !regex
                .map(|r| r.is_match(&node.relative_path.to_lowercase()))
                .unwrap_or(false),

            Self::AbsolutePathIs => node.absolute_path.eq(input),
            Self::IAbsolutePathIs => node.absolute_path.eq_ignore_ascii_case(input),

            Self::AbsolutePathIsNot => !node.absolute_path.eq(input),
            Self::IAbsolutePathIsNot => !node.absolute_path.eq_ignore_ascii_case(input),

            Self::AbsolutePathDoesStartWith => node.absolute_path.starts_with(input),
            Self::IAbsolutePathDoesStartWith => node
                .absolute_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::AbsolutePathDoesNotStartWith => !node.absolute_path.starts_with(input),
            Self::IAbsolutePathDoesNotStartWith => !node
                .absolute_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::AbsolutePathDoesContain => node.absolute_path.contains(input),
            Self::IAbsolutePathDoesContain => node
                .absolute_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::AbsolutePathDoesNotContain => !node.absolute_path.contains(input),
            Self::IAbsolutePathDoesNotContain => !node
                .absolute_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::AbsolutePathDoesEndWith => node.absolute_path.ends_with(input),
            Self::IAbsolutePathDoesEndWith => node
                .absolute_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),

            Self::AbsolutePathDoesNotEndWith => !node.absolute_path.ends_with(input),
            Self::IAbsolutePathDoesNotEndWith => !node
                .absolute_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),

            Self::AbsolutePathDoesMatchRegex => regex
                .map(|r| r.is_match(&node.absolute_path))
                .unwrap_or(false),
            Self::IAbsolutePathDoesMatchRegex => regex
                .map(|r| r.is_match(&node.absolute_path.to_lowercase()))
                .unwrap_or(false),

            Self::AbsolutePathDoesNotMatchRegex => !regex
                .map(|r| r.is_match(&node.absolute_path))
                .unwrap_or(false),
            Self::IAbsolutePathDoesNotMatchRegex => !regex
                .map(|r| r.is_match(&node.absolute_path.to_lowercase()))
                .unwrap_or(false),
        }
    }
}

// Regex doesn't implement Hash and Eq. But we need them.

#[derive(Debug, Clone)]
pub struct CmpRegex(Regex);

impl std::hash::Hash for CmpRegex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state)
    }
}

impl PartialEq for CmpRegex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for CmpRegex {}

// See https://github.com/sayanarijit/xplr/issues/503
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NodeFilterApplicableDeserializer {
    pub filter: NodeFilter,
    pub input: String,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, from = "NodeFilterApplicableDeserializer")]
pub struct NodeFilterApplicable {
    pub filter: NodeFilter,
    pub input: String,

    #[serde(skip)]
    pub regex: Option<CmpRegex>,
}

impl From<NodeFilterApplicableDeserializer> for NodeFilterApplicable {
    fn from(f: NodeFilterApplicableDeserializer) -> Self {
        NodeFilterApplicable::new(f.filter, f.input)
    }
}

impl NodeFilterApplicable {
    pub fn new(filter: NodeFilter, input: String) -> Self {
        use NodeFilter::*;

        let regex = if matches!(
            filter,
            RelativePathDoesMatchRegex
                | RelativePathDoesNotMatchRegex
                | AbsolutePathDoesMatchRegex
                | AbsolutePathDoesNotMatchRegex
        ) {
            Regex::new(&input).ok().map(CmpRegex)
        } else if matches!(
            filter,
            IRelativePathDoesMatchRegex
                | IRelativePathDoesNotMatchRegex
                | IAbsolutePathDoesMatchRegex
                | IAbsolutePathDoesNotMatchRegex
        ) {
            Regex::new(&input.to_lowercase()).ok().map(CmpRegex)
        } else {
            None
        };

        Self {
            filter,
            input,
            regex,
        }
    }

    fn apply(&self, node: &Node) -> bool {
        self.filter
            .apply(node, &self.input, self.regex.as_ref().map(|r| &r.0))
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct NodeSearcherApplicable {
    pub pattern: String,

    #[serde(default)]
    pub recoverable_focus: Option<String>,

    #[serde(default)]
    pub algorithm: SearchAlgorithm,

    #[serde(default)]
    pub unordered: bool,

    #[serde(default)]
    pub exact_mode: bool,

    #[serde(default)]
    pub rank_criteria: Option<Vec<RankCriteria>>,
}

impl NodeSearcherApplicable {
    pub fn new(
        pattern: String,
        recoverable_focus: Option<String>,
        algorithm: SearchAlgorithm,
        unordered: bool,
        exact_mode: bool,
        rank_criteria: Option<Vec<RankCriteria>>,
    ) -> Self {
        Self {
            pattern,
            recoverable_focus,
            algorithm,
            unordered,
            exact_mode,
            rank_criteria,
        }
    }

    pub fn search<I>(&self, nodes: I) -> Vec<Node>
    where
        I: IntoIterator<Item = Node>,
    {
        let engine = self.algorithm.engine(
            &self.pattern,
            self.exact_mode,
            self.rank_criteria.clone(),
        );
        let ranked_nodes = nodes.into_iter().filter_map(|n| {
            let item = Arc::new(PathItem::from(n.relative_path.clone()));
            engine.match_item(item).map(|res| (n, res.rank))
        });

        if self.unordered {
            ranked_nodes.map(|(n, _)| n).collect()
        } else {
            let mut ranked_nodes = ranked_nodes.collect::<Vec<_>>();
            ranked_nodes.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));
            ranked_nodes.into_iter().map(|(n, _)| n).collect()
        }
    }

    pub fn enable_search_order(self) -> Self {
        Self {
            unordered: false,
            ..self
        }
    }

    pub fn disable_search_order(self) -> Self {
        Self {
            unordered: true,
            ..self
        }
    }

    pub fn toggle_search_order(self) -> Self {
        Self {
            unordered: !self.unordered,
            ..self
        }
    }

    pub fn toggle_algorithm(self) -> Self {
        Self {
            algorithm: self.algorithm.toggle(),
            ..self
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExplorerConfig {
    #[serde(default)]
    pub filters: IndexSet<NodeFilterApplicable>,

    #[serde(default)]
    pub sorters: IndexSet<NodeSorterApplicable>,

    #[serde(default)]
    pub searcher: Option<NodeSearcherApplicable>,
}

impl ExplorerConfig {
    pub fn filter(&self, node: &Node) -> bool {
        self.filters.iter().all(|f| f.apply(node))
    }

    pub fn sort(&self, a: &Node, b: &Node) -> Ordering {
        let mut ord = Ordering::Equal;
        for s in self.sorters.iter() {
            ord = ord.then(s.apply(a, b));
        }
        ord
    }

    /// Get a reference to the explorer config's filters.
    pub fn filters(&self) -> &IndexSet<NodeFilterApplicable> {
        &self.filters
    }

    /// Get a reference to the explorer config's sorters.
    pub fn sorters(&self) -> &IndexSet<NodeSorterApplicable> {
        &self.sorters
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub command: String,

    #[serde(default)]
    pub args: Vec<String>,
}
