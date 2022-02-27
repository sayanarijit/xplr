# Node Type

A node-type contains the following fields:

- [meta][4]
- [style][5]

### meta

Type: mapping of string and string

A meta field can contain custom metadata about a node. By default, the "icon"
metadata is set for the [directory][1], [file][2], and
[symlink][3] nodes.

Example:

```lua
xplr.config.node_types.file = {
  meta = {
    icon = "f",
    foo = "bar",
  }
}
```

## Also See:

- [xplr.config.node_types][6]

[1]: node_types.md#directory
[2]: node_types.md#file
[3]: node_types.md#symlink
[4]: #meta
[5]: style.md
[6]: node_types.md
