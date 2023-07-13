# Borders

xplr allows customizing the shape and style of the borders.

### Border

A border is a [sum type][2] that can be one of the following:

- "Top"
- "Right"
- "Bottom"
- "Left"

### Border Type

A border type is a [sum type][2] that can be one of the following:

- "Plain"
- "Rounded"
- "Double"
- "Thick"

### Border Style

The [style][1] of the borders.

## Example

```lua
xplr.config.general.panel_ui.default.borders = { "Top", "Right", "Bottom", "Left" }
xplr.config.general.panel_ui.default.border_type = "Thick"
xplr.config.general.panel_ui.default.border_style.fg = "Black"
xplr.config.general.panel_ui.default.border_style.bg = "Gray"
```

[1]: style.md#style
[2]: sum-type.md
