An experimental, minimal, configurable TUI file explorer, stealing ideas from [`nnn`](https://github.com/jarun/nnn) and [`fzf`](https://github.com/junegunn/fzf).

![Screenshot](https://user-images.githubusercontent.com/11632726/109526906-1b555080-7ad9-11eb-9fd7-03e092220618.gif)


Example usage:
--------------

```bash
# Edit file
vim "$(xplr)"

# Copy file(s)
cp "$(xplr)" "$(xplr)/"

# Search and move file
mv "$(fzf)" "$(xplr)/"
```


Let's brainstorm
----------------

You can also experiment and help by suggesting ideas/opinions.

1. Install

```bash
cargo install xplr
```

2. Create the customizable config file (requires [`yq`](https://github.com/mikefarah/yq))

```bash
mkdir -p ~/.config/xplr

echo "?" | xplr | yq ".config" -y | tee ~/.config/xplr/config.yml
```

3. Check the key bindings in the config file.

4. Run `xplr`.


TODO research
-------------

- [ ] Research FIFO/socket options for better integration with other tools.
- [ ] Research saner configuration formats.
- [ ] Research saner key binding options.
- [ ] Research how to go beyond filesystem and explore any tree-like structure.
- [ ] Research ways to make it faster (load and run).
- [ ] Research ways to implement a plugin system.
- [ ] CLI options and help menu.
- [ ] Go beyond research and implement things.
