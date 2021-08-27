Writing Plugins
===============

Anyone who can write [Lua][1] code, can write xplr plugins.

Just follow the instructions and best practices:


Naming
------

xplr plugins are named using hiphen (`-`) separated words that may also include
integers. They will be plugged using the `require()` function in Lua.


Structure
---------

A minimal plugin should confirm to the following structure:

```
plugin-name
├── README.md
└── src
    └── init.lua
```

You can also use
[this template][2].


### README.md

This is where you document what the plugin does, how to use it, etc.

### src/init.lua

This file is executed to load the plugin. It should expose a `setup()`
function, which will be used by the users to setup the plugin.

Example:

```lua
local function setup(args)
  local xplr = xplr
  -- do stuff with xplr
end

return { setup = setup }
```

Publishing
----------

When publishing plugins on GitHub or other repositories, it's a best practice
to append `.xplr` to the name to make them distinguishable. Similar to the
`*.nvim` naming convention for [Neovim][3] plugins.

Finally, after publishing, don't hesitate to
[let us know][4].


Examples
--------

Visit [Awesome Plugins][5] for xplr plugin examples.


Also See
--------

- [Tutorial: Adding a New Mode](modes.md#tutorial-adding-a-new-mode)
- [Example: Using Environment Variables and Pipes](message.md#example-using-environment-variables-and-pipes)
- [Example: Using Lua Function Calls](message.md#example-using-lua-function-calls)
- [Example: Customizing Table Renderer](column-renderer.md#example-customizing-table-renderer)


[1]:https://www.lua.org
[2]:https://github.com/sayanarijit/plugin-template1.xplr
[3]:https://neovim.io
[4]:https://github.com/sayanarijit/xplr/discussions/categories/show-and-tell
[5]:awesome-plugins.md
