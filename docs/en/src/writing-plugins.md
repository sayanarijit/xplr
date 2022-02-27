# Writing Plugins

Anyone who can write [Lua][1] code, can write xplr plugins.

Just follow the instructions and best practices:

## Naming

xplr plugins are named using hiphen (`-`) separated words that may also include
integers. They will be plugged using the `require()` function in Lua.

## Structure

A minimal plugin should confirm to the following structure:

```
plugin-name
├── README.md
└── init.lua
```

You can also use
[this template][2].

### README.md

This is where you document what the plugin does, how to use it, etc.

### init.lua

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

## Publishing

When publishing plugins on GitHub or other repositories, it's a best practice
to append `.xplr` to the name to make them distinguishable. Similar to the
`*.nvim` naming convention for [Neovim][3] plugins.

Finally, after publishing, don't hesitate to
[let us know][4].

## Examples

Visit [Awesome Plugins][5] for xplr plugin examples.

## Also See

- [Tip: A list of hacks yet to make it as Lua plugins][15]
- [Tip: Some UI and themeing tips][12]
- [Tip: A list of handy utility functions][13]
- [Tip: Share tips and tricks working with Lua][14]
- [Tutorial: Adding a New Mode][6]
- [Example: Using Environment Variables and Pipes][7]
- [Example: Using Lua Function Calls][8]
- [Example: Defining Custom Layout][9]
- [Example: Customizing Table Renderer][10]
- [Example: Render a custom dynamic table][11]

[1]: https://www.lua.org
[2]: https://github.com/sayanarijit/plugin-template1.xplr
[3]: https://neovim.io
[4]: https://github.com/sayanarijit/xplr/discussions/categories/show-and-tell
[5]: awesome-plugins.md
[6]: configure-key-bindings.md#tutorial-adding-a-new-mode
[7]: environment-variables-and-pipes.md#example-using-environment-variables-and-pipes
[8]: lua-function-calls.md#example-using-lua-function-calls
[9]: layout.md#example-defining-custom-layout
[10]: column-renderer.md#example-customizing-table-renderer
[11]: layout.md#example-render-a-custom-dynamic-table
[12]: https://github.com/sayanarijit/xplr/discussions/274
[13]: https://github.com/sayanarijit/xplr/discussions/273
[14]: https://github.com/sayanarijit/xplr/discussions/250
[15]: https://github.com/sayanarijit/xplr/wiki/Hacks
