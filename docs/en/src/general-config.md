General Config
==============

This configuration is exposed via the `xplr.config.general` API. It contains
the following fields:


enable_mouse
------------

Type: boolean

Set it to `true` enable scrolling using mouse.


show_hidden
-----------

Type: boolean

Set it to `true` to show hidden files.


read_only
---------

Type: boolean

Set it to `true` to use only a subset of selected operations that forbids
executing commands or performing write operations on the file-system.


disable_recover_mode
--------------------

Type: boolean

Set it to `true` when the special recover mode gets too annoying to appreciate
the good intentions. When enabled, typing the wrong keys won't result in any
action.


cursor.format
-------------

Type: nullable string

This is the shape of the cursor visible when the input buffer contains some string.


cursor.style
------------

Type: [Style](style.md)

Style of the cursor.


initial_layout
--------------

Type: string

The name of one of the [layout](layouts.md) to use when xplr loads.


initial_mode
------------

Type: string

The name of one of the [mode](modes.md) to use when xplr loads.


initial_sorting
---------------

Type: list of [Node Sorter Applicable](sorting.md#node-sorter-applicable)

Initial group if sorters applied to the nodes list in the table.


table.style
-----------

Type: [Style](style.md)

Default style of the table.


table.col_spacing
-----------------

Type: nullable integer

Default spacing of the columns in the table.


table.col_widths
----------------

Type: nullable list of [Constraint](layouts.md#constraint)

Width of each column in the table.

table.header.height
----------------

Type: nullable integer

Height of the table header.


table.header.style
---------------

Type: [Style](style.md)

Style of table header.


table.header.cols
-----------------

Type: List of column configuration

Each column config contains `format` field (string) and `style` field
([Style](style.md)), that define the content and style of header.


table.row.height
----------------

Type: nullable integer

Height of each row in the table.


table.row.style
---------------

Type: [Style](style.md)

Style of table rows.


table.row.cols
-----------------

Type: List of column configuration

Each column config contains `format` field (string) and `style` field
([Style](style.md)).

However, unlike [table.header.cols](#tableheadercols), the `format` field here
points to a Lua function that receives a
[special argument](https://docs.rs/xplr/latest/xplr/ui/struct.NodeUiMetadata.html)
as input and returns a string that will be displayed in the column.

TODO: Document the argument fields here.

xplr by default provides the following functions:

- `xplr.fn.builtin.fmt_general_table_row_cols_0`
- `xplr.fn.builtin.fmt_general_table_row_cols_1`
- `xplr.fn.builtin.fmt_general_table_row_cols_2`
- `xplr.fn.builtin.fmt_general_table_row_cols_3`
- `xplr.fn.builtin.fmt_general_table_row_cols_4`

You can either overwrite these functions, or create new functions in
`xplr.fn.custom` and point to them.

Terminal colors are supported.

Example:

```lua
xplr.fn.custom.fmt_simple_column = function(m)
  return m.prefix .. m.relative_path .. m.suffix
end

xplr.config.general.table.header.cols = {
  { format = "  path" }
}

xplr.config.general.table.row.cols = {
  { format = "custom.fmt_simple_column" }
}

xplr.config.general.table.col_widths = {
  { Percentage = 100 }
}

-- With this config, you should only see a single column displaying the
-- relative paths.
```

------------

TODO: Continue documentation
