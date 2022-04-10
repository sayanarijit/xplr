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

[1]: https://github.com/dtomvan/xpm.xplr
