Modes
=====

xplr is a modal file explorer. That means the users switch between different
modes, each containing a different set of key bindings to avoid clashes. Users
can switch between these modes at run-time.

The modes can be configured using the `xplr.config.modes` Lua API.

It contains the following fields:

- [builtin](#builtin)
- [custom](#custom)


builtin
-------

Type: mapping of string and [Mode](#mode)

This is exposed by the `xplr.config.modes.builtin` API.

xplr by default provides the following builtin modes:

- default
- recover
- selection_ops
- create
- create_directory
- create_file
- number
- go_to
- rename
- delete
- action
- search
- filter
- relative_path_does_contain
- relative_path_does_not_contain
- sort
- switch_layout
- quit

Visit the [Default Key Bindings](default-key-bindings.md) to see what each mode
does.


custom
------

Type: mapping of string and [Mode](#mode)

This is exposed by the `xplr.config.layouts.custom` API.

It allows the users to define custom modes.

Example:

```lua
xplr.config.modes.custom.example = {
  name = "example",
  key_bindings = {
    on_key = {
      enter = {
        help = "default mode",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "default" }
        }
      }
    }
  }
}

xplr.config.general.initial_mode = "example"

-- when you load xplr, you should be in the "example" mode,
-- pressing "enter" should take you to the "default" mode.
```


Mode
----

A mode contains the following information:

- [name](#name)
- [help](#help)
- [extra_help](#extra_help)
- [key_bindings](#key_bindings)

### name

Type: string

This is the name of the mode visible in the help menu.

### help

Type: nullable string

If specified, the help menu will display this instead of the auto generated
mappings.

### extra_help

Type: nullable string

If specified, the help menu will display this along-side the auto generated
help menu.

### key_bindings

Type: [Key Bindings](#key-bindings)

The key bindings available in that mode.


Key Bindings
------------

Key bindings define how each keyboard input will be handled in a specific mode.

See the [default key bindings](default-key-bindings.md) for example.

Key bindings contains the following information:

- [on_key](#on_key)
- [on_alphabet](#on_alphabet)
- [on_number](#on_number)
- [on_special_character](#on_special_character)
- [default](#default)

### on_key

Type: mapping of [Key](#key) to nullable [Action](#action)

Defines what to do when a specific key is pressed.

### on_alphabet

Type: nullable [Action](#action)

An action to perform if the keyboard input is an alphabet and is not mapped via
the [on_key](#on_key) field.

### on_number

Type: nullable [Action](#action)

An action to perform if the keyboard input is a number and is not mapped via
the [on_key](#on_key) field.

### on_special_character

Type: nullable [Action](#action)

An action to perform if the keyboard input is a special character and is not
mapped via the [on_key](#on_key) field.

### default

Type: nullable [Action](#action)

Default action to perform in case of a keyboard input not mapped via any of the
[on_key](#on_key), [on_alphabet](#on_alphabet), [on_number](#on_number) or
[on_special_character](#on_special_character) field.


Action
------

An action contains the following information:

- help
- [messages](#messages)

### help

Type: nullable string

Description of what it does. If unspecified, it will be excluded from the help
menu.

### messages

Type: A list of [Message](message.md) to send.

The list of messages to send when a key is pressed.


Tutorial: Adding a New Mode
---------------------------

Assuming xplr is [installed](install.md) and [setup](post-install.md), let's
add our own mode to integrate xplr with [fzf](https://github.com/junegunn/fzf).

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
              echo ChangeDirectory: "'"${PWD:?}/${PTH:?}"'" >> "${XPLR_PIPE_MSG_IN:?}"
            else
              echo FocusPath: "'"${PWD:?}/${PTH:?}"'" >> "${XPLR_PIPE_MSG_IN:?}"
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
are [messages](message.md), `$XPLR_PIPE_MSG_IN`,
`$XPLR_PIPE_DIRECTORY_NODES_OUT` are
[environment variables](message.md#environment-variables) exported by `xplr`
before executing the command. They contain the path to the
[input](message.md#input-pipe) and [output](message.md#output-pipes) pipes that
allows external tools to interact with `xplr`.

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

[![xplr-fzf.gif](https://s3.gifyu.com/images/xplr-fzf.gif)](https://gifyu.com/image/tW86)

-----

Visit [Awesome Plugins](awesome-plugins.md) for more [integration](awesome-plugins.md#integration) options.
