# Debug Key Bindings

If you need help debugging or understanding key bindings DYI way, you can
create a `test.lua` file with the following script, launch xplr with
`xplr --extra-config test.lua`, press `#` and play around.

```lua
xplr.config.modes.builtin.default.key_bindings.on_key["#"] = {
  help = "test",
  messages = {
    "PopMode",
    { SwitchModeCustom = "test" },
  },
}

xplr.config.modes.custom.test = {
  name = "test",
  key_bindings = {
    on_key = {
      ["1"] = {
        messages = {
          { LogInfo = "on_key called" },
        },
      },
      a = {
        messages = {
          { LogInfo = "on_key called" },
        },
      },
      ["`"] = {
        messages = {
          { LogInfo = "on_key called" },
        },
      },
      tab = {
        messages = {
          { LogInfo = "on_key called" },
        },
      },
      esc = {
        messages = {
          "PopMode",
        },
      },
      ["ctrl-c"] = {
        messages = {
          "Terminate",
        },
      },
      f1 = {
        messages = {
          { LogInfo = "on_key called" },
        },
      },
    },
    on_alphabet = {
      messages = {
        { LogInfo = "on_alphabet called" },
      },
    },
    on_number = {
      messages = {
        { LogInfo = "on_number called" },
      },
    },
    -- on_alphanumeric = {
    --   messages = {
    --     { LogInfo = "on_alphanumeric called" },
    --   },
    -- },
    on_special_character = {
      messages = {
        { LogInfo = "on_special_character called" },
      },
    },
    -- on_character = {
    --   messages = {
    --     { LogInfo = "on_character called" },
    --   },
    -- },
    on_navigation = {
      messages = {
        { LogInfo = "on_navigation called" },
      },
    },
    on_function = {
      messages = {
        { LogInfo = "on_function called" },
      },
    },
    default = {
      messages = {
        { LogInfo = "default called" },
      },
    },
  },
}
```
