# Installing Plugins

One way to install plugins is to use a plugin manager like [dtomvan/xpm.xplr][1].

But you can also install and manage plugins manually.

## Install Manually

- Add the following line in `~/.config/xplr/init.lua`

  ```lua
  local home = os.getenv("HOME")
  package.path = home
  .. "/.config/xplr/plugins/?/init.lua;"
  .. home
  .. "/.config/xplr/plugins/?.lua;"
  .. package.path
  ```

- Clone the plugin

  ```bash
  mkdir -p ~/.config/xplr/plugins

  git clone https://github.com/sayanarijit/material-landscape2.xplr ~/.config/xplr/plugins/material-landscape2
  ```

- Require the module in `~/.config/xplr/init.lua`

  ```lua
  require("material-landscape2").setup()

  -- The setup arguments might differ for different plugins.
  -- Visit the project README for setup instructions.
  ```

## Luarocks Support

Some plugins may require [luarocks][2] to work.

Setup luarocks with the following steps:

- Install luarocks (via your package managers or follow the [official guide][2]).
- Add `eval "$(luarocks path --lua-version 5.1)"` in your `.bashrc` or `.zshrc`.
- Add the following lines in `~/.config/xplr/init.lua`

  ```lua
  package.path = os.getenv("LUA_PATH") .. ";" .. package.path
  package.cpath = os.getenv("LUA_CPATH") .. ";" .. package.cpath
  ```

  Now you can install packages using luarocks. Be sure to append `--lua-version`.

Example:

```bash
luarocks install luafilesystem --local --lua-version 5.1
```

[1]: https://github.com/dtomvan/xpm.xplr
[2]: https://luarocks.org
