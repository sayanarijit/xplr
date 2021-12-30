# Modes

xplr is a modal file explorer. That means the users switch between different
modes, each containing a different set of key bindings to avoid clashes. Users
can switch between these modes at run-time.

The modes can be configured using the `xplr.config.modes` Lua API.

It contains the following fields:

- [builtin][1]
- [custom][2]

## builtin

Type: mapping of string and [Mode][3]

This is exposed by the `xplr.config.modes.builtin` API.

xplr by default provides the following builtin modes:

- default
- debug
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

Visit the [Default Key Bindings][4] to see what each mode
does.

## custom

Type: mapping of string and [Mode][3]

This is exposed by the `xplr.config.modes.custom` API.

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

## Mode

A mode contains the following information:

- [name][5]
- [help][6]
- [extra_help][7]
- [key_bindings][8]
- [layout][10]

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

Type: [Key Bindings][9]

The key bindings available in that mode.

### layout

Type: nullable [Layout][11]

If specified, this layout will be used to render the UI.

[1]: #builtin
[2]: #custom
[3]: #mode
[4]: default-key-bindings.md
[5]: #name
[6]: #help
[7]: #extra_help
[8]: configure-key-bindings.md#key-bindings
[9]: #key-bindings
[10]: #layout
[11]: layouts.md#Layout
