# Layout

#### Example: Defining Custom Layout

```lua
xplr.config.layouts.builtin.default = {
  Horizontal = {
    config = {
      margin = 1,
      horizontal_margin = 1,
      vertical_margin = 1,
      constraints = {
        { Percentage = 50 },
        { Percentage = 50 },
      }
    },
    splits = {
      "Table",
      "HelpMenu",
    }
  }
}
```

Result:

```
 ╭ /home ─────────────╮╭ Help [default] ────╮
 │   ╭─── path        ││.    show hidden    │
 │   ├▸[ð Desktop/]   ││/    search         │
 │   ├  ð Documents/  ││:    action         │
 │   ├  ð Downloads/  ││?    global help    │
 │   ├  ð GitHub/     ││G    go to bottom   │
 │   ├  ð Music/      ││V    select/unselect│
 │   ├  ð Pictures/   ││ctrl duplicate as   │
 │   ├  ð Public/     ││ctrl next visit     │
 ╰────────────────────╯╰────────────────────╯
```

A layout is a [sum type][56] can be one of the following:

- [Nothing][8]
- [Table][9]
- [InputAndLogs][10]
- [Selection][11]
- [HelpMenu][12]
- [SortAndFilter][13]
- [Static][25]
- [Dynamic][26]
- [Horizontal][14]
- [Vertical][16]
- CustomContent (deprecated, use `Static` or `Dynamic`)

### Nothing

This layout contains a blank panel.

Type: "Nothing"

### Table

This layout contains the table displaying the files and directories in the current
directory.

### InputAndLogs

This layout contains the panel displaying the input prompt and logs.

Type: "InputAndLogs"

### Selection

This layout contains the panel displaying the selected paths.

Type: "Selection"

### HelpMenu

This layout contains the panel displaying the help menu for the current mode in
real-time.

Type: "HelpMenu"

### SortAndFilter

This layout contains the panel displaying the pipeline of sorters and filters applied on
the list of paths being displayed.

Type: "SortAndFilter"

### Static

This is a custom layout to render static content.

Type: { Static = [Custom Panel][27] }

### Dynamic

This is a custom layout to render dynamic content using a function defined in
[xplr.fn][28] that takes [Content Renderer Argument][36] and returns [Custom Panel][27].

Type: { Dynamic = "[Content Renderer][35]" }

### Horizontal

This is a special layout that splits the panel into two horizontal parts.

It contains the following information:

- [config][15]
- [splits][17]

Type: { Vertical = { config = [Layout Config][15], splits = { [Layout][17], ... } }

### Vertical

This is a special layout that splits the panel into two vertical parts.

It contains the following information:

- [config][15]
- [splits][17]

Type: { Vertical = { config = [Layout Config][15], splits = { [Layout][17], ... } }

## Layout Config

A layout config contains the following information:

- [margin][18]
- [horizontal_margin][19]
- [vertical_margin][20]
- [constraints][21]

### margin

Type: nullable integer

The width of the margin in all direction.

### horizontal_Margin

Type: nullable integer

The width of the horizontal margins. Overwrites the [margin][18] value.

### vertical_Margin

Type: nullable integer

The width of the vertical margins. Overwrites the [margin][18] value.

### constraints

Type: nullable list of [Constraint][22]

The constraints applied on the layout.

## Constraint

A constraint is a [sum type][56] can be one of the following:

- { Percentage = int }
- { Ratio = { int, int } }
- { Length = { int }
- { LengthLessThanScreenHeight = int }
- { LengthLessThanScreenWidth = int }
- { LengthLessThanLayoutHeight = int }
- { LengthLessThanLayoutWidth = int }
- { Max = int }
- { MaxLessThanScreenHeight = int }
- { MaxLessThanScreenWidth = int }
- { MaxLessThanLayoutHeight = int }
- { MaxLessThanLayoutWidth = int }
- { Min = int }
- { MinLessThanScreenHeight = int }
- { MinLessThanScreenWidth = int }
- { MinLessThanLayoutHeight = int }
- { MinLessThanLayoutWidth = int }

## splits

Type: list of [Layout][3]

The list of child layouts to fit into the parent layout.

## Custom Panel

Custom panel is a [sum type][56] can be one of the following:

- [CustomParagraph][29]
- [CustomList][30]
- [CustomTable][31]
- [CustomLayout][55]

### CustomParagraph

A paragraph to render. It contains the following fields:

- **ui** (nullable [Panel UI Config][32]): Optional UI config for the panel.
- **body** (string): The string to render.

#### Example: Render a custom static paragraph

```lua
xplr.config.layouts.builtin.default = {
  Static = {
    CustomParagraph = {
      ui = { title = { format = " custom title " } },
      body = "custom body",
    },
  },
}
```

Result:

```
╭ custom title ────────╮
│custom body           │
│                      │
│                      │
╰──────────────────────╯
```

#### Example: Render a custom dynamic paragraph

```lua
xplr.config.layouts.builtin.default = { Dynamic = "custom.render_layout" }

xplr.fn.custom.render_layout = function(ctx)
  return {
    CustomParagraph = {
      ui = { title = { format = ctx.app.pwd } },
      body = xplr.util.to_yaml(ctx.app.focused_node),
    },
  }
end
```

Result:

```
╭/home/sayanarijit───────────────────────────╮
│mime_essence: inode/directory               │
│relative_path: Desktop                      │
│is_symlink: false                           │
│is_readonly: false                          │
│parent: /home/sayanarijit                   │
│absolute_path: /home/sayanarijit/Desktop    │
│is_broken: false                            │
│created: 1668087850396758714                │
│size: 4096                                  │
│gid: 100                                    │
╰────────────────────────────────────────────╯
```

### CustomList

A list to render. It contains the following fields:

- **ui** (nullable [Panel UI Config][32]): Optional UI config for the panel.
- **body** (list of string): The list of strings to display.

#### Example: Render a custom static list

```lua
xplr.config.layouts.builtin.default = {
  Static = {
    CustomList = {
      ui = { title = { format = " custom title " } },
      body = { "1", "2", "3" },
    },
  },
}
```

Result:

```
╭ custom title ─────────────╮
│1                          │
│2                          │
│3                          │
│                           │
╰───────────────────────────╯
```

#### Example: Render a custom dynamic list

```lua
xplr.config.layouts.builtin.default = { Dynamic = "custom.render_layout" }

xplr.fn.custom.render_layout = function(ctx)
  return {
    CustomList = {
      ui = { title = { format = ctx.app.pwd } },
      body = {
        (ctx.app.focused_node or {}).relative_path or "",
        ctx.app.version,
        tostring(ctx.app.pid),
      },
    },
  }
end
```

Result:

```
╭/home/sayanarijit──────────╮
│Desktop                    │
│0.21.2                     │
│17336                      │
│                           │
│                           │
╰───────────────────────────╯
```

## CustomTable

A custom table to render. It contains the following fields:

- **ui** (nullable [Panel UI Config][32]): Optional UI config for the panel.
- **widths** (list of [Constraint][22]): Width of the columns.
- **col_spacing** (nullable int): Spacing between columns. Defaults to 1.
- **body** (list of list of string): The rows and columns to render.

#### Example: Render a custom static table

```lua
xplr.config.layouts.builtin.default = {
  Static = {
    CustomTable = {
      ui = { title = { format = " custom title " } },
      widths = {
        { Percentage = 50 },
        { Percentage = 50 },
      },
      body = {
        { "a", "b" },
        { "c", "d" },
      },
    },
  },
}
```

Result:

```
╭ custom title ────────────────────╮
│a                 b               │
│c                 d               │
│                                  │
│                                  │
│                                  │
╰──────────────────────────────────╯
```

#### Example: Render a custom dynamic table

```lua
xplr.config.layouts.builtin.default = {Dynamic = "custom.render_layout" }

xplr.fn.custom.render_layout = function(ctx)
  return {
    CustomTable = {
      ui = { title = { format = ctx.app.pwd } },
      widths = {
        { Percentage = 50 },
        { Percentage = 50 },
      },
      body = {
        { "", "" },
        { "Layout height", tostring(ctx.layout_size.height) },
        { "Layout width", tostring(ctx.layout_size.width) },
        { "", "" },
        { "Screen height", tostring(ctx.screen_size.height) },
        { "Screen width", tostring(ctx.screen_size.width) },
      },
    },
  }
end
```

Result:

```
╭/home/sayanarijit───────────────────────────╮
│                                            │
│Layout height          12                   │
│Layout width           46                   │
│                                            │
│Screen height          12                   │
│Screen width           46                   │
│                                            │
│                                            │
│                                            │
│                                            │
╰────────────────────────────────────────────╯
```

### CustomLayout

A whole custom layout to render. It doesn't make sense to use it as a
[Static][25] layout, but can be very useful to render as a [Dynamic][26] layout
for use cases where the structure of the layout needs to change without having
to switch modes.

> WARNING: Rendering the same dynamic custom layout recursively will result in
> a ugly crash.

#### Example: Render a custom dynamic layout

```lua
xplr.config.layouts.builtin.default = { Dynamic = "custom.render_layout" }

xplr.fn.custom.render_layout = function(ctx)
  local inner = {
    config = {
      constraints = {
        { Percentage = 50 },
        { Percentage = 50 },
      },
    },
    splits = {
      { Static = { CustomParagraph = { body = "Try your luck..." } } },
      { Static = { CustomParagraph = { body = "Press ctrl-r" } } },
    },
  }

  local layout_type = "Vertical"
  if math.random(1, 2) == 1 then
    layout_type = "Horizontal"
  end

  return { CustomLayout = { [layout_type] = inner } }
end
```

Result:

```
╭─────────────────────╮╭─────────────────────╮
│Try your luck...     ││Press ctrl-r         │
│                     ││                     │
│                     ││                     │
│                     ││                     │
│                     ││                     │
│                     ││                     │
│                     ││                     │
│                     ││                     │
│                     ││                     │
│                     ││                     │
╰─────────────────────╯╰─────────────────────╯
```

Or

```
╭────────────────────────────────────────────╮
│Try your luck...                            │
│                                            │
│                                            │
│                                            │
╰────────────────────────────────────────────╯
╭────────────────────────────────────────────╮
│Press ctrl-r                                │
│                                            │
│                                            │
│                                            │
╰────────────────────────────────────────────╯
```

## Panel UI Config

It contains the following optional fields:

- **title** ({ format = "string", style = [Style][33] }): the title of the panel.
- **style** ([Style][33]): The style of the panel body.
- **borders** (nullable list of [Border][34]): The shape of the borders.
- **border_type** ([Border Type][54]): The type of the borders.
- **border_style** ([Style][33]): The style of the borders.

## Content Renderer

It is a Lua function that receives [a special argument][36] as input and
returns some output that can be rendered in the UI. It is used to render
content body for the custom dynamic layouts.

## Content Renderer Argument

It contains the following information:

- [layout_size][37]
- [screen_size][37]
- [scrolltop][57]
- [app][38]

### Size

It contains the following information:

- x
- y
- height
- width

Every field is of integer type.

### scrolltop

Type: integer

The start index of the visible nodes in the table.

### app

This is a lightweight version of the [Lua Context][39]. In this context, the
heavyweight fields like [directory_buffer][50] are omitted for performance
reasons.

Hence, only the following fields are available.

- [version][40]
- [pwd][41]
- [initial_pwd][53]
- [vroot][52]
- [focused_node][42]
- [selection][43]
- [mode][44]
- [layout][45]
- [input_buffer][46]
- [pid][47]
- [session_path][48]
- [explorer_config][49]

## Also See:

- [xplr.config.layouts][51]

[1]: #builtin
[2]: #custom
[3]: #layout
[4]: #default
[5]: #no_help
[6]: #no_selection
[7]: #no_help_no_selection
[8]: #nothing
[9]: #table
[10]: #inputandlogs
[11]: #selection
[12]: #helpmenu
[13]: #sortandfilter
[14]: #horizontal
[15]: #layout-config
[16]: #vertical
[17]: #splits
[18]: #margin
[19]: #horizontal_margin
[20]: #vertical_margin
[21]: #constraints
[22]: #constraint
[25]: #static
[26]: #dynamic
[27]: #custom-panel
[28]: configuration.md#function
[29]: #customparagraph
[30]: #customlist
[31]: #customtable
[32]: #panel-ui-config
[33]: style.md#style
[34]: borders.md#border
[35]: #content-renderer
[36]: #content-renderer-argument
[37]: #size
[38]: #app
[39]: lua-function-calls.md#lua-context
[40]: lua-function-calls.md#version
[41]: lua-function-calls.md#pwd
[42]: lua-function-calls.md#focused_node
[43]: lua-function-calls.md#selection
[44]: lua-function-calls.md#mode
[45]: lua-function-calls.md#layout
[46]: lua-function-calls.md#input_buffer
[47]: lua-function-calls.md#pid
[48]: lua-function-calls.md#session_path
[49]: lua-function-calls.md#explorer_config
[50]: lua-function-calls.md#directory_buffer
[51]: layouts.md
[52]: lua-function-calls.md#vroot
[53]: lua-function-calls.md#initial_pwd
[54]: borders.md#border-type
[55]: #customlayout
[56]: sum-type.md
[57]: #scrolltop
