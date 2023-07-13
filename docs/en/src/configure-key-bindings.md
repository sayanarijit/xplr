# Configure Key Bindings

In xplr, each keyboard input passes through a bunch of handlers (e.g. `on_key`,
`on_number`, `default` etc.) in a given order. If any of the handlers is
configured to with an [action][16], it will intercept the key and produce
[messages][18] for xplr to handle.

Try [debug key bindings][31] to understand how key bindings actually work.

## Key Bindings

Key bindings contains the following information:

- [on_key][10]
- [on_alphabet][11]
- [on_number][12]
- [on_alphanumeric][32]
- [on_special_character][13]
- [on_character][33]
- [on_navigation][34]
- [on_function][35]
- [default][14]

### on_key

Type: mapping of [Key][15] to nullable [Action][16]

Defines what to do when an exact key is pressed.

### on_alphabet

Type: nullable [Action][16]

An action to perform if the keyboard input is an alphabet and is not mapped via
the [on_key][10] field.

### on_number

Type: nullable [Action][16]

An action to perform if the keyboard input is a number and is not mapped via
the [on_key][10] field.

### on_alphanumeric

Type: nullable [Action][16]

An action to perform if the keyboard input is alphanumeric and is not mapped
via the [on_key][10], [on_alphabet][11] or [on_number][12] field.

### on_special_character

Type: nullable [Action][16]

An action to perform if the keyboard input is a special character and is not
mapped via the [on_key][10] field.

### on_character

Type: nullable [Action][16]

An action to perform if the keyboard input is a character and is not mapped
via the [on_key][10], [on_alphabet][11], [on_number][12], [on_alphanumeric][32]
or [on_special_character][13] field.

### on_navigation

Type: nullable [Action][16]

An action to perform if the keyboard input is a navigation key and is not
mapped via the [on_key][10] field.

### on_function

Type: nullable [Action][16]

An action to perform if the keyboard input is a function key and is not mapped
via the [on_key][10] field.

### default

Type: nullable [Action][16]

Default action to perform in case if a keyboard input not mapped via any of the
`on_*` fields mentioned above.

## Key

A key is a [sum type][36] can be one of the following:

- 0, 1, ... 9
- a, b, ... z
- A, B, ... Z
- f1, f2, ... f12
- backspace
- left
- right
- up
- down
- home
- end
- page-up
- page-down
- back-tab
- delete
- insert
- enter
- tab
- esc
- ctrl-a, ctrl-b, ... ctrl-z
- ctrl-backspace, ctrl-left, ... ctrl-esc
- alt-a, alt-b, ... alt-z

And finally, the special characters - including space (`" "`) with their `ctrl`
bindings.

## Action

An action contains the following information:

- [help][1]
- [messages][17]

### help

Type: nullable string

Description of what it does. If unspecified, it will be excluded from the help
menu.

### messages

Type: A list of [Message][18] to send.

The list of messages to send when a key is pressed.

## Tutorial: Adding a New Mode

Assuming xplr is [installed][19] and [setup][20], let's
add our own mode to integrate xplr with [fzf][21].

We'll call it `fzxplr` mode.

First, let's add a custom mode called `fzxplr`, and map the key `F` to an
action that will call `fzf` to search and focus on a file or enter into a
directory.

```lua
xplr.config.modes.custom.fzxplr = {
  name = "fzxplr",
  key_bindings = {
    on_key = {
      F = {
        help = "search",
        messages = {
          {
            BashExec = [===[
              PTH=$(cat "${XPLR_PIPE_DIRECTORY_NODES_OUT:?}" | awk -F/ '{print $NF}' | fzf)
              if [ -d "$PTH" ]; then
                "$XPLR" -m 'ChangeDirectory: %q' "$PTH"
              else
                "$XPLR" -m 'FocusPath: %q' "$PTH"
              fi
            ]===]
          },
          "PopMode",
        },
      },
    },
    default = {
      messages = {
        "PopMode",
      },
    },
  },
}
```

As you can see, the key `F` in mode `fzxplr` (the name can be anything)
executes a script in `bash`.

`BashExec`, `PopMode`, `SwitchModeBuiltin`, `ChangeDirectory` and `FocusPath`
are [messages][18], `$XPLR`, `$XPLR_PIPE_DIRECTORY_NODES_OUT` are
[environment variables][22] exported by `xplr` before executing the command.
They contain the path to the [input][23] and [output][24] pipes that allows
external tools to interact with `xplr`.

Now that we have our new mode ready, let's add an entry point to this mode via
the `default` mode.

```lua
xplr.config.modes.builtin.default.key_bindings.on_key["F"] = {
  help = "fzf mode",
  messages = {
    { SwitchModeCustom = "fzxplr" },
  },
}
```

Now let's try out the new `xplr`-`fzf` integration.

[![xplr-fzf.gif][25]][26]

---

Visit [Awesome Plugins][27] for more [integration][28] options.

[1]: #help
[10]: #on_key
[11]: #on_alphabet
[12]: #on_number
[13]: #on_special_character
[14]: #default
[15]: #key
[16]: #action
[17]: #messages
[18]: message.md#message
[19]: install.md
[20]: post-install.md
[21]: https://github.com/junegunn/fzf
[22]: environment-variables-and-pipes.md#environment-variables
[23]: environment-variables-and-pipes.md#input-pipe
[24]: environment-variables-and-pipes.md#output-pipes
[25]: https://s3.gifyu.com/images/xplr-fzf.gif
[26]: https://gifyu.com/image/tW86
[27]: awesome-plugins.md
[28]: awesome-plugins.md#integration
[31]: debug-key-bindings.md
[32]: #on_alphanumeric
[33]: #on_character
[34]: #on_navigation
[35]: #on_function
[36]: sum-type.md
