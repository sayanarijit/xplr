

Create the customizable config file
-----------------------------------

```bash
mkdir -p ~/.config/xplr

version="$(xplr | grep ^version: | cut -d' ' -f 2)"

# When the app loads, press `#`

echo version = '"'${version:?}'"' > ~/.config/xplr/init.lua
```

Then
**[copy from here](https://github.com/sayanarijit/xplr/blob/main/src/init.lua)**
and remove / comment out what you don't want to customize.

> **Note:** You don't generally need to create the config file. You can use the
> default configuration for basic operations. However, creating the config file
> is recommended because the project is in its early stage and the defaults
> might change. Creating the config file will save you from unexpected behavior
> when you [upgrade](upgrade-guide.md).
> Also, the default configuration is very minimal (just enough to get you
> started) and you'll probably want to adjust it to suit your workflow.


Run
---

```
xplr
```
