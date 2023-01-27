### xplr.util.version

Get the xplr version details.

Type: function() -> { major: number, minor: number, patch: number }

Example:

```lua
xplr.util.version()
-- { major = 0, minor = 0, patch = 0 }
```

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

Type: function( path:string ) -> [Style][3]|nil

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

Type: function( path:string, base:string ) -> path:string

Example:

```lua
xplr.util.relative_to("/present/working/directory")
-- "."

xplr.util.relative_to("/present/working")
-- ".."

xplr.util.relative_to("/present/working/directory", "/present/foo/bar")
-- "../../working/directory"
```

### xplr.util.path_shorthand

Display the given path in shorthand form using the following rules:

- either relative to your home dir if it makes sense
- or relative to the optional base path / current working directory
- or absolute path if it makes the most sense

Type: function( path:string, base:string|nil ) -> path:string|nil

Example:

```lua
xplr.util.path_shorthand("/home/username/.config")
-- "~/.config"

xplr.util.path_shorthand("/present/working/directory")
-- "../directory"

xplr.util.path_shorthand("/present/working/directory", "/present/foo/bar")
-- "../../working/directory"
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
