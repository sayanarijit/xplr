# Layout

#### Example: Defining Custom Layout

[![layout.png][23]][24]

```lua
xplr.config.layouts.builtin.default = {
  Horizontal = {
    config = {
      margin = 1,
      horizontal_margin = 2,
      vertical_margin = 3,
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

A layout can be one of the following:

- [Nothing][8]
- [Table][9]
- [InputAndLogs][10]
- [Selection][11]
- [HelpMenu][12]
- [SortAndFilter][13]
- [CustomContent][25]
- [Horizontal][14]
- [Vertical][16]

### Nothing

This layout contains a blank panel.

Example: "Nothing"

### Table

This layout contains the table displaying the files and directories in the
current directory.

### InputAndLogs

This layout contains the panel displaying the input prompt and logs.

Example: "InputAndLogs"

### Selection

This layout contains the panel displaying the selected paths.

Example: "Selection"

### HelpMenu

This layout contains the panel displaying the help menu for the current mode in
real-time.

Example: "HelpMenu"

### SortAndFilter

This layout contains the panel displaying the pipeline of sorters and filters
applied of the list of paths being displayed.

Example: "SortAndFilter"

### Custom Content

Custom content is a special layout to render something custom.
It contains the following information:

- [title][33]
- [body][34]

Example: { CustomContent = { title = [title][33], body = [body][34] }

### Horizontal

This is a special layout that splits the panel into two horizontal parts.

It contains the following information:

- [config][15]
- [splits][17]

Example: { Horizontal = { config = [config][15], splits = [splits][17] }

### Vertical

This is a special layout that splits the panel into two vertical parts.

It contains the following information:

- [config][15]
- [splits][17]

Example: { Vertical = { config = [config][15], splits = [splits][17] }

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

A constraint can be one of the following:

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

TODO: document each constraint.

## splits

Type: list of [Layout][3]

The list of child layouts to fit into the parent layout.

## title

Type: nullable string

The title of the panel.

## body

Type: [Content Body][26]

The body of the panel.

## Content Body

Content body can be one of the following:

- [StaticParagraph][27]
- [DynamicParagraph][28]
- [StaticList][29]
- [DynamicList][30]
- [StaticTable][31]
- [DynamicTable][32]

## Static Paragraph

A paragraph to render. It contains the following fields:

- **render** (string): The string to render.

#### Example: Render a custom static paragraph

```lua
xplr.config.layouts.builtin.default = {
  CustomContent = {
    title = "custom title",
    body = {
      StaticParagraph = { render = "custom body" },
    },
  },
}
```

## Dynamic Paragraph

A [Lua function][35] to render a custom paragraph.
It contains the following fields:

- **render** (string): The [lua function][35] that returns the paragraph to
  render.

#### Example: Render a custom dynamic paragraph

```lua
xplr.config.layouts.builtin.default = {
  CustomContent = {
    title = "custom title",
    body = { DynamicParagraph = { render = "custom.render_layout" } },
  },
}

xplr.fn.custom.render_layout = function(ctx)
  return ctx.app.pwd
end
```

## Static List

A list to render. It contains the following fields:

- **render** (list of string): The list to render.

#### Example: Render a custom static list

```lua
xplr.config.layouts.builtin.default = {
  CustomContent = {
    title = "custom title",
    body = {
      StaticList = { render = { "1", "2", "3" } },
    },
  },
}
```

## Dynamic List

A [Lua function][35] to render a custom list.
It contains the following fields:

- **render** (string): The [lua function][35] that returns the list to render.

#### Example: Render a custom dynamic list

```lua
xplr.config.layouts.builtin.default = {
  CustomContent = {
    title = "custom title",
    body = { DynamicList = { render = "custom.render_layout" } },
  },
}

xplr.fn.custom.render_layout = function(ctx)
  return {
    ctx.app.pwd,
    ctx.app.version,
    tostring(ctx.app.pid),
  }
end
```

## Static Table

A table to render. It contains the following fields:

- **widths** (list of [Constraint][22]): Width of the columns.
- **col_spacing** (nullable int): Spacing between columns. Defaults to 1.
- **render** (list of list of string): The rows and columns to render.

#### Example: Render a custom static table

```lua
xplr.config.layouts.builtin.default = {
  CustomContent = {
    title = "custom title",
    body = {
      StaticTable = {
        widths = {
          { Percentage = 50 },
          { Percentage = 50 },
        },
        col_spacing = 1,
        render = {
          { "a", "b" },
          { "c", "d" },
        },
      },
    },
  },
}
```

## Dynamic Table

A [Lua function][35] to render a custom table.
It contains the following fields:

- **widths** (list of [Constraint][22]): Width of the columns.
- **col_spacing** (nullable int): Spacing between columns. Defaults to 1.
- **render** (string): The [lua function][35] that returns the table to render.

#### Example: Render a custom dynamic table

```lua
xplr.config.layouts.builtin.default = {
  CustomContent = {
    title = "custom title",
    body = {
      DynamicTable = {
        widths = {
          { Percentage = 50 },
          { Percentage = 50 },
        },
        col_spacing = 1,
        render = "custom.render_layout",
      },
    },
  },
}

xplr.fn.custom.render_layout = function(ctx)
  return {
    { "", "" },
    { "Layout height", tostring(ctx.layout_size.height) },
    { "Layout width", tostring(ctx.layout_size.width) },
    { "", "" },
    { "Screen height", tostring(ctx.screen_size.height) },
    { "Screen width", tostring(ctx.screen_size.width) },
  }
end
```

## Content Renderer

It is a Lua function that receives [a special argument][36] as input and
returns some output that can be rendered in the UI. It is used to render
content body for the custom dynamic layouts.

## Content Renderer Argument

It contains the following information:

- [layout_size][37]
- [screen_size][37]
- [app][38]

### Size

It contains the following information:

- x
- y
- height
- width

Every field is of integer type.

### app

This is a lightweight version of the [Lua Context][39]. In this context, the
heavyweight fields like [directory_buffer][50] are omitted for performance
reasons.

Hence, only the following fields are avilable.

- [version][40]
- [pwd][41]
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
[23]: https://s6.gifyu.com/images/layout.png
[24]: https://gifyu.com/image/1X38
[25]: #custom-content
[26]: #content-body
[27]: #static-paragraph
[28]: #dynamic-paragraph
[29]: #static-list
[30]: #dynamic-list
[31]: #static-table
[32]: #dynamic-table
[33]: #title
[34]: #body
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
[52]: #vroot
