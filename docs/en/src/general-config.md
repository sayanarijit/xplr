# General Config

This configuration is exposed via the `xplr.config.general` API. It contains
the following fields:

## enable_mouse

Type: boolean

Set it to `true` enable scrolling using mouse.

## show_hidden

Type: boolean

Set it to `true` to show hidden files.

## read_only

Type: boolean

Set it to `true` to use only a subset of selected operations that forbids
executing commands or performing write operations on the file-system.

## enable_recover_mode

Type: boolean

Set it to `true` if you want to enable a safety feature that will save you from
yourself when you type recklessly.

## hide_remaps_in_help_menu

Type: boolean

Set it to `true` if you want to hide all remaps in the help menu.

## initial_layout

Type: string

The name of one of the [layout][2] to use when xplr loads.

## initial_mode

Type: string

The name of one of the [mode][3] to use when xplr loads.

## initial_sorting

Type: list of [Node Sorter Applicable][4]

Initial group if sorters applied to the nodes list in the table.

## table.style

Type: [Style][1]

Default style of the table.

## table.col_spacing

Type: nullable integer

Default spacing of the columns in the table.

## table.col_widths

Type: nullable list of [Constraint][5]

Width of each column in the table.

## table.header.height

Type: nullable integer

Height of the table header.

## table.header.style

Type: [Style][1]

Style of table header.

## table.header.cols

Type: list of column configuration

Each column config contains `format` field (string) and `style` field
([Style][1]), that define the content and style of header.

## table.row.height

Type: nullable integer

Height of each row in the table.

## table.row.style

Type: [Style][1]

Style of table rows.

## table.row.cols

Type: list of column configuration

Each column config contains `format` field (string) and `style` field
([Style][1]).

However, unlike [table.header.cols][6], the `format` field here
points to a [column renderer function][7].

## table.tree

Type: list of tree configuration

It expects a list of three items. The first component of the tree, then the
middle components, and finally the last component of the tree.

Each item requires the `format` field which is a string, and the `style` field,
which is the [Style][1] object.

Example:

```lua
xplr.config.general.table.tree = {
    { format = "├─", style = { add_modifiers = { "Bold" }, bg = nil, fg = "Blue", sub_modifiers = nil } },
    { format = "├─", style = { add_modifiers = { "Bold" }, bg = nil, fg = "Blue", sub_modifiers = nil } },
    { format = "╰─", style = { add_modifiers = { "Bold" }, bg = nil, fg = "Blue", sub_modifiers = nil } },
}
```

## start_fifo

Type: nullable string

Start a FIFO file when xplr loads. It will track the absolute path of the file
under focus.

## prompt.format

Type: string

This is the shape of the prompt for the input buffer.

## prompt.style

Type: [Style][1]

Style of the input prompt.

## logs.error.format

Type: string

The string to indicate an error in logs.

## logs.error.style

Type: [Style][1]

Style for errors shown in logs.

## logs.info.format

Type: string

The string to indicate an information in logs.

## logs.info.style

Type: [Style][1]

Style for infos shown in logs.

## logs.success.format

Type: string

The string to indicate a successful operation in logs.

## logs.success.style

Type: [Style][1]

Style for successful operations in logs.

## logs.warning.format

Type: string

The string that indicates a warning in logs.

## logs.warning.style

Type: [Style][1]

Style for warnings in logs.

## default_ui.prefix

Type: nullable string

The content that is placed before the item name for each row.

## default_ui.suffix

Type: nullable string

The content which is appended to each item name for each row.

## default_ui.style

Type: [Style][1]

Style for the default user interface.

## focus_ui.prefix

Type: nullable string

The string placed before the item name for a focused row.

## focus_ui.suffix

Type: nullable string

The string placed after the item name for a focused row.

## focus_ui.style

Type: [Style][1]

Style for focused elements.

## selection_ui.prefix

Type: nullable string

The string that is placed before the item name for a selected row.

## selection_ui.suffix

Type: nullable string

The string placed after the item name for a selected row.

## selection_ui.style

Type: [Style][1]

Style for selected rows.

## focus_selection_ui.prefix

Type: nullable string

The string placed before item name for a selected row that gets the focus.

## focus_selection_ui.suffix

Type: nullable string

The string placed after the item name for a selected row that gets the focus.

## focus_selection_ui.style

Type: [Style][1]

Style for a selected row that gets the focus.

## sort_and_filter_ui.separator.format

Type: nullable string

The shape of the separator for the `Sort & filter` panel.

## sort_and_filter_ui.separator.style

Type: [Style][1]

Style for `Sort & filter` panel separator.

## sort_and_filter_ui.default_identifier.format

Type: nullable string

The content of the default identifier in `Sort & filter` panel.

## sort_and_filter_ui.default_identifier.style

Type: [Style][1]

Style for the default identifier in `Sort & filter` panel.

## sort_and_filter_ui.filter_identifiers

Type: Table with [filter name][8] as key

The value contains format field (string) and style field (Style), that define
the content and style for the given filter.

Example:

```lua
xplr.config.general.sort_and_filter_ui.filter_identifiers.AbsolutePathDoesContain = {
  format = "abs=~",
  style = { add_modifiers = { "Bold" }, bg = nil, fg = nil, sub_modifiers = nil },
}
```

## sort_and_filter_ui.sort_direction_identifiers.forward.format

Type: nullable string

The shape of the forward direction indicator for sort identifiers in
`Sort & filter` panel.

## sort_and_filter_ui.sort_direction_identifiers.forward.style

Type: [Style][1]

Style of forward direction indicator.

## sort_and_filter_ui.sort_direction_identifiers.reverse.format

Type: nullable string

The shape of the reverse direction indicator for sort identifiers in
`Sort & filter` panel.

## sort_and_filter_ui.sort_direction_identifiers.reverse.style

Type: [Style][1]

Style of reverse direction indicator.

## sort_and_filter_ui.sorter_identifiers

Type: Table with [sorter name][9] as key

The value contains format field (string) and style field (Style), that define
the content and style for the given sorter.

Example:

```lua
sort_and_filter_ui.sorter_identifiers.ByCanonicalAbsolutePath = {
  format = "[c]abs",
  style = { add_modifiers = nil, bg = nil, fg = "Yellow", sub_modifiers = nil },
}
```

## panel_ui.default.borders

Type: nullable list of strings

Defines where to show borders for the panels.

The possible values are any combination of: "Top", "Bottom", "Left" and
"Right".

Example:

```lua
-- Show Left and Right borders only
xplr.config.general.panel_ui.default.borders = {
  "Right",
  "Left",
}
```

## panel_ui.default.style

Type: [Style][1]

Default style for panels.

## panel_ui.default.title.format

Type: nullable string

The content for panel title.

## panel_ui.default.title.style

Type: [Style][1]

Style for panel title.

## panel_ui.help_menu.borders

Type: nullable list of strings

Defines where to show borders for the `Help` panel.

The possible values are any combination of: "Top", "Bottom", "Left" and
"Right".

## panel_ui.help_menu.style

Type: [Style][1]

Style for `Help` panel.

## panel_ui.help_menu.title.format

Type: nullable string

The content for `Help` panel title.

## panel_ui.help_menu.title.style

Type: [Style][1]

Style for `Help` panel title.

## panel_ui.input_and_logs.borders

Type: nullable list of strings

Defines where to show borders for the `Input` & `Logs` panel.

The possible values are any combination of: "Top", "Bottom", "Left" and
"Right".

## panel_ui.input_and_logs.style

Type: [Style][1]

Style for `Input` & `Logs` panel.

## panel_ui.input_and_logs.title.format

Type: nullable string

The content for the `Input` and `Logs` panel title.

## panel_ui.input_and_logs.title.style

Type: [Style][1]

Style for `Input` & `Logs` panel title.

## panel_ui.selection.borders

Type: nullable list of strings

Defines where to show borders for the `Selection` panel.

The possible values are any combination of: "Top", "Bottom", "Left" and
"Right".

## panel_ui.selection.style

Type: [Style][1]

Style for `Selection` panel.

## panel_ui.selection.title.format

Type: nullable string

The content for `Selection` panel title.

## panel_ui.selection.title.style

Type: [Style][1]

Style for `Selection` panel title.

## panel_ui.sort_and_filter.borders

Type: nullable list of strings

Defines where to show borders for the `Sort & filter` panel.

The possible values are any combination of: "Top", "Bottom", "Left" and
"Right".

## panel_ui.sort_and_filter.style

Type: [Style][1]

Style for `Sort & filter` panel.

## panel_ui.sort_and_filter.title.format

Type: nullable string

The content for `Sort & filter` panel title.

## panel_ui.sort_and_filter.title.style

Type: [Style][1]

Style for `Sort & filter` panel title.

[1]: style.md
[2]: layouts.md
[3]: modes.md
[4]: sorting.md#node-sorter-applicable
[5]: layouts.md#constraint
[6]: #tableheadercols
[7]: column-renderer.md
[8]: filtering.md#filter
[9]: sorting.md#sorter
