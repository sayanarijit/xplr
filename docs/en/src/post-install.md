# Post Install

Once [installed][1], use the following steps to setup and run xplr.

## Create the customizable config file

```bash
mkdir -p ~/.config/xplr

version="$(xplr --version | awk '{print $2}')"

echo "version = '${version:?}'" > ~/.config/xplr/init.lua
```

Then
**[copy from here][2]**
and remove / comment out what you don't want to customize.

## Run

```
xplr
```

[1]: install.md
[2]: https://github.com/sayanarijit/xplr/blob/main/src/init.lua
[3]: upgrade-guide.md
