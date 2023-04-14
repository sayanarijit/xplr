### Node Types

This section defines how to deal with different kinds of nodes (files,
directories, symlinks etc.) based on their properties.

One node can fall into multiple categories. For example, a node can have the
_extension_ `md`, and also be a _file_. In that case, the properties from
the more specific category i.e. _extension_ will be used.

This can be configured using the `xplr.config.node_types` Lua API.

#### xplr.config.node_types.directory.style

The style for the directory nodes

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.node_types.directory.meta.icon

Metadata for the directory nodes.
You can set as many metadata as you want.

Type: nullable string

Example:

```lua
xplr.config.node_types.directory.meta.foo = "foo"
xplr.config.node_types.directory.meta.bar = "bar"
```

#### xplr.config.node_types.file.style

The style for the file nodes.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.node_types.file.meta.icon

Metadata for the file nodes.
You can set as many metadata as you want.

Type: nullable string

Example:

```lua
xplr.config.node_types.file.meta.foo = "foo"
xplr.config.node_types.file.meta.bar = "bar"
```

#### xplr.config.node_types.symlink.style

The style for the symlink nodes.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.node_types.symlink.meta.icon

Metadata for the symlink nodes.
You can set as many metadata as you want.

Type: nullable string

Example:

```lua
xplr.config.node_types.symlink.meta.foo = "foo"
xplr.config.node_types.symlink.meta.bar = "bar"
```

#### xplr.config.node_types.mime_essence

Metadata and style based on mime types.
It is possible to use the wildcard `*` to match all mime sub types. It will
be overwritten by the more specific sub types that are defined.

Type: mapping of the following key-value pairs:

- key: string
- value:
  - key: string
  - value: [Node Type](https://xplr.dev/en/node-type)

Example:

```lua
xplr.config.node_types.mime_essence = {
  application = {
    -- application/*
    ["*"] = { meta = { icon = "a" } },

    -- application/pdf
    pdf = { meta = { icon = "" }, style = { fg = "Blue" } },

    -- application/zip
    zip = { meta = { icon = ""} },
  },
}
```

#### xplr.config.node_types.extension

Metadata and style based on extension.

Type: mapping of the following key-value pairs:

- key: string
- value: [Node Type](https://xplr.dev/en/node-type)

Example:

```lua
xplr.config.node_types.extension.md = { meta = { icon = "" }, style = { fg = "Blue" } }
xplr.config.node_types.extension.rs = { meta = { icon = "🦀" } }
```

#### xplr.config.node_types.special

Metadata and style based on special file names.

Type: mapping of the following key-value pairs:

- key: string
- value: [Node Type](https://xplr.dev/en/node-type)

Example:

```lua
xplr.config.node_types.special["Cargo.toml"] = { meta = { icon = "" } }
xplr.config.node_types.special["Downloads"] = { meta = { icon = "" }, style = { fg = "Blue" } }
```
