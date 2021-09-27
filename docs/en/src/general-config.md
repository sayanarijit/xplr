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


enable_recover_mode
--------------------

Type: boolean

Set it to `true` if you want to enable a safety feature that will save you from
yourself when you type recklessly.


cursor.format
-------------

Type: nullable string

This is the shape of the cursor visible when the input buffer contains some string.


cursor.style
------------

Type: [Style][1]

Style of the cursor.


initial_layout
--------------

Type: string

The name of one of the [layout][2] to use when xplr loads.


initial_mode
------------

Type: string

The name of one of the [mode][3] to use when xplr loads.


initial_sorting
---------------

Type: list of [Node Sorter Applicable][4]

Initial group if sorters applied to the nodes list in the table.


table.style
-----------

Type: [Style][1]

Default style of the table.


table.col_spacing
-----------------

Type: nullable integer

Default spacing of the columns in the table.


table.col_widths
----------------

Type: nullable list of [Constraint][5]

Width of each column in the table.

table.header.height
----------------

Type: nullable integer

Height of the table header.


table.header.style
---------------

Type: [Style][1]

Style of table header.


table.header.cols
-----------------

Type: List of column configuration

Each column config contains `format` field (string) and `style` field
([Style][1]), that define the content and style of header.


table.row.height
----------------

Type: nullable integer

Height of each row in the table.


table.row.style
---------------

Type: [Style][1]

Style of table rows.


table.row.cols
-----------------

Type: List of column configuration

Each column config contains `format` field (string) and `style` field
([Style][1]).

However, unlike [table.header.cols][6], the `format` field here
points to a [column renderer function][7].


table.tree
-----------------

Type: List of tree configuration

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

TODO: Continue documentation.


[1]:style.md
[2]:layouts.md
[3]:modes.md
[4]:sorting.md#node-sorter-applicable
[5]:layouts.md#constraint
[6]:#tableheadercols
[7]:column-renderer.md
