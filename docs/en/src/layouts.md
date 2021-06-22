Layouts
=======

xplr layouts define the structure of the UI, i.e. how many panel we see,
placement and size of the panels, how they look etc.

This is configuration exposed via the `xplr.config.layouts` API. It contains
the following fields:

- [builtin](#builtin)
- [custom](#custom)

The users can switch between these layouts at run-time.


builtin
-------

Type: mapping of string and [Layout](#layout)

This is exposed by the `xplr.config.layouts.builtin` API.

xplr by default provides the following builtin layouts:

- [default](#default)
- [no_help](#no_help)
- [no_selection](#no_selection)
- [no_help_no_selection](#no_help_no_selection)

### default

Type: [Layout](#layout)

This is the default layout we see when we run xplr.

### no_help

Type: [Layout](#layout)

This layout hides the help menu.

### no_selection

Type: [Layout](#layout)

This layout hides the selection panel.

### no_help_no_selection

Type: [Layout](#layout)

This layout hides both the help menu and the selection panel.


custom
------

Type: mapping of string and [Layout](#layout)

This is exposed by the `xplr.config.layouts.custom` API.

It allows the users to define any custom layout.

Example:

```lua
xplr.config.layouts.custom.example = "Nothing"
xplr.config.general.initial_layout = "example"

-- when you load xplr, you should see a blank screen
```

Layout
------

A layout can be one of the following:

- ["Nothing"](#nothing)
- ["Table"](#table)
- ["InputAndLogs"](#inputandlogs)
- ["Selection"](#selection)
- ["HelpMenu"](#helpmenu)
- ["SortAndFilter"](#sortandfilter)
- { [Horizontal](#horizontal) = { config = [Layout Config](#layout-config), splits = { [Layout](#layout), ... } }
- { [Vertical](#vertical) = { config = [Layout Config](#layout-config), splits = { [Layout](#layout), ... } }

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

- [config](#layout-config)
- [splits](#splits)

### Vertical

This is a special layout that splits the panel into two vertical parts.

It contains the following information:

- [config](#layout-config)
- [splits](#splits)


Layout Config
-------------

A layout config contains the following information:

- [margin](#margin)
- [horizontal_margin](#horizontal_margin)
- [vertical_margin](#vertical_margin)
- [constraints](#constraints)

### margin

Type: nullable integer

The width of the margin in all direction.

### horizontal_Margin

Type: nullable integer

The width of the horizontal margins. Overwrites the [margin](#margin) value.

### vertical_Margin

Type: nullable integer

The width of the vertical margins. Overwrites the [margin](#margin) value.

### constraints

Type: nullable list of [Constraint](#constraint)

The constraints applied on the layout.


Constraint
----------

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


splits
------

Type: list of [Layout](#layout)

The list of child layouts to fit into the parent layout.


Example
-------

[![layout.png](https://s6.gifyu.com/images/layout.png)](https://gifyu.com/image/1X38)

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
