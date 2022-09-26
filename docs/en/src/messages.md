# Full List of Messages

xplr messages categorized based on their purpose.

## Categories

- [Exploring](#exploring)
- [Screen](#screen)
- [Navigation](#navigation)
- [Reading Input](#reading-input)
- [Switching Mode](#switching-mode)
- [Switching Layout](#switching-layout)
- [Executing Commands](#executing-commands)
- [Calling Lua Functions](#calling-lua-functions)
- [Select Operations](#select-operations)
- [Filter Operations](#filter-operations)
- [Sort Operations](#sort-operations)
- [Mouse Operations](#mouse-operations)
- [Fifo Operations](#fifo-operations)
- [Logging](#logging)
- [Debugging](#debugging)
- [Quit Options](#quit-options)

### Exploring

#### ExplorePwd

Explore the present working directory and register the filtered nodes.
This operation is expensive. So, try to avoid using it too often.

Example:

- Lua: `"ExplorePwd"`
- YAML: `ExplorePwd`

#### ExplorePwdAsync

Explore the present working directory and register the filtered nodes
asynchronously. This operation happens asynchronously. That means, the
xplr directory buffers won't be updated immediately. Hence, it needs to
be used with care and probably with special checks in place. To explore
$PWD synchronously, use `ExplorePwd` instead.

Example:

- Lua: `"ExplorePwdAsync"`
- YAML: `ExplorePwdAsync`

#### ExploreParentsAsync

Explore the present working directory along with its parents and
register the filtered nodes. This operation happens asynchronously.
That means, the xplr directory buffers won't be updated immediately.
Hence, it needs to be used with care and probably with special checks
in place. To explore just the `$PWD` synchronously, use `ExplorePwd`
instead.

Example:

- Lua: `"ExploreParentsAsync"`
- YAML: `ExploreParentsAsync`

### Screen

#### ClearScreen

Clear the screen.

Example:

- Lua: `"ClearScreen"`
- YAML: `ClearScreen`

#### Refresh

Refresh the screen.
But it will not re-explore the directory if the working directory is
the same. If there is some change in the working directory and you want
to re-explore it, use the `Explore` message instead.
Also, it will not clear the screen. Use `ClearScreen` for that.

Example:

- Lua: `"Refresh"`
- YAML: `Refresh`

### Navigation

#### FocusNext

Focus next node.

Example:

- Lua: `"FocusNext"`
- YAML: `FocusNext`

#### FocusNextByRelativeIndex

Focus on the `n`th node relative to the current focus where `n` is a
given value.

Type: { FocusNextByRelativeIndex = int }

Example:

- Lua: `{ FocusNextByRelativeIndex = 2 }`
- YAML: `FocusNextByRelativeIndex: 2`

#### FocusNextByRelativeIndexFromInput

Focus on the `n`th node relative to the current focus where `n` is read
from the input buffer.

Example:

- Lua: `"FocusNextByRelativeIndexFromInput"`
- YAML: `FocusNextByRelativeIndexFromInput`

#### FocusPrevious

Focus on the previous item.

Example:

- Lua: `"FocusPrevious"`
- YAML: `FocusPrevious`

#### FocusPreviousByRelativeIndex

Focus on the `-n`th node relative to the current focus where `n` is a
given value.

Type: { FocusPreviousByRelativeIndex = int }

Example:

- Lua: `{ FocusPreviousByRelativeIndex = 2 }`
- YAML: `FocusPreviousByRelativeIndex: 2`

#### FocusPreviousByRelativeIndexFromInput

Focus on the `-n`th node relative to the current focus where `n` is
read from the input buffer.

Example:

- Lua: `"FocusPreviousByRelativeIndexFromInput"`
- YAML: `FocusPreviousByRelativeIndexFromInput`

#### FocusFirst

Focus on the first node.

Example:

- Lua: `"FocusFirst"`
- YAML: `FocusFirst`

#### FocusLast

Focus on the last node.

Example:

- Lua: `"FocusLast"`
- YAML: `FocusLast`

#### FocusPath

Focus on the given path.

Type: { FocusPath = "string" }

Example:

- Lua: `{ FocusPath = "/path/to/file" }`
- YAML: `FocusPath: /path/to/file`

#### FocusPathFromInput

Focus on the path read from input buffer.

Example:

- Lua: `"FocusPathFromInput"`
- YAML: `FocusPathFromInput`

#### FocusByIndex

Focus on the absolute `n`th node where `n` is a given value.

Type: { FocusByIndex = int }

Example:

- Lua: `{ FocusByIndex = 2 }`
- YAML: `FocusByIndex: 2`

#### FocusByIndexFromInput

Focus on the absolute `n`th node where `n` is read from the input buffer.

Example:

- Lua: `"FocusByIndexFromInput"`
- YAML: `FocusByIndexFromInput`

#### FocusByFileName

Focus on the file by name from the present working directory.

Type: { FocusByFileName = "string" }

Example:

- Lua: `{ FocusByFileName = "filename.ext" }`
- YAML: `FocusByFileName: filename.ext`

#### ScrollUp

Scroll up by terminal height.

Example:

- Lua: `"ScrollUp"`
- YAML: `ScrollUp`

#### ScrollDown

Scroll down by terminal height.

Example:

- Lua: `"ScrollDown"`
- YAML: `ScrollDown`

#### ScrollUpHalf

Scroll up by half of terminal height.

Example:

- Lua: `"ScrollUpHalf"`
- YAML: `ScrollUpHalf`

#### ScrollDownHalf

Scroll down by half of terminal height.

Example:

- Lua: `"ScrollDownHalf"`
- YAML: `ScrollDownHalf`

#### ChangeDirectory

Change the present working directory ($PWD)

Type: { ChangeDirectory = "string" }

Example:

- Lua: `{ ChangeDirectory = "/path/to/directory" }`
- YAML: `ChangeDirectory: /path/to/directory`

#### Enter

Enter into the currently focused path if it's a directory.

Example:

- Lua: `"Enter"`
- YAML: `Enter`

#### Back

Go back to the parent directory.

Example:

- Lua: `"Back"`
- YAML: `Back`

#### LastVisitedPath

Go to the last path visited.

Example:

- Lua: `"LastVisitedPath"`
- YAML: `LastVisitedPath`

#### NextVisitedPath

Go to the next path visited.

Example:

- Lua: `"NextVisitedPath"`
- YAML: `NextVisitedPath`

#### FollowSymlink

Follow the symlink under focus to its actual location.

Example:

Lua: `"FollowSymlink"`
YAML: `FollowSymlink`

### Reading Input

#### SetInputPrompt

Set the input prompt temporarily, until the input buffer is reset.

Type: { SetInputPrompt = string }

Example:

- Lua: `{ SetInputPrompt = "→" }`
- YAML: `SetInputPrompt: →`

#### UpdateInputBuffer

Update the input buffer using cursor based operations.

Type: { UpdateInputBuffer = [Input Opertaion](https://xplr.dev/en/input-operation) }

Example:

- Lua: `{ UpdateInputBuffer = "GoToPreviousWord" }`
- YAML: `UpdateInputBuffer: GoToPreviousWord`

#### UpdateInputBufferFromKey

Update the input buffer from the key read from keyboard input.

Example:

- Lua: `"UpdateInputBufferFromKey"`
- YAML: `UpdateInputBufferFromKey`

#### BufferInput

Append/buffer the given string into the input buffer.

Type: { BufferInput = "string" }

Example:

- Lua: `{ BufferInput = "foo" }`
- YAML: `BufferInput: foo`

#### BufferInputFromKey

Append/buffer the characted read from a keyboard input into the
input buffer.

Example:

- Lua: `"BufferInputFromKey"`
- YAML: `BufferInputFromKey`

#### SetInputBuffer

Set/rewrite the input buffer with the given string.
When the input buffer is not-null (even if empty string)
it will show in the UI.

Type: { SetInputBuffer = "string" }

Example:

- Lua: `{ SetInputBuffer = "foo" }`
- YAML: `SetInputBuffer: foo`

#### RemoveInputBufferLastCharacter

Remove input buffer's last character.

Example:

- Lua: `"RemoveInputBufferLastCharacter"`
- YAML: `RemoveInputBufferLastCharacter`

#### RemoveInputBufferLastWord

Remove input buffer's last word.

Example:

- Lua: `"RemoveInputBufferLastWord"`
- YAML: `RemoveInputBufferLastWord`

#### ResetInputBuffer

Reset the input buffer back to null. It will not show in the UI.

Example:

- Lua: `"ResetInputBuffer"`
- YAML: `ResetInputBuffer`

### Switching Mode

#### SwitchMode

Switch input [mode](https://xplr.dev/en/modes).

Type : { SwitchMode = "string" }

Example:

- Lua: `{ SwitchMode = "default" }`
- YAML: SwitchMode: default

> **NOTE:** To be specific about which mode to switch to, use
> `SwitchModeBuiltinKeepingInputBuffer` or
> `SwitchModeCustomKeepingInputBuffer` instead.

#### SwitchModeKeepingInputBuffer

Switch input [mode](https://xplr.dev/en/modes).
It keeps the input buffer.

Type: { SwitchModeKeepingInputBuffer = "string" }

Example:

- Lua: `{ SwitchModeKeepingInputBuffer = "default" }`
- YAML: `SwitchModeKeepingInputBuffer: default`

> **NOTE:** To be specific about which mode to switch to, use
> `SwitchModeBuiltinKeepingInputBuffer` or
> `SwitchModeCustomKeepingInputBuffer` instead.

#### SwitchModeBuiltin

Switch to a [builtin mode](https://xplr.dev/en/modes#builtin).
It clears the input buffer.

Type: { SwitchModeBuiltin = "string" }

Example:

- Lua: `{ SwitchModeBuiltin = "default" }`
- YAML: `SwitchModeBuiltin: default`

#### SwitchModeBuiltinKeepingInputBuffer

Switch to a [builtin mode](https://xplr.dev/en/modes#builtin).
It keeps the input buffer.

Type: { SwitchModeBuiltinKeepingInputBuffer = "string" }

Example:

- Lua: `{ SwitchModeBuiltinKeepingInputBuffer = "default" }`
- YAML: `SwitchModeBuiltinKeepingInputBuffer: default`

#### SwitchModeCustom

Switch to a [custom mode](https://xplr.dev/en/modes#custom).
It clears the input buffer.

Type: { SwitchModeCustom = "string" }

Example:

- Lua: `{ SwitchModeCustom = "my_custom_mode" }`
- YAML: `SwitchModeCustom: my_custom_mode`

#### SwitchModeCustomKeepingInputBuffer

Switch to a [custom mode](https://xplr.dev/en/modes#custom).
It keeps the input buffer.

Type: { SwitchModeCustomKeepingInputBuffer = "string" }

Example:

- Lua: `{ SwitchModeCustomKeepingInputBuffer = "my_custom_mode" }`
- YAML: `SwitchModeCustomKeepingInputBuffer: my_custom_mode`

#### PopMode

Pop the last mode from the history and switch to it.
It clears the input buffer.

Example:

- Lua: `"PopMode"`
- YAML: `PopMode`

#### PopModeKeepingInputBuffer

Pop the last mode from the history and switch to it.
It keeps the input buffer.

Example:

- Lua: `PopModeKeepingInputBuffer`
- YAML: `PopModeKeepingInputBuffer`

### Switching Layout

#### SwitchLayout

Switch [layout](https://xplr.dev/en/layouts).

Type: { SwitchLayout = "string" }

Example:

- Lua: `{ SwitchLayout = "default" }`
- YAML: `SwitchLayout: default`

> **NOTE:** To be specific about which layout to switch to, use `SwitchLayoutBuiltin` or
> `SwitchLayoutCustom` instead.

#### SwitchLayoutBuiltin

Switch to a [builtin layout](https://xplr.dev/en/layouts#builtin).

Type: { SwitchLayoutBuiltin = "string" }

Example:

- Lua: `{ SwitchLayoutBuiltin = "default" }`
- YAML: `SwitchLayoutBuiltin: default`

#### SwitchLayoutCustom

Switch to a [custom layout](https://xplr.dev/en/layouts#custom).

Type: { SwitchLayoutCustom = "string" }

Example:

- Lua: `{ SwitchLayoutCustom = "my_custom_layout" }`
- YAML: `SwitchLayoutCustom: my_custom_layout`

### Executing Commands

#### Call

Call a shell command with the given arguments.
Note that the arguments will be shell-escaped.
So to read the variables, the `-c` option of the shell
can be used.
You may need to pass `ExplorePwd` depening on the expectation.

Type: { Call = { command = string, args = { "list", "of", "string" } }

Example:

- Lua: `{ Call = { command = "bash", args = { "-c", "read -p test" } } }`
- YAML: `Call: { command: bash, args: ["-c", "read -p test"] }`

#### CallSilently

Like `Call` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

Type: { CallSilently = "string" }

Example:

- Lua: `{ CallSilently = { command = "tput", args = { "bell" } } }`
- YAML: `CallSilently: { command: tput, args: ["bell"] }`

#### BashExec

An alias to `Call: {command: bash, args: ["-c", "{string}"], silent: false}`
where `{string}` is the given value.

Type: { BashExec = "string" }

Example:

- Lua: `{ BashExec = "read -p test" }`
- YAML: `BashExec: "read -p test"`

#### BashExecSilently

Like `BashExec` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

Type: { BashExecSilently = "string" }

Example:

- Lua: `{ BashExecSilently = "tput bell" }`
- YAML: `BashExecSilently: "tput bell"`

### Calling Lua Functions

#### CallLua

Call a Lua function.

A [Lua Context](https://xplr.dev/en/lua-function-calls#lua-context)
object will be passed to the function as argument.
The function can optionally return a list of messages for xplr to
handle after the executing the function.

Type: { CallLua = "string" }

Example:

- Lua: `{ CallLua = "custom.some_custom_funtion" }`
- YAML: `CallLua: custom.some_custom_funtion`

#### CallLuaSilently

Like `CallLua` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

Type: { CallLuaSilently = "string" }

Example:

- Lua: `{ CallLuaSilently = "custom.some_custom_function" }`
- YAML: `CallLuaSilently: custom.some_custom_function`

#### LuaEval

Execute Lua code without needing to define a function.

If the `string` is a callable, xplr will try to call it with with the
[Lua Context](https://xplr.dev/en/lua-function-calls#lua-context)
argument.

Type: { LuaEval = "string" }

Example:

- Lua: `{ LuaEval = [[return { { LogInfo = io.read() } }]] }`
- Lua: `{ LuaEval = [[function(app) return { { LogInfo = app.pwd } } end]] }`
- YAML: `LuaEval: "return { { LogInfo = io.read() } }"`
- YAML: `LuaEval: "function(app) return { { LogInfo = app.pwd } } end"`

#### LuaEvalSilently

Like `LuaEval` but without the flicker. The stdin, stdout
stderr will be piped to null. So it's non-interactive.

Type: { LuaEvalSilently = "string" }

Example:

- Lua: `{ LuaEvalSilently = [[return { { LogInfo = "foo" } }]] }`
- YAML: `LuaEvalSilently: "return { { LogInfo = 'foo' } }"`

### Select Operations

#### Select

Select the focused node.

Example:

- Lua: `"Select"`
- YAML: `Select`

#### SelectAll

Select all the visible nodes.

Example:

- Lua: `"SelectAll"`
- YAML: `SelectAll`

#### SelectPath

Select the given path.

Type: { SelectPath = "string" }

Example:

- Lua: `{ SelectPath = "/path/to/file" }`
- YAML: `SelectPath: /path/to/file`

#### UnSelect

Unselect the focused node.

Example:

- Lua: `"UnSelect"`
- YAML: `UnSelect`

#### UnSelectAll

Unselect all the visible nodes.

Example:

- Lua: `"UnSelectAll"`
- YAML: `UnSelectAll`

#### UnSelectPath

UnSelect the given path.

Type: { UnSelectPath = "string)" }

Example:

- Lua: `{ UnSelectPath = "/path/to/file" }`
- YAML: `UnSelectPath: /path/to/file`

#### ToggleSelection

Toggle selection on the focused node.

Example:

- Lua: `"ToggleSelection"`
- YAML `ToggleSelection`

#### ToggleSelectAll

Toggle between select all and unselect all.
Example:

- Lua: `"ToggleSelectAll"`
- YAML: `ToggleSelectAll`

#### ToggleSelectionByPath

Toggle selection by file path.

Type: { ToggleSelectionByPath = "string" }

Example:

- Lua: `{ ToggleSelectionByPath = "/path/to/file" }`
- YAML: `ToggleSelectionByPath: /path/to/file`

#### ClearSelection

Clear the selection.

Example:

- Lua: `"ClearSelection"`
- YAML: `ClearSelection`

### Filter Operations

#### AddNodeFilter

Add a [filter](https://xplr.dev/en/filtering#filter) to exclude nodes
while exploring directories.

Type: { AddNodeFilter = { filter = [Filter](https://xplr.dev/en/filtering#filter), input = "string" }

Example:

- Lua: `{ AddNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" } }`
- YAML: `AddNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

#### RemoveNodeFilter

Remove an existing [filter](https://xplr.dev/en/filtering#filter).

Type: { RemoveNodeFilter = { filter = [Filter](https://xplr.dev/en/filtering), input = "string" }

Example:

- Lua: `{ RemoveNodeFilter: { filter: "RelativePathDoesStartWith", input: "foo" } }`
- YAML: `RemoveNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

#### ToggleNodeFilter

Remove a [filter](https://xplr.dev/en/filtering#filter) if it exists,
else, add a it.

Type: { ToggleNodeFilter = { filter = [Filter](https://xplr.dev/en/filtering), input = "string" }

Example:

- Lua: `{ ToggleNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" } }`
- YAML: `ToggleNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

#### AddNodeFilterFromInput

Add a node [filter](https://xplr.dev/en/filtering#filter) reading the
input from the buffer.

Type: { AddNodeFilterFromInput = [Filter](https://xplr.dev/en/filtering) }

Example:

- Lua: `{ AddNodeFilterFromInput = "RelativePathDoesStartWith" }`
- YAML: `AddNodeFilterFromInput: RelativePathDoesStartWith`

#### RemoveNodeFilterFromInput

Remove a node [filter](https://xplr.dev/en/filtering#filter) reading
the input from the buffer.

Type: { RemoveNodeFilterFromInput = [Filter](https://xplr.dev/en/filtering) }

Example:

- Lua: `{ RemoveNodeFilterFromInput = "RelativePathDoesStartWith" }`
- YAML: `RemoveNodeFilterFromInput: RelativePathDoesStartWith`

#### RemoveLastNodeFilter

Remove the last node [filter](https://xplr.dev/en/filtering).

Example:

- Lua: `"RemoveLastNodeFilter"`
- YAML: `RemoveLastNodeFilter`

#### ResetNodeFilters

Reset the node [filters](https://xplr.dev/en/filtering) back to the
default configuration.

Example:

- Lua: `"ResetNodeFilters"`
- YAML: `ResetNodeFilters`

#### ClearNodeFilters

Clear all the node [filters](https://xplr.dev/en/filtering).

Example:

- Lua: `"ClearNodeFilters"`
- YAML: `ClearNodeFilters`

### Sort Operations

#### AddNodeSorter

Add a [sorter](https://xplr.dev/en/sorting#sorter) to sort nodes while
exploring directories.

Type: { AddNodeSorter = { sorter = [Sorter](https://xplr.dev/en/sorting#sorter), reverse = bool } }

Example:

- Lua: `{ AddNodeSorter = { sorter = "ByRelativePath", reverse = false } }`
- YAML: `AddNodeSorter: { sorter: ByRelativePath, reverse: false }`

#### RemoveNodeSorter

Remove an existing [sorter](https://xplr.dev/en/sorting#sorter).

Type: { RemoveNodeSorter = [Sorter](https://xplr.dev/en/sorting#sorter) }

Example:

- Lua: `{ RemoveNodeSorter = "ByRelativePath" }`
- YAML: `RemoveNodeSorter: ByRelativePath`

#### ReverseNodeSorter

Reverse a node [sorter](https://xplr.dev/en/sorting#sorter).

Type: { ReverseNodeSorter = [Sorter](https://xplr.dev/en/sorting#sorter) }

Example:

- Lua: `{ ReverseNodeSorter = "ByRelativePath" }`
- YAML: `ReverseNodeSorter: ByRelativePath`

#### ToggleNodeSorter

Remove a [sorter](https://xplr.dev/en/sorting#sorter) if it exists,
else, add a it.

Type: { ToggleNodeSorter = { sorter = [Sorter](https://xplr.dev/en/sorting#sorter), reverse = bool } }

Example:

- Lua: `{ ToggleSorterSorter: { sorter = "ByRelativePath", reverse = false } }`
- YAML: `ToggleSorterSorter: {sorter: ByRelativePath, reverse: false }`

#### ReverseNodeSorters

Reverse the node [sorters](https://xplr.dev/en/sorting#sorter).

Example:

- Lua: `"ReverseNodeSorters"`
- YAML: `ReverseNodeSorters`

#### RemoveLastNodeSorter

Remove the last node [sorter](https://xplr.dev/en/sorting#sorter).

Example:

- Lua: `"RemoveLastNodeSorter"`
- YAML: `RemoveLastNodeSorter`

#### ResetNodeSorters

Reset the node [sorters](https://xplr.dev/en/sorting#sorter) back to
the default configuration.

Example:

- Lua: `"ResetNodeSorters"`
- YAML: `ResetNodeSorters`

#### ClearNodeSorters

Clear all the node [sorters](https://xplr.dev/en/sorting#sorter).

Example:

- Lua: `"ClearNodeSorters"`
- YAML: `ClearNodeSorters`

### Mouse Operations

#### EnableMouse

Enable mouse

Example:

- Lua: `"EnableMouse"`
- YAML: `EnableMouse`

#### DisableMouse

Disable mouse

Example:

- Lua: `"DisableMouse"`
- YAML: `DisableMouse`

#### ToggleMouse

Toggle mouse

Example:

- Lua: `"ToggleMouse"`
- YAML: `ToggleMouse`

### Fifo Operations

#### StartFifo

Start piping the focused path to the given fifo path

Type: { StartFifo = "string" }

Example:

- Lua: `{ StartFifo = "/tmp/xplr.fifo }`
- YAML: `StartFifo: /tmp/xplr.fifo`

#### StopFifo

Close the active fifo and stop piping.

Example:

- Lua: `"StopFifo"`
- YAML: `StopFifo`

#### ToggleFifo

Toggle betwen {Start|Stop}Fifo

Type: { ToggleFifo = "string" }

Example:

- Lua: `{ ToggleFifo = "/path/to/fifo" }`
- YAML: `ToggleFifo: /path/to/fifo`

### Logging

#### LogInfo

Log information message.

Type: { LogInfo = "string" }

Example:

- Lua: `{ LogInfo = "launching satellite" }`
- YAML: `LogInfo: launching satellite`

#### LogSuccess

Log a success message.

Type: { LogSuccess = "String" }

Example:

- Lua: `{ LogSuccess = "satellite reached destination" }`
- YAML: `LogSuccess: satellite reached destination`

#### LogWarning

Log an warning message.

Type: { LogWarning = "string" }

Example:

- Lua: `{ LogWarning = "satellite is heating" }`
- YAML: `LogWarning: satellite is heating`

#### LogError

Log an error message.

Type: { LogError = "string" }

Example:

- Lua: `{ LogError = "satellite crashed" }`
- YAML: `LogError: satellite crashed`

### Debugging

#### Debug

Write the application state to a file, without quitting. Also helpful
for debugging.

Type: { Debug = "string" }

Example:

- Lua: `{ Debug = "/path/to/file" }`
- YAML: `Debug: /path/to/file`

### Quit Options

#### Quit

Example:

- Lua: `"Quit"`
- YAML: `Quit`

Quit with returncode zero (success).

#### PrintPwdAndQuit

Print $PWD and quit.

Example:

- Lua: `"PrintPwdAndQuit"`
- YAML: `PrintPwdAndQuit`

#### PrintFocusPathAndQuit

Print the path under focus and quit. It can be empty string if there's
nothing to focus.

Example:

- Lua: `"PrintFocusPathAndQuit"`
- YAML: `PrintFocusPathAndQuit`

#### PrintSelectionAndQuit

Print the selected paths and quit. It can be empty is no path is
selected.

Example:

- Lua: `"PrintSelectionAndQuit"`
- YAML: `PrintSelectionAndQuit`

#### PrintResultAndQuit

Print the selected paths if it's not empty, else, print the focused
node's path.

Example:

- Lua: `"PrintResultAndQuit"`
- YAML: `PrintResultAndQuit`

#### PrintAppStateAndQuit

Print the state of application in YAML format. Helpful for debugging or
generating the default configuration file.

Example:

- Lua: `"PrintAppStateAndQuit"`
- YAML: `PrintAppStateAndQuit`

#### Terminate

Terminate the application with a non-zero return code.

Example:

- Lua: `"Terminate"`
- YAML: `Terminate`

## Also See:

- [Message](message.md)
