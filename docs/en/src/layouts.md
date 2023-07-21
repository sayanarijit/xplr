### Layouts

xplr layouts define the structure of the UI, i.e. how many panel we see,
placement and size of the panels, how they look etc.

This is configuration exposed via the `xplr.config.layouts` API.

`xplr.config.layouts.builtin` contain some built-in panels which can be
overridden, but you can't add or remove panels in it.

You can add new panels in `xplr.config.layouts.custom`.

##### Example: Defining Custom Layout

```lua
xplr.config.layouts.builtin.default = {
  Horizontal = {
    config = {
      margin = 1,
      horizontal_margin = 1,
      vertical_margin = 1,
      constraints = {
        { Percentage = 50 },
        { Percentage = 50 },
      }
    },
    splits = {
      "Table",
      "HelpMenu",
    }
  }
}
```

Result:

```
╭ /home ─────────────╮╭ Help [default] ────╮
│   ╭─── path        ││.         show hidde│
│   ├▸[ð Desktop/]   ││/    ctrl search    │
│   ├  ð Documents/  ││:         action    │
│   ├  ð Downloads/  ││?         global hel│
│   ├  ð GitHub/     ││G         go to bott│
│   ├  ð Music/      ││V    ctrl select/uns│
│   ├  ð Pictures/   ││ctrl      duplicate │
│   ├  ð Public/     ││ctrl tab  next visit│
╰────────────────────╯╰────────────────────╯
```

#### xplr.config.layouts.builtin.default

The default layout

Type: [Layout](https://xplr.dev/en/layout)

#### xplr.config.layouts.builtin.no_help

The layout without help menu

Type: [Layout](https://xplr.dev/en/layout)

#### xplr.config.layouts.builtin.no_selection

The layout without selection panel

Type: [Layout](https://xplr.dev/en/layout)

#### xplr.config.layouts.builtin.no_help_no_selection

The layout without help menu and selection panel

Type: [Layout](https://xplr.dev/en/layout)

#### xplr.config.layouts.custom

This is where you can define custom layouts

Type: mapping of the following key-value pairs:

- key: string
- value: [Layout](https://xplr.dev/en/layout)

Example:

```lua
xplr.config.layouts.custom.example = "Nothing" -- Show a blank screen
xplr.config.general.initial_layout = "example" -- Load the example layout
```
