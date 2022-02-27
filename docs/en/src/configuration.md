# Configuration

xplr can be configured using [Lua][3] via a special file named `init.lua`,
which can be placed in `~/.config/xplr/` (local to user) or `/etc/xplr/`
(global) depending on the use case.

When xplr loads, it first executes the [built-in init.lua][1] to set the
default values, which is then overwritten by another config file, if found
using the following lookup order:

1. `--config /path/to/init.lua`
2. `~/.config/xplr/init.lua`
3. `/etc/xplr/init.lua`

The first one found will be loaded by xplr and the lookup will stop.

The loaded config can be further extended using the `-C` or `--extra-config`
command-line option.

> **NEED HELP:** Auto generate rest of the docs from [src/init.lua][1]
> using [docs/script/generate.lua][2].

[1]: https://github.com/sayanarijit/xplr/blob/main/src/init.lua
[2]: https://github.com/sayanarijit/xplr/blob/main/docs/script/generate.lua
[3]: https://www.lua.org
