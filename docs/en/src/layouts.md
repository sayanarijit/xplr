Layouts
=======

xplr layouts define the structure of the UI, i.e. how many panel we see,
placement and size of the panels, how they look etc.

This is configuration exposed via the `xplr.config.layouts` API. It contains
the following fields:

- [builtin][1]
- [custom][2]

The users can switch between these layouts at run-time.


builtin
-------

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


custom
------

Type: mapping of string and [Layout][3]

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

- ["Nothing"][8]
- ["Table"][9]
- ["InputAndLogs"][10]
- ["Selection"][11]
- ["HelpMenu"][12]
- ["SortAndFilter"][13]
- { [Horizontal][14] = { config = [Layout Config][15], splits = { [Layout][3], ... } }
- { [Vertical][16] = { config = [Layout Config][15], splits = { [Layout][3], ... } }

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


Layout Config
-------------

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

Type: list of [Layout][3]

The list of child layouts to fit into the parent layout.


Example
-------

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


[1]:#builtin
[2]:#custom
[3]:#layout
[4]:#default
[5]:#no_help
[6]:#no_selection
[7]:#no_help_no_selection
[8]:#nothing
[9]:#table
[10]:#inputandlogs
[11]:#selection
[12]:#helpmenu
[13]:#sortandfilter
[14]:#horizontal
[15]:#layout-config
[16]:#vertical
[17]:#splits
[18]:#margin
[19]:#horizontal_margin
[20]:#vertical_margin
[21]:#constraints
[22]:#constraint
[23]:https://s6.gifyu.com/images/layout.png
[24]:https://gifyu.com/image/1X38