Message
=======

You can think of xplr as a server. Just like web servers listen to HTTP
requests, xplr listens to [messages](full-list-of-messages).

You can send these messages to an xplr session in the following ways:

- Via [key bindings](modes.md#key-bindings)
- Via [Lua function calls](#lua-function-calls)
- Via shell command using the [input pipe](#input-pipe)


Format
------

To send messages using the [key bindings](modes.md#key-bindings) or
[Lua functions calls](#lua-functions-calls), messages are represented in
[Lua](https://www.lua.org/) syntax. For example:

- "Quit"
- { FocusPath = "/path/to/file" }
- { Call = { command = "bash", args = { "-c", "read -p test" } } }

However, to send messages using the [input pipe](#input-pipe), they need to be
represented using
[YAML](http://yaml.org/) (or [JSON](https://www.json.org)) syntax. For example:

- Quit
- FocusPath: "/path/to/file"
- Call: { command: bash, args: ["-c", "read -p test"] }


Full List of Messages
---------------------

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

### : { FocusNextByRelativeIndex = int }

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

Switch input [mode](modes.md).

> **NOTE:** To be specific about which mode to switch to, use `SwitchModeBuiltin` or
`SwitchModeCustom` instead.

**YAML Example:** `SwitchMode: default`

**Lua Example:** `{ SwitchMode = "default" }`

### { SwitchModeBuiltin = "string" }

**YAML:** `SwitchModeBuiltin: string`

Switch to a [builtin mode](modes.md#builtin).

**YAML Example:** `SwitchModeBuiltin: default`

**Lua Example:** `{ SwitchModeBuiltin = "default" }`

### { SwitchModeCustom = "string" }

**YAML:** `SwitchModeCustom: string`

Switch to a [custom mode](modes.md#custom).

**YAML Example:** `SwitchModeCustom: my_custom_mode`

**Lua Example:** `{ SwitchModeCustom = "my_custom_mode" }`

### "PopMode"

**YAML:** `PopMode`

Pop the last mode from the history and switch to it.

### { SwitchLayout = "string" }

**YAML:** `SwitchLayout: string`

Switch [layout](layouts.md).

> **NOTE:** To be specific about which layout to switch to, use `SwitchLayoutBuiltin` or
`SwitchLayoutCustom` instead.

**YAML Example:** `SwitchLayout: default`

**Lua Example:** `{ SwitchLayout = "default" }`

### { SwitchLayoutBuiltin = "string" }

**YAML:** `SwitchLayoutBuiltin: string`

Switch to a [builtin layout](layouts.md#builtin).

**YAML Example:** `SwitchLayoutBuiltin: default`

**Lua Example:** `{ SwitchLayoutBuiltin = "default" }`

### { SwitchLayoutCustom = "string" }

**YAML:** `SwitchLayoutCustom: string`

Switch to a [custom layout](layouts.md#custom).

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
A [`CallLuaArg`](#calllua-argument) object will be passed to the
[function](#lua-function-calls) as argument.
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

### { AddNodeFilter = { filter = "[NodeFilter](filtering.md#filter)", input = "string" }

**YAML:** `AddNodeFilter: { filter = [NodeFilter](filtering.md#filter), input = string`

Add a [filter](filtering.md) to exclude nodes while exploring directories.

**YAML Example:** `AddNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

**Lua Example:** `{ AddNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" }`

### { RemoveNodeFilter = { filter = "[NodeFilter](filtering.md#filter)", input = "string" }

**YAML:** `RemoveNodeFilter: { filter = [NodeFilter](filtering.md#filter), input = string`

Remove an existing [filter](filtering.md).

**YAML Example:** `RemoveNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

**Lua Example:** `{ RemoveNodeFilter: { filter: "RelativePathDoesStartWith", input: "foo" } }`

### { ToggleNodeFilter = { filter = "[NodeFilter](filtering.md#filter)", input = "string" }

**YAML:** `ToggleNodeFilter: { filter = [NodeFilter](filtering.md#filter), input = string`

Remove a [filter](filter.md) if it exists, else, add a it.

**YAML Example:** `ToggleNodeFilter: { filter: RelativePathDoesStartWith, input: foo }`

**Lua Example:** `{ ToggleNodeFilter = { filter = "RelativePathDoesStartWith", input = "foo" } }`

### { AddNodeFilterFromInput = "[NodeFilter](filtering.md#filter)" }

**YAML:** `AddNodeFilterFromInput: [NodeFilter](filtering.md#filter)`

Add a node [filter](filtering.md) reading the input from the buffer.

**YAML Example:** `AddNodeFilterFromInput: RelativePathDoesStartWith`

**Lua Example:** `{ AddNodeFilterFromInput = "RelativePathDoesStartWith" }`

### { RemoveNodeFilterFromInput = "[NodeFilter](filtering.md#filter)" }

**YAML:** `RemoveNodeFilterFromInput: [NodeFilter](filtering.md#filter)`

Remove a node [filter](filtering.md) reading the input from the buffer.

**YAML Example:** `RemoveNodeFilterFromInput: RelativePathDoesStartWith`

**Lua Example:** `{ RemoveNodeFilterFromInput = "RelativePathDoesStartWith" }`

### "RemoveLastNodeFilter"

**YAML:** `RemoveLastNodeFilter`

Remove the last node [filter](filtering.md).

### "ResetNodeFilters"

**YAML:** `ResetNodeFilters`

Reset the node [filters](filtering.md) back to the default configuration.

### "ClearNodeFilters"

**YAML:** `ClearNodeFilters`

Clear all the node [filters](filtering.md).

### { AddNodeSorter = { sorter = "[NodeSorter](sorting.md#sorter)", reverse = bool } }

**YAML:** `AddNodeSorter: { sorter: [NodeSorter](sorting.md#sorter), reverse = bool }`

Add a [sorter](sorting.md#sorter) to sort nodes while exploring directories.

**YAML Example:** `AddNodeSorter: { sorter: ByRelativePath, reverse: false }`

**YAML Example:** `{ AddNodeSorter = { sorter = "ByRelativePath", reverse = false } }`


### { RemoveNodeSorter = "[NodeSorter](sorting.md#sorter)" }

**YAML:** `RemoveNodeSorter: [NodeSorter](sorting.md#sorter)`

Remove an existing [sorter](sorting.md#sorter).

**YAML Example:** `RemoveNodeSorter: ByRelativePath`

**Lua Example:** `{ RemoveNodeSorter = "ByRelativePath" }`

### { ReverseNodeSorter = "[NodeSorter](sorting.md#sorter)" }

**YAML:** `ReverseNodeSorter: [NodeSorter](sorting.md#sorter)`

Reverse a node [sorter](sorting.md#sorter).

**YAML Example:** `ReverseNodeSorter: ByRelativePath`

**Lua Example:** `{ ReverseNodeSorter = "ByRelativePath" }`

### { ToggleNodeSorter = { sorter = "[NodeSorter](sorting.md#sorter)", reverse = bool } }

**YAML:** `ToggleNodeSorter: { sorter: [NodeSorter](sorting.md#sorter), reverse = bool }`

Remove a [sorter](#sorting.md#sorter) if it exists, else, add a it.

**YAML Example:** `ToggleSorterSorter: {sorter: ByRelativePath, reverse: false }`

**Lua Example:** `{ ToggleSorterSorter: { sorter = "ByRelativePath", reverse = false } }`

### "ReverseNodeSorters"

**YAML:** `ReverseNodeSorters`

Reverse the node [sorters](sorting.md#sorter).

### "RemoveLastNodeSorter"

**YAML:** `RemoveLastNodeSorter`

Remove the last node [sorter](sorting.md#sorter).

### "ResetNodeSorters"

**YAML:** `ResetNodeSorters`

Reset the node [sorters](sorting.md#sorter) back to the default configuration.

### "ClearNodeSorters"

**YAML:** `ClearNodeSorters`

Clear all the node [sorters](sorting.md#sorter).

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

**YAML Example:**  `Debug: /path/to/file`

**Lua Example:**  `{ Debug = "/path/to/file" }`

### "Terminate"

**YAML:** `Terminate`

Terminate the application with a non-zero return code.


Lua Function Calls
------------------

xplr allows users to define lua functions using the `xplr.fn.custom` Lua API.

These functions can be called using messages like `CallLua`, `CallLuaSilently`.

When called the function receives a [special argument](#calllua-argument) that
contains some useful information. The function can optionally return a list of
messages which will be handled by xplr.

### CallLua Argument

This is a special argument passed to the lua functions when called using the
`CallLua`, `CallLuaSilently` messages.

It contains the following information:

- version
- config
- pwd
- focused_node
- directory_buffer
- selection
- mode
- layout
- input_buffer
- pid
- session_path
- explorer_config
- history
- last_modes

TODO: Document each. For now, refer to the
[rust doc](https://docs.rs/xplr/latest/xplr/app/struct.CallLuaArg.html#fields).

### Example:

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


Environment Variables and Pipes
-------------------------------

Alternative to `CallLua`, `CallLuaSilently` messages that call Lua functions,
there are `Call`, `CallSilently`, `BashExec`, `BashExecSilently` messages
that call shell commands.

However, unlike the Lua functions, these shell commands have to read the useful
information and send messages via environment variables and temporary files
called "pipe"s. These environment variables and files are only available when
a command is being executed.

Visit the [**fzf integration tutorial**](modes.html#tutorial-adding-a-new-mode)
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

- [XPLR_PIPE_MSG_IN](#xplr_pipe_msg_in)

#### Output pipes

`XPLR_PIPE_*_OUT` are the output pipes that contain data which cannot be
exposed directly via environment variables, like multi-line string.

- [XPLR_PIPE_SELECTION_OUT](#xplr_pipe_selection_out)
- [XPLR_PIPE_GLOBAL_HELP_MENU_OUT](#xplr_pipe_global_help_menu_out)
- [XPLR_PIPE_LOGS_OUT](#xplr_pipe_logs_out)
- [XPLR_PIPE_RESULT_OUT](#xplr_pipe_result_out)
- [XPLR_PIPE_HISTORY_OUT](#xplr_pipe_history_out)
- [XPLR_PIPE_DIRECTORY_NODES_OUT](#xplr_pipe_directory_nodes_out)

#### XPLR_PIPE_MSG_IN

Append new-line delimited messages to this pipe in [YAML](www.yaml.org) (or
[JSON](www.json.org)) syntax. These messages will be read and handled by xplr
after the command execution.

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
[files table](layouts.md#table).


### Example:

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

