Upgrade Guide
=============

When you upgrade xplr, you might see an error like this

```
Incompatible script version in: /home/sayanarijit/.config/xplr/init.lua. The script version is: 0.9.0, the required version is: 0.10.1. Visit https://github.com/sayanarijit/xplr/wiki/Upgrade-Guide
```

All you need to do is follow the [instructions](#instructions) starting from
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

- `1.0.0` -> `1.0.x`: Bug fix (fully compatible).
- `1.0.0` -> `1.x.x`: Only backwards compatible. You can't generally use for
  e.g. `app-1.0.0` with `config-1.1.0`. But vice versa is fine.
- `1.0.0` -> `x.x.x`: Not compatible at all.

Note that until we're `v1`, we'll be using the `{minor}` version number as
`{major}`, and the `{patch}` fix number as `{minor}` to determine
compatibility.

</details>

### Instructions

#### [v0.13.7](https://github.com/sayanarijit/xplr/releases/tag/v0.13.7) -> [v0.14.3](https://github.com/sayanarijit/xplr/releases/tag/v0.14.3)

- macOS users need to place their config file (`init.lua`) in
  `$HOME/.config/xplr/` or `/etc/xplr/`.
- Library users please refer to the latest API docs.
- Check out the new messages: `{Start|Stop|Toggle}Fifo`. These enable support
  for [FIFO based file previews](https://github.com/sayanarijit/xplr/pull/229#issue-662426960).
- You can disable the recover mode using `config.general.disable_recover_mode = true`.
- Try running `xplr --help`. Yes, CLI has been implemented.
- Since version `v0.14.3`, `StartFifo` and `ToggleFifo` will write to the FIFO
  path when called. So, there's no need to pipe the focus path explicitely.
- Since version `v0.14.3`, general config `xplr.config.start_fifo` is available
  which can be set to a file path to start a fifo when xplr starts.

<sub>Like this project so far? **[Please consider contributing](contribute.md)**.</sub>

#### [v0.12.1](https://github.com/sayanarijit/xplr/releases/tag/v0.12.1) -> [v0.13.7](https://github.com/sayanarijit/xplr/releases/tag/v0.13.7)
- Lua functions called using [`CallLua`](https://docs.rs/xplr/latest/xplr/app/enum.ExternalMsg.html#variant.CallLua) and [`CallLuaSilently`](https://docs.rs/xplr/latest/xplr/app/enum.ExternalMsg.html#variant.CallLuaSilently) messages will receive [`CallLuaArg`](https://docs.rs/xplr/latest/xplr/app/struct.CallLuaArg.html) object as the function argument (instead of the [`App`](https://docs.rs/xplr/latest/xplr/app/struct.App.html) object).
- Each `node_types` config will inherit defaults from matching less specifig `node_types` config and overwrite them.
- Since version `v0.13.2`, you don't need to use/send `Refresh` anymore. It will be auto-handled by xplr.

#### [v0.11.1](https://github.com/sayanarijit/xplr/releases/tag/v0.11.1) -> [v0.12.1](https://github.com/sayanarijit/xplr/releases/tag/v0.12.1)

- `xplr.config.node_types.mime_essence` has split into type and subtype. Hence, instead of `xplr.config.node_types.mime_essence["text/plain"] = ..` use `xplr.config.node_types.mime_essence["text"] = { plain = .. }`.
- You can also define `xplr.config.node_types.mime_essence["text"]["*"]` that will match all text types (`text/*`).

#### [v0.10.2](https://github.com/sayanarijit/xplr/releases/tag/v0.10.2) -> [v0.11.1](https://github.com/sayanarijit/xplr/releases/tag/v0.11.1)

- `remaps:` has been removed to avoid confusion. Use lua assignments instead.
  For e.g.
  ```
  xplr.config.modes.builtin.default.key_bindings.on_key["v"] = xplr.config.modes.builtin.default.key_bindings.on_key.space
  ```

#### [v0.9.1](https://github.com/sayanarijit/xplr/releases/tag/v0.9.1) -> [v0.10.2](https://github.com/sayanarijit/xplr/releases/tag/v0.10.2)

- [`config.yml`](https://github.com/sayanarijit/xplr/blob/85696ded7a/src/config.yml) has been fully replaced with [`init.lua`](https://github.com/sayanarijit/xplr/blob/main/src/init.lua). If you have a lot of customization in your `config.yml`, [xplr-yml2lua](https://github.com/sayanarijit/xplr-yml2lua) can help you with migrating it to `init.lua`.
- `Handlebars` templates has been replaced with [Lua functions](https://github.com/sayanarijit/xplr/blob/bfdb7736b99bc3c5ae53e7d621ba0e7ca2299b14/src/init.lua#L2005-L2064). You can either remove the customizations or overwrite the functions accordingly.
- Added new messages `CallLua` and `CallLuaSilently` to call lua functions. The app state will be passed as input to the functions, and the returned messages will be handled by xplr. `CallLua` and `CallLuaSilently` are more flexible (and probably faster) alternatives to `Call`, `CallSilently`, `BashExec` and `BashExecSilently`. [e.g.](https://github.com/sayanarijit/xplr/pull/177#issue-650643573)

#### [v0.9.0](https://github.com/sayanarijit/xplr/releases/tag/v0.9.0) -> [v0.9.1](https://github.com/sayanarijit/xplr/releases/tag/v0.9.1)

- You can now set `remaps: {key: null}` to un-map a key.
- `gx` will open the item under focus.
- New key map `:sx` will open the selected items.

#### [v0.8.0](https://github.com/sayanarijit/xplr/releases/tag/v0.8.0) -> [v0.9.0](https://github.com/sayanarijit/xplr/releases/tag/v0.9.0)

Your previous config should mostly work fine. However, in case you are using `SwitchMode` heavily in your custom config, follow along.

- Introduced new message `PopMode`. You might want to use this message instead of `SwitchMode*` when returning back to the previous mode.
- After using (the group of) `PopMode` and `SwitchMode*` messages, you are now required to `Refresh` manually to avoid the UI lag.
- Pressing any invalid key will now lead you to the `recover` mode and will protect you from typing further invalid keys. Press `esc` to escape the `recover` mode.
- Introduced new message `LogWarning`, similar to other `Log*` messages.
- Creating files and directories has been optimized for batch creation.

#### [v0.7.2](https://github.com/sayanarijit/xplr/releases/tag/v0.7.2) -> [v0.8.0](https://github.com/sayanarijit/xplr/releases/tag/v0.8.0)

If you have made changes to the config file,

- Replace message `Explore` with `ExplorePwd` or `ExplorePwdAsync` or probably `ExploreParentsAsync`.
- Pipe `$XPLR_PIPE_FOCUS_OUT` has been removed. Use `$XPLR_FOCUS_PATH` env var instead.
- You might want to review your path escaping logics. For e.g. use `echo FocusPath: "'"$PWD"'" >> $PIPE` instead of `echo "FocusPath: $PWD" >> $PIPE`.

#### [v0.7.0](https://github.com/sayanarijit/xplr/releases/tag/v0.7.0) -> [v0.7.2](https://github.com/sayanarijit/xplr/releases/tag/v0.7.2)

- Just update the `version` in your config file.
- For version >= `v0.7.1`, you might want to free up or remap the `tab` key in `search` mode to enable easy selection during search.

#### [v0.6.0](https://github.com/sayanarijit/xplr/releases/tag/v0.6.0) -> [v0.7.0](https://github.com/sayanarijit/xplr/releases/tag/v0.7.0)

If you haven't made any changes in the config file, you should be fine just updating the version number. Else,

- You can make the `Table: ...`, `InputAndLogs: ...` layout values null and define the common properties in the `general.panel_ui` instead.

#### [v0.5.13](https://github.com/sayanarijit/xplr/releases/tag/v0.5.13) -> [v0.6.0](https://github.com/sayanarijit/xplr/releases/tag/v0.6.0)

If you haven't made any changes in the config file, you should be fine just updating the version number. Else,

- Rename `add_modifier: {bits: 1}` to `add_modifiers: [Bold]`, `sub_modifier: {bits: 1}` to `sub_modifiers: [Bold]` and so on.
- Rename `percentage: 10` to `Percentage: 10`, `ratio: 1` to `Ratio: 1` and so on.
- You might want to free up or remap the `ctrl-w` key binding in `default` mode to enable layout switching.

Optionally, checkout this new theme to learn more about what's new.

#### [v0.5.0](https://github.com/sayanarijit/xplr/releases/tag/v0.5.0) -> [v0.5.13](https://github.com/sayanarijit/xplr/releases/tag/v0.5.13)

- Just update the `version` in your config file.
- For versions >= `v0.5.8`, you can set `$OPENER` env var to declare a global GUI file opener (to open files using keys `gx`).
- You might also want to update other mappings to handle files with names starting with `-` (hiphen). For example, instead of `rm ${filename}` use `rm -- ${filename}`. Same goes for `cp`, `mv`, `cat`, `touch` etc.
- For version >= `v0.5.13`, you might want to use the more specific `SwitchModeBuiltin` and `SwitchModeCustom` messages instead of the general `SwitchMode` message.

#### [v0.4.3](https://github.com/sayanarijit/xplr/releases/tag/v0.4.3) -> [v0.5.0](https://github.com/sayanarijit/xplr/releases/tag/v0.5.0)

If you haven't have any changes in the config file, you should be fine just updating the version number.

Else do the following

- Replace `{RelativePathIs, case_sensitive: true}` with `RelativePathIs`.
- Replace `{RelativePathIs, case_sensitive: false}` with `IRelativePathIs`.
- Do the same with other filters you are using.
- You might want to update your `backspace` handling to use the `RemoveInputBufferLastCharacter` message.
- You might want to free-up `f`, `s`, `ctrl-r` and `ctrl-u` key bindings in the default mode, or remap them.
- You might want to use the new UI variables.
- Update your config version to `v0.5.0`.

#### [v0.4.2](https://github.com/sayanarijit/xplr/releases/tag/v0.4.2) -> [v0.4.3](https://github.com/sayanarijit/xplr/releases/tag/v0.4.3)


If you have customized `general.table.row.cols`, you might want to [update it](https://github.com/sayanarijit/xplr/blob/af1cda5762/src/config.yml#L46-L48) to use the new variables with better symlink support.

#### [v0.4.1](https://github.com/sayanarijit/xplr/releases/tag/v0.4.1) -> [v0.4.2](https://github.com/sayanarijit/xplr/releases/tag/v0.4.2)

In case you have mapped the keys `q`, `ctrl-i` and `ctrl-o`, you may want to revisit the default mode key bindings and remap accordingly to use the new functionalities.

#### [v0.3.13](https://github.com/sayanarijit/xplr/releases/tag/v0.3.13) -> [v0.4.1](https://github.com/sayanarijit/xplr/releases/tag/v0.4.1)

A lot has changed (apologies). But I promise from now on, upgrading will be much less painful (thanks to [@maximbaz](https://github.com/maximbaz)'s valuable [inputs](https://github.com/sayanarijit/xplr/issues/45#issue-854447104) and [code reviews](https://github.com/sayanarijit/xplr/pull/47)).

So, to start with the upgrade, let's remove everything from your config file except the `version` field and your custom modifications. If  `version` is the only thing remaining, update it to `v0.4.1` and you are done.

Else, do the following

- Rename `general.focused_ui` to `general.focus_ui` ([see here](https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L124)).
- Rename `filetypes` to `node_types`. ([see here](https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L145))
- Rename `custom` field to `meta`. ([see here](https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L154-L155))
- Move `icon` to `meta.icon`. ([see here](https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L45))
- Rename `normal_ui` to `default_ui`. ([see here](https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L114))
- Split `modes` into `modes.builtin` and `modes.custom` ([see here](https://github.com/sayanarijit/xplr/blob/055c1083d6/src/config.yml#L180-L181)). Migrate your custom modes to `modes.custom`. And copy only the changes in the in-built modes in `modes.builtin`.
- Finally, update the `version` to `v0.4.1`.

#### [v0.3.8](https://github.com/sayanarijit/xplr/releases/tag/v0.3.8) -> [v0.3.13](https://github.com/sayanarijit/xplr/releases/tag/v0.3.13)
Your current config should work fine. However, you might want to replace some `Call` and `BashExec` messages with `CallSilently` and `BashExecSilently` to remove the flickering of the screen.

If you haven't made any changes to the configuration, you can delete and regenerate it.

Else, do the following

- Check the new default config by temporarily removing your current config (with backup) and dumping the new config.
- Search for `Call` and `BashExec` in the new config.
- Compare and probably replace the associated actions in your current config

#### [v0.3.0](https://github.com/sayanarijit/xplr/releases/tag/v0.3.0) -> [v0.3.8](https://github.com/sayanarijit/xplr/releases/tag/v0.3.8)

Your current config should work fine. However, you might want to replace some `ResetNodeFilters` messages with `RemoveNodeFilter` and `RemoveNodeFilterFromInput` to get a better search and filter experience.

If you haven't made any changes to the configuration, you can delete and regenerate it.

Else, do the following

- Check the new default config by temporarily removing your current config (with backup) and dumping the new config.
- Search for `RemoveNodeFilterFromInput` in the new config.
- Compare and probably replace the associated actions in your current config.

#### v0.2.14 -> [v0.3.0](https://github.com/sayanarijit/xplr/releases/tag/v0.3.0)

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
