Configuration
=============

xplr can be configured using [Lua][1] via a special file
named `init.lua`
([example][2]), which
can be placed in `~/.config/xplr/` (user specific) or `/etc/xplr/` (global)
depending on the use case.

When a user specific configuration is available, the global configuration file
will be ignored.

However, it's also possible to place the file anywhere, with any name and use
the command-line argument `-c` / `--config` to specify its path explicitely. In
that case, both `~/.config/xplr/init.lua` and `/etc/xplr/init.lua` will be
ignored.


How Config Is Loaded
--------------------

When xplr loads, it first executes the built-in
[init.lua][2] to set
the default values, which is then overwritten by another config file, if found
using the following lookup order:

**--config /path/to/init.lua** > **~/.config/xplr/init.lua** > **/etc/xplr/init.lua**


config
------

The xplr configuration, exposed as `xplr.config` Lua API contains the
following fields:

- [general][3]
- [modes][4]
- [layouts][5]
- [node_types][6]


[1]:https://www.lua.org/
[2]:https://github.com/sayanarijit/xplr/blob/main/src/init.lua
[3]:general-config.md
[4]:modes.md
[5]:layouts.md
[6]:node_types.md