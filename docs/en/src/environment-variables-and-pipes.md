# Environment Variables and Pipes

Alternative to `CallLua`, `CallLuaSilently` messages that call Lua functions,
there are `Call0`, `CallSilently0`, `BashExec0`, `BashExecSilently0` messages
that call shell commands.

### Example: Simple file opener using xdg-open and $XPLR_FOCUS_PATH

```lua
xplr.config.modes.builtin.default.key_bindings.on_key["X"] = {
  help = "open",
  messages = {
    {
      BashExecSilently0 = [===[
        xdg-open "${XPLR_FOCUS_PATH:?}"
      ]===],
    },
  },
}
```

However, unlike the Lua functions, these shell commands have to read the useful
information and send messages via environment variables and temporary files
called "pipe"s. These environment variables and files are only available when
a command is being executed.

### Example: Using Environment Variables and Pipes

```lua
xplr.config.modes.builtin.default.key_bindings.on_key["space"] = {
  help = "ask name and greet",
  messages = {
    {
      BashExec0 = [===[
        echo "What's your name?"

        read name
        greeting="Hello $name!"
        message="$greeting You are inside $PWD"

        "$XPLR" -m 'LogSuccess: %q' "$message"
      ]===]
    }
  }
}

-- Now, when you press "space" in default mode, you will be prompted for your
-- name. Enter your name to receive a nice greeting and to know your location.
```

Visit the [**fzf integration tutorial**][19] for another example.

To see the environment variables and pipes, invoke the shell by typing `:!` in default
mode and run the following command:

```
env | grep ^XPLR
```

You will see something like:

```
XPLR=xplr
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
called ["pipe"s][18].

The other variables are single-line variables containing simple information:

- [XPLR][38]
- [XPLR_APP_VERSION][30]
- [XPLR_FOCUS_INDEX][31]
- [XPLR_FOCUS_PATH][32]
- [XPLR_INPUT_BUFFER][33]
- [XPLR_INITIAL_PWD][40]
- [XPLR_MODE][34]
- [XPLR_PID][35]
- [XPLR_SESSION_PATH][36]
- [XPLR_VROOT][39]

### Environment variables

#### XPLR

The binary path of xplr command.

#### XPLR_APP_VERSION

Self-explanatory.

#### XPLR_FOCUS_INDEX

Contains the index of the currently focused item, as seen in
[column-renderer/index][10].

#### XPLR_FOCUS_PATH

Contains the full path of the currently focused node.

#### XPLR_INITIAL_PWD

The $PWD then xplr started.

#### XPLR_INPUT_BUFFER

The line currently in displaying in the xplr input buffer. For e.g. the search
input while searching. See [Reading Input][37].

#### XPLR_MODE

Contains the mode xplr is currently in, see [modes][11].

#### XPLR_PID

Contains the process ID of the current xplr process.

#### XPLR_SESSION_PATH

Contains the current session path, like /tmp/runtime-"$USER"/xplr/session/"$XPLR_PID"/,
you can find temporary files here, such as pipes.

#### XPLR_VROOT

Contains the path of current virtual root, is set.

### Pipes

#### Input pipe

Currently there is only one input pipe.

- [XPLR_PIPE_MSG_IN][20]

#### Output pipes

`XPLR_PIPE_*_OUT` are the output pipes that contain data which cannot be
exposed directly via environment variables, like multi-line strings.
These pipes can be accessed as plain text files located in $XPLR_SESSION_PATH.

Depending on the message (e.g. `Call` or `Call0`), each line will be separated
by newline or null character (`\n` or `\0`).

- [XPLR_PIPE_SELECTION_OUT][21]
- [XPLR_PIPE_GLOBAL_HELP_MENU_OUT][22]
- [XPLR_PIPE_LOGS_OUT][23]
- [XPLR_PIPE_RESULT_OUT][24]
- [XPLR_PIPE_HISTORY_OUT][25]
- [XPLR_PIPE_DIRECTORY_NODES_OUT][26]

#### XPLR_PIPE_MSG_IN

Append new messages to this pipe in [YAML][27] (or [JSON][7]) syntax. These
messages will be read and handled by xplr after the command execution.

Depending on the message (e.g. `Call` or `Call0`) you need to separate each
message using newline or null character (`\n` or `\0`).

> **_NOTE:_** Since version `v0.20.0`, it's recommended to avoid writing
> directly to this file, as safely escaping YAML strings is a lot of work. Use
> `xplr -m` / `xplr --pipe-msg-in` to pass messages to xplr in a safer way.
>
> It uses [jf][41] syntax to safely convert an YAML template into a valid message.
>
> Example: `"$XPLR" -m 'ChangeDirectory: %q' "${HOME:?}"`

#### XPLR_PIPE_SELECTION_OUT

List of selected paths.

#### XPLR_PIPE_GLOBAL_HELP_MENU_OUT

The full help menu.

#### XPLR_PIPE_LOGS_OUT

List of logs.

#### XPLR_PIPE_RESULT_OUT

Result (selected paths if any, else the focused path)

#### XPLR_PIPE_HISTORY_OUT

List of last visited paths (similar to jump list in vim).

#### XPLR_PIPE_DIRECTORY_NODES_OUT

List of paths, filtered and sorted as displayed in the [files table][28].

[7]: https://www.json.org
[10]: column-renderer.md#index
[11]: modes.md#modes
[18]: #pipes
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
[30]: #xplr_app_version
[31]: #xplr_focus_index
[32]: #xplr_focus_path
[33]: #xplr_input_buffer
[34]: #xplr_mode
[35]: #xplr_pid
[36]: #xplr_session_path
[37]: messages.md#reading-input
[38]: #xplr
[39]: #xplr_vroot
[40]: #xplr_initial_pwd
[41]: https://github.com/sayanarijit/jf
