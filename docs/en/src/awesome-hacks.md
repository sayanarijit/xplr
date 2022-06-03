# Awesome Hacks

Here's a list of cool xplr hacks, i.e. snippets of code that you can just copy
and paste into your [configuration][1] or the appropriate file, that are
too small or too niche for a full fledge [plugin][2].

Do you have something cool to share?

[Edit this file][3] or [share them here][4] or [let us know][5].

### cd on quit

Change directory using xplr.

<details>
<summary>Expand for details</summary>

- Author: [@sayanarijit][8]
- Tested on: Linux

With this alias set, you can navigate directories using xplr by entering
xcd command, and when you quit by pressing enter, you will enter the
directory.

You can of course, quit with plain Quit (i.e. by pressing q) to
gracefully cancel "cd on quit".

```bash
alias xcd='cd "$(xplr --print-pwd-as-result)"'
```

</details>

### Spawn multiple sessions in different tabs (iTerm2)

Creating a new session that starts with iTerm2.

<details>
<summary>Expand for details</summary>

- Author: [@lmburns][9]
- Requires: iTerm2
- Tested on: MacOS

```lua
xplr.config.modes.builtin.default.key_bindings.on_key["ctrl-n"] = {
  help = "new session",
  messages = {
    { BashExecSilently = [[
        osascript <<EOF
        tell application "iTerm2"
          tell current window
            create tab with default profile
            tell current session to write text "xplr"
          end tell
        end tell
      ]]
    },
  },
}
```

</details>

### Bookmark

Bookmark files using `m` and fuzzy search bookmarks using backtick.

<details>
<summary>Expand for details</summary>

[![xplr-bookmark.gif][7]][6]

- Author: [@sayanarijit][8]
- Requires: fzf
- Tested on: Linux

```lua
xplr.config.modes.builtin.default.key_bindings.on_key.m = {
  help = "bookmark",
  messages = {
    {
      BashExecSilently = [===[
        PTH="${XPLR_FOCUS_PATH:?}"
        if echo "${PTH:?}" >> "${XPLR_SESSION_PATH:?}/bookmarks"; then
          echo "LogSuccess: ${PTH:?} added to bookmarks" >> "${XPLR_PIPE_MSG_IN:?}"
        else
          echo "LogError: Failed to bookmark ${PTH:?}" >> "${XPLR_PIPE_MSG_IN:?}"
        fi
      ]===],
    },
  },
}

xplr.config.modes.builtin.default.key_bindings.on_key["`"] = {
  help = "go to bookmark",
  messages = {
    {
      BashExec = [===[
        PTH=$(cat "${XPLR_SESSION_PATH:?}/bookmarks" | fzf --no-sort)
        if [ "$PTH" ]; then
          echo FocusPath: "'"${PTH:?}"'" >> "${XPLR_PIPE_MSG_IN:?}"
        fi
      ]===],
    },
  },
}
```

</details>

### Persistent, multi-session bookmark

A bookmark mode that allows for a bookmark file to be used throughout multiples
sessions. It is set to the environment variable `$XPLR_BOOKMARK_FILE`. A
bookmark can be added, deleted, or jumped to.

<details>
<summary>Expand for details</summary>

- Author: [@lmburns][9]
- Requires: fzf, sd
- Tested on: MacOS

```lua
-- Bookmark: mode binding
xplr.config.modes.custom.bookmark = {
  name = "bookmark",
  key_bindings = {
    on_key = {
      m = {
        help = "bookmark dir",
        messages = {
          { BashExecSilently = [[
              PTH="${XPLR_FOCUS_PATH:?}"
              if [ -d "${PTH}" ]; then
                PTH="${PTH}"
              elif [ -f "${PTH}" ]; then
                PTH="$(dirname "${PTH}")"
              fi
              if echo "${PTH:?}" >> "${XPLR_BOOKMARK_FILE:?}"; then
                echo "LogSuccess: ${PTH:?} added to bookmarks" >> "${XPLR_PIPE_MSG_IN:?}"
              else
                echo "LogError: Failed to bookmark ${PTH:?}" >> "${XPLR_PIPE_MSG_IN:?}"
              fi
            ]]
          },
        },
      },
      g = {
        help = "go to bookmark",
        messages = {
          {
            BashExec = [===[
              PTH=$(cat "${XPLR_BOOKMARK_FILE:?}" | fzf --no-sort)
              if [ "$PTH" ]; then
                echo FocusPath: "'"${PTH:?}"'" >> "${XPLR_PIPE_MSG_IN:?}"
              fi
            ]===]
          },
        },
      },
      d = {
        help = "delete bookmark",
        messages = {
          { BashExec = [[
              PTH=$(cat "${XPLR_BOOKMARK_FILE:?}" | fzf --no-sort)
              sd "$PTH\n" "" "${XPLR_BOOKMARK_FILE:?}"
            ]]
          },
        },
      },
      esc = {
        help = "cancel",
        messages = {
          "PopMode",
        },
      },
    },
  },
}
```

</details>

### Another bookmark manager type thing, taken from [wfxr's zsh plugin][13].

Another bookmark manager type thing, taken from [wfxr's zsh plugin][13] which has colored output with fzf.

<details>
<summary>Expand for details</summary>

- Author: [@lmburns][9]
- Requires: fzf, exa
- Tested on: MacOS

```lua
xplr.config.modes.builtin.go_to.key_bindings.on_key.b = {
  help = "bookmark jump",
  messages = {
    "PopMode",
    { BashExec = [===[
        field='\(\S\+\s*\)'
        esc=$(printf '\033')
        N="${esc}[0m"
        R="${esc}[31m"
        G="${esc}[32m"
        Y="${esc}[33m"
        B="${esc}[34m"
        pattern="s#^${field}${field}${field}${field}#$Y\1$R\2$N\3$B\4$N#"
        PTH=$(sed 's#: # -> #' "$PATHMARKS_FILE"| nl| column -t \
        | gsed "${pattern}" \
        | fzf --ansi \
            --height '40%' \
            --preview="echo {}|sed 's#.*->  ##'| xargs exa --color=always" \
            --preview-window="right:50%" \
        | sed 's#.*->  ##')
        if [ "$PTH" ]; then
          echo ChangeDirectory: "'"${PTH:?}"'" >> "${XPLR_PIPE_MSG_IN:?}"
        fi
      ]===]
    },
  }
}
```

</details>

### Fuzzy search history

Fuzzy search the last visited directories.

<details>
<summary>Expand for details</summary>

- Author: [@sayanarijit][8]
- Requires: fzf
- Tested on: Linux

```lua
xplr.config.modes.builtin.go_to.key_bindings.on_key.h = {
  help = "history",
  messages = {
    "PopMode",
    {
      BashExec = [===[
        PTH=$(cat "${XPLR_PIPE_HISTORY_OUT:?}" | sort -u | fzf --no-sort)
        if [ "$PTH" ]; then
          echo ChangeDirectory: "'"${PTH:?}"'" >> "${XPLR_PIPE_MSG_IN:?}"
        fi
      ]===],
    },
  },
}
```

</details>

### Batch rename

Batch rename the selected or visible files and directories in $PWD.

<details>
<summary>Expand for details</summary>

[![xplr-rename.gif][11]][10]

- Author: [@sayanarijit][8]
- Requires: [pipe-rename][12]
- Tested on: Linux

```lua
xplr.config.modes.builtin.default.key_bindings.on_key.R = {
  help = "batch rename",
  messages = {
    {
      BashExec = [===[
       SELECTION=$(cat "${XPLR_PIPE_SELECTION_OUT:?}")
       NODES=${SELECTION:-$(cat "${XPLR_PIPE_DIRECTORY_NODES_OUT:?}")}
       if [ "$NODES" ]; then
         echo -e "$NODES" | renamer
         echo ExplorePwdAsync >> "${XPLR_PIPE_MSG_IN:?}"
       fi
     ]===],
    },
  },
}
```

</details>

### Serve $PWD

Serve $PWD using a static web server via LAN.

<details>
<summary>Expand for details</summary>

- Author: [@sayanarijit][8]
- Requires: [sfz][14], fzf
- Tested on: Linux

```lua
xplr.config.modes.builtin.default.key_bindings.on_key.S = {
  help = "serve $PWD",
  messages = {
    {
      BashExec = [===[
        IP=$(ip addr | grep -w inet | cut -d/ -f1 | grep -Eo '[0-9]{1,3}\.[0-9]{      1,3}\.[0-9]{1,3}\.[0-9]{1,3}' | fzf --prompt 'Select IP > ')
        echo "IP: ${IP:?}"
        read -p "Port (default 5000): " PORT
        echo
        sfz --all --cors --no-ignore --bind ${IP:?} --port ${PORT:-5000} . &
        sleep 1 && read -p '[press enter to exit]'
        kill -9 %1
      ]===],
    },
  },
}
```

</details>

### Image viewer (imv)

Preview images using [imv][17].

<details>
<summary>Expand for details</summary>

- Author: [@sayanarijit][8]
- Requires: [imv][17], [xdotool][18]
- Tested on: Linux

```lua
xplr.config.modes.builtin.default.key_bindings.on_key.P = {
  help = "preview",
  messages = {
    {
      BashExecSilently = [===[
        FIFO_PATH="/tmp/xplr.fifo"

        if [ -e "$FIFO_PATH" ]; then
          echo StopFifo >> "$XPLR_PIPE_MSG_IN"
          rm "$FIFO_PATH"
        else
          mkfifo "$FIFO_PATH"
          "$HOME/.local/bin/imv-open.sh" "$FIFO_PATH" "$XPLR_FOCUS_PATH" &
          echo "StartFifo: '$FIFO_PATH'" >> "$XPLR_PIPE_MSG_IN"
        fi
      ]===],
    },
  },
}
```

$HOME/.local/bin/imv-open.sh

```bash
#!/usr/bin/env bash

FIFO_PATH="$1"
IMAGE="$2"
MAINWINDOW="$(xdotool getactivewindow)"
IMV_PID="$(pgrep imv)"

if [ ! "$IMV_PID" ]; then
  imv "$IMAGE" &
  IMV_PID=$!
fi

sleep 0.5

xdotool windowactivate "$MAINWINDOW"

while read -r path; do
  imv-msg "$IMV_PID" close all
  imv-msg "$IMV_PID" open "$path"
done < "$FIFO_PATH"

imv-msg "$IMV_PID" quit
[ -e "$FIFO_PATH" ] && rm -f "$FIFO_PATH"
```

</details>

### Text preview pane

Preview text files in a native xplr pane (should be fast enough).

<details>
<summary>Expand for details</summary>

- Author: [@sayanarijit][8]
- Requires: none
- Tested on: Linux

```lua
local function stat(node)
  return node.mime_essence
end

local function read(path, lines)
  local out = ""
  local p = io.open(path)

  if p == nil then
    return stat(path)
  end

  local i = 0
  for line in p:lines() do
    out = out .. line .. "\n"
    if i == lines then
      break
    end
    i = i + 1
  end
  p:close()

  return out
end

xplr.config.layouts.builtin.default = {
  Horizontal = {
    config = {
      constraints = {
        { Percentage = 60 },
        { Percentage = 40 },
      },
    },
    splits = {
      "Table",
      {
        CustomContent = {
          title = "preview",
          body = { DynamicParagraph = { render = "custom.preview_pane.render" } },
        },
      },
    },
  },
}

xplr.fn.custom.preview_pane = {}
xplr.fn.custom.preview_pane.render = function(ctx)
  local n = ctx.app.focused_node

  if n.canonical then
    n = n.canonical
  end

  if n then
    if n.is_file then
      return read(n.absolute_path, ctx.layout_size.height)
    else
      return stat(n)
    end
  else
    return ""
  end
end
```

</details>

## Also See:

- [Awesome Plugins][15]
- [Awesome Integrations][16]

[1]: configuration.md
[2]: plugin.md
[3]: https://github.com/sayanarijit/xplr/edit/main/docs/en/src/awesome-hacks.md
[4]: https://github.com/sayanarijit/xplr/discussions/categories/show-and-tell
[5]: community.md
[6]: https://gifyu.com/image/rGSR
[7]: https://s4.gifyu.com/images/xplr-bookmark.gif
[8]: https://github.com/sayanarijit
[9]: https://github.com/lmburns
[10]: https://gifyu.com/image/rGbo
[11]: https://s4.gifyu.com/images/xplr-rename.gif
[12]: https://github.com/marcusbuffett/pipe-rename
[13]: https://github.com/wfxr/formarks
[14]: https://github.com/weihanglo/sfz
[15]: awesome-plugins.md
[16]: awesome-integrations.md
[17]: https://sr.ht/~exec64/imv
[18]: https://www.semicomplete.com/projects/xdotool
