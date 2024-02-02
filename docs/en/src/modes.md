### Modes

xplr is a modal file explorer. That means the users switch between different
modes, each containing a different set of key bindings to avoid clashes.
Users can switch between these modes at run-time.

The modes can be configured using the `xplr.config.modes` Lua API.

`xplr.config.modes.builtin` contain some built-in modes which can be
overridden, but you can't add or remove modes in it.

#### xplr.config.modes.builtin.default

The builtin default mode.
Visit the [Default Key Bindings](https://xplr.dev/en/default-key-bindings)
to see what each mode does.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.debug_error

The builtin debug error mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.recover

The builtin recover mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.go_to_path

The builtin go to path mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.move_to

The builtin move_to mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.copy_to

The builtin copy_to mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.selection_ops

The builtin selection ops mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.create

The builtin create mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.create_directory

The builtin create directory mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.create_file

The builtin create file mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.number

The builtin number mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.go_to

The builtin go to mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.rename

The builtin rename mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.duplicate_as

The builtin duplicate as mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.delete

The builtin delete mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.action

The builtin action mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.quit

The builtin quit mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.search

The builtin search mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.filter

The builtin filter mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.relative_path_does_match_regex

The builtin relative_path_does_match_regex mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.relative_path_does_not_match_regex

The builtin relative_path_does_not_match_regex mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.sort

The builtin sort mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.switch_layout

The builtin switch layout mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.vroot

The builtin vroot mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.builtin.edit_permissions

The builtin edit permissions mode.

Type: [Mode](https://xplr.dev/en/mode)

#### xplr.config.modes.custom

This is where you define custom modes.

Type: mapping of the following key-value pairs:

- key: string
- value: [Mode](https://xplr.dev/en/mode)

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
          { SwitchModeBuiltin = "default" },
        },
      },
    },
  },
}

xplr.config.general.initial_mode = "example"
```
