# Installing Plugins

Until we get a cool plugin manager, let's install plugins manually using the
following procedure:

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
