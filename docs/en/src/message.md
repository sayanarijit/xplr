Message
=======

You can think of xplr as a server. Just like web servers listen to HTTP
requests, xplr listens to messages.

See the
[**full list of messages**](https://docs.rs/xplr/latest/xplr/app/enum.ExternalMsg.html#variants)
that xplr can handle.

You can send messages to an xplr session in the following ways:

- Via [key bindings](modes.md#key-bindings)
- Via [Lua function calls](#lua-function-calls)
- Via shell using the [input pipe](#input-pipe)


Format
------

To send messages using [key bindings](modes.md#key-bindings) or
[Lua functions calls](#lua-functions-calls), these are represented in
[Lua](https://www.lua.org/) syntax. For example:

- "Quit"
- { FocusPath = "/path/to/file" }
- { Call = { command = "bash", args = { "-c", "read -p test" } } }

However, to send messages using the [input pipe](#input-pipe), they need to be
represented using
[YAML](http://yaml.org/) (or [JSON](https://www.json.org)) syntax. For example:

- Foo
- FocusPath: "/path/to/file"
- Call: { command: bash, args: ["-c", "read -p test"] }


Lua Function Calls
------------------

xplr allows users to define lua functions using the `xplr.fn.custom` Lua API.

These functions can be called using messages like `CallLua`, `CallLuaSilently`.

When called the function receives a [special argument](#calllua-argument) that
contains some useful information. The function can optionally return a list of
messages which will be handled by xplr.

Example:

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
  help = "and & greet",
  messages = {
    { CallLua = "custom.ask_name_and_greet" }
  }
}

-- Now, when you press "space" in default mode, you will be prompted for your
-- name. Enter your name to receive a nice greeting and to know your location.
```

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


Environment Variables and Pipes
-------------------------------

Alternative to `CallLua`, `CallLuaSilently` messages that call Lua functions,
there are `Call`, `CallLuaSilently`, `BashExec`, `BashExecSilently` messages
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

### Input pipe

Current there is only one input pipe.

- [XPLR_PIPE_MSG_IN](#xplr_pipe_msg_in)

### Output pipes

`XPLR_PIPE_*_OUT` are the output pipes that contain data which cannot be
exposed directly via environment variables, like multi-line string.

- [XPLR_PIPE_SELECTION_OUT](#xplr_pipe_selection_out)
- [XPLR_PIPE_GLOBAL_HELP_MENU_OUT](#xplr_pipe_global_help_menu_out)
- [XPLR_PIPE_LOGS_OUT](#xplr_pipe_logs_out)
- [XPLR_PIPE_RESULT_OUT](#xplr_pipe_result_out)
- [XPLR_PIPE_HISTORY_OUT](#xplr_pipe_history_out)
- [XPLR_PIPE_DIRECTORY_NODES_OUT](#xplr_pipe_directory_nodes_out)

### XPLR_PIPE_MSG_IN

Append new-line delimited messages to this pipe in [YAML](www.yaml.org) (or
[JSON](www.json.org)) syntax. These messages will be read and handled by xplr
after the command execution.

### XPLR_PIPE_SELECTION_OUT

New-line delimited list of selected paths.

### XPLR_PIPE_GLOBAL_HELP_MENU_OUT

The full help menu.

### XPLR_PIPE_LOGS_OUT

New-line delimited list of logs.

### XPLR_PIPE_RESULT_OUT

New-line delimited result (selected paths if any, else the focused path)

### XPLR_PIPE_HISTORY_OUT

New-line delimited list of last visited paths (similar to jump list in vim).

### XPLR_PIPE_DIRECTORY_NODES_OUT

New-line delimited list of paths, filtered and sorted as displayed in the
[files table](layouts.md#table).
