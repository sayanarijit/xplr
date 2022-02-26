# Borders

xplr allows customizing the shape and style of the borders.

### Border

A border can be one of the following:

- Top
- Right
- Bottom
- Left

### Border Type

A border can be one of the following:

- Plain
- Rounded
- Double
- Thick

## Example

```lua
xplr.config.general.panel_ui.default.borders = { "Top", "Right", "Bottom", "Left" }
xplr.config.general.panel_ui.default.border_type = "Thick"
xplr.config.general.panel_ui.default.border_style.fg = "Black"
xplr.config.general.panel_ui.default.border_style.bg = "Gray"
```
