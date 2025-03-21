### General Configuration

The general configuration properties are grouped together in
`xplr.config.general`.

#### xplr.config.general.disable_debug_error_mode

Set it to `true` if you want to ignore the startup errors. You can still see
the errors in the logs.

Type: boolean

#### xplr.config.general.enable_mouse

Set it to `true` if you want to enable mouse scrolling.

Type: boolean

#### xplr.config.general.show_hidden

Set it to `true` to show hidden files by default.

Type: boolean

#### xplr.config.general.read_only

Set it to `true` to use only a subset of selected operations that forbids
executing commands or performing write operations on the file-system.

Type: boolean

#### xplr.config.general.enable_recover_mode

Set it to `true` if you want to enable a safety feature that will save you
from yourself when you type recklessly.

Type: boolean

#### xplr.config.general.hide_remaps_in_help_menu

Set it to `true` if you want to hide all remaps in the help menu.

Type: boolean

#### xplr.config.general.paginated_scrolling

Set it to `true` if you want paginated scrolling.

Type: boolean

#### xplr.config.general.scroll_padding

Set the padding value to the scroll area.
Only applicable when `xplr.config.general.paginated_scrolling = false`.

Type: boolean

#### xplr.config.general.enforce_bounded_index_navigation

Set it to `true` if you want the cursor to stay in the same position when
the focus is on the first path and you navigate to the previous path
(by pressing `up`/`k`), or when the focus is on the last path and you
navigate to the next path (by pressing `down`/`j`).
The default behavior is to rotate from the last/first path.

Type: boolean

#### xplr.config.general.prompt.format

This is the shape of the prompt for the input buffer.

Type: nullable string

#### xplr.config.general.prompt.style

This is the style of the prompt for the input buffer.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.logs.info.format

The string to indicate an information in logs.

Type: nullable string

#### xplr.config.general.logs.info.style

The style for the information logs.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.logs.success.format

The string to indicate an success in logs.

Type: nullable string

#### xplr.config.general.logs.success.style

The style for the success logs.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.logs.warning.format

The string to indicate an warnings in logs.

Type: nullable string

#### xplr.config.general.logs.warning.style

The style for the warnings logs.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.logs.error.format

The string to indicate an error in logs.

Type: nullable string

#### xplr.config.general.logs.error.style

The style for the error logs.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.table.header.cols

Columns to display in the table header.

Type: nullable list of tables with the following fields:

- format: nullable string
- style: [Style](https://xplr.dev/en/style)

#### xplr.config.general.table.header.style

Style of the table header.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.table.header.height

Height of the table header.

Type: nullable integer

#### xplr.config.general.table.row.cols

Columns to display in each row in the table.

Type: nullable list of tables with the following fields:

- format: nullable string
- style: [Style](https://xplr.dev/en/style)

#### xplr.config.general.table.row.style

Default style of the table.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.table.row.height

Height of the table rows.

Type: nullable integer

#### xplr.config.general.table.style

Default style of the table.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.table.tree

Tree to display in the table.

Type: nullable list of tables with the following fields:

- format: nullable string
- style: [Style](https://xplr.dev/en/style)

#### xplr.config.general.table.col_spacing

Spacing between the columns in the table.

Type: nullable integer

#### xplr.config.general.table.col_widths

Constraint for the column widths.

Type: nullable list of [Constraint](https://xplr.dev/en/layouts#constraint)

#### xplr.config.general.selection.item.format

Renderer for each item in the selection list.

Type: nullable string

#### xplr.config.general.selection.item.style

Style for each item in the selection list.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.search.algorithm

The default search algorithm

Type: [Search Algorithm](https://xplr.dev/en/searching#algorithm)

#### xplr.config.general.search.unordered

The default search ordering

Type: boolean

#### xplr.config.general.search.exact_mode

The fuzzy search exact mode.

Type: boolean

#### xplr.config.general.search.rank_criteria

The fuzzy search rank criteria.

Type: nullable list of [Rank Criteria](https://xplr.dev/en/searching#rank_criteria)

#### xplr.config.general.default_ui.prefix

The content that is placed before the item name for each row by default.

Type: nullable string

#### xplr.config.general.default_ui.suffix

The content which is appended to each item name for each row by default.

Type: nullable string

#### xplr.config.general.default_ui.style

The default style of each item for each row.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.focus_ui.prefix

The string placed before the item name for a focused row.

Type: nullable string

#### xplr.config.general.focus_ui.suffix

The string placed after the item name for a focused row.

Type: nullable string

#### xplr.config.general.focus_ui.style

Style for focused item.
Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.selection_ui.prefix

The string placed before the item name for a selected row.

Type: nullable string

#### xplr.config.general.selection_ui.suffix

The string placed after the item name for a selected row.

Type: nullable string

#### xplr.config.general.selection_ui.style

Style for selected rows.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.focus_selection_ui.prefix

The string placed before item name for a selected row that gets the focus.

Type: nullable string

#### xplr.config.general.focus_selection_ui.suffix

The string placed after the item name for a selected row that gets the focus.

Type: nullable string

#### xplr.config.general.focus_selection_ui.style

Style for a selected row that gets the focus.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.sort_and_filter_ui.separator.format

The shape of the separator for the Sort & filter panel.

Type: nullable string

#### xplr.config.general.sort_and_filter_ui.separator.style

The style of the separator for the Sort & filter panel.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.sort_and_filter_ui.default_identifier.format

The content of the default identifier in Sort & filter panel.

Type: nullable string

#### xplr.config.general.sort_and_filter_ui.default_identifier.style

Style for the default identifier in Sort & filter panel.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.forward.format

The shape of the forward direction indicator for sort identifiers in Sort & filter panel.

Type: nullable string

#### xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.forward.style

Style of forward direction indicator in Sort & filter panel.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.reverse.format

The shape of the reverse direction indicator for sort identifiers in Sort & filter panel.

Type: nullable string

#### xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.reverse.style

Style of reverse direction indicator in Sort & filter panel.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.sort_and_filter_ui.sorter_identifiers

The identifiers used to denote applied sorters in the Sort & filter panel.

Type: nullable mapping of the following key-value pairs:

- key: [Sorter](https://xplr.dev/en/sorting#sorter)
- value:
  - format: nullable string
  - style: [Style](https://xplr.dev/en/style)

#### xplr.config.general.sort_and_filter_ui.filter_identifiers

The identifiers used to denote applied filters in the Sort & filter panel.

Type: nullable mapping of the following key-value pairs:

- key: [Filter](https://xplr.dev/en/filtering#filter)
- value:
  - format: nullable string
  - style: [Style](https://xplr.dev/en/style)

#### xplr.config.general.sort_and_filter_ui.search_identifiers

The identifiers used to denote applied search input.

Type: { format = nullable string, style = [Style](https://xplr.dev/en/style) }

#### xplr.config.general.sort_and_filter_ui.search_direction_identifiers.ordered.format

The shape of ordered indicator for search ordering identifiers in Sort & filter panel.

Type: nullable string

#### xplr.config.general.sort_and_filter_ui.search_direction_identifiers.unordered.format

The shape of unordered indicator for search ordering identifiers in Sort & filter panel.

Type: nullable string

#### xplr.config.general.panel_ui.default.title.format

The content for panel title by default.

Type: nullable string

#### xplr.config.general.panel_ui.default.title.style

The style for panel title by default.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.default.style

Style of the panels by default.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.default.borders

Defines where to show borders for the panels by default.

Type: nullable list of [Border](https://xplr.dev/en/borders#border)

#### xplr.config.general.panel_ui.default.border_type

Type of the borders by default.

Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)

#### xplr.config.general.panel_ui.default.border_style

Style of the panel borders by default.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.table.title.format

The content for the table panel title.

Type: nullable string

#### xplr.config.general.panel_ui.table.title.style

Style of the table panel title.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.table.style

Style of the table panel.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.table.borders

Defines where to show borders for the table panel.

Type: nullable list of [Border](https://xplr.dev/en/borders#border)

#### xplr.config.general.panel_ui.table.border_type

Type of the borders for table panel.

Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)

#### xplr.config.general.panel_ui.table.border_style

Style of the table panel borders.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.help_menu.title.format

The content for the help menu panel title.

Type: nullable string

#### xplr.config.general.panel_ui.help_menu.title.style

Style of the help menu panel title.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.help_menu.style

Style of the help menu panel.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.help_menu.borders

Defines where to show borders for the help menu panel.

Type: nullable list of [Border](https://xplr.dev/en/borders#border)

#### xplr.config.general.panel_ui.help_menu.border_type

Type of the borders for help menu panel.

Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)

#### xplr.config.general.panel_ui.help_menu.border_style

Style of the help menu panel borders.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.input_and_logs.title.format

The content for the input & logs panel title.

Type: nullable string

#### xplr.config.general.panel_ui.input_and_logs.title.style

Style of the input & logs panel title.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.input_and_logs.borders

#### xplr.config.general.panel_ui.input_and_logs.style

Style of the input & logs panel.

Type: [Style](https://xplr.dev/en/style)
Defines where to show borders for the input & logs panel.

Type: nullable list of [Border](https://xplr.dev/en/borders#border)

#### xplr.config.general.panel_ui.input_and_logs.border_type

Type of the borders for input & logs panel.

Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)

#### xplr.config.general.panel_ui.input_and_logs.border_style

Style of the input & logs panel borders.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.selection.title.format

The content for the selection panel title.

Type: nullable string

#### xplr.config.general.panel_ui.selection.title.style

Style of the selection panel title.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.selection.borders

#### xplr.config.general.panel_ui.selection.style

Style of the selection panel.

Type: [Style](https://xplr.dev/en/style)
Defines where to show borders for the selection panel.

Type: nullable list of [Border](https://xplr.dev/en/borders#border)

#### xplr.config.general.panel_ui.selection.border_type

Type of the borders for selection panel.

Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)

#### xplr.config.general.panel_ui.selection.border_style

Style of the selection panel borders.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.sort_and_filter.title.format

The content for the sort & filter panel title.

Type: nullable string

#### xplr.config.general.panel_ui.sort_and_filter.title.style

Style of the sort & filter panel title.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.sort_and_filter.style

Style of the sort & filter panel.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.panel_ui.sort_and_filter.borders

Defines where to show borders for the sort & filter panel.

Type: nullable list of [Border](https://xplr.dev/en/borders#border)

#### xplr.config.general.panel_ui.sort_and_filter.border_type

Type of the borders for sort & filter panel.

Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)

#### xplr.config.general.panel_ui.sort_and_filter.border_style

Style of the sort & filter panel borders.

Type: [Style](https://xplr.dev/en/style)

#### xplr.config.general.initial_sorting

Initial group if sorters applied to the nodes list in the table.

Type: nullable list of [Node Sorter](https://xplr.dev/en/sorting#node-sorter-applicable)

#### xplr.config.general.initial_mode

The name of one of the modes to use when xplr loads.
This isn't the default mode. To modify the default mode, overwrite
[xplr.config.modes.builtin.default](https://xplr.dev/en/modes#xplrconfigmodesbuiltindefault).

Type: nullable string

#### xplr.config.general.initial_layout

The name of one of the layouts to use when xplr loads.
This isn't the default layout. To modify the default layout, overwrite
[xplr.config.layouts.builtin.default](https://xplr.dev/en/layouts#xplrconfiglayoutsbuiltindefault).

Type: nullable string

#### xplr.config.general.start_fifo

Set it to a file path to start fifo when xplr loads.
Generally it is used to integrate with external tools like previewers.

Type: nullable string

#### xplr.config.general.global_key_bindings

Use it to define a set of key bindings that are available by default in
every [mode](https://xplr.dev/en/mode). They can be overwritten.

Type: [Key Bindings](https://xplr.dev/en/configure-key-bindings#key-bindings)
