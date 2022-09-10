# Mode

A mode contains the following information:

- [name][5]
- [help][6]
- [extra_help][7]
- [key_bindings][9]
- [layout][10]
- [prompt][13]

### name

Type: string

This is the name of the mode visible in the help menu.

### help

Type: nullable string

If specified, the help menu will display this instead of the auto generated
mappings.

### extra_help

Type: nullable string

If specified, the help menu will display this along-side the auto generated
help menu.

### key_bindings

Type: [Key Bindings][8]

The key bindings available in that mode.

### layout

Type: nullable [Layout][11]

If specified, this layout will be used to render the UI.

### prompt

Type: nullable string

If set, this prompt will be displayed in the input buffer when in this mode.

## Also See:

- [xplr.config.modes][12]

[1]: #builtin
[2]: #custom
[3]: #mode
[4]: default-key-bindings.md
[5]: #name
[6]: #help
[7]: #extra_help
[8]: configure-key-bindings.md#key-bindings
[9]: #key_bindings
[10]: #layout
[11]: layout.md#layout
[12]: modes.md
[13]: #prompt
