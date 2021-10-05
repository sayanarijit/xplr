# Layouts

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

xplr layouts define the structure of the UI, i.e. how many panel we see,
placement and size of the panels, how they look etc.

This is configuration exposed via the `xplr.config.layouts` API. It contains
the following fields:

- [builtin][1]
- [custom][2]

The users can switch between these layouts at run-time.

## builtin

Type: mapping of string and [Layout][3]

This is exposed by the `xplr.config.layouts.builtin` API.

xplr by default provides the following builtin layouts:

- [default][4]
- [no_help][5]
- [no_selection][6]
- [no_help_no_selection][7]

### default

Type: [Layout][3]

This is the default layout we see when we run xplr.

### no_help

Type: [Layout][3]

This layout hides the help menu.

### no_selection

Type: [Layout][3]

This layout hides the selection panel.

### no_help_no_selection

Type: [Layout][3]

This layout hides both the help menu and the selection panel.

## custom

Type: mapping of string and [Layout][3]

This is exposed by the `xplr.config.layouts.custom` API.

It allows the users to define any custom layout.

Example:

```lua
xplr.config.layouts.custom.example = "Nothing"
xplr.config.general.initial_layout = "example"

-- when you load xplr, you should see a blank screen
```

## Layout

A layout can be one of the following:

- ["Nothing"][8]
- ["Table"][9]
- ["InputAndLogs"][10]
- ["Selection"][11]
- ["HelpMenu"][12]
- ["SortAndFilter"][13]
- { [CustomContent][25] = { [title][33], [body][34] }
- { [Horizontal][14] = { [config][15], [splits][17] }
- { [Vertical][16] = { [config][15], [splits][17] }

### Nothing

This layout contains a blank panel.

### Table

This layout contains the table displaying the files and directories in the
current directory.

### InputAndLogs

This layout contains the panel displaying the input prompt and logs.

### Selection

This layout contains the panel displaying the selected paths.

### HelpMenu

This layout contains the panel displaying the help menu for the current mode in
real-time.

### SortAndFilter

This layout contains the panel displaying the pipeline of sorters and filters
applied of the list of paths being displayed.

### Horizontal

This is a special layout that splits the panel into two horizontal parts.

It contains the following information:

- [config][15]
- [splits][17]

### Vertical

This is a special layout that splits the panel into two vertical parts.

It contains the following information:

- [config][15]
- [splits][17]

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
- { MaxthLessThanLayoutWidth = int }
- { Min = int }
- { MinLessThanScreenHeight = int }
- { MinLessThanScreenWidth = int }
- { MinLessThanLayoutHeight = int }
- { MinLessThanLayoutWidth = int }

TODO: document each constraint.

## splits

Type: list of [Layout][3]

The list of child layouts to fit into the parent layout.

## Custom Content

Custom content is a special layout to render something custom.
It contains the following information:

- [title][33]
- [body][34]

### title

Type: nullable string

The title of the panel.

### body

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

### Static Paragraph

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

### Dynamic Paragraph

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

### Static List

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

### Dynamic List

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
  return ctx.app.history.paths
end
```

### Static Table

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

### Dynamic Table

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

## Size

It contains the following information:

- x
- y
- height
- width

Every field is of integer type.

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
[38]: message.md#calllua-argument
