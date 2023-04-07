# Lua Function Calls

xplr allows you to define lua functions using the `xplr.fn.custom` Lua API.

These functions can be called using messages like `CallLua`, `CallLuaSilently`.

When called the function receives a [special argument][14] that
contains some useful information. The function can optionally return a list of
messages which will be handled by xplr.

## Example: Using Lua Function Calls

```lua
-- Define the function
xplr.fn.custom.ask_name_and_greet = function(app)
  print("What's your name?")

  local name = io.read()
  local greeting = "Hello " .. name .. "!"
  local message = greeting .. " You are inside " .. app.pwd

  return {
    { LogSuccess = message },
  }
end

-- Map the function to a key (space)
xplr.config.modes.builtin.default.key_bindings.on_key.space = {
  help = "ask name and greet",
  messages = {
    { CallLua = "custom.ask_name_and_greet" }
  }
}

-- Now, when you press "space" in default mode, you will be prompted for your
-- name. Enter your name to receive a nice greeting and to know your location.
```

Visit the [xplr.util][85] API docs for some useful utility / helper functions
that you can use in your Lua function calls.

## Lua Context

This is a special argument passed to the lua functions when called using the
`CallLua`, `CallLuaSilently` messages.

It contains the following information:

- [version][29]
- [pwd][31]
- [initial_pwd][76]
- [vroot][75]
- [focused_node][32]
- [directory_buffer][33]
- [selection][34]
- [mode][35]
- [layout][36]
- [input_buffer][37]
- [pid][38]
- [session_path][39]
- [explorer_config][40]
- [history][41]
- [last_modes][42]

### version

Type: string

xplr version. Can be used to test compatibility.

### pwd

Type: string

The present working directory.

### initial_pwd

Type: string

The initial working directory when xplr started.

### vroot

Type: nullable string

The current virtual root.

### focused_node

Type: nullable [Node][44]

The node under focus.

### directory_buffer

Type: nullable [Directory Buffer][62]

The directory buffer being rendered.

### selection

Type: list of selected [Node][44]s

The selected nodes.

### mode

Type: [Mode][8]

Current mode.

### layout

Type: [Layout][11]

Current layout.

### input_buffer

Type: nullable string

The input buffer.

### pid

Type: integer

The xplr session PID.

### session_path

Type: string

The session path.

### explorer_config

Type: [Explorer Config][66]

The configuration for exploring paths.

### history

Type: [History][70]

### last_modes

Type: list of [Mode][8]

Last modes, not popped yet.

## Node

A node contains the following fields:

- [parent][45]
- [relative_path][46]
- [absolute_path][47]
- [extension][48]
- [is_symlink][49]
- [is_broken][50]
- [is_dir][51]
- [is_file][52]
- [is_readonly][53]
- [mime_essence][54]
- [size][55]
- [human_size][56]
- [permissions][57]
- [created][71]
- [last_modified][72]
- [uid][73]
- [gid][74]
- [canonical][58]
- [symlink][59]

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

Like size but in human readable format.

### permissions

Type: [Permission][60]

The [permissions][60] applied to the node.

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

Type: nullable [Resolved Node Metadata][61]

If the node is a symlink, it will hold information about the symlink resolved
node. Else, it will hold information the actual node. It the symlink is broken,
it will be null.

### symlink

Type: nullable [Resolved Node Metadata][61]

If the node is a symlink and is not broken, it will hold information about the
symlink resolved node. However, it will never hold information about the actual
node. It will instead be null.

## Directory Buffer

Directory buffer contains the following fields:

- [parent][45]
- [nodes][63]
- [total][64]
- [focus][65]

### parent

Type: string

The parent path of the node.

### nodes

Type: list of [Node][44]s

A list of visible nodes.

### total

Type: int

The count of nodes being rendered.

### focus

Type: int

The index of the node under focus. It can be `0` even when there's no node to
focus on.

## History

History contains the following fields:

- [loc][68]
- [paths][69]

### loc

Type: int

Location of the current path in history.

### paths

Type: list of string

Visited paths.

## Explorer Config

Explorer config contains the following fields:

- [filters][77]
- [sorters][78]
- [searcher][79]

### filters

List of filters to apply.

Type: list of [Node Filter Applicable][80]

### sorters

Add list or sorters to the pipeline.

Type: list of [Node Sorter Applicable][81]

### searcher

The searcher to use (if any).

Type: nullable [Node Searcher Applicable][82]

## Also See:

- [xplr.util][85]

[7]: https://www.json.org
[8]: modes.md#mode
[9]: modes.md#builtin
[10]: modes.md#custom
[11]: layouts.md
[12]: layouts.md#builtin
[13]: layouts.md#custom
[14]: #lua-context
[15]: filtering.md#filter
[16]: filtering.md
[17]: sorting.md#sorter
[29]: #version
[30]: #config
[31]: #pwd
[32]: #focused_node
[33]: #directory_buffer
[34]: #selection
[35]: #mode
[36]: #layout
[37]: #input_buffer
[38]: #pid
[39]: #session_path
[40]: #explorer_config
[41]: #history
[42]: #last_modes
[43]: configuration.md#config
[44]: #node
[45]: #parent
[46]: #relative_path
[47]: #absolute_path
[48]: #extension
[49]: #is_symlink
[50]: #is_broken
[51]: #is_dir
[52]: #is_file
[53]: #is_readonly
[54]: #mime_essence
[55]: #size
[56]: #human_size
[57]: #permissions
[58]: #canonical
[59]: #symlink
[60]: column-renderer.md#permission
[61]: column-renderer.md#resolved-node-metadata
[62]: #directory-buffer
[63]: #nodes
[64]: #total
[65]: #focus
[66]: #explorer-config
[67]: #history
[68]: #loc
[69]: #paths
[70]: #history-1
[71]: #created
[72]: #last_modified
[73]: #uid
[74]: #gid
[75]: #vroot
[76]: #initial_pwd
[77]: #filters
[78]: #sorters
[79]: #searcher
[80]: filtering.md#node-filter-applicable
[81]: sorting.md#node-sorter-applicable
[82]: searching.md#node-searcher-applicable
[85]: xplr.util.md
