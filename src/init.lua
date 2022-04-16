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

-- The style for the informations logs.
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
  { format = "╭──── path", style = {} },
  { format = "permissions", style = {} },
  { format = "size", style = {} },
  { format = "type", style = {} },
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
  { format = "├─", style = {} },
  { format = "├─", style = {} },
  { format = "╰─", style = {} },
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
  fg = "LightGreen",
  add_modifiers = { "Bold" },
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
  fg = "LightGreen",
  add_modifiers = { "Bold" },
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
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.forward.format =
  "↓"

-- Style of forward direction indicator in Sort & filter panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.forward.style =
  nil

-- The shape of the reverse direction indicator for sort identifiers in Sort & filter panel.
--
-- Type: nullable string
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.reverse.format =
  "↑"

-- Style of reverse direction indicator in Sort & filter panel.
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.sort_direction_identifiers.reverse.style =
  nil

-- The identifiers used to denote applied sorters in the Sort & filter panel.
--
-- Type: nullable mapping of the following key-value pairs:
--
-- * key: [Sorter](https://xplr.dev/en/sorting#sorter)
-- * value:
--   * format: nullable string
--   * style: [Style](https://xplr.dev/en/style)
xplr.config.general.sort_and_filter_ui.sorter_identifiers = {
  ByCanonicalAbsolutePath = { format = "[c]abs", style = {} },
  ByCanonicalExtension = { format = "[c]ext", style = {} },
  ByCanonicalIsDir = { format = "[c]dir", style = {} },
  ByCanonicalIsFile = { format = "[c]file", style = {} },
  ByCanonicalIsReadonly = { format = "[c]ro", style = {} },
  ByCanonicalMimeEssence = { format = "[c]mime", style = {} },
  ByCanonicalSize = { format = "[c]size", style = {} },
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
  BySymlinkAbsolutePath = { format = "[s]abs", style = {} },
  BySymlinkExtension = { format = "[s]ext", style = {} },
  BySymlinkIsDir = { format = "[s]dir", style = {} },
  BySymlinkIsFile = { format = "[s]file", style = {} },
  BySymlinkIsReadonly = { format = "[s]ro", style = {} },
  BySymlinkMimeEssence = { format = "[s]mime", style = {} },
  BySymlinkSize = { format = "[s]size", style = {} },
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
  AbsolutePathDoesContain = { format = "abs=~", style = {} },
  AbsolutePathDoesEndWith = { format = "abs=$", style = {} },
  AbsolutePathDoesNotContain = { format = "abs!~", style = {} },
  AbsolutePathDoesNotEndWith = { format = "abs!$", style = {} },
  AbsolutePathDoesNotStartWith = { format = "abs!^", style = {} },
  AbsolutePathDoesStartWith = { format = "abs=^", style = {} },
  AbsolutePathIs = { format = "abs==", style = {} },
  AbsolutePathIsNot = { format = "abs!=", style = {} },
  IAbsolutePathDoesContain = { format = "[i]abs=~", style = {} },
  IAbsolutePathDoesEndWith = { format = "[i]abs=$", style = {} },
  IAbsolutePathDoesNotContain = { format = "[i]abs!~", style = {} },
  IAbsolutePathDoesNotEndWith = { format = "[i]abs!$", style = {} },
  IAbsolutePathDoesNotStartWith = { format = "[i]abs!^", style = {} },
  IAbsolutePathDoesStartWith = { format = "[i]abs=^", style = {} },
  IAbsolutePathIs = { format = "[i]abs==", style = {} },
  IAbsolutePathIsNot = { format = "[i]abs!=", style = {} },
  IRelativePathDoesContain = { format = "[i]rel=~", style = {} },
  IRelativePathDoesEndWith = { format = "[i]rel=$", style = {} },
  IRelativePathDoesNotContain = { format = "[i]rel!~", style = {} },
  IRelativePathDoesNotEndWith = { format = "[i]rel!$", style = {} },
  IRelativePathDoesNotStartWith = { format = "[i]rel!^", style = {} },
  IRelativePathDoesStartWith = { format = "[i]rel=^", style = {} },
  IRelativePathIs = { format = "[i]rel==", style = {} },
  IRelativePathIsNot = { format = "[i]rel!=", style = {} },
  RelativePathDoesContain = { format = "rel=~", style = {} },
  RelativePathDoesEndWith = { format = "rel=$", style = {} },
  RelativePathDoesNotContain = { format = "rel!~", style = {} },
  RelativePathDoesNotEndWith = { format = "rel!$", style = {} },
  RelativePathDoesNotStartWith = { format = "rel!^", style = {} },
  RelativePathDoesStartWith = { format = "rel=^", style = {} },
  RelativePathIs = { format = "rel==", style = {} },
  RelativePathIsNot = { format = "rel!=", style = {} },
}

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
--
-- Type: nullable string
xplr.config.general.initial_mode = "default"

-- The name of one of the layouts to use when xplr loads.
--
-- Type: nullable string
xplr.config.general.initial_layout = "default"

-- Set it to a file path to start fifo when xplr loads.
-- Generally it is used to integrate with external tools like previewers.
--
-- Type: nullable string
xplr.config.general.start_fifo = nil

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
xplr.config.node_types.directory.style = {
  fg = "Cyan",
  add_modifiers = { "Bold" },
}

-- Metadata for the directory nodes
--
-- Type: nullable string
xplr.config.node_types.directory.meta.icon = "ð"

-- The style for the file nodes
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.node_types.file.style = {}

-- Metadata for the file nodes
--
-- Type: nullable string
xplr.config.node_types.file.meta.icon = "ƒ"

-- The style for the symlink nodes
--
-- Type: [Style](https://xplr.dev/en/style)
xplr.config.node_types.symlink.style = {
  fg = "Magenta",
  add_modifiers = { "Italic" },
}

-- Metadata for the symlink nodes
--
-- Type: nullable string
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
--     ["*"] = { meta = { icon = "a" } }
--
--     -- application/pdf
--     pdf = { meta = { icon = "" }, style = { fg = "Blue" } },

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
-- ![demo](https://s6.gifyu.com/images/layout.png)
--
-- ```lua
-- xplr.config.layouts.builtin.default = {
--   Horizontal = {
--     config = {
--       margin = 1,
--       horizontal_margin = 2,
--       vertical_margin = 3,
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
              { Percentage = 50 },
              { Percentage = 50 },
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
            ToggleNodeFilter = {
              filter = "RelativePathDoesNotStartWith",
              input = ".",
            },
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
      ["?"] = {
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
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      ["ctrl-f"] = {
        help = "search",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "search" },
          { SetInputBuffer = "" },
          "ExplorePwdAsync",
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
      down = {
        help = "down",
        messages = {
          "FocusNext",
        },
      },
      enter = {
        help = "quit with result",
        messages = {
          "PrintResultAndQuit",
        },
      },
      esc = {
        help = nil,
        messages = {},
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
      left = {
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
            BashExecSilently = [===[
              echo SetInputBuffer: "'"$(basename "${XPLR_FOCUS_PATH}")"'" >> "${XPLR_PIPE_MSG_IN:?}"
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
            BashExecSilently = [===[
              echo SetInputBuffer: "'"$(basename "${XPLR_FOCUS_PATH}")"'" >> "${XPLR_PIPE_MSG_IN:?}"
            ]===],
          },
        },
      },
      right = {
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
      space = {
        help = "toggle selection",
        messages = {
          "ToggleSelection",
          "FocusNext",
        },
      },
      up = {
        help = "up",
        messages = {
          "FocusPrevious",
        },
      },
      ["~"] = {
        help = "go home",
        messages = {
          {
            BashExecSilently = [===[
              echo ChangeDirectory: "'"${HOME:?}"'" >> "${XPLR_PIPE_MSG_IN:?}"
            ]===],
          },
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

xplr.config.modes.builtin.default.key_bindings.on_key["tab"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["ctrl-i"]

xplr.config.modes.builtin.default.key_bindings.on_key["v"] =
  xplr.config.modes.builtin.default.key_bindings.on_key.space

xplr.config.modes.builtin.default.key_bindings.on_key["V"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["ctrl-a"]

xplr.config.modes.builtin.default.key_bindings.on_key["/"] =
  xplr.config.modes.builtin.default.key_bindings.on_key["ctrl-f"]

xplr.config.modes.builtin.default.key_bindings.on_key["h"] =
  xplr.config.modes.builtin.default.key_bindings.on_key.left

xplr.config.modes.builtin.default.key_bindings.on_key["j"] =
  xplr.config.modes.builtin.default.key_bindings.on_key.down

xplr.config.modes.builtin.default.key_bindings.on_key["k"] =
  xplr.config.modes.builtin.default.key_bindings.on_key.up

xplr.config.modes.builtin.default.key_bindings.on_key["l"] =
  xplr.config.modes.builtin.default.key_bindings.on_key.right

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
          CustomContent = {
            title = "debug error",
            body = {
              StaticParagraph = {
                render = [[

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
        },
        "InputAndLogs",
      },
    },
  },
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      enter = {
        help = "open logs in editor",
        messages = {
          {
            BashExec = [===[
              ${EDITOR:-vi} "${XPLR_PIPE_LOGS_OUT:?}"
            ]===],
          },
        },
      },
      esc = {
        help = "escape",
        messages = {
          "PopMode",
        },
      },
      q = {
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
    CustomContent = {
      title = " recover ",
      body = {
        StaticParagraph = {
          render = [[

  You pressed an invalid key and went into "recover" mode.
  This mode saves you from performing unwanted actions.

  Let's calm down, press `escape`, and try again.

  To disable this mode, set `xplr.config.general.enable_recover_mode`
  to `false` in your config file.
          ]],
        },
      },
    },
  },
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      esc = {
        help = "escape",
        messages = {
          "PopMode",
        },
      },
    },
    default = {
      messages = {},
    },
  },
}

-- The builtin selection ops mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.selection_ops = {
  name = "selection ops",
  key_bindings = {
    on_key = {
      ["c"] = {
        help = "copy here",
        messages = {
          {
            BashExec = [===[
              (while IFS= read -r line; do
              if cp -vr -- "${line:?}" ./; then
                echo LogSuccess: $line copied to $PWD >> "${XPLR_PIPE_MSG_IN:?}"
              else
                echo LogError: Failed to copy $line to $PWD >> "${XPLR_PIPE_MSG_IN:?}"
              fi
              done < "${XPLR_PIPE_SELECTION_OUT:?}")
              echo ExplorePwdAsync >> "${XPLR_PIPE_MSG_IN:?}"
              echo ClearSelection >> "${XPLR_PIPE_MSG_IN:?}"
              read -p "[enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      esc = {
        help = "cancel",
        messages = {
          "PopMode",
        },
      },
      ["m"] = {
        help = "move here",
        messages = {
          {
            BashExec = [===[
              (while IFS= read -r line; do
              if mv -v -- "${line:?}" ./; then
                echo LogSuccess: $line moved to $PWD >> "${XPLR_PIPE_MSG_IN:?}"
              else
                echo LogError: Failed to move $line to $PWD >> "${XPLR_PIPE_MSG_IN:?}"
              fi
              done < "${XPLR_PIPE_SELECTION_OUT:?}")
              echo ExplorePwdAsync >> "${XPLR_PIPE_MSG_IN:?}"
              read -p "[enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["x"] = {
        help = "open in gui",
        messages = {
          {
            BashExecSilently = [===[
              if [ -z "$OPENER" ]; then
                if command -v xdg-open; then
                  OPENER=xdg-open
                  elif command -v open; then
                  OPENER=open
                else
                  echo 'LogError: $OPENER not found' >> "${XPLR_PIPE_MSG_IN:?}"
                  exit 1
                fi
              fi
              (while IFS= read -r line; do
              $OPENER "${line:?}" > /dev/null 2>&1
              done < "${XPLR_PIPE_RESULT_OUT:?}")
            ]===],
          },
          "ClearScreen",
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
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      ["d"] = {
        help = "create directory",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "create directory" },
          { SetInputBuffer = "" },
        },
      },
      esc = {
        help = "cancel",
        messages = {
          "PopMode",
        },
      },
      ["f"] = {
        help = "create file",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "create file" },
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
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      enter = {
        help = "create directory",
        messages = {
          {
            BashExecSilently = [===[
              PTH="$XPLR_INPUT_BUFFER"
              if [ "${PTH}" ]; then
                mkdir -p -- "${PTH:?}" \
                && echo "SetInputBuffer: ''" >> "${XPLR_PIPE_MSG_IN:?}" \
                && echo ExplorePwd >> "${XPLR_PIPE_MSG_IN:?}" \
                && echo LogSuccess: $PTH created >> "${XPLR_PIPE_MSG_IN:?}" \
                && echo FocusPath: "'"$PTH"'" >> "${XPLR_PIPE_MSG_IN:?}"
              else
                echo PopMode >> "${XPLR_PIPE_MSG_IN:?}"
              fi
            ]===],
          },
        },
      },
      esc = {
        help = "cancel",
        messages = {
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

-- The builtin create file mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.create_file = {
  name = "create file",
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = { "Terminate" },
      },
      enter = {
        help = "create file",
        messages = {
          {
            BashExecSilently = [===[
              PTH="$XPLR_INPUT_BUFFER"
              if [ "$PTH" ]; then
                mkdir -p -- "$(dirname $PTH)" \
                && touch -- "$PTH" \
                && echo "SetInputBuffer: ''" >> "${XPLR_PIPE_MSG_IN:?}" \
                && echo LogSuccess: $PTH created >> "${XPLR_PIPE_MSG_IN:?}" \
                && echo ExplorePwd >> "${XPLR_PIPE_MSG_IN:?}" \
                && echo FocusPath: "'"$PTH"'" >> "${XPLR_PIPE_MSG_IN:?}"
              else
                echo PopMode >> "${XPLR_PIPE_MSG_IN:?}"
              fi
            ]===],
          },
        },
      },
      esc = {
        help = "cancel",
        messages = {
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

-- The builtin number mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.number = {
  name = "number",
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      down = {
        help = "to down",
        messages = {
          "FocusNextByRelativeIndexFromInput",
          "PopMode",
        },
      },
      enter = {
        help = "to index",
        messages = {
          "FocusByIndexFromInput",
          "PopMode",
        },
      },
      esc = {
        help = "cancel",
        messages = {
          "PopMode",
        },
      },
      up = {
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
  xplr.config.modes.builtin.number.key_bindings.on_key.down
xplr.config.modes.builtin.number.key_bindings.on_key["k"] =
  xplr.config.modes.builtin.number.key_bindings.on_key.up

-- The builtin go to mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.go_to = {
  name = "go to",
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      esc = {
        help = "cancel",
        messages = {
          "PopMode",
        },
      },
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
      ["x"] = {
        help = "open in gui",
        messages = {
          {
            BashExecSilently = [===[
              if [ -z "$OPENER" ]; then
                if command -v xdg-open; then
                  OPENER=xdg-open
                  elif command -v open; then
                  OPENER=open
                else
                  echo 'LogError: $OPENER not found' >> "${XPLR_PIPE_MSG_IN:?}"
                  exit 1
                fi
              fi
              $OPENER "${XPLR_FOCUS_PATH:?}" > /dev/null 2>&1
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
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      enter = {
        help = "rename",
        messages = {
          {
            BashExecSilently = [===[
              SRC="${XPLR_FOCUS_PATH:?}"
              TARGET="${XPLR_INPUT_BUFFER:?}"
              if [ -e "${TARGET:?}" ]; then
                echo LogError: $TARGET already exists >> "${XPLR_PIPE_MSG_IN:?}"
              else
                mv -- "${SRC:?}" "${TARGET:?}" \
                  && echo ExplorePwd >> "${XPLR_PIPE_MSG_IN:?}" \
                  && echo FocusPath: "'"$TARGET"'" >> "${XPLR_PIPE_MSG_IN:?}" \
                  && echo LogSuccess: $SRC renamed to $TARGET >> "${XPLR_PIPE_MSG_IN:?}"
              fi
            ]===],
          },
          "PopMode",
        },
      },
      esc = {
        help = "cancel",
        messages = {
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
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      enter = {
        help = "duplicate",
        messages = {
          {
            BashExecSilently = [===[
              SRC="${XPLR_FOCUS_PATH:?}"
              TARGET="${XPLR_INPUT_BUFFER:?}"
              if [ -e "${TARGET:?}" ]; then
                echo LogError: $TARGET already exists >> "${XPLR_PIPE_MSG_IN:?}"
              else
                cp -r -- "${SRC:?}" "${TARGET:?}" \
                  && echo ExplorePwd >> "${XPLR_PIPE_MSG_IN:?}" \
                  && echo FocusPath: "'"$TARGET"'" >> "${XPLR_PIPE_MSG_IN:?}" \
                  && echo LogSuccess: $SRC duplicated as $TARGET >> "${XPLR_PIPE_MSG_IN:?}"
              fi
            ]===],
          },
          "PopMode",
        },
      },
      esc = {
        help = "cancel",
        messages = {
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
  key_bindings = {
    on_key = {
      ["D"] = {
        help = "force delete",
        messages = {
          {
            BashExec = [===[
              (while IFS= read -r line; do
              if rm -rfv -- "${line:?}"; then
                echo LogSuccess: $line deleted >> "${XPLR_PIPE_MSG_IN:?}"
              else
                echo LogError: Failed to delete $line >> "${XPLR_PIPE_MSG_IN:?}"
              fi
              done < "${XPLR_PIPE_RESULT_OUT:?}")
              echo ExplorePwdAsync >> "${XPLR_PIPE_MSG_IN:?}"
              read -p "[enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      ["d"] = {
        help = "delete",
        messages = {
          {
            BashExec = [===[
              (while IFS= read -r line; do
              if [ -d "$line" ] && [ ! -L "$line" ]; then
                if rmdir -v -- "${line:?}"; then
                  echo LogSuccess: $line deleted >> "${XPLR_PIPE_MSG_IN:?}"
                else
                  echo LogError: Failed to delete $line >> "${XPLR_PIPE_MSG_IN:?}"
                fi
              else
                if rm -v -- "${line:?}"; then
                  echo LogSuccess: $line deleted >> "${XPLR_PIPE_MSG_IN:?}"
                else
                  echo LogError: Failed to delete $line >> "${XPLR_PIPE_MSG_IN:?}"
                fi
              fi
              done < "${XPLR_PIPE_RESULT_OUT:?}")
              echo ExplorePwdAsync >> "${XPLR_PIPE_MSG_IN:?}"
              read -p "[enter to continue]"
            ]===],
          },
          "PopMode",
        },
      },
      esc = {
        help = "cancel",
        messages = {
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
  key_bindings = {
    on_key = {
      ["!"] = {
        help = "shell",
        messages = {
          { Call = { command = "bash", args = { "-i" } } },
          "ExplorePwdAsync",
          "PopMode",
        },
      },
      ["c"] = {
        help = "create",
        messages = {
          "PopMode",
          { SwitchModeBuiltin = "create" },
        },
      },
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      ["e"] = {
        help = "open in editor",
        messages = {
          {
            BashExec = [===[
              ${EDITOR:-vi} "${XPLR_FOCUS_PATH:?}"
            ]===],
          },
          "PopMode",
        },
      },
      esc = {
        help = "cancel",
        messages = {
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
  key_bindings = {
    on_key = {
      enter = {
        help = "just quit",
        messages = {
          "Quit",
        },
      },
      p = {
        help = "quit printing pwd",
        messages = {
          "PrintPwdAndQuit",
        },
      },
      f = {
        help = "quit printing focus",
        messages = {
          "PrintFocusPathAndQuit",
        },
      },
      s = {
        help = "quit printing selection",
        messages = {
          "PrintSelectionAndQuit",
        },
      },
      r = {
        help = "quit printing result",
        messages = {
          "PrintResultAndQuit",
        },
      },
      esc = {
        help = "cancel",
        messages = {
          "PopMode",
        },
      },
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
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
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      down = {
        help = "down",
        messages = {
          "FocusNext",
        },
      },
      enter = {
        help = "focus",
        messages = {
          { RemoveNodeFilterFromInput = "IRelativePathDoesContain" },
          "PopMode",
          "ExplorePwdAsync",
        },
      },
      right = {
        help = "enter",
        messages = {
          { RemoveNodeFilterFromInput = "IRelativePathDoesContain" },
          "Enter",
          { SetInputBuffer = "" },
          "ExplorePwdAsync",
        },
      },
      left = {
        help = "back",
        messages = {
          { RemoveNodeFilterFromInput = "IRelativePathDoesContain" },
          "Back",
          { SetInputBuffer = "" },
          "ExplorePwdAsync",
        },
      },
      tab = {
        help = "toggle selection",
        messages = {
          "ToggleSelection",
          "FocusNext",
        },
      },
      up = {
        help = "up",
        messages = {
          "FocusPrevious",
        },
      },
    },
    default = {
      messages = {
        { RemoveNodeFilterFromInput = "IRelativePathDoesContain" },
        "UpdateInputBufferFromKey",
        { AddNodeFilterFromInput = "IRelativePathDoesContain" },
        "ExplorePwdAsync",
      },
    },
  },
}

xplr.config.modes.builtin.search.key_bindings.on_key["esc"] =
  xplr.config.modes.builtin.search.key_bindings.on_key.enter
xplr.config.modes.builtin.search.key_bindings.on_key["ctrl-n"] =
  xplr.config.modes.builtin.search.key_bindings.on_key.down
xplr.config.modes.builtin.search.key_bindings.on_key["ctrl-p"] =
  xplr.config.modes.builtin.search.key_bindings.on_key.up

-- The builtin filter mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.filter = {
  name = "filter",
  key_bindings = {
    on_key = {
      ["R"] = {
        help = "relative does not contain",
        messages = {
          { SwitchModeBuiltin = "relative_path_does_not_contain" },
          { SetInputBuffer = "" },
          { AddNodeFilterFromInput = "IRelativePathDoesNotContain" },
          "ExplorePwdAsync",
        },
      },
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
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
      enter = {
        help = "done",
        messages = {
          "PopMode",
        },
      },
      ["r"] = {
        help = "relative does contain",
        messages = {
          { SwitchModeBuiltin = "relative_path_does_contain" },
          { SetInputBuffer = "" },
          { AddNodeFilterFromInput = "IRelativePathDoesContain" },
          "ExplorePwdAsync",
        },
      },
    },
  },
}

xplr.config.modes.builtin.filter.key_bindings.on_key["esc"] =
  xplr.config.modes.builtin.filter.key_bindings.on_key.enter

-- The builtin relative_path_does_contain mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.relative_path_does_contain = {
  name = "relative path does contain",
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      enter = {
        help = "apply filter",
        messages = {
          "PopMode",
        },
      },
      esc = {
        help = "cancel",
        messages = {
          { RemoveNodeFilterFromInput = "IRelativePathDoesContain" },
          "PopMode",
          "ExplorePwdAsync",
        },
      },
    },
    default = {
      messages = {
        { RemoveNodeFilterFromInput = "IRelativePathDoesContain" },
        "UpdateInputBufferFromKey",
        { AddNodeFilterFromInput = "IRelativePathDoesContain" },
        "ExplorePwdAsync",
      },
    },
  },
}

-- The builtin relative_path_does_not_contain mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.relative_path_does_not_contain = {
  name = "relative path does not contain",
  key_bindings = {
    on_key = {
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      enter = {
        help = "apply filter",
        messages = {
          "PopMode",
        },
      },
      esc = {
        help = "cancel",
        messages = {
          { RemoveNodeFilterFromInput = "IRelativePathDoesNotContain" },
          "PopMode",
          "ExplorePwdAsync",
        },
      },
    },
    default = {
      messages = {
        { RemoveNodeFilterFromInput = "IRelativePathDoesNotContain" },
        "UpdateInputBufferFromKey",
        { AddNodeFilterFromInput = "IRelativePathDoesNotContain" },
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
          {
            AddNodeSorter = {
              sorter = "ByCanonicalExtension",
              reverse = true,
            },
          },
          "ExplorePwdAsync",
        },
      },
      ["M"] = {
        help = "by canonical mime essence reverse",
        messages = {
          {
            AddNodeSorter = {
              sorter = "ByCanonicalMimeEssence",
              reverse = true,
            },
          },
          "ExplorePwdAsync",
        },
      },
      ["N"] = {
        help = "by node type reverse",
        messages = {
          {
            AddNodeSorter = {
              sorter = "ByCanonicalIsDir",
              reverse = true,
            },
          },
          {
            AddNodeSorter = {
              sorter = "ByCanonicalIsFile",
              reverse = true,
            },
          },
          {
            AddNodeSorter = {
              sorter = "ByIsSymlink",
              reverse = true,
            },
          },
          "ExplorePwdAsync",
        },
      },
      ["R"] = {
        help = "by relative path reverse",
        messages = {
          {
            AddNodeSorter = {
              sorter = "ByIRelativePath",
              reverse = true,
            },
          },
          "ExplorePwdAsync",
        },
      },
      ["S"] = {
        help = "by size reverse",
        messages = {
          {
            AddNodeSorter = {
              sorter = "BySize",
              reverse = true,
            },
          },
          "ExplorePwdAsync",
        },
      },
      backspace = {
        help = "remove last sorter",
        messages = {
          "RemoveLastNodeSorter",
          "ExplorePwdAsync",
        },
      },
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
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
          {
            AddNodeSorter = {
              sorter = "ByCanonicalExtension",
              reverse = false,
            },
          },
          "ExplorePwdAsync",
        },
      },
      enter = {
        help = "done",
        messages = {
          "PopMode",
        },
      },
      ["m"] = {
        help = "by canonical mime essence",
        messages = {
          {
            AddNodeSorter = {
              sorter = "ByCanonicalMimeEssence",
              reverse = false,
            },
          },
          "ExplorePwdAsync",
        },
      },
      ["n"] = {
        help = "by node type",
        messages = {
          {
            AddNodeSorter = { sorter = "ByCanonicalIsDir", reverse = false },
          },
          {
            AddNodeSorter = { sorter = "ByCanonicalIsFile", reverse = false },
          },
          {
            AddNodeSorter = { sorter = "ByIsSymlink", reverse = false },
          },
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
    },
  },
}

xplr.config.modes.builtin.sort.key_bindings.on_key["esc"] =
  xplr.config.modes.builtin.sort.key_bindings.on_key.enter

-- The builtin switch layout mode.
--
-- Type: [Mode](https://xplr.dev/en/mode)
xplr.config.modes.builtin.switch_layout = {
  name = "switch layout",
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
      ["ctrl-c"] = {
        help = "terminate",
        messages = {
          "Terminate",
        },
      },
      esc = {
        help = "cancel",
        messages = {
          "PopMode",
        },
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
  local r = m.tree .. m.prefix

  if m.meta.icon == nil then
    r = r .. ""
  else
    r = r .. m.meta.icon .. " "
  end

  r = r .. m.relative_path

  if m.is_dir then
    r = r .. "/"
  end

  r = r .. m.suffix .. " "

  if m.is_symlink then
    r = r .. "-> "

    if m.is_broken then
      r = r .. "×"
    else
      r = r .. m.symlink.absolute_path

      if m.symlink.is_dir then
        r = r .. "/"
      end
    end
  end

  return r
end

-- Renders the third column in the table
xplr.fn.builtin.fmt_general_table_row_cols_2 = function(m)
  local no_color = os.getenv("NO_COLOR")

  local function green(x)
    if no_color == nil then
      return "\x1b[32m" .. x .. "\x1b[0m"
    else
      return x
    end
  end

  local function yellow(x)
    if no_color == nil then
      return "\x1b[33m" .. x .. "\x1b[0m"
    else
      return x
    end
  end

  local function red(x)
    if no_color == nil then
      return "\x1b[31m" .. x .. "\x1b[0m"
    else
      return x
    end
  end

  local function bit(x, color, cond)
    if cond then
      return color(x)
    else
      return color("-")
    end
  end

  local p = m.permissions

  local r = ""

  r = r .. bit("r", green, p.user_read)
  r = r .. bit("w", yellow, p.user_write)

  if p.user_execute == false and p.setuid == false then
    r = r .. bit("-", red, p.user_execute)
  elseif p.user_execute == true and p.setuid == false then
    r = r .. bit("x", red, p.user_execute)
  elseif p.user_execute == false and p.setuid == true then
    r = r .. bit("S", red, p.user_execute)
  else
    r = r .. bit("s", red, p.user_execute)
  end

  r = r .. bit("r", green, p.group_read)
  r = r .. bit("w", yellow, p.group_write)

  if p.group_execute == false and p.setuid == false then
    r = r .. bit("-", red, p.group_execute)
  elseif p.group_execute == true and p.setuid == false then
    r = r .. bit("x", red, p.group_execute)
  elseif p.group_execute == false and p.setuid == true then
    r = r .. bit("S", red, p.group_execute)
  else
    r = r .. bit("s", red, p.group_execute)
  end

  r = r .. bit("r", green, p.other_read)
  r = r .. bit("w", yellow, p.other_write)

  if p.other_execute == false and p.setuid == false then
    r = r .. bit("-", red, p.other_execute)
  elseif p.other_execute == true and p.setuid == false then
    r = r .. bit("x", red, p.other_execute)
  elseif p.other_execute == false and p.setuid == true then
    r = r .. bit("T", red, p.other_execute)
  else
    r = r .. bit("t", red, p.other_execute)
  end

  return r
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
  if m.is_symlink and not m.is_broken then
    return m.symlink.mime_essence
  else
    return m.mime_essence
  end
end

-- This is where the custom functions can be added.
--
-- There is currently no restriction on what kind of functions can be defined
-- in `xplr.fn.custom`.
--
-- You can also use nested tables such as
-- `xplr.fn.custom.my_plugin.my_function` to define custom functions.
xplr.fn.custom = {}
