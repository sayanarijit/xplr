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

> **Note:** You don't generally need to create the config file. You can use the
> default configuration for basic operations. However, creating the config file
> is recommended because the project is in its early stage and the defaults
> might change. Creating the config file will save you from unexpected behavior
> when you [upgrade][3].
> Also, the default configuration is meant to be overwritten to suit your
> workflow.

## Run

```
xplr
```

[1]: install.md
[2]: https://github.com/sayanarijit/xplr/blob/main/src/init.lua
[3]: upgrade-guide.md
