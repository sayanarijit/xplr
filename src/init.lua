---@diagnostic disable
local xplr = xplr -- The globally exposed configuration to be overridden.
---@diagnostic enable

-- This is the built-in configuration file that gets loaded and sets the
-- default values when xplr loads, before loading any other custom
-- configuration file.
--
-- You can use this file as a reference to create a your custom config file.
--
-- To create a custom configuration file, you need to define the script version
-- for compatibility checks.
--
-- See https://xplr.dev/en/upgrade-guide
--
-- ```lua
-- version = "0.0.0"
-- ```

-- # Configuration ------------------------------------------------------------
--
-- xplr can be configured using [Lua][1] via a special file named `init.lua`,
-- which can be placed in `~/.config/xplr/` (local to user) or `/etc/xplr/`
-- (global) depending on the use case.
--
-- When xplr loads, it first executes the [built-in init.lua][2] to set the
-- default values, which is then overwritten by another config file, if found
-- using the following lookup order:
--
-- 1. `--config /path/to/init.lua`
-- 2. `~/.config/xplr/init.lua`
-- 3. `/etc/xplr/init.lua`
--
-- The first one found will be loaded by xplr and the lookup will stop.
--
-- The loaded config can be further extended using the `-C` or `--extra-config`
-- command-line option.
--
--
-- [1]: https://www.lua.org
-- [2]: https://github.com/sayanarijit/xplr/blob/main/src/init.lua
-- [3]: https://xplr.dev/en/upgrade-guide

-- ## Config ------------------------------------------------------------------
--
-- The xplr configuration, exposed via `xplr.config` Lua API contains the
-- following sections.
--
-- See:
--
-- * [xplr.config.general](https://xplr.dev/en/general-config)
-- * [xplr.config.node_types](https://xplr.dev/en/node_types)
-- * [xplr.config.layouts](https://xplr.dev/en/layouts)
-- * [xplr.config.modes](https://xplr.dev/en/modes)

-- ### General Configuration --------------------------------------------------
--
-- The general configuration properties are grouped together in
-- `xplr.config.general`.

-- Set it to `true` if you want to ignore the startup errors. You can still see
-- the errors in the logs.
--
-- Type: boolean
xplr.config.general.disable_debug_error_mode = false

-- Set it to `true` if you want to enable mouse scrolling.
--
-- Type: boolean
xplr.config.general.enable_mouse = false

-- Set it to `true` to show hidden files by default.
--
-- Type: boolean
xplr.config.general.show_hidden = false

-- Set it to `true` to use only a subset of selected operations that forbids
-- executing commands or performing write operations on the file-system.
--
-- Type: boolean
xplr.config.general.read_only = false

-- Set it to `true` if you want to enable a safety feature that will save you
-- from yourself when you type recklessly.
--
-- Type: boolean
xplr.config.general.enable_recover_mode = false

-- Set it to `true` if you want to hide all remaps in the help menu.
--
-- Type: boolean
xplr.config.general.hide_remaps_in_help_menu = false

-- Set it to `true` if you want paginated scrolling.
--
-- Type: boolean
xplr.config.general.paginated_scrolling = false

-- Set the padding value to the scroll area.
-- Only applicable when `xplr.config.general.paginated_scrolling = false`.
--
-- Type: boolean
xplr.config.general.scroll_padding = 5

-- Set it to `true` if you want the cursor to stay in the same position when
-- the focus is on the first path and you navigate to the previous path
-- (by pressing `up`/`k`), or when the focus is on the last path and you
-- navigate to the next path (by pressing `down`/`j`).
-- The default behavior is to rotate from the last/first path.
--
-- Type: boolean
xplr.config.general.enforce_bounded_index_navigation = false

-- This is the shape of the prompt for the input buffer.
--
-- Type: nullable string
xplr.config.general.prompt.format = "❯ "

-- This is the style of the prompt for the input buffer.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.prompt.style = {}

-- The string to indicate an information in logs.
--
-- Type: nullable string
xplr.config.general.logs.info.format = "INFO"

-- The style for the information logs.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.logs.info.style = { fg = "LightBlue" }

-- The string to indicate an success in logs.
--
-- Type: nullable string
xplr.config.general.logs.success.format = "SUCCESS"

-- The style for the success logs.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.logs.success.style = { fg = "Green" }

-- The string to indicate an warnings in logs.
--
-- Type: nullable string
xplr.config.general.logs.warning.format = "WARNING"

-- The style for the warnings logs.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.logs.warning.style = { fg = "Yellow" }

-- The string to indicate an error in logs.
--
-- Type: nullable string
xplr.config.general.logs.error.format = "ERROR"

-- The style for the error logs.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.logs.error.style = { fg = "Red" }

-- Columns to display in the table header.
--
-- Type: nullable list of tables with the following fields:
--
-- * format: nullable string
-- * style: [Style](https://xplr.dev/en/style)
xplr.config.general.table.header.cols = {
  { format = " index", style = {} },
  { format = "╭─── path", style = {} },
  { format = "perm", style = {} },
  { format = "size", style = {} },
  { format = "modified", style = {} },
}

-- Style of the table header.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.table.header.style = {}

-- Height of the table header.
--
-- Type: nullable integer
xplr.config.general.table.header.height = 1

-- Columns to display in each row in the table.
--
-- Type: nullable list of tables with the following fields:
--
-- * format: nullable string
-- * style: [Style](https://xplr.dev/en/style)
xplr.config.general.table.row.cols = {
  {
    format = "builtin.fmt_general_table_row_cols_0",
    style = {},
  },
  {
    format = "builtin.fmt_general_table_row_cols_1",
    style = {},
  },
  {
    format = "builtin.fmt_general_table_row_cols_2",
    style = {},
  },
  {
    format = "builtin.fmt_general_table_row_cols_3",
    style = {},
  },
  {
    format = "builtin.fmt_general_table_row_cols_4",
    style = {},
  },
}

-- Default style of the table.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.table.row.style = {}

-- Height of the table rows.
--
-- Type: nullable integer
xplr.config.general.table.row.height = 0

-- Default style of the table.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.table.style = {}

-- Tree to display in the table.
--
-- Type: nullable list of tables with the following fields:
--
-- * format: nullable string
-- * style: [Style](https://xplr.dev/en/style)
xplr.config.general.table.tree = {
  { format = "├", style = {} },
  { format = "├", style = {} },
  { format = "╰", style = {} },
}

-- Spacing between the columns in the table.
--
-- Type: nullable integer
xplr.config.general.table.col_spacing = 1

-- Constraint for the column widths.
--
-- Type: nullable list of [Constraint](https://xplr.dev/en/layouts#constraint)
xplr.config.general.table.col_widths = {
  { Percentage = 10 },
  { Percentage = 50 },
  { Percentage = 10 },
  { Percentage = 10 },
  { Percentage = 20 },
}

-- Renderer for each item in the selection list.
--
-- Type: nullable string
xplr.config.general.selection.item.format = "builtin.fmt_general_selection_item"

-- Style for each item in the selection list.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.selection.item.style = {}

-- The default search algorithm
--
-- Type: [Search Algorithm](https://xplr.dev/en/searching#algorithm)
xplr.config.general.search.algorithm = "Fuzzy"

-- The default search ordering
--
-- Type: boolean
xplr.config.general.search.unordered = false

-- The fuzzy search exact mode.
--
-- Type: boolean
xplr.config.general.search.exact_mode = false

-- The fuzzy search rank criteria.
--
-- Type: nullable list of [Rank Criteria](https://xplr.dev/en/searching#rank_criteria)
xplr.config.general.search.rank_criteria = nil

-- The content that is placed before the item name for each row by default.
--
-- Type: nullable string
xplr.config.general.default_ui.prefix = "  "

-- The content which is appended to each item name for each row by default.
--
-- Type: nullable string
xplr.config.general.default_ui.suffix = ""

-- The default style of each item for each row.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.default_ui.style = {}

-- The string placed before the item name for a focused row.
--
-- Type: nullable string
xplr.config.general.focus_ui.prefix = "▸["

-- The string placed after the item name for a focused row.
--
-- Type: nullable string
xplr.config.general.focus_ui.suffix = "]"

-- Style for focused item.
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.focus_ui.style = { add_modifiers = { "Bold" } }

-- The string placed before the item name for a selected row.
--
-- Type: nullable string
xplr.config.general.selection_ui.prefix = " {"

-- The string placed after the item name for a selected row.
--
-- Type: nullable string
xplr.config.general.selection_ui.suffix = "}"

-- Style for selected rows.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.selection_ui.style = {
  fg = "DarkGray",
  add_modifiers = { "CrossedOut" },
}

-- The string placed before item name for a selected row that gets the focus.
--
-- Type: nullable string
xplr.config.general.focus_selection_ui.prefix = "▸["

-- The string placed after the item name for a selected row that gets the focus.
--
-- Type: nullable string
xplr.config.general.focus_selection_ui.suffix = "]"

-- Style for a selected row that gets the focus.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.focus_selection_ui.style = {
  add_modifiers = { "Bold", "CrossedOut" },
}

-- The shape of the separator for the Sort & filter panel.
--
-- Type: nullable string
xplr.config.general.sort_and_filter_ui.separator.format = " › "

-- The style of the separator for the Sort & filter panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.separator.style = {
  add_modifiers = { "Dim" },
}

-- The content of the default identifier in Sort & filter panel.
--
-- Type: nullable string
xplr.config.general.sort_and_filter_ui.default_identifier.format = nil

-- Style for the default identifier in Sort & filter panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.default_identifier.style = {}

-- The shape of the forward direction indicator for sort identifiers in Sort & filter panel.
--
-- Type: nullable string
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.forward.format = "↓"

-- Style of forward direction indicator in Sort & filter panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.forward.style = nil

-- The shape of the reverse direction indicator for sort identifiers in Sort & filter panel.
--
-- Type: nullable string
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.reverse.format = "↑"

-- Style of reverse direction indicator in Sort & filter panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.reverse.style = nil

-- The identifiers used to denote applied sorters in the Sort & filter panel.
--
-- Type: nullable mapping of the following key-value pairs:
--
-- * key: [Sorter](https://xplr.dev/en/sorting#sorter)
-- * value:
--   * format: nullable string
--   * style: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.sorter_identifiers = {
  ByExtension = { format = "ext", style = {} },
  ByICanonicalAbsolutePath = { format = "[ci]abs", style = {} },
  ByIRelativePath = { format = "[i]rel", style = {} },
  ByISymlinkAbsolutePath = { format = "[si]abs", style = {} },
  ByIsBroken = { format = "⨯", style = {} },
  ByIsDir = { format = "dir", style = {} },
  ByIsFile = { format = "file", style = {} },
  ByIsReadonly = { format = "ro", style = {} },
  ByIsSymlink = { format = "sym", style = {} },
  ByMimeEssence = { format = "mime", style = {} },
  ByRelativePath = { format = "rel", style = {} },
  BySize = { format = "size", style = {} },
  ByCreated = { format = "created", style = {} },
  ByLastModified = { format = "modified", style = {} },
  ByCanonicalAbsolutePath = { format = "[c]abs", style = {} },
  ByCanonicalExtension = { format = "[c]ext", style = {} },
  ByCanonicalIsDir = { format = "[c]dir", style = {} },
  ByCanonicalIsFile = { format = "[c]file", style = {} },
  ByCanonicalIsReadonly = { format = "[c]ro", style = {} },
  ByCanonicalMimeEssence = { format = "[c]mime", style = {} },
  ByCanonicalSize = { format = "[c]size", style = {} },
  ByCanonicalCreated = { format = "[c]created", style = {} },
  ByCanonicalLastModified = { format = "[c]modified", style = {} },
  BySymlinkAbsolutePath = { format = "[s]abs", style = {} },
  BySymlinkExtension = { format = "[s]ext", style = {} },
  BySymlinkIsDir = { format = "[s]dir", style = {} },
  BySymlinkIsFile = { format = "[s]file", style = {} },
  BySymlinkIsReadonly = { format = "[s]ro", style = {} },
  BySymlinkMimeEssence = { format = "[s]mime", style = {} },
  BySymlinkSize = { format = "[s]size", style = {} },
  BySymlinkCreated = { format = "[s]created", style = {} },
  BySymlinkLastModified = { format = "[s]modified", style = {} },
}

-- The identifiers used to denote applied filters in the Sort & filter panel.
--
-- Type: nullable mapping of the following key-value pairs:
--
-- * key: [Filter](https://xplr.dev/en/filtering#filter)
-- * value:
--   * format: nullable string
--   * style: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.filter_identifiers = {
  RelativePathDoesContain = { format = "rel=~", style = {} },
  RelativePathDoesEndWith = { format = "rel=$", style = {} },
  RelativePathDoesNotContain = { format = "rel!~", style = {} },
  RelativePathDoesNotEndWith = { format = "rel!$", style = {} },
  RelativePathDoesNotStartWith = { format = "rel!^", style = {} },
  RelativePathDoesStartWith = { format = "rel=^", style = {} },
  RelativePathIs = { format = "rel==", style = {} },
  RelativePathIsNot = { format = "rel!=", style = {} },
  RelativePathDoesMatchRegex = { format = "rel=/", style = {} },
  RelativePathDoesNotMatchRegex = { format = "rel!/", style = {} },
  IRelativePathDoesContain = { format = "[i]rel=~", style = {} },
  IRelativePathDoesEndWith = { format = "[i]rel=$", style = {} },
  IRelativePathDoesNotContain = { format = "[i]rel!~", style = {} },
  IRelativePathDoesNotEndWith = { format = "[i]rel!$", style = {} },
  IRelativePathDoesNotStartWith = { format = "[i]rel!^", style = {} },
  IRelativePathDoesStartWith = { format = "[i]rel=^", style = {} },
  IRelativePathIs = { format = "[i]rel==", style = {} },
  IRelativePathIsNot = { format = "[i]rel!=", style = {} },
  IRelativePathDoesMatchRegex = { format = "[i]rel=/", style = {} },
  IRelativePathDoesNotMatchRegex = { format = "[i]rel!/", style = {} },
  AbsolutePathDoesContain = { format = "abs=~", style = {} },
  AbsolutePathDoesEndWith = { format = "abs=$", style = {} },
  AbsolutePathDoesNotContain = { format = "abs!~", style = {} },
  AbsolutePathDoesNotEndWith = { format = "abs!$", style = {} },
  AbsolutePathDoesNotStartWith = { format = "abs!^", style = {} },
  AbsolutePathDoesStartWith = { format = "abs=^", style = {} },
  AbsolutePathIs = { format = "abs==", style = {} },
  AbsolutePathIsNot = { format = "abs!=", style = {} },
  AbsolutePathDoesMatchRegex = { format = "abs=/", style = {} },
  AbsolutePathDoesNotMatchRegex = { format = "abs!/", style = {} },
  IAbsolutePathDoesContain = { format = "[i]abs=~", style = {} },
  IAbsolutePathDoesEndWith = { format = "[i]abs=$", style = {} },
  IAbsolutePathDoesNotContain = { format = "[i]abs!~", style = {} },
  IAbsolutePathDoesNotEndWith = { format = "[i]abs!$", style = {} },
  IAbsolutePathDoesNotStartWith = { format = "[i]abs!^", style = {} },
  IAbsolutePathDoesStartWith = { format = "[i]abs=^", style = {} },
  IAbsolutePathIs = { format = "[i]abs==", style = {} },
  IAbsolutePathIsNot = { format = "[i]abs!=", style = {} },
  IAbsolutePathDoesMatchRegex = { format = "[i]abs=/", style = {} },
  IAbsolutePathDoesNotMatchRegex = { format = "[i]abs!/", style = {} },
}

-- The identifiers used to denote applied search input.
--
-- Type: { format = nullable string, style = [Style](https://xplr.dev/en/style) }
xplr.config.general.sort_and_filter_ui.search_identifiers = {
  Fuzzy = { format = "fzy:", style = {} },
  Regex = { format = "reg:", style = {} },
}

-- The shape of ordered indicator for search ordering identifiers in Sort & filter panel.
--
-- Type: nullable string
xplr.config.general.sort_and_filter_ui.search_direction_identifiers.ordered.format =
  "↓"

-- The shape of unordered indicator for search ordering identifiers in Sort & filter panel.
--
-- Type: nullable string
xplr.config.general.sort_and_filter_ui.search_direction_identifiers.unordered.format = ""

-- The content for panel title by default.
--
-- Type: nullable string
xplr.config.general.panel_ui.default.title.format = nil

-- The style for panel title by default.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.default.title.style = {
  fg = "Reset",
  add_modifiers = { "Bold" },
}

-- Style of the panels by default.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.default.style = {}

-- Defines where to show borders for the panels by default.
--
-- Type: nullable list of [Border](https://xplr.dev/en/borders#border)
xplr.config.general.panel_ui.default.borders = {
  "Top",
  "Right",
  "Bottom",
  "Left",
}

-- Type of the borders by default.
--
-- Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)
xplr.config.general.panel_ui.default.border_type = "Rounded"

-- Style of the panel borders by default.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.default.border_style = { fg = "DarkGray" }

-- The content for the table panel title.
--
-- Type: nullable string
xplr.config.general.panel_ui.table.title.format = nil

-- Style of the table panel title.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.table.title.style = {}

-- Style of the table panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.table.style = {}

-- Defines where to show borders for the table panel.
--
-- Type: nullable list of [Border](https://xplr.dev/en/borders#border)
xplr.config.general.panel_ui.table.borders = nil

-- Type of the borders for table panel.
--
-- Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)
xplr.config.general.panel_ui.table.border_type = nil

-- Style of the table panel borders.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.table.border_style = {}

-- The content for the help menu panel title.
--
-- Type: nullable string
xplr.config.general.panel_ui.help_menu.title.format = nil

-- Style of the help menu panel title.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.help_menu.title.style = {}

-- Style of the help menu panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.help_menu.style = {}

-- Defines where to show borders for the help menu panel.
--
-- Type: nullable list of [Border](https://xplr.dev/en/borders#border)
xplr.config.general.panel_ui.help_menu.borders = nil

-- Type of the borders for help menu panel.
--
-- Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)
xplr.config.general.panel_ui.help_menu.border_type = nil

-- Style of the help menu panel borders.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.help_menu.border_style = {}

-- The content for the input & logs panel title.
--
-- Type: nullable string
xplr.config.general.panel_ui.input_and_logs.title.format = nil

-- Style of the input & logs panel title.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.input_and_logs.title.style = {}

-- Style of the input & logs panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.input_and_logs.style = {}
-- Defines where to show borders for the input & logs panel.
--
-- Type: nullable list of [Border](https://xplr.dev/en/borders#border)
xplr.config.general.panel_ui.input_and_logs.borders = nil

-- Type of the borders for input & logs panel.
--
-- Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)
xplr.config.general.panel_ui.input_and_logs.border_type = nil

-- Style of the input & logs panel borders.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.input_and_logs.border_style = {}

-- The content for the selection panel title.
--
-- Type: nullable string
xplr.config.general.panel_ui.selection.title.format = nil

-- Style of the selection panel title.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.selection.title.style = {}

-- Style of the selection panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.selection.style = {}
-- Defines where to show borders for the selection panel.
--
-- Type: nullable list of [Border](https://xplr.dev/en/borders#border)
xplr.config.general.panel_ui.selection.borders = nil

-- Type of the borders for selection panel.
--
-- Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)
xplr.config.general.panel_ui.selection.border_type = nil

-- Style of the selection panel borders.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.selection.border_style = {}

-- The content for the sort & filter panel title.
--
-- Type: nullable string
xplr.config.general.panel_ui.sort_and_filter.title.format = nil

-- Style of the sort & filter panel title.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.sort_and_filter.title.style = {}

-- Style of the sort & filter panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.sort_and_filter.style = {}

-- Defines where to show borders for the sort & filter panel.
--
-- Type: nullable list of [Border](https://xplr.dev/en/borders#border)
xplr.config.general.panel_ui.sort_and_filter.borders = nil

-- Type of the borders for sort & filter panel.
--
-- Type: nullable [Border Type](https://xplr.dev/en/borders#border-type)
xplr.config.general.panel_ui.sort_and_filter.border_type = nil

-- Style of the sort & filter panel borders.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.panel_ui.sort_and_filter.border_style = {}

-- Initial group if sorters applied to the nodes list in the table.
--
-- Type: nullable list of [Node Sorter](https://xplr.dev/en/sorting#node-sorter-applicable)
xplr.config.general.initial_sorting = {
  { sorter = "ByCanonicalIsDir", reverse = true },
  { sorter = "ByIRelativePath", reverse = false },
}

-- The name of one of the modes to use when xplr loads.
-- This isn't the default mode. To modify the default mode, overwrite
-- [xplr.config.modes.builtin.default](https://xplr.dev/en/modes#xplrconfigmodesbuiltindefault).
--
-- Type: nullable string
xplr.config.general.initial_mode = "default"

-- The name of one of the layouts to use when xplr loads.
-- This isn't the default layout. To modify the default layout, overwrite
-- [xplr.config.layouts.builtin.default](https://xplr.dev/en/layouts#xplrconfiglayoutsbuiltindefault).
--
-- Type: nullable string
xplr.config.general.initial_layout = "default"

-- Set it to a file path to start fifo when xplr loads.
-- Generally it is used to integrate with external tools like previewers.
--
-- Type: nullable string
xplr.config.general.start_fifo = nil

-- Use it to define a set of key bindings that are available by default in
-- every [mode](https://xplr.dev/en/mode). They can be overwritten.
--
-- Type: [Key Bindings](https://xplr.dev/en/configure-key-bindings#key-bindings)
xplr.config.general.global_key_bindings = {
  on_key = {
    ["f1"] = {
      help = "global help menu",
      messages = {
        {
          BashExec = [===[
            [ -z "$PAGER" ] && PAGER="less -+F"
            cat -- "${XPLR_PIPE_GLOBAL_HELP_MENU_OUT}" | ${PAGER:?}
          ]===],
        },
      },
    },
    ["esc"] = {
      messages = {
        "PopMode",
      },
    },
    ["ctrl-c"] = {
      messages = {
        "Terminate",
      },
    },
  },
}

-- ### Node Types -------------------------------------------------------------
--
-- This section defines how to deal with different kinds of nodes (files,
-- directories, symlinks etc.) based on their properties.
--
-- One node can fall into multiple categories. For example, a node can have the
-- *extension* `md`, and also be a *file*. In that case, the properties from
-- the more  specific category i.e. *extension* will be used.
--
-- This can be configured using the `xplr.config.node_types` Lua API.

-- The style for the directory nodes
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.node_types.directory.style = {}

-- Metadata for the directory nodes.
-- You can set as many metadata as you want.
--
-- Type: nullable string
--
-- Example:
--
-- ```lua
-- xplr.config.node_types.directory.meta.foo = "foo"
-- xplr.config.node_types.directory.meta.bar = "bar"
-- ```
xplr.config.node_types.directory.meta.icon = "ð"

-- The style for the file nodes.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.node_types.file.style = {}

-- Metadata for the file nodes.
-- You can set as many metadata as you want.
--
-- Type: nullable string
--
-- Example:
--
-- ```lua
-- xplr.config.node_types.file.meta.foo = "foo"
-- xplr.config.node_types.file.meta.bar = "bar"
-- ```
xplr.config.node_types.file.meta.icon = "ƒ"

-- The style for the symlink nodes.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.node_types.symlink.style = {}

-- Metadata for the symlink nodes.
-- You can set as many metadata as you want.
--
-- Type: nullable string
--
-- Example:
--
-- ```lua
-- xplr.config.node_types.symlink.meta.foo = "foo"
-- xplr.config.node_types.symlink.meta.bar = "bar"
-- ```
xplr.config.node_types.symlink.meta.icon = "§"

-- Metadata and style based on mime types.
-- It is possible to use the wildcard `*` to match all mime sub types. It will
-- be overwritten by the more specific sub types that are defined.
--
-- Type: mapping of the following key-value pairs:
--
-- * key: string
-- * value:
--   * key: string
--   * value: [Node Type](https://xplr.dev/en/node-type)
--
-- Example:
--
-- ```lua
-- xplr.config.node_types.mime_essence = {
--   application = {
--     -- application/*
--     ["*"] = { meta = { icon = "a" } },
--
--     -- application/pdf
--     pdf = { meta = { icon = "" }, style = { fg = "Blue" } },
--
--     -- application/zip
--     zip = { meta = { icon = ""} },
--   },
-- }
-- ```
xplr.config.node_types.mime_essence = {}

-- Metadata and style based on extension.
--
-- Type: mapping of the following key-value pairs:
--
-- * key: string
-- * value: [Node Type](https://xplr.dev/en/node-type)
--
-- Example:
--
-- ```lua
-- xplr.config.node_types.extension.md = { meta = { icon = "" }, style = { fg = "Blue" } }
-- xplr.config.node_types.extension.rs = { meta = { icon = "🦀" } }
-- ```
xplr.config.node_types.extension = {}

-- Metadata and style based on special file names.
--
-- Type: mapping of the following key-value pairs:
--
-- * key: string
-- * value: [Node Type](https://xplr.dev/en/node-type)
--
-- Example:
--
-- ```lua
-- xplr.config.node_types.special["Cargo.toml"] = { meta = { icon = "" } }
-- xplr.config.node_types.special["Downloads"] = { meta = { icon = "" }, style = { fg = "Blue" } }
-- ```
xplr.config.node_types.special = {}

-- ### Layouts ----------------------------------------------------------------
--
-- xplr layouts define the structure of the UI, i.e. how many panel we see,
-- placement and size of the panels, how they look etc.
--
-- This is configuration exposed via the `xplr.config.layouts` API.
--
-- `xplr.config.layouts.builtin` contain some built-in panels which can be
-- overridden, but you can't add or remove panels in it.
--
-- You can add new panels in `xplr.config.layouts.custom`.
--
-- ##### Example: Defining Custom Layout
--
-- ```lua
-- xplr.config.layouts.builtin.default = {
--   Horizontal = {
--     config = {
--       margin = 1,
--       horizontal_margin = 1,
--       vertical_margin = 1,
--       constraints = {
--         { Percentage = 50 },
--         { Percentage = 50 },
--       }
--     },
--     splits = {
--       "Table",
--       "HelpMenu",
--     }
--   }
-- }
-- ```
--
-- Result:
--
-- ```
-- ╭ /home ─────────────╮╭ Help [default] ────╮
-- │   ╭─── path        ││.    show hidden    │
-- │   ├▸[ð Desktop/]   ││/    search         │
-- │   ├  ð Documents/  ││:    action         │
-- │   ├  ð Downloads/  ││?    global help    │
-- │   ├  ð GitHub/     ││G    go to bottom   │
-- │   ├  ð Music/      ││V    select/unselect│
-- │   ├  ð Pictures/   ││ctrl duplicate as   │
-- │   ├  ð Public/     ││ctrl next visit     │
-- ╰────────────────────╯╰────────────────────╯
-- ```

-- The default layout
--
-- Type: [Layout](https://xplr.dev/en/layout)
xplr.config.layouts.builtin.default = {
  Horizontal = {
    config = {
      constraints = {
        { Percentage = 70 },
        { Percentage = 30 },
      },
    },
    splits = {
      {
        Vertical = {
          config = {
            constraints = {
              { Length = 3 },
              { Min = 1 },
              { Length = 3 },
            },
          },
          splits = {
            "SortAndFilter",
            "Table",
            "InputAndLogs",
          },
        },
      },
      {
        Vertical = {
          config = {
            constraints = {
              { Percentage = 30 },
              { Percentage = 70 },
            },
          },
          splits = {
            "Selection",
            "HelpMenu",
          },
        },
      },
    },
  },
}

-- The layout without help menu
--
-- Type: [Layout](https://xplr.dev/en/layout)
xplr.config.layouts.builtin.no_help = {
  Horizontal = {
    config = {
      constraints = {
        { Percentage = 70 },
        { Percentage = 30 },
      },
    },
    splits = {
      {
        Vertical = {
          config = {
            constraints = {
              { Length = 3 },
              { Min = 1 },
              { Length = 3 },
            },
          },
          splits = {
            "SortAndFilter",
            "Table",
            "InputAndLogs",
          },
        },
      },
      "Selection",
    },
  },
}

-- The layout without selection panel
--
-- Type: [Layout](https://xplr.dev/en/layout)
xplr.config.layouts.builtin.no_selection = {
  Horizontal = {
    config = {
      constraints = {
        { Percentage = 70 },
        { Percentage = 30 },
      },
    },
    splits = {
      {
        Vertical = {
          config = {
            constraints = {
              { Length = 3 },
              { Min = 1 },
              { Length = 3 },
            },
          },
          splits = {
            "SortAndFilter",
            "Table",
            "InputAndLogs",
          },
        },
      },
      "HelpMenu",
    },
  },
}

-- The layout without help menu and selection panel
--
-- Type: [Layout](https://xplr.dev/en/layout)
xplr.config.layouts.builtin.no_help_no_selection = {
  Vertical = {
    config = {
      constraints = {
        { Length = 3 },
        { Min = 1 },
        { Length = 3 },
      },
    },
    splits = {
      "SortAndFilter",
      "Table",
      "InputAndLogs",
    },
  },
}

-- This is where you can define custom layouts
--
-- Type: mapping of the following key-value pairs:
--
-- * key: string
-- * value: [Layout](https://xplr.dev/en/layout)
--
-- Example:
--
-- ```lua
-- xplr.config.layouts.custom.example = "Nothing" -- Show a blank screen
-- xplr.config.general.initial_layout = "example" -- Load the example layout
-- ```
xplr.config.layouts.custom = {}

-- ### Modes ------------------------------------------------------------------
--
-- xplr is a modal file explorer. That means the users switch between different
-- modes, each containing a different set of key bindings to avoid clashes.
-- Users can switch between these modes at run-time.
--
-- The modes can be configured using the `xplr.config.modes` Lua API.
--
-- `xplr.config.modes.builtin` contain some built-in modes which can be
-- overridden, but you can't add or remove modes in it.

-- The builtin default mode.
-- Visit the [Default Key Bindings](https://xplr.dev/en/default-key-bindings)
-- to see what each mode does.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.default = {
  name = "default",
  key_bindings = {
    on_key = {
      ["#"] = {
        messages = {
          "PrintAppStateAndQuit",
        },
      },
      ["."] = {
        help = "show hidden",
        messages = {
          {
            ToggleNodeFilter = { filter = "RelativePathDoesNotStartWith", input = "." },
          },
          "ExplorePwdAsync",
        },
      },
      [":"] = {
        help = "action",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "action" },
        },
      },
      ["G"] = {
        help = "go to bottom",
        messages = {
          "PopMode",
          "FocusLast",
        },
      },
      ["ctrl-a"] = {
        help = "select/unselect all",
        messages = {
          "ToggleSelectAll",
        },
      },
      ["ctrl-f"] = {
        help = "search",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "search" },
          { SetInputBuffer = "" },
        },
      },
      ["ctrl-i"] = {
        help = "next visited path",
        messages = {
          "NextVisitedPath",
        },
      },
      ["ctrl-o"] = {
        help = "last visited path",
        messages = {
          "LastVisitedPath",
        },
      },
      [")"] = {
        help = "next deep branch",
        messages = {
          "NextVisitedDeepBranch",
        },
      },
      ["("] = {
        help = "prev deep branch",
        messages = {
          "PreviousVisitedDeepBranch",
        },
      },
      ["ctrl-r"] = {
        help = "refresh screen",
        messages = {
          "ClearScreen",
        },
      },
      ["ctrl-u"] = {
        help = "clear selection",
        messages = {
          "ClearSelection",
        },
      },
      ["ctrl-w"] = {
        help = "switch layout",
        messages = {
          { SwitchModeBuiltin = "switch_layout" },
        },
      },
      ["d"] = {
        help = "delete",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "delete" },
        },
      },
      ["down"] = {
        help = "down",
        messages = {
          "FocusNext",
        },
      },
      ["enter"] = {
        help = "quit with result",
        messages = {
          "PrintResultAndQuit",
        },
      },
      ["f"] = {
        help = "filter",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "filter" },
        },
      },
      ["g"] = {
        help = "go to",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "go_to" },
        },
      },
      ["left"] = {
        help = "back",
        messages = {
          "Back",
        },
      },
      ["q"] = {
        help = "quit",
        messages = {
          "Quit",
        },
      },
      ["r"] = {
        help = "rename",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "rename" },
          {
            BashExecSilently0 = [===[
              NAME=$(basename "${XPLR_FOCUS_PATH:?}")
              "$XPLR" -m 'SetInputBuffer: %q' "${NAME:?}"
            ]===],
          },
        },
      },
      ["ctrl-d"] = {
        help = "duplicate as",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "duplicate_as" },
          {
            BashExecSilently0 = [===[
              NAME=$(basename "${XPLR_FOCUS_PATH:?}")
              "$XPLR" -m 'SetInputBuffer: %q' "${NAME:?}"
            ]===],
          },
        },
      },
      ["right"] = {
        help = "enter",
        messages = {
          "Enter",
        },
      },
      ["s"] = {
        help = "sort",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "sort" },
        },
      },
      ["space"] = {
        help = "toggle selection",
        messages = {
          "ToggleSelection",
          "FocusNext",
        },
      },
      ["up"] = {
        help = "up",
        messages = {
          "FocusPrevious",
        },
      },
      ["~"] = {
        help = "go home",
        messages = {
          {
            BashExecSilently0 = [===[
              "$XPLR" -m 'ChangeDirectory: %q' "${HOME:?}"
            ]===],
          },
        },
      },
      ["page-up"] = {
        help = "scroll up",
        messages = {
          "ScrollUp",
        },
      },
      ["page-down"] = {
        help = "scroll down",
        messages = {
          "ScrollDown",
        },
      },
      ["{"] = {
        help = "scroll up half",
        messages = {
          "ScrollUpHalf",
        },
      },
      ["}"] = {
        help = "scroll down half",
        messages = {
          "ScrollDownHalf",
        },
      },
      ["ctrl-n"] = {
        help = "next selection",
        messages = {
          "FocusNextSelection",
        },
      },
      ["ctrl-p"] = {
        help = "prev selection",
        messages = {
          "FocusPreviousSelection",
        },
      },
      ["m"] = {
        help = "move to",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "move_to" },
          { SetInputBuffer = "" },
        },
      },
      ["c"] = {
        help = "copy to",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "copy_to" },
          { SetInputBuffer = "" },
        },
      },
    },
    on_number = {
      help = "input",
      messages = {
        "PopMode",
        { SwitchModeBuiltin = "number" },
        "BufferInputFromKey",
      },
    },
  },
}

xplr.config.modes.builtin.default.key_bindings.on_key["v"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["space"]
xplr.config.modes.builtin.default.key_bindings.on_key["V"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["ctrl-a"]
xplr.config.modes.builtin.default.key_bindings.on_key["/"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["ctrl-f"]
xplr.config.modes.builtin.default.key_bindings.on_key["h"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["left"]
xplr.config.modes.builtin.default.key_bindings.on_key["j"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["down"]
xplr.config.modes.builtin.default.key_bindings.on_key["k"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["up"]
xplr.config.modes.builtin.default.key_bindings.on_key["l"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["right"]
xplr.config.modes.builtin.default.key_bindings.on_key["tab"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["ctrl-i"] -- compatibility workaround
xplr.config.modes.builtin.default.key_bindings.on_key["?"] =
  xplr.config.general.global_key_bindings.on_key["f1"]

-- The builtin debug error mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.debug_error = {
  name = "debug error",
  layout = {
    Vertical = {
      config = {
        constraints = {
          { Min = 14 },
          { MinLessThanScreenHeight = 14 },
        },
      },
      splits = {
        {
          Static = {
            CustomParagraph = {
              ui = { title = { format = "debug error" } },
              body = [[

  Some errors occurred during startup.
  If you think this is a bug, please report it at:

  https://github.com/sayanarijit/xplr/issues/new

  Press `enter` to open the logs in your $EDITOR.
  Press `escape` to ignore the errors and continue with the default config.

  To disable this mode, set `xplr.config.general.disable_debug_error_mode`
  to `true` in your config file.
              ]],
            },
          },
        },
        "InputAndLogs",
      },
    },
  },
  key_bindings = {
    on_key = {
      ["enter"] = {
        help = "open logs in editor",
        messages = {
          {
            BashExec = [===[
              cat "${XPLR_PIPE_LOGS_OUT:?}" | ${EDITOR:-vi} -
            ]===],
          },
        },
      },
      ["q"] = {
        help = "quit",
        messages = {
          "Quit",
        },
      },
    },
    default = {
      messages = {},
    },
  },
}

-- The builtin recover mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.recover = {
  name = "recover",
  layout = {
    Static = {
      CustomParagraph = {
        ui = { title = { format = "recover" } },
        body = [[

  You pressed an invalid key and went into "recover" mode.
  This mode saves you from performing unwanted actions.

  Let's calm down, press `escape`, and try again.

  To disable this mode, set `xplr.config.general.enable_recover_mode`
  to `false` in your config file.
        ]],
      },
    },
  },
  key_bindings = {
    default = {
      messages = {},
    },
  },
}

-- The builtin go to path mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.go_to_path = {
  name = "go to path",
  key_bindings = {
    on_key = {
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExecSilently0 = [===[
              PTH="$XPLR_INPUT_BUFFER"
              PTH_ESC=$(printf %q "$PTH")
              if [ -d "$PTH" ]; then
                "$XPLR" -m 'ChangeDirectory: %q' "$PTH"
              elif [ -e "$PTH" ]; then
                "$XPLR" -m 'FocusPath: %q' "$PTH"
              else
                "$XPLR" -m 'LogError: %q' "could not find $PTH_ESC"
              fi
            ]===],
          },
          "PopMode",
        },
      },
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin move_to mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.move_to = {
  name = "move_to",
  prompt = "ð ❯ ",
  key_bindings = {
    on_key = {
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExec0 = [===[
              DEST="$XPLR_INPUT_BUFFER"
              [ -z "$DEST" ] && exit
              if [ ! -d "$DEST" ] && ! mkdir -p -- "$DEST"; then
                  "$XPLR" -m 'LogError: %q' "could not create $DEST"
                  exit
              fi
              "$XPLR" -m "ChangeDirectory: %q" "$DEST"
              ! cd -- "$DEST" && exit
              DEST="$(pwd)" && echo "PWD=$DEST"
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                BASENAME=$(basename -- "$PTH")
                BASENAME_ESC=$(printf %q "$BASENAME")
                if [ -e "$BASENAME" ]; then
                  echo
                  echo "$BASENAME_ESC exists, do you want to overwrite it?"
                  read -p "[y]es, [n]o, [S]kip: " ANS < /dev/tty
                  case "$ANS" in
                    [yY]*)
                      ;;
                    [nN]*)
                      read -p "Enter new name: " BASENAME < /dev/tty
                      BASENAME_ESC=$(printf %q "$BASENAME")
                      ;;
                    *)
                      continue
                      ;;
                  esac
                fi
                if mv -v -- "${PTH:?}" "./${BASENAME:?}"; then
                  "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC moved to $BASENAME_ESC"
                  "$XPLR" -m 'FocusPath: %q' "$BASENAME"
                else
                  "$XPLR" -m 'LogError: %q' "could not move $PTH_ESC to $BASENAME_ESC"
                fi
              done < "${XPLR_PIPE_RESULT_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin copy_to mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.copy_to = {
  name = "copy_to",
  prompt = "ð ❯ ",
  key_bindings = {
    on_key = {
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExec0 = [===[
              DEST="$XPLR_INPUT_BUFFER"
              [ -z "$DEST" ] && exit
              if [ ! -d "$DEST" ] && ! mkdir -p -- "$DEST"; then
                  "$XPLR" -m 'LogError: %q' "could not create $DEST"
                  exit
              fi
              "$XPLR" -m "ChangeDirectory: %q" "$DEST"
              ! cd -- "$DEST" && exit
              DEST="$(pwd)" && echo "PWD=$DEST"
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                BASENAME=$(basename -- "$PTH")
                BASENAME_ESC=$(printf %q "$BASENAME")
                if [ -e "$BASENAME" ]; then
                  echo
                  echo "$BASENAME_ESC exists, do you want to overwrite it?"
                  read -p "[y]es, [n]o, [S]kip: " ANS < /dev/tty
                  case "$ANS" in
                    [yY]*)
                      ;;
                    [nN]*)
                      read -p "Enter new name: " BASENAME < /dev/tty
                      BASENAME_ESC=$(printf %q "$BASENAME")
                      ;;
                    *)
                      continue
                      ;;
                  esac
                fi
                if cp -vr -- "${PTH:?}" "./${BASENAME:?}"; then
                  "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC copied to $BASENAME_ESC"
                  "$XPLR" -m 'FocusPath: %q' "$BASENAME"
                else
                  "$XPLR" -m 'LogError: %q' "could not copy $PTH_ESC to $BASENAME_ESC"
                fi
              done < "${XPLR_PIPE_RESULT_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin selection ops mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.selection_ops = {
  name = "selection ops",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["e"] = {
        help = "edit selection",
        messages = {
          {
            BashExec0 = [===[
              TMPFILE="$(mktemp)"
              while IFS= read -r -d '' PTH; do
                echo $(printf %q "${PTH:?}") >> "${TMPFILE:?}"
              done < "${XPLR_PIPE_SELECTION_OUT:?}"
              ${EDITOR:-vi} "${TMPFILE:?}"
              [ ! -e "$TMPFILE" ] && exit
              "$XPLR" -m ClearSelection
              while IFS= read -r PTH_ESC; do
                "$XPLR" -m 'SelectPath: %q' "$(eval printf %s ${PTH_ESC:?})"
              done < "${TMPFILE:?}"
              rm -- "${TMPFILE:?}"
            ]===],
          },
          "PopMode",
        },
      },
      ["l"] = {
        help = "list selection",
        messages = {
          {
            BashExec0 = [===[
              [ -z "$PAGER" ] && PAGER="less -+F"

              while IFS= read -r -d '' PTH; do
                echo $(printf %q "$PTH")
              done < "${XPLR_PIPE_SELECTION_OUT:?}" | ${PAGER:?}
            ]===],
          },
          "PopMode",
        },
      },
      ["c"] = {
        help = "copy here",
        messages = {
          {
            BashExec0 = [===[
              "$XPLR" -m ExplorePwd
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                BASENAME=$(basename -- "$PTH")
                BASENAME_ESC=$(printf %q "$BASENAME")
                if [ -e "$BASENAME" ]; then
                  echo
                  echo "$BASENAME_ESC exists, do you want to overwrite it?"
                  read -p "[y]es, [n]o, [S]kip: " ANS < /dev/tty
                  case "$ANS" in
                    [yY]*)
                      ;;
                    [nN]*)
                      read -p "Enter new name: " BASENAME < /dev/tty
                      BASENAME_ESC=$(printf %q "$BASENAME")
                      ;;
                    *)
                      continue
                      ;;
                  esac
                fi
                if cp -vr -- "${PTH:?}" "./${BASENAME:?}"; then
                  "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC copied to $BASENAME_ESC"
                  "$XPLR" -m 'FocusPath: %q' "$BASENAME"
                else
                  "$XPLR" -m 'LogError: %q' "could not copy $PTH_ESC to $BASENAME_ESC"
                fi
              done < "${XPLR_PIPE_SELECTION_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["m"] = {
        help = "move here",
        messages = {
          {
            BashExec0 = [===[
              "$XPLR" -m ExplorePwd
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                BASENAME=$(basename -- "$PTH")
                BASENAME_ESC=$(printf %q "$BASENAME")
                if [ -e "$BASENAME" ]; then
                  echo
                  echo "$BASENAME_ESC exists, do you want to overwrite it?"
                  read -p "[y]es, [n]o, [S]kip: " ANS < /dev/tty
                  case "$ANS" in
                    [yY]*)
                      ;;
                    [nN]*)
                      read -p "Enter new name: " BASENAME < /dev/tty
                      BASENAME_ESC=$(printf %q "$BASENAME")
                      ;;
                    *)
                      continue
                      ;;
                  esac
                fi
                if mv -v -- "${PTH:?}" "./${BASENAME:?}"; then
                  "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC moved to $BASENAME_ESC"
                  "$XPLR" -m 'FocusPath: %q' "$BASENAME"
                else
                  "$XPLR" -m 'LogError: %q' "could not move $PTH_ESC to $BASENAME_ESC"
                fi
              done < "${XPLR_PIPE_SELECTION_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["s"] = {
        help = "softlink here",
        messages = {
          {
            BashExec0 = [===[
              "$XPLR" -m ExplorePwd
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                BASENAME=$(basename -- "$PTH")
                BASENAME_ESC=$(printf %q "$BASENAME")
                if [ -e "$BASENAME" ]; then
                  echo
                  echo "$BASENAME_ESC exists, do you want to overwrite it?"
                  read -p "[y]es, [n]o, [S]kip: " ANS < /dev/tty
                  case "$ANS" in
                    [yY]*)
                      ;;
                    [nN]*)
                      read -p "Enter new name: " BASENAME < /dev/tty
                      BASENAME_ESC=$(printf %q "$BASENAME")
                      ;;
                    *)
                      continue
                      ;;
                  esac
                fi
                if ln -sv -- "${PTH:?}" "./${BASENAME:?}"; then
                  "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC softlinked as $BASENAME_ESC"
                  "$XPLR" -m 'FocusPath: %q' "$BASENAME"
                else
                  "$XPLR" -m 'LogError: %q' "could not softlink $PTH_ESC as $BASENAME_ESC"
                fi
              done < "${XPLR_PIPE_SELECTION_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["h"] = {
        help = "hardlink here",
        messages = {
          {
            BashExec0 = [===[
              "$XPLR" -m ExplorePwd
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                BASENAME=$(basename -- "$PTH")
                BASENAME_ESC=$(printf %q "$BASENAME")
                if [ -e "$BASENAME" ]; then
                  echo
                  echo "$BASENAME_ESC exists, do you want to overwrite it?"
                  read -p "[y]es, [n]o, [S]kip: " ANS < /dev/tty
                  case "$ANS" in
                    [yY]*)
                      ;;
                    [nN]*)
                      read -p "Enter new name: " BASENAME < /dev/tty
                      BASENAME_ESC=$(printf %q "$BASENAME")
                      ;;
                    *)
                      continue
                      ;;
                  esac
                fi
                if ln -v -- "${PTH:?}" "./${BASENAME:?}"; then
                  "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC hardlinked as $BASENAME_ESC"
                  "$XPLR" -m 'FocusPath: %q' "$BASENAME"
                else
                  "$XPLR" -m 'LogError: %q' "could not hardlink $PTH_ESC as $BASENAME_ESC"
                fi
              done < "${XPLR_PIPE_SELECTION_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["u"] = {
        help = "clear selection",
        messages = {
          "ClearSelection",
          "PopMode",
        },
      },
    },
  },
}

-- The builtin create mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.create = {
  name = "create",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["c"] = {
        help = "create file or directory/",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "create_conditional" },
          { SetInputBuffer = "" },
        },
      },
      ["d"] = {
        help = "create directory",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "create_directory" },
          { SetInputBuffer = "" },
        },
      },
      ["f"] = {
        help = "create file",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "create_file" },
          { SetInputBuffer = "" },
        },
      },
    },
  },
}

-- The builtin create directory mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.create_directory = {
  name = "create directory",
  prompt = "ð ❯ ",
  key_bindings = {
    on_key = {
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExecSilently0 = [===[
              PTH="$XPLR_INPUT_BUFFER"
              PTH_ESC=$(printf %q "$PTH")
              if [ "$PTH" ]; then
                mkdir -p -- "$PTH" \
                && "$XPLR" -m 'SetInputBuffer: ""' \
                && "$XPLR" -m ExplorePwd \
                && "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC created" \
                && "$XPLR" -m 'FocusPath: %q' "$PTH"
              else
                "$XPLR" -m PopMode
              fi
            ]===],
          },
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin create file mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.create_file = {
  name = "create file",
  prompt = "ƒ ❯ ",
  key_bindings = {
    on_key = {
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExecSilently0 = [===[
              PTH="$XPLR_INPUT_BUFFER"
              PTH_ESC=$(printf %q "$PTH")
              if [ "$PTH" ]; then
                mkdir -p -- "$(dirname $(realpath -m $PTH))"  # This may fail.
                touch -- "$PTH" \
                && "$XPLR" -m 'SetInputBuffer: ""' \
                && "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC created" \
                && "$XPLR" -m 'ExplorePwd' \
                && "$XPLR" -m 'FocusPath: %q' "$PTH"
              else
                "$XPLR" -m PopMode
              fi
            ]===],
          },
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin create conditional mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.create_conditional = {
  name = "create conditional",
  prompt = "* ❯ ",
  key_bindings = {
    on_key = {
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExecSilently0 = [===[
              PTH="$XPLR_INPUT_BUFFER"
              PTH_ESC=$(printf %q "$PTH")
              if [ "$PTH" ]; then
                if [ "${PTH: -1}" = "/" ]; then
                  mkdir -p -- "$PTH" \
                  && "$XPLR" -m 'SetInputBuffer: ""' \
                  && "$XPLR" -m ExplorePwd \
                  && "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC created" \
                  && "$XPLR" -m 'FocusPath: %q' "$PTH"
                else
                  mkdir -p -- "$(dirname -- "$PTH")" \
                  && touch -- "$PTH" \
                  && "$XPLR" -m 'SetInputBuffer: ""' \
                  && "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC created" \
                  && "$XPLR" -m 'ExplorePwd' \
                  && "$XPLR" -m 'FocusPath: %q' "$PTH"
                fi
              else
                "$XPLR" -m PopMode
              fi
            ]===],
          },
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin number mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.number = {
  name = "number",
  prompt = ":",
  key_bindings = {
    on_key = {
      ["down"] = {
        help = "to down",
        messages = {
          "FocusNextByRelativeIndexFromInput",
          "PopMode",
        },
      },
      ["enter"] = {
        help = "to index",
        messages = {
          "FocusByIndexFromInput",
          "PopMode",
        },
      },
      ["up"] = {
        help = "to up",
        messages = {
          "FocusPreviousByRelativeIndexFromInput",
          "PopMode",
        },
      },
    },
    on_navigation = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
    on_number = {
      help = "input",
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

xplr.config.modes.builtin.number.key_bindings.on_key["j"] =
  xplr.config.modes.builtin.number.key_bindings.on_key["down"]
xplr.config.modes.builtin.number.key_bindings.on_key["k"] =
  xplr.config.modes.builtin.number.key_bindings.on_key["up"]

-- The builtin go to mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.go_to = {
  name = "go to",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["f"] = {
        help = "follow symlink",
        messages = {
          "FollowSymlink",
          "PopMode",
        },
      },
      ["g"] = {
        help = "top",
        messages = {
          "FocusFirst",
          "PopMode",
        },
      },
      ["p"] = {
        help = "path",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "go_to_path" },
          { SetInputBuffer = "" },
        },
      },
      ["i"] = {
        help = "initial $PWD",
        messages = {
          "PopMode",
          {
            BashExecSilently0 = [===[
              "$XPLR" -m 'ChangeDirectory: %q' "${XPLR_INITIAL_PWD:?}"
            ]===],
          },
        },
      },
      ["x"] = {
        help = "open in gui",
        messages = {
          {
            BashExecSilently0 = [===[
              if [ -z "$OPENER" ]; then
                if command -v xdg-open; then
                  OPENER=xdg-open
                elif command -v open; then
                  OPENER=open
                else
                  "$XPLR" -m 'LogError: %q' "$OPENER not found"
                  exit 1
                fi
              fi
              while IFS= read -r -d '' PTH; do
                $OPENER "${PTH:?}" > /dev/null 2>&1
              done < "${XPLR_PIPE_RESULT_OUT:?}"
            ]===],
          },
          "ClearScreen",
          "PopMode",
        },
      },
    },
  },
}

-- The builtin rename mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.rename = {
  name = "rename",
  key_bindings = {
    on_key = {
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExecSilently0 = [===[
              SRC="${XPLR_FOCUS_PATH:?}"
              SRC_ESC=$(printf %q "$SRC")
              TARGET="${XPLR_INPUT_BUFFER:?}"
              TARGET_ESC=$(printf %q "$TARGET")
              if [ -e "${TARGET:?}" ]; then
                "$XPLR" -m 'LogError: %q' "$TARGET_ESC already exists"
              else
                mv -- "${SRC:?}" "${TARGET:?}" \
                  && "$XPLR" -m ExplorePwd \
                  && "$XPLR" -m 'FocusPath: %q' "$TARGET" \
                  && "$XPLR" -m 'LogSuccess: %q' "$SRC_ESC renamed to $TARGET_ESC"
              fi
            ]===],
          },
          "PopMode",
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin duplicate as mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.duplicate_as = {
  name = "duplicate as",
  key_bindings = {
    on_key = {
      ["tab"] = {
        help = "try complete",
        messages = {
          "TryCompletePath",
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExecSilently0 = [===[
              SRC="${XPLR_FOCUS_PATH:?}"
              SRC_ESC=$(printf %q "$SRC")
              TARGET="${XPLR_INPUT_BUFFER:?}"
              TARGET_ESC=$(printf %q "$TARGET")
              if [ -e "${TARGET:?}" ]; then
                "$XPLR" -m 'LogError: %q' "$TARGET_ESC already exists"
              else
                cp -r -- "${SRC:?}" "${TARGET:?}" \
                  && "$XPLR" -m ExplorePwd \
                  && "$XPLR" -m 'FocusPath: %q' "$TARGET_ESC" \
                  && "$XPLR" -m 'LogSuccess: %q' "$SRC_ESC duplicated as $TARGET_ESC"
              fi
            ]===],
          },
          "PopMode",
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- The builtin delete mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.delete = {
  name = "delete",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["D"] = {
        help = "force delete",
        messages = {
          {
            BashExec0 = [===[
              while IFS= read -r -d '' PTH; do
                printf '%q\n' "$PTH"
              done < "${XPLR_PIPE_RESULT_OUT:?}"
              echo
              read -p "Permanently delete these files? [Y/n]: " ANS
              [ "${ANS:-Y}" = "Y" ] || [ "$ANS" = "y" ] || exit 0
              echo

              "$XPLR" -m ExplorePwd
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                if rm -rfv -- "${PTH:?}"; then
                  "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC deleted"
                else
                  "$XPLR" -m 'LogError: %q' "could not delete $PTH_ESC"
                  "$XPLR" -m 'FocusPath: %q' "$PTH"
                fi
              done < "${XPLR_PIPE_RESULT_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["d"] = {
        help = "delete",
        messages = {
          {
            BashExec0 = [===[
              while IFS= read -r -d '' PTH; do
                printf '%q\n' "$PTH"
              done < "${XPLR_PIPE_RESULT_OUT:?}"
              echo
              read -p "Permanently delete these files? [Y/n]: " ANS
              [ "${ANS:-Y}" = "Y" ] || [ "$ANS" = "y" ] || exit 0
              echo

              "$XPLR" -m ExplorePwd
              while IFS= read -r -d '' PTH; do
                PTH_ESC=$(printf %q "$PTH")
                if [ -d "$PTH" ] && [ ! -L "$PTH" ]; then
                  if rmdir -v -- "${PTH:?}"; then
                    "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC deleted"
                  else
                    "$XPLR" -m 'LogError: %q' "could not delete $PTH_ESC"
                    "$XPLR" -m 'FocusPath: %q' "$PTH"
                  fi
                else
                  if rm -v -- "${PTH:?}"; then
                    "$XPLR" -m 'LogSuccess: %q' "$PTH_ESC deleted"
                  else
                    "$XPLR" -m 'LogError: %q' "could not delete $PTH_ESC"
                    "$XPLR" -m 'FocusPath: %q' "$PTH"
                  fi
                fi
              done < "${XPLR_PIPE_RESULT_OUT:?}"
              echo
              read -p "[press enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
    },
  },
}

-- The builtin action mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.action = {
  name = "action to",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["!"] = {
        help = "shell",
        messages = {
          "PopMode",
          { Call0 = { command = os.getenv("SHELL") or "bash", args = { "-i" } } },
          "ExplorePwdAsync",
        },
      },
      ["c"] = {
        help = "create",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "create" },
        },
      },
      ["e"] = {
        help = "open in editor",
        messages = {
          {
            BashExec0 = [===[
              ${EDITOR:-vi} "${XPLR_FOCUS_PATH:?}"
            ]===],
          },
          "PopMode",
        },
      },
      ["l"] = {
        help = "logs",
        messages = {
          {
            BashExec = [===[
              [ -z "$PAGER" ] && PAGER="less -+F"
              cat -- "${XPLR_PIPE_LOGS_OUT}" | ${PAGER:?}
            ]===],
          },
          "PopMode",
        },
      },
      ["s"] = {
        help = "selection operations",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "selection_ops" },
        },
      },
      ["m"] = {
        help = "toggle mouse",
        messages = {
          "PopMode",
          "ToggleMouse",
        },
      },
      ["p"] = {
        help = "edit permissions",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "edit_permissions" },
          {
            BashExecSilently0 = [===[
              PERM=$(stat -c '%a' -- "${XPLR_FOCUS_PATH:?}")
              "$XPLR" -m 'SetInputBuffer: %q' "${PERM:?}"
            ]===],
          },
        },
      },
      ["v"] = {
        help = "vroot",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "vroot" },
        },
      },
      ["q"] = {
        help = "quit options",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "quit" },
        },
      },
    },
    on_number = {
      help = "go to index",
      messages = {
        "PopMode",
        { SwitchModeBuiltin = "number" },
        "BufferInputFromKey",
      },
    },
  },
}

-- The builtin quit mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.quit = {
  name = "quit",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["enter"] = {
        help = "just quit",
        messages = {
          "Quit",
        },
      },
      ["p"] = {
        help = "quit printing pwd",
        messages = {
          "PrintPwdAndQuit",
        },
      },
      ["f"] = {
        help = "quit printing focus",
        messages = {
          "PrintFocusPathAndQuit",
        },
      },
      ["s"] = {
        help = "quit printing selection",
        messages = {
          "PrintSelectionAndQuit",
        },
      },
      ["r"] = {
        help = "quit printing result",
        messages = {
          "PrintResultAndQuit",
        },
      },
    },
  },
}

-- The builtin search mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.search = {
  name = "search",
  prompt = "/",
  key_bindings = {
    on_key = {
      ["up"] = {
        help = "up",
        messages = {
          "FocusPrevious",
        },
      },
      ["down"] = {
        help = "down",
        messages = {
          "FocusNext",
        },
      },
      ["ctrl-z"] = {
        help = "toggle ordering",
        messages = {
          "ToggleSearchOrder",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-a"] = {
        help = "toggle search algorithm",
        messages = {
          "ToggleSearchAlgorithm",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-r"] = {
        help = "regex search",
        messages = {
          "SearchRegexFromInput",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-f"] = {
        help = "fuzzy search",
        messages = {
          "SearchFuzzyFromInput",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-s"] = {
        help = "sort (no search order)",
        messages = {
          "DisableSearchOrder",
          "ExplorePwdAsync",
          { SwitchModeBuiltinKeepingInputBuffer = "sort" },
        },
      },
      ["right"] = {
        help = "enter",
        messages = {
          "Enter",
          { SetInputBuffer = "" },
        },
      },
      ["left"] = {
        help = "back",
        messages = {
          "Back",
          { SetInputBuffer = "" },
        },
      },
      ["tab"] = {
        help = "toggle selection",
        messages = {
          "ToggleSelection",
          "FocusNext",
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          "AcceptSearch",
          "PopMode",
        },
      },
      ["esc"] = {
        help = "cancel",
        messages = {
          "CancelSearch",
          "PopMode",
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
        "SearchFromInput",
        "ExplorePwdAsync",
      },
    },
  },
}

xplr.config.modes.builtin.search.key_bindings.on_key["ctrl-n"] =
  xplr.config.modes.builtin.search.key_bindings.on_key["down"]
xplr.config.modes.builtin.search.key_bindings.on_key["ctrl-p"] =
  xplr.config.modes.builtin.search.key_bindings.on_key["up"]

-- The builtin filter mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.filter = {
  name = "filter",
  key_bindings = {
    on_key = {
      ["r"] = {
        help = "relative path does match regex",
        messages = {
          { SwitchModeBuiltin = "relative_path_does_match_regex" },
          { SetInputBuffer = "" },
          { AddNodeFilterFromInput = "RelativePathDoesMatchRegex" },
          "ExplorePwdAsync",
        },
      },
      ["R"] = {
        help = "relative path does not match regex",
        messages = {
          { SwitchModeBuiltin = "relative_path_does_not_match_regex" },
          { SetInputBuffer = "" },
          { AddNodeFilterFromInput = "RelativePathDoesNotMatchRegex" },
          "ExplorePwdAsync",
        },
      },
      ["backspace"] = {
        help = "remove last filter",
        messages = {
          "RemoveLastNodeFilter",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-r"] = {
        help = "reset filters",
        messages = {
          "ResetNodeFilters",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-u"] = {
        help = "clear filters",
        messages = {
          "ClearNodeFilters",
          "ExplorePwdAsync",
        },
      },
    },
  },
}

-- The builtin relative_path_does_match_regex mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.relative_path_does_match_regex = {
  name = "relative path does match regex",
  prompt = xplr.config.general.sort_and_filter_ui.filter_identifiers.RelativePathDoesMatchRegex.format,
  key_bindings = {
    on_key = {
      ["enter"] = {
        help = "submit",
        messages = {
          "PopMode",
        },
      },
      ["esc"] = {
        messages = {
          { RemoveNodeFilterFromInput = "RelativePathDoesMatchRegex" },
          "PopMode",
          "ExplorePwdAsync",
        },
      },
    },
    default = {
      messages = {
        { RemoveNodeFilterFromInput = "RelativePathDoesMatchRegex" },
        "UpdateInputBufferFromKey",
        { AddNodeFilterFromInput = "RelativePathDoesMatchRegex" },
        "ExplorePwdAsync",
      },
    },
  },
}

-- The builtin relative_path_does_not_match_regex mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.relative_path_does_not_match_regex = {
  name = "relative path does not match regex",
  prompt = xplr.config.general.sort_and_filter_ui.filter_identifiers.RelativePathDoesNotMatchRegex.format,
  key_bindings = {
    on_key = {
      ["enter"] = {
        help = "submit",
        messages = {
          "PopMode",
        },
      },
      ["esc"] = {
        messages = {
          { RemoveNodeFilterFromInput = "RelativePathDoesNotMatchRegex" },
          "PopMode",
          "ExplorePwdAsync",
        },
      },
    },
    default = {
      messages = {
        { RemoveNodeFilterFromInput = "RelativePathDoesNotMatchRegex" },
        "UpdateInputBufferFromKey",
        { AddNodeFilterFromInput = "RelativePathDoesNotMatchRegex" },
        "ExplorePwdAsync",
      },
    },
  },
}

-- The builtin sort mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.sort = {
  name = "sort",
  key_bindings = {
    on_key = {
      ["!"] = {
        help = "reverse sorters",
        messages = {
          "ReverseNodeSorters",
          "ExplorePwdAsync",
        },
      },
      ["E"] = {
        help = "by canonical extension reverse",
        messages = {
          { AddNodeSorter = { sorter = "ByCanonicalExtension", reverse = true } },
          "ExplorePwdAsync",
        },
      },
      ["M"] = {
        help = "by canonical mime essence reverse",
        messages = {
          { AddNodeSorter = { sorter = "ByCanonicalMimeEssence", reverse = true } },
          "ExplorePwdAsync",
        },
      },
      ["N"] = {
        help = "by node type reverse",
        messages = {
          { AddNodeSorter = { sorter = "ByCanonicalIsDir", reverse = true } },
          { AddNodeSorter = { sorter = "ByCanonicalIsFile", reverse = true } },
          { AddNodeSorter = { sorter = "ByIsSymlink", reverse = true } },
          "ExplorePwdAsync",
        },
      },
      ["R"] = {
        help = "by relative path reverse",
        messages = {
          { AddNodeSorter = { sorter = "ByIRelativePath", reverse = true } },
          "ExplorePwdAsync",
        },
      },
      ["S"] = {
        help = "by size reverse",
        messages = {
          { AddNodeSorter = { sorter = "BySize", reverse = true } },
          "ExplorePwdAsync",
        },
      },
      ["backspace"] = {
        help = "remove last sorter",
        messages = {
          "RemoveLastNodeSorter",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-r"] = {
        help = "reset sorters",
        messages = {
          "ResetNodeSorters",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-u"] = {
        help = "clear sorters",
        messages = {
          "ClearNodeSorters",
          "ExplorePwdAsync",
        },
      },
      ["e"] = {
        help = "by canonical extension",
        messages = {
          { AddNodeSorter = { sorter = "ByCanonicalExtension", reverse = false } },
          "ExplorePwdAsync",
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          "PopModeKeepingInputBuffer",
        },
      },
      ["esc"] = {
        messages = {
          "PopModeKeepingInputBuffer",
        },
      },
      ["m"] = {
        help = "by canonical mime essence",
        messages = {
          { AddNodeSorter = { sorter = "ByCanonicalMimeEssence", reverse = false } },
          "ExplorePwdAsync",
        },
      },
      ["n"] = {
        help = "by node type",
        messages = {
          { AddNodeSorter = { sorter = "ByCanonicalIsDir", reverse = false } },
          { AddNodeSorter = { sorter = "ByCanonicalIsFile", reverse = false } },
          { AddNodeSorter = { sorter = "ByIsSymlink", reverse = false } },
          "ExplorePwdAsync",
        },
      },
      ["r"] = {
        help = "by relative path",
        messages = {
          { AddNodeSorter = { sorter = "ByIRelativePath", reverse = false } },
          "ExplorePwdAsync",
        },
      },
      ["s"] = {
        help = "by size",
        messages = {
          { AddNodeSorter = { sorter = "BySize", reverse = false } },
          "ExplorePwdAsync",
        },
      },
      ["c"] = {
        help = "by created",
        messages = {
          { AddNodeSorter = { sorter = "ByCreated", reverse = false } },
          "ExplorePwdAsync",
        },
      },
      ["C"] = {
        help = "by created reverse",
        messages = {
          { AddNodeSorter = { sorter = "ByCreated", reverse = true } },
          "ExplorePwdAsync",
        },
      },
      ["l"] = {
        help = "by last modified",
        messages = {
          { AddNodeSorter = { sorter = "ByLastModified", reverse = false } },
          "ExplorePwdAsync",
        },
      },
      ["L"] = {
        help = "by last modified reverse",
        messages = {
          { AddNodeSorter = { sorter = "ByLastModified", reverse = true } },
          "ExplorePwdAsync",
        },
      },
    },
  },
}

-- The builtin switch layout mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.switch_layout = {
  name = "switch layout",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["1"] = {
        help = "default",
        messages = {
          { SwitchLayoutBuiltin = "default" },
          "PopMode",
        },
      },
      ["2"] = {
        help = "no help menu",
        messages = {
          { SwitchLayoutBuiltin = "no_help" },
          "PopMode",
        },
      },
      ["3"] = {
        help = "no selection panel",
        messages = {
          { SwitchLayoutBuiltin = "no_selection" },
          "PopMode",
        },
      },
      ["4"] = {
        help = "no help or selection",
        messages = {
          { SwitchLayoutBuiltin = "no_help_no_selection" },
          "PopMode",
        },
      },
    },
  },
}

-- The builtin vroot mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.vroot = {
  name = "vroot",
  layout = "HelpMenu",
  key_bindings = {
    on_key = {
      ["v"] = {
        help = "toggle vroot",
        messages = {
          "PopMode",
          "ToggleVroot",
        },
      },
      ["."] = {
        help = "vroot $PWD",
        messages = {
          "PopMode",
          {
            BashExecSilently0 = [===[
              "$XPLR" -m 'SetVroot: %q' "${PWD:?}"
            ]===],
          },
        },
      },
      ["/"] = {
        help = "vroot /",
        messages = {
          "PopMode",
          { SetVroot = "/" },
        },
      },
      ["~"] = {
        help = "vroot $HOME",
        messages = {
          "PopMode",
          {
            BashExecSilently0 = [===[
              "$XPLR" -m 'SetVroot: %q' "${HOME:?}"
            ]===],
          },
        },
      },
      ["ctrl-r"] = {
        help = "reset vroot",
        messages = {
          "PopMode",
          "ResetVroot",
        },
      },
      ["ctrl-u"] = {
        help = "unset vroot",
        messages = {
          "PopMode",
          "UnsetVroot",
        },
      },
    },
  },
}

-- The builtin edit permissions mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.edit_permissions = {
  name = "edit permissions",
  key_bindings = {
    on_key = {
      ["u"] = {
        help = "+user",
        messages = {
          {
            BashExecSilently0 = [===[
              PERM="${XPLR_INPUT_BUFFER:-000}"
              U="${PERM: -3:-2}"
              G="${PERM: -2:-1}"
              O="${PERM: -1}"

              U="$(( (${U:-0} + 1) % 8 ))"
              "$XPLR" -m 'SetInputBuffer: %q' "${U:-0}${G:-0}${O:-0}"
            ]===],
          },
        },
      },
      ["U"] = {
        help = "-user",
        messages = {
          {
            BashExecSilently0 = [===[
              PERM="${XPLR_INPUT_BUFFER:-000}"
              U="${PERM: -3:-2}"
              G="${PERM: -2:-1}"
              O="${PERM: -1}"

              U="$(( ${U:-0}-1 < 0 ? 7 : ${U:-0}-1 ))"
              "$XPLR" -m 'SetInputBuffer: %q' "${U:-0}${G:-0}${O:-0}"
            ]===],
          },
        },
      },
      ["g"] = {
        help = "+group",
        messages = {
          {
            BashExecSilently0 = [===[
              PERM="${XPLR_INPUT_BUFFER:-000}"
              U="${PERM: -3:-2}"
              G="${PERM: -2:-1}"
              O="${PERM: -1}"

              G="$(( (${G:-0} + 1) % 8 ))"
              "$XPLR" -m 'SetInputBuffer: %q' "${U:-0}${G:-0}${O:-0}"
            ]===],
          },
        },
      },
      ["G"] = {
        help = "-group",
        messages = {
          {
            BashExecSilently0 = [===[
              PERM="${XPLR_INPUT_BUFFER:-000}"
              U="${PERM: -3:-2}"
              G="${PERM: -2:-1}"
              O="${PERM: -1}"

              G="$(( ${G:-0}-1 < 0 ? 7 : ${G:-0}-1 ))"
              "$XPLR" -m 'SetInputBuffer: %q' "${U:-0}${G:-0}${O:-0}"
            ]===],
          },
        },
      },
      ["o"] = {
        help = "+other",
        messages = {
          {
            BashExecSilently0 = [===[
              PERM="${XPLR_INPUT_BUFFER:-000}"
              U="${PERM: -3:-2}"
              G="${PERM: -2:-1}"
              O="${PERM: -1}"

              O="$(( (${O:-0} + 1) % 8 ))"
              "$XPLR" -m 'SetInputBuffer: %q' "${U:-0}${G:-0}${O:-0}"
            ]===],
          },
        },
      },
      ["O"] = {
        help = "-other",
        messages = {
          {
            BashExecSilently0 = [===[
              PERM="${XPLR_INPUT_BUFFER:-000}"
              U="${PERM: -3:-2}"
              G="${PERM: -2:-1}"
              O="${PERM: -1}"

              O="$(( ${O:-0}-1 < 0 ? 7 : ${O:-0}-1 ))"
              "$XPLR" -m 'SetInputBuffer: %q' "${U:-0}${G:-0}${O:-0}"
            ]===],
          },
        },
      },
      ["m"] = {
        help = "max",
        messages = {
          {
            BashExecSilently0 = [===[
              "$XPLR" -m 'SetInputBuffer: %q' "777"
            ]===],
          },
        },
      },
      ["M"] = {
        help = "min",
        messages = {
          {
            BashExecSilently0 = [===[
              "$XPLR" -m 'SetInputBuffer: %q' "000"
            ]===],
          },
        },
      },
      ["ctrl-r"] = {
        help = "reset",
        messages = {
          {
            BashExecSilently0 = [===[
              PERM=$(stat -c '%a' -- "${XPLR_FOCUS_PATH:?}")
              "$XPLR" -m 'SetInputBuffer: %q' "${PERM:?}"
            ]===],
          },
        },
      },
      ["enter"] = {
        help = "submit",
        messages = {
          {
            BashExecSilently0 = [===[
              chmod "${XPLR_INPUT_BUFFER:?}" -- "${XPLR_FOCUS_PATH:?}"
            ]===],
          },
          "PopMode",
          "ExplorePwdAsync",
        },
      },
    },
    default = {
      messages = {
        "UpdateInputBufferFromKey",
      },
    },
  },
}

-- This is where you define custom modes.
--
-- Type: mapping of the following key-value pairs:
--
-- * key: string
-- * value: [Mode](https://xplr.dev/en/mode)
--
-- Example:
--
-- ```lua
-- xplr.config.modes.custom.example = {
--   name = "example",
--   key_bindings = {
--     on_key = {
--       enter = {
--         help = "default mode",
--         messages = {
--           "PopMode",
--           { SwitchModeBuiltin = "default" },
--         },
--       },
--     },
--   },
-- }
--
-- xplr.config.general.initial_mode = "example"
-- ```
xplr.config.modes.custom = {}

-- ## Function ----------------------------------------------------------------
--
-- While `xplr.config` defines all the static parts of the configuration,
-- `xplr.fn` defines all the dynamic parts using functions.
--
-- See: [Lua Function Calls](https://xplr.dev/en/lua-function-calls)
--
-- As always, `xplr.fn.builtin` is where the built-in functions are defined
-- that can be overwritten.

xplr.fn.builtin.fmt_general_selection_item = function(n)
  local nl = xplr.util.paint("\\n", { add_modifiers = { "Italic", "Dim" } })
  local sh_config = { with_prefix_dots = true, without_suffix_dots = true }
  local shortened = xplr.util.shorten(n.absolute_path, sh_config)
  if n.is_dir then
    shortened = shortened .. "/"
  end
  local meta_style = xplr.util.node_type(n).style
  local ls_style = xplr.util.lscolor(n.absolute_path)
  local style = xplr.util.style_mix({ ls_style, meta_style })
  return xplr.util.paint(shortened:gsub("\n", nl), style)
end

-- Renders the first column in the table
xplr.fn.builtin.fmt_general_table_row_cols_0 = function(m)
  local r = ""
  if m.is_before_focus then
    r = r .. " -"
  else
    r = r .. "  "
  end

  r = r .. m.relative_index .. "│" .. m.index

  return r
end

-- Renders the second column in the table
xplr.fn.builtin.fmt_general_table_row_cols_1 = function(m)
  local nl = xplr.util.paint("\\n", { add_modifiers = { "Italic", "Dim" } })
  local r = m.tree .. m.prefix
  local ls_style = xplr.util.lscolor(m.absolute_path)
  local style = xplr.util.style_mix({ ls_style, m.style })

  if m.meta.icon == nil then
    r = r .. ""
  else
    r = r .. m.meta.icon .. " "
  end

  local rel = m.relative_path
  if m.is_dir then
    rel = rel .. "/"
  end
  r = r .. xplr.util.paint(xplr.util.shell_escape(rel), style)

  r = r .. m.suffix .. " "

  if m.is_symlink then
    r = r .. "-> "

    if m.is_broken then
      r = r .. "×"
    else
      local symlink_path =
        xplr.util.shorten(m.symlink.absolute_path, { base = m.parent })
      if m.symlink.is_dir then
        symlink_path = symlink_path .. "/"
      end
      r = r .. symlink_path:gsub("\n", nl)
    end
  end

  return r
end

-- Renders the third column in the table
xplr.fn.builtin.fmt_general_table_row_cols_2 = function(m)
  local r = xplr.util.paint("r", { fg = "Green" })
  local w = xplr.util.paint("w", { fg = "Yellow" })
  local x = xplr.util.paint("x", { fg = "Red" })
  local s = xplr.util.paint("s", { fg = "Red" })
  local S = xplr.util.paint("S", { fg = "Red" })
  local t = xplr.util.paint("t", { fg = "Red" })
  local T = xplr.util.paint("T", { fg = "Red" })

  return xplr.util
    .permissions_rwx(m.permissions)
    :gsub("r", r)
    :gsub("w", w)
    :gsub("x", x)
    :gsub("s", s)
    :gsub("S", S)
    :gsub("t", t)
    :gsub("T", T)
end

-- Renders the fourth column in the table
xplr.fn.builtin.fmt_general_table_row_cols_3 = function(m)
  if not m.is_dir then
    return m.human_size
  else
    return ""
  end
end

-- Renders the fifth column in the table
xplr.fn.builtin.fmt_general_table_row_cols_4 = function(m)
  return tostring(os.date("%a %b %d %H:%M:%S %Y", m.last_modified / 1000000000))
end

-- DEPRECATED: This function is just for compatibility.
-- Use message `TryCompletePath` instead.
xplr.fn.builtin.try_complete_path = function(m)
  return {
    {
      LogWarning = "xplr.fn.builtin.try_complete_path is deprecated. Use message `TryCompletePath` instead.",
    },
    "TryCompletePath",
  }
end

-- This is where the custom functions can be added.
--
-- There is currently no restriction on what kind of functions can be defined
-- in `xplr.fn.custom`.
--
-- You can also use nested tables such as
-- `xplr.fn.custom.my_plugin.my_function` to define custom functions.
xplr.fn.custom = {}

-- ## Hooks -------------------------------------------------------------------
--
-- This section of the configuration cannot be overwritten by another config
-- file or plugin, since this is an optional lua return statement specific to
-- each config file. It can be used to define things that should be explicit
-- for reasons like performance concerns, such as hooks.
--
-- Plugins should expose the hooks, and require users to subscribe to them
-- explicitly.
--
-- Example:
--
-- ```lua
-- return {
--   -- Add messages to send when the xplr loads.
--   -- This is similar to the `--on-load` command-line option.
--   --
--   -- Type: list of [Message](https://xplr.dev/en/message#message)s
--   on_load = {
--     { LogSuccess = "Configuration successfully loaded!" },
--     { CallLuaSilently = "custom.some_plugin_with_hooks.on_load" },
--   },
--
--   -- Add messages to send when the directory changes.
--   --
--   -- Type: list of [Message](https://xplr.dev/en/message#message)s
--   on_directory_change = {
--     { LogSuccess = "Changed directory" },
--     { CallLuaSilently = "custom.some_plugin_with_hooks.on_directory_change" },
--   },
--
--   -- Add messages to send when the focus changes.
--   --
--   -- Type: list of [Message](https://xplr.dev/en/message#message)s
--   on_focus_change = {
--     { LogSuccess = "Changed focus" },
--     { CallLuaSilently = "custom.some_plugin_with_hooks.on_focus_change" },
--   }
--
--   -- Add messages to send when the mode is switched.
--   --
--   -- Type: list of [Message](https://xplr.dev/en/message#message)s
--   on_mode_switch = {
--     { LogSuccess = "Switched mode" },
--     { CallLuaSilently = "custom.some_plugin_with_hooks.on_mode_switch" },
--   }
--
--   -- Add messages to send when the layout is switched
--   --
--   -- Type: list of [Message](https://xplr.dev/en/message#message)s
--   on_layout_switch = {
--     { LogSuccess = "Switched layout" },
--     { CallLuaSilently = "custom.some_plugin_with_hooks.on_layout_switch" },
--   }
--
--   -- Add messages to send when the selection changes
--   --
--   -- Type: list of [Message](https://xplr.dev/en/message#message)s
--   on_selection_change = {
--     { LogSuccess = "Selection changed" },
--     { CallLuaSilently = "custom.some_plugin_with_hooks.on_selection_change" },
--   }
-- }
-- ```

return {
  on_load = {},
  on_directory_change = {},
  on_focus_change = {},
  on_mode_switch = {},
  on_layout_switch = {},
}

-- ----------------------------------------------------------------------------
-- > Note:
-- >
-- > It's not recommended to copy the entire configuration, unless you want to
-- > freeze it and miss out on useful updates to the defaults.
-- >
-- > Instead, you can use this as a reference to overwrite only the parts you
-- > want to update.
-- >
-- > If you still want to copy the entire configuration, make sure to put your
-- > customization before the return statement.
