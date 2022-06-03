# Configuration

xplr can be configured using [Lua][1] via a special file named `init.lua`,
which can be placed in `~/.config/xplr/` (local to user) or `/etc/xplr/`
(global) depending on the use case.

When xplr loads, it first executes the [built-in init.lua][2] to set the
default values, which is then overwritten by another config file, if found
using the following lookup order:

1. `--config /path/to/init.lua`
2. `~/.config/xplr/init.lua`
3. `/etc/xplr/init.lua`

The first one found will be loaded by xplr and the lookup will stop.

The loaded config can be further extended using the `-C` or `--extra-config`
command-line option.

[1]: https://www.lua.org
[2]: https://github.com/sayanarijit/xplr/blob/main/src/init.lua
[3]: https://xplr.dev/en/upgrade-guide

## Config

The xplr configuration, exposed via `xplr.config` Lua API contains the
following sections.

See:

- [xplr.config.general](https://xplr.dev/en/general-config)
- [xplr.config.node_types](https://xplr.dev/en/node_types)
- [xplr.config.layouts](https://xplr.dev/en/layouts)
- [xplr.config.modes](https://xplr.dev/en/modes)

## Function

While `xplr.config` defines all the static parts of the configuration,
`xplr.fn` defines all the dynamic parts using functions.

See: [Lua Function Calls](https://xplr.dev/en/lua-function-calls)

As always, `xplr.fn.builtin` is where the built-in functions are defined
that can be overwritten.

#### xplr.fn.builtin.try_complete_path

Tries to auto complete the path in the input buffer

#### xplr.fn.builtin.fmt_general_table_row_cols_0

Renders the first column in the table

#### xplr.fn.builtin.fmt_general_table_row_cols_1

Renders the second column in the table

#### xplr.fn.builtin.fmt_general_table_row_cols_2

Renders the third column in the table

#### xplr.fn.builtin.fmt_general_table_row_cols_3

Renders the fourth column in the table

#### xplr.fn.builtin.fmt_general_table_row_cols_4

Renders the fifth column in the table

#### xplr.fn.custom

This is where the custom functions can be added.

There is currently no restriction on what kind of functions can be defined
in `xplr.fn.custom`.

You can also use nested tables such as
`xplr.fn.custom.my_plugin.my_function` to define custom functions.
