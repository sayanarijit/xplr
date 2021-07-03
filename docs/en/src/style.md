Style
=====

A style object contains the following information:

- [fg][1]
- [bg][2]
- [add_modifiers][3]
- [sub_modifiers][4]

### fg

Type: nullable [Color][5]

The foreground color.


### bg

Type: nullable [Color][5]

The background color.


### add_modifiers

Type: nullable list of [Modifier][6]

Modifiers to add.


### sub_modifiers

Type: nullable list of [Modifier][6]

Modifiers to remove.


Color
-----

Color can be one of the following:

- "Reset"
- "Black"
- "Red"
- "Green"
- "Yellow"
- "Blue"
- "Magenta"
- "Cyan"
- "Gray"
- "DarkGray"
- "LightRed"
- "LightGreen"
- "LightYellow"
- "LightBlue"
- "LightMagenta"
- "LightCyan"
- "White"
- { Rgb = { int, int, int } }
- { Indexed = int }


Modifier
--------

Modifier can be one of the following:

- "Bold"
- "Dim"
- "Italic"
- "Underlined"
- "SlowBlink"
- "RapidBlink"
- "Reversed"
- "Hidden"
- "CrossedOut"


Example
-------

```lua
xplr.config.general.cursor.style.fg = "Red"
xplr.config.general.cursor.style.bg = { Rgb = { 100, 150, 200 } }
xplr.config.general.cursor.style.add_modifiers = { "Bold", "Italic" }
xplr.config.general.cursor.style.sub_modifiers = { "Hidden" }
```


[1]:#fg
[2]:#bg
[3]:#add_modifiers
[4]:#sub_modifiers
[5]:#color
[6]:#modifier