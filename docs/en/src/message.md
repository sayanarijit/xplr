# Message

You can think of xplr as a server. Just like web servers listen to HTTP
requests, xplr listens to [messages][1].

You can send these messages to an xplr session in the following ways:

- Via [key bindings][2]
- Via [Lua function calls][3]
- Via shell command using the [input pipe][4]

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

### { BufferInput = "string" }

**YAML:** `BufferInput(String)`

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

### { CallLua = "string" }

**YAML:** `CallLua: string`

Call a Lua function.
A [`CallLuaArg`][14] object will be passed to the
[function][3] as argument.
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

## Lua Function Calls

xplr allows users to define lua functions using the `xplr.fn.custom` Lua API.

These functions can be called using messages like `CallLua`, `CallLuaSilently`.

When called the function receives a [special argument][14] that
contains some useful information. The function can optionally return a list of
messages which will be handled by xplr.

### CallLua Argument

This is a special argument passed to the lua functions when called using the
`CallLua`, `CallLuaSilently` messages.

It contains the following information:

- [version][29]
- [config][30]
- [pwd][31]
- [focused_node][32]
- [directory_buffer][33]
- [selection][34]
- [mode][35]
- [layout][36]
- [input_buffer][37]
- [pid][38]
- [session_path][39]
- [explorer_config][40]
- [history][41]
- [last_modes][42]

### version

Type: string

xplr version. Can be used to test compatibility.

### config

Type: [Config][43]

The loaded configuration.

### pwd

Type: string

The present working directory/

### focused_node

Type: nullable [Node][44]

The node under focus.

### directory_buffer

Type: nullable [DirectoryBuffer][62]

The directory buffer being rendered.

### selection

Type: list of selected [Node][44]s

The selected nodes.

### mode

Type: [Mode][8]

Current mode.

### layout

Type: [Layout][11]

Current layout.

### input_buffer

Type: nullable string

The input buffer.

### pid

Type: integer

The xplr session PID.

### session_path

Type: string

The session path.

### explorer_config

[TODO][66]

### history

Type: [History][70]

### last_modes

Type: list of [Mode][8]

Last modes, not popped yet.

### parent

Type: string

The parent path of the node.

### relative_path

Type: string

The path relative to the parent, i.e. the file/directory name with extension.

### absolute_path

Type: string

The absolute path (without resolving symlinks) of the node.

### extension

Type: string

The extension of the node.

### is_symlink

Type: boolean

`true` if the node is a symlink.

### is_broken

Type: boolean

`true` if the node is a broken symlink.

### is_dir

Type: boolean

`true` if the node is a directory.

### is_file

Type: boolean

`true` if the node is a file.

### is_readonly

Type: boolean

`true` if the node is real-only.

### mime_essence

Type: string

The mime type of the node. For e.g. `text/csv`, `image/jpeg` etc.

### size

Type: integer

The size of the exact node. The size of a directory won't be calculated
recursively.

### human_size

Type: string

Like size but in human readable format.

### permissions

Type: [Permission][60]

The [permissions][60] applied to the node.

### canonical

Type: nullable [Resolved Node Metadata][61]

If the node is a symlink, it will hold information about the symlink resolved
node. Else, it will hold information the actual node. It the symlink is broken,
it will be null.

### symlink

Type: nullable [Resolved Node Metadata][61]

If the node is a symlink and is not broken, it will hold information about the
symlink resolved node. However, it will never hold information about the actual
node. It will instead be null.

### Node

A node contains the following fields:

- [parent][45]
- [relative_path][46]
- [absolute_path][47]
- [extension][48]
- [is_symlink][49]
- [is_broken][50]
- [is_dir][51]
- [is_file][52]
- [is_readonly][53]
- [mime_essence][54]
- [size][55]
- [human_size][56]
- [permissions][57]
- [canonical][58]
- [symlink][59]

### DirectoryBuffer

Directory buffer contains the following fields:

- [parent][45]
- [nodes][63]
- [total][64]
- [focus][65]

#### parent

Type: string

The parent path of the node.

#### nodes

Type: list of [Node][44]s

A list of visible nodes.

#### total

Type: int

The count of nodes being rendered.

#### focus

Type: int

The index of the node under focus. It can be `0` even when there's no node to
focus on.

### History

History contains the following fields:

- [loc][68]
- [paths][69]

#### loc

Type: int

Location of the current path in history.

#### paths

Type: list of string

Visited paths.

### Example: Using Lua Function Calls

```lua
-- Define the function
xplr.fn.custom.ask_name_and_greet = function(app)
  print("What's your name?")

  local name = io.read()
  local greeting = "Hello " .. name .. "!"
  local message = greeting .. " You are inside " .. app.pwd

  return {
    { LogSuccess = message },
  }
end

-- Map the function to a key (space)
xplr.config.modes.builtin.default.key_bindings.on_key.space = {
  help = "ask name and greet",
  messages = {
    { CallLua = "custom.ask_name_and_greet" }
  }
}

-- Now, when you press "space" in default mode, you will be prompted for your
-- name. Enter your name to receive a nice greeting and to know your location.
```

## Environment Variables and Pipes

Alternative to `CallLua`, `CallLuaSilently` messages that call Lua functions,
there are `Call`, `CallSilently`, `BashExec`, `BashExecSilently` messages
that call shell commands.

However, unlike the Lua functions, these shell commands have to read the useful
information and send messages via environment variables and temporary files
called "pipe"s. These environment variables and files are only available when
a command is being executed.

Visit the [**fzf integration tutorial**][19]
for example.

### Environment variables

To see the environment variables, invoke the shell by typing `:!` in default
mode and run the following command:

```
env | grep ^XPLR_
```

You will see something like:

```
XPLR_FOCUS_INDEX=0
XPLR_MODE=action to
XPLR_PIPE_SELECTION_OUT=/run/user/1000/xplr/session/122278/pipe/selection_out
XPLR_INPUT_BUFFER=
XPLR_PIPE_GLOBAL_HELP_MENU_OUT=/run/user/1000/xplr/session/122278/pipe/global_help_menu_out
XPLR_PID=122278
XPLR_PIPE_MSG_IN=/run/user/1000/xplr/session/122278/pipe/msg_in
XPLR_PIPE_LOGS_OUT=/run/user/1000/xplr/session/122278/pipe/logs_out
XPLR_PIPE_RESULT_OUT=/run/user/1000/xplr/session/122278/pipe/result_out
XPLR_PIPE_HISTORY_OUT=/run/user/1000/xplr/session/122278/pipe/history_out
XPLR_FOCUS_PATH=/home/sayanarijit/Documents/GitHub/xplr/docs/en/book
XPLR_SESSION_PATH=/run/user/1000/xplr/session/122278
XPLR_APP_VERSION=0.14.3
XPLR_PIPE_DIRECTORY_NODES_OUT=/run/user/1000/xplr/session/122278/pipe/directory_nodes_out
```

The environment variables starting with `XPLR_PIPE_` are the temporary files
called "pipe"s.

#### Input pipe

Current there is only one input pipe.

- [XPLR_PIPE_MSG_IN][20]

#### Output pipes

`XPLR_PIPE_*_OUT` are the output pipes that contain data which cannot be
exposed directly via environment variables, like multi-line string.

- [XPLR_PIPE_SELECTION_OUT][21]
- [XPLR_PIPE_GLOBAL_HELP_MENU_OUT][22]
- [XPLR_PIPE_LOGS_OUT][23]
- [XPLR_PIPE_RESULT_OUT][24]
- [XPLR_PIPE_HISTORY_OUT][25]
- [XPLR_PIPE_DIRECTORY_NODES_OUT][26]

#### XPLR_PIPE_MSG_IN

Append new-line delimited messages to this pipe in [YAML][27]
(or [JSON][7]) syntax. These messages will be read and
handled by xplr after the command execution.

#### XPLR_PIPE_SELECTION_OUT

New-line delimited list of selected paths.

#### XPLR_PIPE_GLOBAL_HELP_MENU_OUT

The full help menu.

#### XPLR_PIPE_LOGS_OUT

New-line delimited list of logs.

#### XPLR_PIPE_RESULT_OUT

New-line delimited result (selected paths if any, else the focused path)

#### XPLR_PIPE_HISTORY_OUT

New-line delimited list of last visited paths (similar to jump list in vim).

#### XPLR_PIPE_DIRECTORY_NODES_OUT

New-line delimited list of paths, filtered and sorted as displayed in the
[files table][28].

### Example: Using Environment Variables and Pipes

```lua
xplr.config.modes.builtin.default.key_bindings.on_key.space = {
  help = "ask name and greet",
  messages = {
    {
      BashExec = [===[
      echo "What's your name?"

      read name
      greeting="Hello $name!"
      message="$greeting You are inside $PWD"

      echo LogSuccess: '"'$message'"' >> "${XPLR_PIPE_MSG_IN:?}"
      ]===]
    }
  }
}

-- Now, when you press "space" in default mode, you will be prompted for your
-- name. Enter your name to receive a nice greeting and to know your location.
```

[1]: #full-list-of-messages
[2]: modes.md#key-bindings
[3]: #lua-function-calls
[4]: #input-pipe
[5]: https://www.lua.org/
[6]: http://yaml.org/
[7]: https://www.json.org
[8]: modes.md#mode
[9]: modes.md#builtin
[10]: modes.md#custom
[11]: layouts.md#layout
[12]: layouts.md#builtin
[13]: layouts.md#custom
[14]: #calllua-argument
[15]: filtering.md#filter
[16]: filtering.md
[17]: sorting.md#sorter
[18]: https://docs.rs/xplr/latest/xplr/app/struct.CallLuaArg.html#fields
[19]: modes.md#tutorial-adding-a-new-mode
[20]: #xplr_pipe_msg_in
[21]: #xplr_pipe_selection_out
[22]: #xplr_pipe_global_help_menu_out
[23]: #xplr_pipe_logs_out
[24]: #xplr_pipe_result_out
[25]: #xplr_pipe_history_out
[26]: #xplr_pipe_directory_nodes_out
[27]: https://www.yaml.org
[28]: layouts.md#table
[29]: #version
[30]: #config
[31]: #pwd
[32]: #focused_node
[33]: #directory_buffer
[34]: #selection
[35]: #mode
[36]: #layout
[37]: #input_buffer
[38]: #pid
[39]: #session_path
[40]: #explorer_config
[41]: #history
[42]: #last_modes
[43]: configuration.md#config
[44]: #node
[45]: #parent
[46]: #relative_path
[47]: #absolute_path
[48]: #extension
[49]: #is_symlink
[50]: #is_broken
[51]: #is_dir
[52]: #is_file
[53]: #is_readonly
[54]: #mime_essence
[55]: #size
[56]: #human_size
[57]: #permissions
[58]: #canonical
[59]: #symlink
[60]: column-renderer.md#permission
[61]: column-renderer.md#resolved-node-metadata
[62]: #directorybuffer
[63]: #nodes
[64]: #total
[65]: #focus
[66]: https://docs.rs/xplr/latest/xplr/app/struct.ExplorerConfig.html
[67]: #history
[68]: #loc
[69]: #paths
[70]: #history-1
