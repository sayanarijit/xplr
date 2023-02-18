# Column Renderer

A column renderer is a Lua function that receives a [special argument][1] and
returns a string that will be displayed in each specific field of the
[files table][2].

## Example: Customizing Table Renderer

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

xplr by default provides the following column renderers:

- `xplr.fn.builtin.fmt_general_table_row_cols_0`
- `xplr.fn.builtin.fmt_general_table_row_cols_1`
- `xplr.fn.builtin.fmt_general_table_row_cols_2`
- `xplr.fn.builtin.fmt_general_table_row_cols_3`
- `xplr.fn.builtin.fmt_general_table_row_cols_4`

You can either overwrite these functions, or create new functions in
`xplr.fn.custom` and point to them.

Terminal colors are supported.

## Table Renderer Argument

The special argument contains the following fields

- [parent][3]
- [relative_path][4]
- [absolute_path][5]
- [extension][6]
- [is_symlink][7]
- [is_broken][8]
- [is_dir][9]
- [is_file][10]
- [is_readonly][11]
- [mime_essence][12]
- [size][13]
- [human_size][14]
- [permissions][15]
- [created][34]
- [last_modified][35]
- [uid][36]
- [gid][37]
- [canonical][16]
- [symlink][17]
- [index][18]
- [relative_index][19]
- [is_before_focus][20]
- [is_after_focus][21]
- [tree][22]
- [prefix][23]
- [suffix][24]
- [is_selected][25]
- [is_focused][26]
- [total][27]
- [style][38]
- [meta][28]

### parent

Type: string

The parent path of the node.

### relative_path

Type: string

The path relative to the parent, i.e. the file/directory name with extension.

### absolute_path

Type: string

The absolute path (without resolving symlinks) of the node.

### extension

Type: string

The extension of the node.

### is_symlink

Type: boolean

`true` if the node is a symlink.

### is_broken

Type: boolean

`true` if the node is a broken symlink.

### is_dir

Type: boolean

`true` if the node is a directory.

### is_file

Type: boolean

`true` if the node is a file.

### is_readonly

Type: boolean

`true` if the node is real-only.

### mime_essence

Type: string

The mime type of the node. For e.g. `text/csv`, `image/jpeg` etc.

### size

Type: integer

The size of the exact node. The size of a directory won't be calculated
recursively.

### human_size

Type: string

Like [size][29] but in human readable format.

### permissions

Type: [Permission][30]

The [permissions][30] applied to the node.

### created

Type: nullable integer

Creation time in nanosecond since UNIX epoch.

### last_modified

Type: nullable integer

Last modification time in nanosecond since UNIX epoch.

### uid

Type: integer

User ID of the file owner.

### gid

Type: integer

Group ID of the file owner.

### canonical

Type: nullable [Resolved Node Metadata][31]

If the node is a symlink, it will hold information about the symlink resolved
node. Else, it will hold information the actual node. It the symlink is broken,
it will be null.

### symlink

Type: nullable [Resolved Node Metadata][31]

If the node is a symlink and is not broken, it will hold information about the
symlink resolved node. However, it will never hold information about the actual
node. It will instead be null.

### index

Type: integer

Index (starting from 0) of the node.

### relative_index

Type: integer

Relative index from the focused node (i.e. 0th node).

### is_before_focus

Type: boolean

`true` if the node is before the focused node.

### is_after_focus

Type: boolean

`true` if the node is after the focused node.

### tree

Type: string

The [tree component][32] based on the node's index.

### prefix

Type: string

The prefix applicable for the node.

### suffix

Type: string

The suffix applicable for the node.

### is_selected

Type: boolean

`true` if the node is selected.

### is_focused

Type: boolean

`true` if the node is under focus.

### total

Type: integer

The total number of the nodes.

### style

Type: [Style][39]

The applicable [style object][39] for the node.

### meta

Type: mapping of string and string

The applicable [meta object][33] for the node.

## Permission

Permission contains the following fields:

- user_read
- user_write
- user_execute
- group_read
- group_write
- group_execute
- other_read
- other_write
- other_execute
- sticky
- setgid
- setuid

Each field holds a boolean value.

## Resolved Node Metadata

It contains the following fields.

- [absolute_path][5]
- [extension][6]
- [is_dir][9]
- [is_file][10]
- [is_readonly][11]
- [mime_essence][12]
- [size][13]
- [human_size][14]
- [created][34]
- [last_modified][35]
- [uid][36]
- [gid][37]

[1]: #table-renderer-argument
[2]: layout.md#table
[3]: #parent
[4]: #relative_path
[5]: #absolute_path
[6]: #extension
[7]: #is_symlink
[8]: #is_broken
[9]: #is_dir
[10]: #is_file
[11]: #is_readonly
[12]: #mime_essence
[13]: #size
[14]: #human_size
[15]: #permissions
[16]: #canonical
[17]: #symlink
[18]: #index
[19]: #relative_index
[20]: #is_before_focus
[21]: #is_after_focus
[22]: #tree
[23]: #prefix
[24]: #suffix
[25]: #is_selected
[26]: #is_focused
[27]: #total
[28]: #meta
[29]: #size
[30]: #permission
[31]: #resolved-node-metadata
[32]: general-config.md#tabletree
[33]: node-type.md#meta
[34]: #created
[35]: #last_modified
[36]: #uid
[37]: #gid
[38]: #style
[39]: style.md#style
