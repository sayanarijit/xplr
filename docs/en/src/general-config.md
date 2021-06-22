General Config
==============

This is configuration exposed via the `xplr.config.general` API. It contains
the following fields:


enable_mouse
------------

Type: boolean

Set it to `true` enable scrolling using mouse.


show_hidden
-----------

Type: boolean

Set it to `true` to show hidden files.


read_only
---------

Type: boolean

Set it to `true` to use only a subset of selected operations that forbids
executing commands or performing write operations on the file-system.


disable_recover_mode
--------------------

Type: boolean

Set it to `true` when the special recover mode gets too annoying to appreciate
the good intentions. When enabled, typing the wrong keys won't result in any
action.


cursor.format
-------------

Type: nullable string

This is the shape of the cursor visible when the input buffer contains some string.


cursor.style
------------

Type: [Style](style.md)

Style of the cursor.


initial_layout
--------------

Type: string

The name of one of the defined [layouts](layouts.md) to use when xplr loads.


TODO: Continue documentation
