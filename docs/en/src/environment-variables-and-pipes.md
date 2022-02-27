# Environment Variables and Pipes

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

[7]: https://www.json.org
[19]: configure-key-bindings.md#tutorial-adding-a-new-mode
[20]: #xplr_pipe_msg_in
[21]: #xplr_pipe_selection_out
[22]: #xplr_pipe_global_help_menu_out
[23]: #xplr_pipe_logs_out
[24]: #xplr_pipe_result_out
[25]: #xplr_pipe_history_out
[26]: #xplr_pipe_directory_nodes_out
[27]: https://www.yaml.org
[28]: layout.md#table
