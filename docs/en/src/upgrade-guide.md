# Upgrade Guide

When you upgrade xplr, you might see an error like this

```
Incompatible script version in: /home/user/.config/xplr/init.lua. The script version is: 0.21.0, the required version is: 1.0.0. Visit https://xplr.dev/en/upgrade-guide
```

All you need to do is follow the [instructions][1] starting from
your config version, all the way to the required version.

<details>
<summary>Expand for more information</summary>

With every update, we either implement a `major` breaking change (e.g.
deprecating or replacing messages), or a `minor` feature addition (e.g. adding
new messages) or `patch`, fixes, and optimization (e.g. performance
optimization).

Knowing that we use the `{major}.{minor}.{patch}` versioning format,

- Major version mismatch are generally incompatible. xplr will fail
  with error.
- Minor version upgrades (not downgrades) and patch fixes are backwards
  compatible. You might get notified by log a message which you can disable by
  updating the version in your config file.
- However, if the config file has a higher value for the minor version
  than the app, then also xplr will fail with error, suggesting you
  to visit this page.
  Though in that case, you will be downgrading your config file based on your
  app version.

e.g.

- `1.0.0` -> `1.0.x`: Patch (fully compatible).
- `1.0.0` -> `1.x.x`: Only backwards compatible. You can't generally use for
  e.g. `app-1.0.0` with `config-1.1.0`. But vice versa is fine.
- `1.0.0` -> `x.x.x`: Not compatible at all.

</details>

### Instructions

#### [v0.21.10][49] -> [v0.1.1][51]

- This release added a new message `TryCompletePath` which will try to complete
  the path in the input buffer. You should use this message instead of the
  fragile `xplr.fn.builtin.try_complete_path` Lua function, which will still
  keep working (with a warning log message) for compatibility reasons.

**ANNOUNCEMENT:**

This is not really a breaking change, but a declaration of the first stable
release. You cat set `version = "1.0.0"` in your config file without any
concern. With this release, xplr v1 can be considered feature complete.

As of now, there is no plan for xplr v2, and so, the project will enter cleanup
and maintenance mode. Thank you everyone, who directly or indirectly contributed
to xplr, for all your effort, support and feedback. Looking forward to more
such collaborations in the future.

#### [v0.20.2][48] -> [v0.21.10][49]

- Some plugins might stop rendering colors. Wait for them to update.
- Rename `xplr.config.general.sort_and_filter_ui.search_identifier` to
  `xplr.config.general.sort_and_filter_ui.search_identifiers`.
- Resolved Node API will not contain the `permissions` field anymore.
  Use the utility function `xplr.util.node` to get its permissions.
- Layout `CustomContent` has been undocumented. It will stay for compatibility,
  but you should prefer using the following new layouts, because they support
  custom title:
  - Static
  - Dynamic
- Use the new messages for improved search operations:
  - Search
  - SearchFromInput
  - SearchFuzzyUnordered
  - SearchFuzzyUnorderedFromInput
  - SearchRegex
  - SearchRegexFromInput
  - SearchRegexUnordered
  - SearchRegexUnorderedFromInput
  - ToggleSearchAlgorithm
  - EnableSearchOrder
  - DisableSearchOrder
  - ToggleSearchOrder
- Use skim's [search syntax][50] to customize the search.
- Set your preferred search algorithm and ordering:
  `xplr.config.general.search.algorithm = "Fuzzy" -- or "Regex"`.
  `xplr.config.general.search.unordered = false -- or true`
- You need to clear the selection list manually after performing batch
  operation like copy, softlink creation etc.
- Use the following new key bindings:
  - `:sl` to list selection in a $PAGER.
  - `:ss` to create softlink of the selected items.
  - `:sh` to create hardlink of the selected items.
  - `:se` to edit selection list in your $EDITOR.
  - Better conflict handling: prompt for action.
- Navigate between the selected paths using the following messages:
  - FocusPreviousSelection (`ctrl-p`)
  - FocusNextSelection (`ctrl-n`)
- Use `LS_COLORS` environment variable, along with the following utility
- functions for applying better styling/theaming.
  - xplr.util.lscolor
  - xplr.util.paint
  - xplr.util.textwrap
  - xplr.util.style_mix
- Use new the fields in Column Renderer Argument:
  - style
  - permissions
- Use the following config to specify how the paths in selection list should be
  rendered:
  - xplr.config.general.selection.item.format
  - xplr.config.general.selection.item.style
- Use the following utility functions to work with the file permissions:
  - xplr.util.permissions_rwx
  - xplr.util.permissions_octal
- Type `:p` to edit file permissions interactively.
- Also check out the following utility functions:
  - xplr.util.layout_replace
  - xplr.util.relative_to
  - xplr.util.shorthand
  - xplr.util.clone
  - xplr.util.exists
  - xplr.util.is_dir
  - xplr.util.is_file
  - xplr.util.is_symlink
  - xplr.util.is_absolute
  - xplr.util.path_split
  - xplr.util.node
  - xplr.util.node_type
  - xplr.util.shell_escape
- Executables will me marked with the mime type: `application/x-executable`.
- macOS legacy coreutils will be generally supported, but please update it.
- Since v0.21.2 you can use the on_selection_change hook.
- Since v0.21.4 you can use function keys upto F24 and the following new
  messages:
  - NextVisitedDeepBranch (bound to `)` key)
  - PreviousVisitedDeepBranch (bound to `(` key)
- Since v0.21.6:
  - You can use `c` and `m` keys in default mode to quickly copy
    and move focused or selected files, without having to change directory.
  - Use `xplr.util.debug()` to debug lua values.
- Since v0.21.8:
  - Scroll behavior will default to vim-like continuous scrolling. You can set
    `xplr.config.general.paginated_scrolling = true` to revert back to the
    paginated scrolling.
  - Set `xplr.config.general.scroll_padding` to customize the scroll padding.
  - The calculated `scrolltop` value will be passed as part of the
    `Content Rendeder Argument` in `Dynamic` layout renderer functions.

Thanks to @noahmayr for contributing to a major part of this release.

#### [v0.19.4][47] -> [v0.20.2][48]

- BREAKING: xplr shell (`:!`) will default to null (`\0`) delimited pipes, as
  opposed to newline (`\n`) delimited ones (i.e. will use `Call0` instead of
  `Call`).
- Use new messages for safer file path handling (`\0` delimited):
  - Call0
  - CallSilently0
  - BashExec0
  - BashExecSilently0
- Use new sub-commands for safer message passing:
  - `-m FORMAT [ARGUMENT]...` / `--pipe-msg-in FORMAT [ARGUMENT]...`
  - `-M FORMAT [ARGUMENT]...` / `--print-msg-in FORMAT [ARGUMENT]...`
    Where FORMAT is a YAML string that may contain `%s`, `%q` and `%%`
    placeholders and ARGUMENT is the value per placeholder. See `init.lua`.
- Following hooks can be defined in the config files using an optional
  `return { on_* = { list, of, messages }, ... }` statement at the end.
  - on_load
  - on_focus_change
  - on_directory_change
  - on_mode_switch (since v0.20.2)
  - on_layout_switch (since v0.20.2)
- Use `--vroot` to isolate navigation of an xplr session inside a specific
  directory. Interaction still requires passing full path, and shell,
  lua functions etc still can access paths outside vroot.
- Use the following messages to switch vroot at runtime, or the use key
  bindings available in the new builtin mode "vroot" (mapped to `:` `v`).
  - SetVroot
  - UnsetVroot
  - ToggleVroot
  - ResetVroot
- Use `$XPLR_INITIAL_PWD` and Lua equivalent to implement workspace like
  features without using virtual root. Use keys `gi` to go to the initial
  working directory from anywhere.
- Use the convenient `xplr.util` utility functions in your Lua function calls.
  See xplr.util API docs.

#### [v0.18.0][46] -> [v0.19.4][47]

- BREAKING: The builtin modes cannot be accessed using space separated names
  anymore. Use underscore separated mode names. For e.g.
  `SwitchModeBuiltin: create file` becomes `SwitchModeBuiltin: create_file` and
  so on. Kindly go through your config, find and update them, or copy from the
  latest `init.lua`.
- Now you can use `xplr.config.general.global_key_bindings` to define a set of
  key bindings that are available by default in every mode. e.g. `esc`
  and `ctrl-c`, and remove boilerplate code from your config.
- You can use the new builtin mode `go_to_path` which can be used for typing or
  pasting paths to enter into or to focus on. Type `g` `p` to enter this mode.
- Now you can use basic tab completion in the `go_to_path`, `create_file`,
  `create_directory`, `rename` and `duplicate_as` modes.
- Use the builtin function `xplr.fn.builtin.try_complete_path` to add easy tab
  completion support into your own configuration.
- Now you can open OSC 7 compatible terminals into the `xplr`'s current working
  directory by spawning new terminal sessions via the terminal supported key
  bindings.
- Use `NO_COLOR` environment variable to disable OSC 7 compliance along with
  colors.
- If you have fully copied the default `init.lua` locally, you might want to
  go through the latest improvements in `init.lua`. Specifically the `search`,
  `filter` and `sort` modes. Also, search for `SetInputPrompt` and the `tab`
  key bindings.
- Since version 0.19.1, you can access uid and gid of the file owner in the Lua
  API.
- The input buffer will support more readline-like keys.
  Also, added "DeleteTillEnd" as another cursor based "InputOperation" option.
- Fixed applying regex based filters via the CLI and `$XPLR_PIPE_MSG_IN` pipe.
- You can use the `prompt` field to define input prompt for each mode, instead
  of using the `SetInputPrompt` message.
- Since version v0.19.4, the native search will default to skim-v2 based fuzzy
  matching. `esc` while in search mode will recover the initial focus. People
  who prefer the regex based search, can use the `regex-search.xplr` plugin.
  The following messages will be available for search based operations:
  - SearchFuzzy
  - SearchFuzzyFromInput
  - AcceptSearch
  - CancelSearch
- Since version v0.19.4, quick scrolling operations are supported using the
  following messages and keys:
  - ScrollUp -------- page-up
  - ScrollDown ------ page-down
  - ScrollUpHalf ---- {
  - ScrollDownHalf -- }

#### [v0.17.6][45] -> [v0.18.0][46]

- Key binding `f` `r` and `f` `R` will now filter using regex.
- Key binding `f` `backspace` will now remove the last filter.
- Search mode now defaults to regex search.
- `Node` metadata in the Lua API will contain two new fields:
  - `created`
  - `last_modified`
- The last column in the files table now displays the last modification time.
- You can now use `--read0`, `--write0` and `-0`/`--null` to read and/or print
  null character delimited paths.
- You can now the following regex filters:
  - `RelativePathDoesMatchRegex`
  - `RelativePathDoesNotMatchRegex`
  - `IRelativePathDoesMatchRegex`
  - `IRelativePathDoesNotMatchRegex`
  - `AbsolutePathDoesMatchRegex`
  - `AbsolutePathDoesNotMatchRegex`
  - `IAbsolutePathDoesMatchRegex`
  - `IAbsolutePathDoesNotMatchRegex`
- You can use a new `SetInputPrompt` to set the input prompt dynamically.
- You can now use the following timestamp based sorters:
  - "ByCreated"
  - "ByLastModified"
  - "ByCanonicalCreated"
  - "ByCanonicalLastModified"
  - "BySymlinkCreated"
  - "BySymlinkLastModified"

#### [v0.16.4][44] -> [v0.17.6][45]

- Deprecated `app.directory_buffer`, `app.history`, and `app.last_modes` in
  the custom dynamic layout renderer context.
  As of now, there's no way to access these fields in dynamic layouts. While
  `app.history` and `app.last_modes` can be re-added upon request
  (with justification), `app.directory_buffer` has been deprecated for good.
  However, there's no change in the `CallLua*` context.
- Set `xplr.config.general.hide_remaps_in_help_menu` to `true` to hide the
  remaps in help menu.
- `None` will be serialized to `nil` in Lua.
- `LuaEval` can now return a function that will be called with the Lua Context
  argument. Refer to the `Full List of Messages` doc for example.
- From version v0.17.1, set `xplr.config.general.disable_debug_error_mode` to
  `true` to disable switching to the "debug error" mode when startup errors
  occur.
- From version v0.17.2, you can use CLI argument `--print-pwd-as-result` for cd
  on quit, and key binding `ctrl-d` to duplicate a path in the same directory
  with a different name.
- Since version v0.17.3, you can use `border_type`, `border_style` to customize
  borders, and `enforce_bounded_index_navigation` to customize up/down
  navigation behavior when focus is on the top or bottom node.

#### [v0.15.2][43] -> [v0.16.4][44]

- Deprecated `config.general.cursor`. The default terminal cursor will be used
  for the time being.
- Opening xplr inside a symlink will not resolve the path.
- You can now replace most boilerplate configuration handling keys to send
  `BufferInputFromKey`, `RemoveInputBufferLastCharacter`,
  `RemoveInputBufferLastWord`, `SetInputBuffer = ""` etc. messages with a
  single `UpdateInputBufferFromKey` message.
- You can now pass multiple paths as command-line arguments or via stdin to
  select paths, e.g. `xplr -- $PWD /path/to/select/1 /path/to/select/2`.
- Pass `--force-focus` to focus on the first path even if it's a directory,
  e.g. `xplr . --force-focus`.
- Use new messages `LuaEval` and `LuaEvalSilently` to run Lua code without
  needing to define a function. However, the `app` context won't be available.
- You can now use new key handlers in the config:
  - on_alphanumeric
  - on_character
  - on_navigation
  - on_function

#### [v0.14.7][3] -> [v0.15.2][43]

- Deprecated `config` field from `CallLua` argument. Use the globally available
  `xplr.config` instead.
- `xplr.config.general.disable_recover_mode` has been deprecated. Use
  `xplr.config.general.enable_recover_mode` instead.
- Use `xplr.config.general.focus_selection_ui` to highlight selected files
  under focus differently than files under focus that are not selected.
- Use `PopModeKeepingInputBuffer`, and SwitchMode alternatives to switching to
  different modes without resetting the input buffer.
- Use the new `CustomContent` layout option to render custom content.
- Use the new `layout` field in a mode to define custom layout for a specific
  mode.
- Library users please refer to the latest API docs and examples.

#### [v0.13.7][2] -> [v0.14.7][3]

- macOS users need to place their config file (`init.lua`) in
  `$HOME/.config/xplr/` or `/etc/xplr/`.
- Library users please refer to the latest API docs.
- Check out the new messages: `{Start|Stop|Toggle}Fifo`. These enable support
  for [FIFO based file previews][4].
- You can disable the recover mode using `config.general.disable_recover_mode = true`.
- Try running `xplr --help`. Yes, CLI has been implemented.
- Since version `v0.14.3`, `StartFifo` and `ToggleFifo` will write to the FIFO
  path when called. So, there's no need to pipe the focus path explicitly.
- Since version `v0.14.3`, general config `xplr.config.start_fifo` is available
  which can be set to a file path to start a fifo when xplr starts.
- Since version `v0.14.4`, `$XPLR_SESSION_PATH` can be used to dump session
  related data.
- Since version `v0.14.6`, the `-C` or `--extra-config` CLI argument is
  available.

#### [v0.12.1][6] -> [v0.13.7][2]

- Lua functions called using [`CallLua`][7] and [`CallLuaSilently`][8] messages will receive [`CallLuaArg`][9] object as the function argument (instead of the [`App`][10] object).
- Each `node_types` config will inherit defaults from matching less specific `node_types` config and overwrite them.
- Since version `v0.13.2`, you don't need to use/send `Refresh` anymore. It will be auto-handled by xplr.

#### [v0.11.1][11] -> [v0.12.1][6]

- `xplr.config.node_types.mime_essence` has split into type and subtype. Hence, instead of `xplr.config.node_types.mime_essence["text/plain"] = ..` use `xplr.config.node_types.mime_essence["text"] = { plain = .. }`.
- You can also define `xplr.config.node_types.mime_essence["text"]["*"]` that will match all text types (`text/*`).

#### [v0.10.2][12] -> [v0.11.1][11]

- `remaps:` has been removed to avoid confusion. Use lua assignments instead.
  For e.g.
  ```
  xplr.config.modes.builtin.default.key_bindings.on_key["v"] = xplr.config.modes.builtin.default.key_bindings.on_key.space
  ```

#### [v0.9.1][13] -> [v0.10.2][12]

- [`config.yml`][14] has been fully replaced with [`init.lua`][15]. If you have a lot of customization in your `config.yml`, [xplr-yml2lua][16] can help you with migrating it to `init.lua`.
- `Handlebars` templates has been replaced with [Lua functions][17]. You can either remove the customizations or overwrite the functions accordingly.
- Added new messages `CallLua` and `CallLuaSilently` to call lua functions. The app state will be passed as input to the functions, and the returned messages will be handled by xplr. `CallLua` and `CallLuaSilently` are more flexible (and probably faster) alternatives to `Call`, `CallSilently`, `BashExec` and `BashExecSilently`. [e.g.][18]

#### [v0.9.0][19] -> [v0.9.1][13]

- You can now set `remaps: {key: null}` to un-map a key.
- `gx` will open the item under focus.
- New key map `:sx` will open the selected items.

#### [v0.8.0][20] -> [v0.9.0][19]

Your previous config should mostly work fine. However, in case you are using `SwitchMode` heavily in your custom config, follow along.

- Introduced new message `PopMode`. You might want to use this message instead of `SwitchMode*` when returning back to the previous mode.
- After using (the group of) `PopMode` and `SwitchMode*` messages, you are now required to `Refresh` manually to avoid the UI lag.
- Pressing any invalid key will now lead you to the `recover` mode and will protect you from typing further invalid keys. Press `esc` to escape the `recover` mode.
- Introduced new message `LogWarning`, similar to other `Log*` messages.
- Creating files and directories has been optimized for batch creation.

#### [v0.7.2][21] -> [v0.8.0][20]

If you have made changes to the config file,

- Replace message `Explore` with `ExplorePwd` or `ExplorePwdAsync` or probably `ExploreParentsAsync`.
- Pipe `$XPLR_PIPE_FOCUS_OUT` has been removed. Use `$XPLR_FOCUS_PATH` env var instead.
- You might want to review your path escaping logics. For e.g. use `echo FocusPath: "'"$PWD"'" >> $PIPE` instead of `echo "FocusPath: $PWD" >> $PIPE`.

#### [v0.7.0][22] -> [v0.7.2][21]

- Just update the `version` in your config file.
- For version >= `v0.7.1`, you might want to free up or remap the `tab` key in `search` mode to enable easy selection during search.

#### [v0.6.0][23] -> [v0.7.0][22]

If you haven't made any changes in the config file, you should be fine just updating the version number. Else,

- You can make the `Table: ...`, `InputAndLogs: ...` layout values null and define the common properties in the `general.panel_ui` instead.

#### [v0.5.13][24] -> [v0.6.0][23]

If you haven't made any changes in the config file, you should be fine just updating the version number. Else,

- Rename `add_modifier: {bits: 1}` to `add_modifiers: [Bold]`, `sub_modifier: {bits: 1}` to `sub_modifiers: [Bold]` and so on.
- Rename `percentage: 10` to `Percentage: 10`, `ratio: 1` to `Ratio: 1` and so on.
- You might want to free up or remap the `ctrl-w` key binding in `default` mode to enable layout switching.

Optionally, checkout this new theme to learn more about what's new.

#### [v0.5.0][25] -> [v0.5.13][24]

- Just update the `version` in your config file.
- For versions >= `v0.5.8`, you can set `$OPENER` env var to declare a global GUI file opener (to open files using keys `gx`).
- You might also want to update other mappings to handle files with names starting with `-` (hiphen). For example, instead of `rm ${filename}` use `rm -- ${filename}`. Same goes for `cp`, `mv`, `cat`, `touch` etc.
- For version >= `v0.5.13`, you might want to use the more specific `SwitchModeBuiltin` and `SwitchModeCustom` messages instead of the general `SwitchMode` message.

#### [v0.4.3][26] -> [v0.5.0][25]

If you haven't have any changes in the config file, you should be fine just updating the version number.

Else do the following

- Replace `{RelativePathIs, case_sensitive: true}` with `RelativePathIs`.
- Replace `{RelativePathIs, case_sensitive: false}` with `IRelativePathIs`.
- Do the same with other filters you are using.
- You might want to update your `backspace` handling to use the `RemoveInputBufferLastCharacter` message.
- You might want to free-up `f`, `s`, `ctrl-r` and `ctrl-u` key bindings in the default mode, or remap them.
- You might want to use the new UI variables.
- Update your config version to `v0.5.0`.

#### [v0.4.2][27] -> [v0.4.3][26]

If you have customized `general.table.row.cols`, you might want to [update it][28] to use the new variables with better symlink support.

#### [v0.4.1][29] -> [v0.4.2][27]

In case you have mapped the keys `q`, `ctrl-i` and `ctrl-o`, you may want to revisit the default mode key bindings and remap accordingly to use the new functionalities.

#### [v0.3.13][30] -> [v0.4.1][29]

A lot has changed (apologies). But I promise from now on, upgrading will be much less painful (thanks to [@maximbaz][31]'s valuable [inputs][32] and [code reviews][33]).

So, to start with the upgrade, let's remove everything from your config file except the `version` field and your custom modifications. If `version` is the only thing remaining, update it to `v0.4.1` and you are done.

Else, do the following

- Rename `general.focused_ui` to `general.focus_ui` ([see here][34]).
- Rename `filetypes` to `node_types`. ([see here][35])
- Rename `custom` field to `meta`. ([see here][36])
- Move `icon` to `meta.icon`. ([see here][37])
- Rename `normal_ui` to `default_ui`. ([see here][38])
- Split `modes` into `modes.builtin` and `modes.custom` ([see here][39]). Migrate your custom modes to `modes.custom`. And copy only the changes in the in-built modes in `modes.builtin`.
- Finally, update the `version` to `v0.4.1`.

#### [v0.3.8][40] -> [v0.3.13][30]

Your current config should work fine. However, you might want to replace some `Call` and `BashExec` messages with `CallSilently` and `BashExecSilently` to remove the flickering of the screen.

If you haven't made any changes to the configuration, you can delete and regenerate it.

Else, do the following

- Check the new default config by temporarily removing your current config (with backup) and dumping the new config.
- Search for `Call` and `BashExec` in the new config.
- Compare and probably replace the associated actions in your current config

#### [v0.3.0][41] -> [v0.3.8][40]

Your current config should work fine. However, you might want to replace some `ResetNodeFilters` messages with `RemoveNodeFilter` and `RemoveNodeFilterFromInput` to get a better search and filter experience.

If you haven't made any changes to the configuration, you can delete and regenerate it.

Else, do the following

- Check the new default config by temporarily removing your current config (with backup) and dumping the new config.
- Search for `RemoveNodeFilterFromInput` in the new config.
- Compare and probably replace the associated actions in your current config.

#### v0.2.14 -> [v0.3.0][41]

If you haven't made any changes to the configuration, you can delete and regenerate it.

Else do the following:

- `$XPLR_APP_YAML` has been removed. You can use `Debug` to export the app state.
- `$XPLR_RESULT` has been ported to file `$XPLR_PIPE_RESULT_OUT`. Use `cat` instead of `echo`, `<` instead of `<<<` etc.
- `$XPLR_GLOBAL_HELP_MENU` has been ported to
  file `$XPLR_PIPE_GLOBAL_HELP_MENU_OUT`. Use `cat` instead of `echo`, `<` instead of `<<<` etc.
- `$XPLR_DIRECTORY_NODES` has been ported to
  file `$XPLR_PIPE_DIRECTORY_NODES_OUT`. Use `cat` instead of `echo`, `<` instead of `<<<` etc.
- `$XPLR_LOGS` has been ported to file `$XPLR_PIPE_LOGS_OUT`. Use `cat` instead of `echo`, `<` instead of `<<<` etc.
- `$XPLR_PIPE_RESULT` has been ported to file `$XPLR_PIPE_RESULT_OUT`. Use `cat` instead of `echo`, `<` instead of `<<<` etc.
- Finally, update the `version` in your config file.

[1]: #instructions
[2]: https://github.com/sayanarijit/xplr/releases/tag/v0.13.7
[3]: https://github.com/sayanarijit/xplr/releases/tag/v0.14.7
[4]: https://github.com/sayanarijit/xplr/pull/229#issue-662426960
[6]: https://github.com/sayanarijit/xplr/releases/tag/v0.12.1
[7]: messages.md#calllua
[8]: messages.md#callluasilently
[9]: lua-function-calls.md#lua-context
[10]: https://docs.rs/xplr/latest/xplr/app/struct.App.html
[11]: https://github.com/sayanarijit/xplr/releases/tag/v0.11.1
[12]: https://github.com/sayanarijit/xplr/releases/tag/v0.10.2
[13]: https://github.com/sayanarijit/xplr/releases/tag/v0.9.1
[14]: https://github.com/sayanarijit/xplr/blob/85696ded7a/src/config.yml
[15]: https://github.com/sayanarijit/xplr/blob/main/src/init.lua
[16]: https://github.com/sayanarijit/xplr-yml2lua
[17]: https://github.com/sayanarijit/xplr/blob/bfdb7736b99bc3c5ae53e7d621ba0e7ca2299b14/src/init.lua#L2005-L2064
[18]: https://github.com/sayanarijit/xplr/pull/177#issue-650643573
[19]: https://github.com/sayanarijit/xplr/releases/tag/v0.9.0
[20]: https://github.com/sayanarijit/xplr/releases/tag/v0.8.0
[21]: https://github.com/sayanarijit/xplr/releases/tag/v0.7.2
[22]: https://github.com/sayanarijit/xplr/releases/tag/v0.7.0
[23]: https://github.com/sayanarijit/xplr/releases/tag/v0.6.0
[24]: https://github.com/sayanarijit/xplr/releases/tag/v0.5.13
[25]: https://github.com/sayanarijit/xplr/releases/tag/v0.5.0
[26]: https://github.com/sayanarijit/xplr/releases/tag/v0.4.3
[27]: https://github.com/sayanarijit/xplr/releases/tag/v0.4.2
[28]: https://github.com/sayanarijit/xplr/blob/af1cda5762/src/config.yml#L46-L48
[29]: https://github.com/sayanarijit/xplr/releases/tag/v0.4.1
[30]: https://github.com/sayanarijit/xplr/releases/tag/v0.3.13
[31]: https://github.com/maximbaz
[32]: https://github.com/sayanarijit/xplr/issues/45#issue-854447104
[33]: https://github.com/sayanarijit/xplr/pull/47
[34]: https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L124
[35]: https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L145
[36]: https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L154-L155
[37]: https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L45
[38]: https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L114
[39]: https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L180-L181
[40]: https://github.com/sayanarijit/xplr/releases/tag/v0.3.8
[41]: https://github.com/sayanarijit/xplr/releases/tag/v0.3.0
[42]: https://github.com/sayanarijit/xplr/releases/tag/v0.14.4
[43]: https://github.com/sayanarijit/xplr/releases/tag/v0.15.2
[44]: https://github.com/sayanarijit/xplr/releases/tag/v0.16.4
[45]: https://github.com/sayanarijit/xplr/releases/tag/v0.17.6
[46]: https://github.com/sayanarijit/xplr/releases/tag/v0.18.0
[47]: https://github.com/sayanarijit/xplr/releases/tag/v0.19.4
[48]: https://github.com/sayanarijit/xplr/releases/tag/v0.20.2
[49]: https://github.com/sayanarijit/xplr/releases/tag/v0.21.10
[50]: https://github.com/lotabout/skim#search-syntax
[51]: https://github.com/sayanarijit/xplr/releases/tag/v1.0.1
