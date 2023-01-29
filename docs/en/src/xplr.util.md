### xplr.util.version

Get the xplr version details.

Type: function() -> { major: number, minor: number, patch: number }

Example:

```lua
xplr.util.version()
-- { major = 0, minor = 0, patch = 0 }
```

### xplr.util.clone

Clone/deepcopy a Lua value. Doesn't work with functions.

Type: function( value ) -> value

Example:

```lua
local val = { foo = "bar" }
local val_clone = xplr.util.clone(val)
val.foo = "baz"
print(val_clone.foo)
-- "bar"
```

### xplr.util.exists

Check if the given path exists.

Type: function( path:string ) -> boolean

Example:

```lua
xplr.util.exists("/foo/bar")
-- true
```

### xplr.util.is_dir

Check if the given path is a directory.

Type: function( path:string ) -> boolean

Example:

```lua
xplr.util.is_dir("/foo/bar")
-- true
```

### xplr.util.is_file

Check if the given path is a file.

Type: function( path:string ) -> boolean

Example:

```lua
xplr.util.is_file("/foo/bar")
-- true
```

### xplr.util.is_symlink

Check if the given path is a symlink.

Type: function( path:string ) -> boolean

Example:

```lua
xplr.util.is_file("/foo/bar")
-- true
```

### xplr.util.is_absolute

Check if the given path is an absolute path.

Type: function( path:string ) -> boolean

Example:

```lua
xplr.util.is_absolute("/foo/bar")
-- true
```

### xplr.util.path_split

Split a path into its components.

Type: function( path:string ) -> boolean

Example:

```lua
xplr.util.path_split("/foo/bar")
-- { "/", "foo", "bar" }

xplr.util.path_split(".././foo")
-- { "..", "foo" }
```

### xplr.util.node

Get [Node][5] information of a given path.
Doesn't check if the path exists.
Returns nil if the path is "/".
Errors out if absolute path can't be obtained.

Type: function( path:string ) -> [Node][5]|nil

Example:

```lua
xplr.util.node("./bar")
-- { parent = "/pwd", relative_path = "bar", absolute_path = "/pwd/bar", ... }

xplr.util.node("/")
-- nil
```

[5]: https://xplr.dev/en/lua-function-calls#node

### xplr.util.dirname

Get the directory name of a given path.

Type: function( path:string ) -> path:string|nil

Example:

```lua
xplr.util.dirname("/foo/bar")
-- "/foo"
```

### xplr.util.basename

Get the base name of a given path.

Type: function( path:string ) -> path:string|nil

Example:

```lua
xplr.util.basename("/foo/bar")
-- "bar"
```

### xplr.util.absolute

Get the absolute path of the given path by prepending $PWD.
It doesn't check if the path exists.

Type: function( path:string ) -> path:string

Example:

```lua
xplr.util.absolute("foo/bar")
-- "/tmp/foo/bar"
```

### xplr.util.explore

Explore directories with the given explorer config.

Type: function( path:string, [ExplorerConfig][1]|nil ) -> { [Node][2], ... }

Example:

```lua

xplr.util.explore("/tmp")
-- { { absolute_path = "/tmp/a", ... }, ... }

xplr.util.explore("/tmp", app.explorer_config)
-- { { absolute_path = "/tmp/a", ... }, ... }
```

[1]: https://xplr.dev/en/lua-function-calls#explorer-config
[2]: https://xplr.dev/en/lua-function-calls#node

### xplr.util.shell_execute

Execute shell commands safely.

Type: function( program:string, args:{ string, ... }|nil ) -> { stdout = string, stderr = string, returncode = number|nil }

Example:

```lua
xplr.util.shell_execute("pwd")
-- "/present/working/directory"

xplr.util.shell_execute("bash", {"-c", "xplr --help"})
-- { stdout = "xplr...", stderr = "", returncode = 0 }
```

### xplr.util.shell_quote

Quote commands and paths safely.

Type: function( string ) -> string

Example:

```lua
xplr.util.shell_quote("a'b\"c")
-- 'a'"'"'b"c'
```

### xplr.util.from_json

Load JSON string into Lua value.

Type: function( string ) -> any

Example:

```lua
xplr.util.from_json([[{"foo": "bar"}]])
-- { foo = "bar" }
```

### xplr.util.to_json

Dump Lua value into JSON (i.e. also YAML) string.

Type: function( value ) -> string

Example:

```lua
xplr.util.to_json({ foo = "bar" })
-- [[{ "foo": "bar" }]]

xplr.util.to_json({ foo = "bar" }, { pretty = true })
-- [[{
--   "foo": "bar"
-- }]]
```

### xplr.util.from_yaml

Load YAML (i.e. also JSON) string into Lua value.

Type: function( string ) -> value

Example:

```lua
xplr.util.from_yaml([[{foo: bar}]])
-- { foo = "bar" }
```

### xplr.util.to_yaml

Dump Lua value into YAML string.

Type: function( value ) -> string

Example:

```lua
xplr.util.to_yaml({ foo = "bar" })
-- "foo: bar"
```

### xplr.util.lscolor

Get a [Style][3] object for the given path based on the LS_COLORS
environment variable.

Type: function( path:string ) -> Style[3]|nil

Example:

```lua
xplr.util.lscolor("Desktop")
-- { fg = "Red", bg = nil, add_modifiers = {}, sub_modifiers = {} }
```

[3]: https://xplr.dev/en/style

### xplr.util.paint

Apply style (escape sequence) to string using a given [Style][3] object.

Type: function( string, [Style][3]|nil ) -> string

Example:

```lua
xplr.util.paint("Desktop", { fg = "Red", bg = nil, add_modifiers = {}, sub_modifiers = {} })
-- "\u001b[31mDesktop\u001b[0m"
```

### xplr.util.relative_to

Get the relative path based on the given base path or current working dir.
Will error if it fails to determine a relative path.

Type: function( path:string, config:Config|nil ) -> path:string

Config type: { base:string|nil, with_prefix_dots:bookean|nil, without_suffix_dots:boolean|nil }

- If `base` path is given, the path will be relative to it.
- If `with_prefix_dots` is true, the path will always start with dots `..` / `.`
- If `without_suffix_dots` is true, the name will be visible instead of dots `..` / `.`

Example:

```lua
xplr.util.relative_to("/present/working/directory")
-- "."

xplr.util.relative_to("/present/working/directory/foo")
-- "foo"

xplr.util.relative_to("/present/working/directory/foo", { with_prefix_dots = true })
-- "./foo"

xplr.util.relative_to("/present/working/directory", { without_suffix_dots = true })
-- "../directory"

xplr.util.relative_to("/present/working")
-- ".."

xplr.util.relative_to("/present/working", { without_suffix_dots = true })
-- "../../working"

xplr.util.relative_to("/present/working/directory", { base = "/present/foo/bar" })
-- "../../working/directory"
```

### xplr.util.shortened

Shorten the given absolute path using the following rules:

- either relative to your home dir if it makes sense
- or relative to the current working directory
- or absolute path if it makes the most sense

Type: Similar to `xplr.util.relative_to`

Example:

```lua
xplr.util.shortened("/home/username/.config")
-- "~/.config"

xplr.util.shortened("/present/working/directory")
-- "."

xplr.util.shortened("/present/working/directory/foo")
-- "foo"

xplr.util.shortened("/present/working/directory/foo", { with_prefix_dots = true })
-- "./foo"

xplr.util.shortened("/present/working/directory", { without_suffix_dots = true })
-- "../directory"

xplr.util.shortened("/present/working/directory", { base = "/present/foo/bar" })
-- "../../working/directory"

xplr.util.shortened("/tmp")
-- "/tmp"
```

### xplr.util.textwrap

Wrap the given text to fit the specified width.
It will try to not split words when possible.

Type: function( string, options:number|table ) -> { string, ...}

Options type: { width = number, initial_indent = string|nil, subsequent_indent = string|nil, break_words = boolean|nil }

Example:

```lua
xplr.util.textwrap("this will be cut off", 11)
-- { "this will', 'be cut off" }

xplr.util.textwrap(
  "this will be cut off",
  { width = 12, initial_indent = "", subsequent_indent = "    ", break_words = false }
)
-- { "this will be", "    cut off" }
```

### xplr.util.layout_replace

Find the target layout in the given layout and replace it with the replacement layout,
returning a new layout.

Type: function( layout:[Layout][4], target:[Layout][4], replacement:[Layout][4] ) -> layout:[Layout][4]

Example:

```lua
local layout = {
  Horizontal = {
    splits = {
      "Table",  -- Target
      "HelpMenu",
    },
    config = ...,
  }
}

xplr.util.layout_replace(layout, "Table", "Selection")
-- {
--   Horizontal = {
--     splits = {
--       "Selection",  -- Replaced
--       "HelpMenu",
--     },
--     config = ...
--   }
-- }
```

[4]: https://xplr.dev/en/layout
