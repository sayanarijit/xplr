Introduction
============

`xplr` is a terminal UI based file explorer that aims to increase our terminal
productivity by being a flexible, interactive orchestrator for the ever growing
awesome command-line utilities that work with the file-system.

To achieve its goal, `xplr` strives to be a fast, minimal and more importantly,
hackable file explorer.

`xplr` is not meant to be a replacement for the standard shell commands or the
GUI file managers. Rather, it aims to integrate them all and expose an
intuitive, scriptable, keyboard controlled, real-time visual interface, also
being an ideal candidate for further integration, enabling the users to achieve
insane terminal productivity.


Features
--------

### Hackable

xplr is built with configurability in mind. So it allows you to perform a vast
set of operations and make it behave just the way you want.

A few things you can do with the [`xplr`][xplr] configuration

- [Hack the layout](layouts.md)
- [Hack the key bindings](modes.md)
- [Extend with plugins](awesome-plugins.md)


## Fast

Although speed is not the primary concern, [`xplr`][xplr] is already fast
enough so that you can take it out for a walk into your `node_modules` or
`/nix/store` any time you want. I currently
[measure the most commonly used operations](https://github.com/sayanarijit/xplr/tree/main/benches)
and I have seen it improve significantly over time, and it's only the start.

**Tip:** A quick and easy way to optimize UI rendering is reducing the number
of columns in the table.

**Note:** If you feel [`xplr`][xplr] is not behaving at its optimal, this is
probably because I am waiting for someone to complain. I want to avoid
optimizing things I don't need to, because optimization often requires either
complexity or feature sacrifice or both.


## Minimalist

[`xplr`][xplr] prefers to stay minimal, both in terms of features and binary
size, but just like speed, minimalism isn't as aggressively pursued as
configurability. If adding some feature, lines of code, or a dependency allows
the users to be a little more productive or allows [`xplr`][xplr] to be a
little more configurable, it will be considered. But of-course, the `bulk vs
productivity gain per user` balance will also be considered in the
decision-making.


## Other features

- [Embedded LuaJIT](https://github.com/sayanarijit/xplr/discussions/183) for
  portability and extensibility.
- **Switchable recover mode:** Saves you from doing unwanted things when in a
  hurry.
- **Sane (vim-like) defaults:**
  - Use <kbd>h</kbd>, <kbd>j</kbd>, <kbd>k</kbd>, <kbd>l</kbd> or arrow keys
    for basic navigation.
  - Go to top using <kbd>g</kbd><kbd>g</kbd>, and bottom using <kbd>G</kbd>.
  - Travel history using <kbd>ctrl-o</kbd> and <kbd>ctrl-i</kbd>.
  - Go to home directory using <kbd>~</kbd>.
  - Enter search mode with <kbd>/</kbd> or <kbd>ctrl-f</kbd>.
  - Go to absolute index (e.g. `4`) using <kbd>4</kbd><kbd>enter</kbd> or
    <kbd>:</kbd><kbd>4</kbd><kbd>enter</kbd>.
  - Go to relative index (e.g. `4 down`) using <kbd>4</kbd><kbd>down</kbd> or
    <kbd>:</kbd><kbd>4</kbd><kbd>down</kbd>.
  - Follow symlink using <kbd>g</kbd><kbd>f</kbd>.
  - Open in GUI using <kbd>g</kbd><kbd>x</kbd>.
  - Spawn terminal using <kbd>:</kbd><kbd>!</kbd>.
  - Toggle selection using <kbd>v</kbd> or <kbd>space</kbd>.
  - Toggle select all using <kbd>V</kbd> or <kbd>ctrl-a</kbd>.
  - Clear selections using <kbd>ctrl-u</kbd>.
- **Separate keys for navigation:** navigation keys are separated from the
  action keys (e.g. file opening action) to avoid mistakenly performing
  unwanted actions while navigating.
- **Always visible panels** to save you brain cycles:
  - Selection list.
  - Help menu.
  - Input & logs.
  - Filter and sort pipeline.
- **Batch creation:** Create multiple files and directories without repeating
  keys.
- **Batch sort & filter:** Apply sorters and filters in without repeating keys.
- **Custom file properties:** Display custom file properties with custom colors
  in the table using Lua functions.
- **Input buffer:** Read user input using the built-in input buffer with
  customizable behavior.
- **Switchable layouts:** Switch layouts dynamically without leaving `xplr`.
- **Saved locations:** Never lose context when traveling back and forth
  directories.
- **Auto refresh state:** Auto refresh app state when the `$PWD` changes.
- **Manually refresh UI** when other apps mess it up.
- **FIFO-based previews:** Easy to manage FIFO file that can be used to
  [integrate with previewers](https://github.com/sayanarijit/xplr/pull/229).
- **Different quit options:**
  - Quit with success without any output (<kbd>q</kbd>).
  - Quit with success and the result printed on stdout (<kbd>enter</kbd>).
  - Quit with success and the present working directory printed on stdout
    (<kbd>:</kbd><kbd>q</kbd><kbd>p</kbd>).
  - Quit with success and the path under focus printed on stdout
    (<kbd>:</kbd><kbd>q</kbd><kbd>f</kbd>).
  - Quit with success and the selection printed on stdout
    (<kbd>:</kbd><kbd>q</kbd><kbd>s</kbd>).
  - Quit with failure (<kbd>ctrl-c</kbd>).
