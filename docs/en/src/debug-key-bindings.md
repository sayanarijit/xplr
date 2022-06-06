# Debug Key Bindings

If you need help debugging or understanding key bindings DYI way, you can
create a `test.lua` file with the following script, launch xplr with
`xplr --extra-config test.lua`, press `#` and play around.

```lua
-- The global key bindings inherited by all the modes.
xplr.config.general.global_key_bindings = {
  on_key = {
    esc = {
      help = "escape",
      messages = {
        { LogInfo = "global on_key(esc) called" },
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
}

-- Press `#` to enter the `debug key bindings` mode.
xplr.config.modes.builtin.default.key_bindings.on_key["#"] = {
  help = "test",
  messages = {
    "PopMode",
    { SwitchModeCustom = "debug_key_bindings" },
  },
}

-- The `debug key bindings` mode.
xplr.config.modes.custom.debug_key_bindings = {
  name = "debug key bindings",
  key_bindings = {
    on_key = {
      ["1"] = {
        messages = {
          { LogInfo = "on_key(1) called" },
        },
      },
      a = {
        messages = {
          { LogInfo = "on_key(a) called" },
        },
      },
      ["`"] = {
        messages = {
          { LogInfo = "on_key(`) called" },
        },
      },
      tab = {
        messages = {
          { LogInfo = "on_key(tab) called" },
        },
      },
      f1 = {
        messages = {
          { LogInfo = "on_key(f1) called" },
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
