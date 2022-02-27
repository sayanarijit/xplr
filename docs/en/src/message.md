# Message

You can think of xplr as a server. Just like web servers listen to HTTP
requests, xplr listens to [messages][1].

You can send these messages to an xplr session in the following ways:

- Via command-line (currently during start-up only, using `--on-load`)
- Via [key bindings][2]
- Via [Lua function calls][3]
- Via shell command using the [input pipe][4]
- Via socket (coming soon)

## Format

To send messages using the [key bindings][2] or
[Lua function calls][3], messages are represented in
[Lua][5] syntax. For example:

- "Quit"
- { FocusPath = "/path/to/file" }
- { Call = { command = "bash", args = { "-c", "read -p test" } } }

However, to send messages using the [input pipe][4], they need to be
represented using
[YAML][6] (or [JSON][7]) syntax. For example:

- Quit
- FocusPath: "/path/to/file"
- Call: { command: bash, args: ["-c", "read -p test"] }

## Full List of Messages

### "ExplorePwd"

**YAML:** `ExplorePwd`

Explore the present working directory and register the filtered nodes.
This operation is expensive. So, try to avoid using it too often.

### "ExplorePwdAsync"

**YAML:** `ExplorePwdAsync`

Explore the present working directory and register the filtered nodes asynchronously.
This operation happens asynchronously. That means, the xplr directory buffers won't be updated
immediately. Hence, it needs to be used with care and probably with special checks in place.
To explore `$PWD` synchronously, use `ExplorePwd` instead.

### "ExploreParentsAsync"

**YAML:** `ExploreParentsAsync`

Explore the present working directory along with its parents and register the filtered nodes.
This operation happens asynchronously. That means, the xplr directory buffers won't be updated
immediately. Hence, it needs to be used with care and probably with special checks in place.
To explore just the `$PWD` synchronously, use `ExplorePwd` instead.

### "Refresh"

**YAML:** `Refresh`

Refresh the UI.
But it will not re-explore the directory if the working directory is the same.
If there is some change in the working directory and you want to re-explore it,
use the `Explore` message instead.
Also, it will not clear the screen. Use `ClearScreen` for that.

### "ClearScreen"

**YAML:** `ClearScreen`

Clears the screen.

### "FocusNext"

**YAML:** `FocusNext`

Focus next node.

### { FocusNextByRelativeIndex = int }

**YAML:** `FocusNextByRelativeIndex: int`

Focus on the `n`th node relative to the current focus where `n` is a given value.

**YAML Example:** `FocusNextByRelativeIndex: 2`

**Lua Example:** `{ FocusNextByRelativeIndex = 2 }`

### "FocusNextByRelativeIndexFromInput"

**YAML:** `FocusNextByRelativeIndexFromInput`

Focus on the `n`th node relative to the current focus where `n` is read from
the input buffer.

### "FocusPrevious"

**YAML:** `FocusPrevious`

Focus on the previous item.

### { FocusPreviousByRelativeIndex = int }

**YAML:** `FocusPreviousByRelativeIndex: int`

Focus on the `-n`th node relative to the current focus where `n` is a given value.

**YAML Example:** `FocusPreviousByRelativeIndex: 2`

**Lua Example:** `{ FocusPreviousByRelativeIndex = 2 }`

### "FocusPreviousByRelativeIndexFromInput"

**YAML:** `FocusPreviousByRelativeIndexFromInput`

Focus on the `-n`th node relative to the current focus where `n` is read from
the input buffer.

### "FocusFirst"

**YAML:** `FocusFirst`

Focus on the first node.

### "FocusLast"

**YAML:** `FocusLast`

Focus on the last node.

### { FocusPath = "string" }

**YAML:** `FocusPath: string`

Focus on the given path.

**YAML Example:** `FocusPath: /path/to/file`

**Lua Example:** `{ FocusPath = "/path/to/file" }`

### "FocusPathFromInput"

**YAML:** `FocusPathFromInput`

Focus on the path read from input buffer.

### { FocusByIndex = int }

**YAML:** `FocusByIndex: int`

Focus on the absolute `n`th node where `n` is a given value.

**YAML Example:** `FocusByIndex: 2`

**Lua Example:** `{ FocusByIndex = 2 }`

### "FocusByIndexFromInput"

**YAML:** `FocusByIndexFromInput`

Focus on the absolute `n`th node where `n` is read from the input buffer.

### { FocusByFileName = "string" }

**YAML:** `FocusByFileName: string`

Focus on the file by name from the present working directory.

**YAML Example:** `FocusByFileName: filename.ext`

**Lua Example:** `{ FocusByFileName = "filename.ext" }`

### { ChangeDirectory = "string" }

**YAML:** `ChangeDirectory: string`

Change the present working directory ($PWD)

**YAML Example:** `ChangeDirectory: /path/to/directory`

**Lua Example:** `{ ChangeDirectory = "/path/to/directory" }`

### "Enter"

**YAML:** `Enter`

Enter into the currently focused path if it's a directory.

### "Back"

**YAML:** `Back`

Go back to the parent directory.

### "LastVisitedPath"

**YAML:** `LastVisitedPath`

Go to the last path visited.

### "NextVisitedPath"

**YAML:** `NextVisitedPath`

Go to the next path visited.

### "FollowSymlink"

**YAML:** `FollowSymlink`

Follow the symlink under focus to its actual location.

### { UpdateInputBuffer = [Input Opertaion][71] }

**YAML:** `BufferInput: Input Operation`

Update the input buffer using cursor based operations.

**YAML Example:** `UpdateInputBuffer: GoToPreviousWord`

**Lua Example:** `{ UpdateInputBuffer = "GoToPreviousWord" }`

### "UpdateInputBufferFromKey"

**YAML:** `UpdateInputBufferFromKey`

Update the input buffer from the key read from keyboard input.

### { BufferInput = "string" }

**YAML:** `BufferInput: string`

Append/buffer the given string into the input buffer.

**YAML Example:** `BufferInput: foo`

**Lua Example:** `{ BufferInput = "foo" }`

### "BufferInputFromKey"

**YAML:** `BufferInputFromKey`

Append/buffer the characted read from a keyboard input into the
input buffer.

### { SetInputBuffer = "string" }

**YAML:** `SetInputBuffer: string`

Set/rewrite the input buffer with the given string.
When the input buffer is not-null (even if empty string)
it will show in the UI.

**YAML Example:** `SetInputBuffer: foo`

**Lua Example:** `{ SetInputBuffer = "foo" }`

### "RemoveInputBufferLastCharacter"

**YAML:** `RemoveInputBufferLastCharacter`

Remove input buffer's last character.

### "RemoveInputBufferLastWord"

**YAML:** `RemoveInputBufferLastWord`

Remove input buffer's last word.

### "ResetInputBuffer"

**YAML:** `ResetInputBuffer`

Reset the input buffer back to null. It will not show in the UI.

### { SwitchMode = "string" }

**YAML:** `SwitchMode: string`

Switch input [mode][8].
It clears the input buffer.

> **NOTE:** To be specific about which mode to switch to, use `SwitchModeBuiltin` or
> `SwitchModeCustom` instead.

**YAML Example:** `SwitchMode: default`

**Lua Example:** `{ SwitchMode = "default" }`

### { SwitchModeKeepingInputBuffer = "string" }

**YAML:** `SwitchModeKeepingInputBuffer: string`

Switch input [mode][8].
It keeps the input buffer.

> **NOTE:** To be specific about which mode to switch to, use
> `SwitchModeBuiltinKeepingInputBuffer` or
> `SwitchModeCustomKeepingInputBuffer` instead.

**YAML Example:** `SwitchModeKeepingInputBuffer: default`

**Lua Example:** `{ SwitchModeKeepingInputBuffer = "default" }`

### { SwitchModeBuiltin = "string" }

**YAML:** `SwitchModeBuiltin: string`

Switch to a [builtin mode][9].
It clears the input buffer.

**YAML Example:** `SwitchModeBuiltin: default`

**Lua Example:** `{ SwitchModeBuiltin = "default" }`

### { SwitchModeBuiltinKeepingInputBuffer = "string" }

**YAML:** `SwitchModeBuiltinKeepingInputBuffer: string`

Switch to a [builtin mode][9].
It keeps the input buffer.

**YAML Example:** `SwitchModeBuiltinKeepingInputBuffer: default`

**Lua Example:** `{ SwitchModeBuiltinKeepingInputBuffer = "default" }`

### { SwitchModeCustom = "string" }

**YAML:** `SwitchModeCustom: string`

Switch to a [custom mode][10].
It clears the input buffer.

**YAML Example:** `SwitchModeCustom: my_custom_mode`

**Lua Example:** `{ SwitchModeCustom = "my_custom_mode" }`

### { SwitchModeCustomKeepingInputBuffer = "string" }

**YAML:** `SwitchModeCustom: string`

Switch to a [custom mode][10].
It keeps the input buffer.

**YAML Example:** `SwitchModeCustomKeepingInputBuffer: my_custom_mode`

**Lua Example:** `{ SwitchModeCustomKeepingInputBuffer = "my_custom_mode" }`

### "PopMode"

**YAML:** `PopMode`

Pop the last mode from the history and switch to it.
It clears the input buffer.

### PopModeKeepingInputBuffer

**YAML:** `PopModeKeepingInputBuffer`

Pop the last mode from the history and switch to it.
It keeps the input buffer.

### { SwitchLayout = "string" }

**YAML:** `SwitchLayout: string`

Switch [layout][11].

> **NOTE:** To be specific about which layout to switch to, use `SwitchLayoutBuiltin` or
> `SwitchLayoutCustom` instead.

**YAML Example:** `SwitchLayout: default`

**Lua Example:** `{ SwitchLayout = "default" }`

### { SwitchLayoutBuiltin = "string" }

**YAML:** `SwitchLayoutBuiltin: string`

Switch to a [builtin layout][12].

**YAML Example:** `SwitchLayoutBuiltin: default`

**Lua Example:** `{ SwitchLayoutBuiltin = "default" }`

### { SwitchLayoutCustom = "string" }

**YAML:** `SwitchLayoutCustom: string`

Switch to a [custom layout][13].

**YAML Example:** `SwitchLayoutCustom: my_custom_layout`

**Lua Example:** `{ SwitchLayoutCustom = "my_custom_layout" }`

### { Call = "string" }

**YAML:** `Call: string`

Call a shell command with the given arguments.
Note that the arguments will be shell-escaped.
So to read the variables, the `-c` option of the shell
can be used.
You may need to pass `ExplorePwd` depening on the expectation.

**YAML Example:** `Call: { command: bash, args: ["-c", "read -p test"] }`

**Lua Example:** `{ Call = { command = "bash", args = { "-c", "read -p test" } } }`

### { CallSilently = "string" }

**YAML:** `CallSilently: string`

Like `Call` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

**YAML Example:** `CallSilently: { command: tput, args: ["bell"] }`

**Lua Example:** `{ CallSilently = { command = "tput", args = { "bell" } } }`

### { BashExec = "string" }

**YAML:** `BashExec: string`

An alias to `Call: {command: bash, args: ["-c", "{string}"], silent: false}`
where `{string}` is the given value.

**YAML Example:** `BashExec: "read -p test"`

**Lua Example:** `{ BashExec = "read -p test" }`

### { BashExecSilently = "string" }

**YAML:** `BashExecSilently(String)`

Like `BashExec` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

**YAML Example:** `BashExecSilently: "tput bell"`

**Lua Example:** `{ BashExecSilently = "tput bell" }`

### { CallLua = "string" }

**YAML:** `CallLua: string`

Call a Lua function.

A [Lua Context][14] object will be passed to the [function][3] as argument.
The function can optionally return a list of messages for xplr to handle
after the executing the function.

**YAML Example:** `CallLua: custom.some_custom_funtion`

**Lua Example:** `{ CallLua = "custom.some_custom_funtion" }`

### { CallLuaSilently = "string" }

**YAML:** `CallLuaSilently: string`

Like `CallLua` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

**YAML Example:** `CallLuaSilently: custom.some_custom_function`

**Lua Example:** `{ CallLuaSilently = "custom.some_custom_function" }`

### { LuaEval = "string" }

**YAML:** `LuaEval: string`

Execute Lua code without needing to define a function.

If the `string` is a callable, xplr will try to call it with with the
[Lua Context][14] argument.

**YAML Example:** `LuaEval: "return { { LogInfo = io.read() } }"`
**YAML Example:** `LuaEval: "function(app) return { { LogInfo = app.pwd } } end"`

**Lua Example:** `{ LuaEval = [[return { { LogInfo = io.read() } }]] }`
**Lua Example:** `{ LuaEval = [[function(app) return { { LogInfo = app.pwd } } end]] }`

### { LuaEvalSilently = "string" }

**YAML:** `LuaEvalSilently: string`

Like `LuaEval` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

**YAML Example:** `LuaEvalSilently: "return { { LogInfo = 'foo' } }"`

**Lua Example:** `{ LuaEvalSilently = [[return { { LogInfo = "foo" } }]] }`

### "Select"

**YAML:** `Select`

Select the focused node.

### "SelectAll"

**YAML:** `SelectAll`

Select all the visible nodes.

### { SelectPath = "string" }

**YAML:** `SelectPath: string`

Select the given path.

**YAML Example:** `SelectPath: /path/to/file`

**Lua Example:** `{ SelectPath = "/path/to/file" }`

### "UnSelect"

**YAML:** `UnSelect`

Unselect the focused node.

### "UnSelectAll"

**YAML:** `UnSelectAll`

Unselect all the visible nodes.

### { UnSelectPath = "string)" }

**YAML:** `UnSelectPath: string`

UnSelect the given path.

**YAML Example:** `UnSelectPath: /path/to/file`

**Lua Example:** `{ UnSelectPath = "/path/to/file" }`

### "ToggleSelection"

**YAML:** `ToggleSelection`

Toggle selection on the focused node.

### "ToggleSelectAll"

**YAML:** `ToggleSelectAll`

Toggle between select all and unselect all.

### { ToggleSelectionByPath = "string" }

**YAML:** `ToggleSelectionByPath: string`

Toggle selection by file path.

**YAML Example:** `ToggleSelectionByPath: /path/to/file`

**Lua Example:** `{ ToggleSelectionByPath = "/path/to/file" }`

### "ClearSelection"

**YAML:** `ClearSelection`

Clear the selection.

### { AddNodeFilter = { filter = [Filter][15], input = "string" }

**YAML:** `AddNodeFilter: { filter = Filter, input = string }`

Add a [filter][16] to exclude nodes while exploring directories.

**YAML Example:** `AddNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

**Lua Example:** `{ AddNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" } }`

### { RemoveNodeFilter = { filter = [Filter][15], input = "string" }

**YAML:** `RemoveNodeFilter: { filter = Filter, input = string`

Remove an existing [filter][16].

**YAML Example:** `RemoveNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

**Lua Example:** `{ RemoveNodeFilter: { filter: "RelativePathDoesStartWith", input: "foo" } }`

### { ToggleNodeFilter = { filter = [Filter][15], input = "string" }

**YAML:** `ToggleNodeFilter: { filter = Filter, input = string }`

Remove a [filter][16] if it exists, else, add a it.

**YAML Example:** `ToggleNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

**Lua Example:** `{ ToggleNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" } }`

### { AddNodeFilterFromInput = [Filter][15] }

**YAML:** `AddNodeFilterFromInput: Filter`

Add a node [filter][16] reading the input from the buffer.

**YAML Example:** `AddNodeFilterFromInput: RelativePathDoesStartWith`

**Lua Example:** `{ AddNodeFilterFromInput = "RelativePathDoesStartWith" }`

### { RemoveNodeFilterFromInput = [Filter][15] }

**YAML:** `RemoveNodeFilterFromInput: Filter`

Remove a node [filter][16] reading the input from the buffer.

**YAML Example:** `RemoveNodeFilterFromInput: RelativePathDoesStartWith`

**Lua Example:** `{ RemoveNodeFilterFromInput = "RelativePathDoesStartWith" }`

### "RemoveLastNodeFilter"

**YAML:** `RemoveLastNodeFilter`

Remove the last node [filter][16].

### "ResetNodeFilters"

**YAML:** `ResetNodeFilters`

Reset the node [filters][16] back to the default configuration.

### "ClearNodeFilters"

**YAML:** `ClearNodeFilters`

Clear all the node [filters][16].

### { AddNodeSorter = { sorter = [Sorter][17], reverse = bool } }

**YAML:** `AddNodeSorter: { sorter: Sorter, reverse = bool }`

Add a [sorter][17] to sort nodes while exploring directories.

**YAML Example:** `AddNodeSorter: { sorter: ByRelativePath, reverse: false }`

**YAML Example:** `{ AddNodeSorter = { sorter = "ByRelativePath", reverse = false } }`

### { RemoveNodeSorter = [Sorter][17] }

**YAML:** `RemoveNodeSorter: Sorter`

Remove an existing [sorter][17].

**YAML Example:** `RemoveNodeSorter: ByRelativePath`

**Lua Example:** `{ RemoveNodeSorter = "ByRelativePath" }`

### { ReverseNodeSorter = [Sorter][17] }

**YAML:** `ReverseNodeSorter: Sorter`

Reverse a node [sorter][17].

**YAML Example:** `ReverseNodeSorter: ByRelativePath`

**Lua Example:** `{ ReverseNodeSorter = "ByRelativePath" }`

### { ToggleNodeSorter = { sorter = [Sorter][17], reverse = bool } }

**YAML:** `ToggleNodeSorter: { sorter: Sorter, reverse = bool }`

Remove a [sorter][17] if it exists, else, add a it.

**YAML Example:** `ToggleSorterSorter: {sorter: ByRelativePath, reverse: false }`

**Lua Example:** `{ ToggleSorterSorter: { sorter = "ByRelativePath", reverse = false } }`

### "ReverseNodeSorters"

**YAML:** `ReverseNodeSorters`

Reverse the node [sorters][17].

### "RemoveLastNodeSorter"

**YAML:** `RemoveLastNodeSorter`

Remove the last node [sorter][17].

### "ResetNodeSorters"

**YAML:** `ResetNodeSorters`

Reset the node [sorters][17] back to the default configuration.

### "ClearNodeSorters"

**YAML:** `ClearNodeSorters`

Clear all the node [sorters][17].

### "EnableMouse"

**YAML:** `EnableMouse`

Enable mouse

### "DisableMouse"

**YAML:** `DisableMouse`

Disable mouse

### "ToggleMouse"

**YAML:** `ToggleMouse`

Toggle mouse

### { StartFifo = "string" }

**YAML:** `StartFifo: string`

Start piping the focused path to the given fifo path

**YAML Example:** `StartFifo: /tmp/xplr.fifo`

**Lua Example:** `{ StartFifo = "/tmp/xplr.fifo }`

### "StopFifo"

**YAML:** `StopFifo`

Close the active fifo and stop piping.

### { ToggleFifo = "string" }

**YAML:** `ToggleFifo: string`

Toggle betwen {Start|Stop}Fifo

**YAML Example:** `ToggleFifo: /path/to/fifo`

**Lua Example:** `{ ToggleFifo = "/path/to/fifo" }`

### { LogInfo = "string" }

**YAML:** `LogInfo: string`

Log information message.

**YAML Example:** `LogInfo: launching satellite`

**Lua Example:** `{ LogInfo = "launching satellite" }`

### { LogSuccess = "String" }

**YAML:** `LogSuccess: string`

Log a success message.

**YAML Example:** `LogSuccess: satellite reached destination`.

**Lua Example:** `{ LogSuccess = "satellite reached destination" }`.

### { LogWarning = "string" }

**YAML:** `LogWarning: string`

Log an warning message.

**YAML Example:** `LogWarning: satellite is heating`

**Lua Example:** `{ LogWarning = "satellite is heating" }`

### { LogError = "string" }

**YAML:** `LogError: string`

Log an error message.

**YAML Example:** `LogError: satellite crashed`

**Lua Example:** `{ LogError = "satellite crashed" }`

### "Quit"

**YAML:** `Quit`

Quit with returncode zero (success).

### "PrintPwdAndQuit"

**YAML:** `PrintPwdAndQuit`

Print $PWD and quit.

### "PrintFocusPathAndQuit"

**YAML:** `PrintFocusPathAndQuit`

Print the path under focus and quit. It can be empty string if there's nothing to focus.

### "PrintSelectionAndQuit"

**YAML:** `PrintSelectionAndQuit`

Print the selected paths and quit. It can be empty is no path is selected.

### "PrintResultAndQuit"

**YAML:** `PrintResultAndQuit`

Print the selected paths if it's not empty, else, print the focused node's path.

### "PrintAppStateAndQuit"

**YAML:** `PrintAppStateAndQuit`

Print the state of application in YAML format. Helpful for debugging or generating
the default configuration file.

### { Debug = "string" }

**YAML:** `Debug: string`

Write the application state to a file, without quitting. Also helpful for debugging.

**YAML Example:** `Debug: /path/to/file`

**Lua Example:** `{ Debug = "/path/to/file" }`

### "Terminate"

**YAML:** `Terminate`

Terminate the application with a non-zero return code.

## InputOperation

Cursor based input operation can be one of the following:

- { SetCursor = int }
- { InsertCharacter = str }
- "GoToPreviousCharacter"
- "GoToNextCharacter"
- "GoToPreviousWord"
- "GoToNextWord"
- "GoToStart"
- "GoToEnd"
- "DeletePreviousCharacter"
- "DeleteNextCharacter"
- "DeletePreviousWord"
- "DeleteNextWord"
- "DeleteLine"

[1]: #full-list-of-messages
[2]: key-bindings.md
[3]: lua-function-calls.md
[4]: environment-variables-and-pipes.md#input-pipe
[5]: https://www.lua.org/
[6]: http://yaml.org/
[7]: https://www.json.org
[8]: modes.md#mode
[9]: modes.md#builtin
[10]: modes.md#custom
[11]: layouts.md
[12]: layouts.md#builtin
[13]: layouts.md#custom
[14]: lua-function-calls.md#lua-context
[15]: filtering.md#filter
[16]: filtering.md
[17]: sorting.md#sorter
[71]: #inputoperation
