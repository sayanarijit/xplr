Configuration
=============

xplr can be configured using [Lua](https://www.lua.org/) via a special file
named `init.lua`
([example](https://github.com/sayanarijit/xplr/blob/main/src/init.lua)), which
can be placed in `~/.config/xplr/` (user specific) or `/etc/xplr/` (global)
depending on the use case.

When a user specific configuration is available, the global configuration file
will be ignored.

However, it's also possible to place the file anywhere, with any name and use
the command-line argument `-c` / `--config` to specify its path explicitely. In
that case, both `~/.config/xplr/init.lua` and `/etc/xplr/init.lua` will be
ignored.


Loading Order
-------------

When xplr loads, it first executes the built-in
[init.lua](https://github.com/sayanarijit/xplr/blob/main/src/init.lua) to set
the default values. It is then overwritten by another config file, if found
using the following lookup order:

**--config /path/to/init.lua** > **~/.config/xplr/init.lua** > **/etc/xplr/init.lua**
