# Introduction

xplr is a terminal UI based file explorer that aims to increase our terminal
productivity by being a flexible, interactive orchestrator for the ever growing
awesome command-line utilities that work with the file-system.

To achieve its goal, xplr strives to be a fast, minimal and more importantly,
hackable file explorer.

xplr is not meant to be a replacement for the standard shell commands or the
GUI file managers. Rather, it aims to [integrate them all][14] and expose an
intuitive, scriptable, [keyboard controlled][2],
[real-time visual interface][1], also being an ideal candidate for [further
integration][15], enabling you to achieve insane terminal productivity.

## Concept

### Hackable

xplr is built with configurability in mind. So it allows you to perform a vast
set of operations and make it look and behave just the way you want.

A few things you can do with the xplr configuration

- [Hacks][16]
- [Plugins][3]
- [Integrations][15]

### Fast

Although speed is not the primary concern, xplr is already fast enough so that
you can take it out for a walk into your `node_modules` or `/nix/store` any
time you want, and it will only get faster. Still, if you feel like it's
somehow making you slow, just report it. Most probably we're just waiting for
someone to complain.

**Tip:** A quick and easy way to optimize the UI rendering is reducing the
number of columns in the table.

### Minimal

xplr is being referred to as a _File Explorer_, not a _File Manager_. This
is because at the core, xplr is only an explorer, and [outsources][18] the file
management operations to external commands. This helps xplr stay minimal, and
focus only on doing what it does best.

So, just like speed, minimalism isn't as as aggressively pursued as
hackability. xplr simply prefers to stay minimal and looks for the opportunity
to lose some kb if it makes sense.

## Features

Some of the coolest features xplr provide beside the basic stuff:

- [Embedded LuaJIT][5] for portability and extensibility.
- [A simple modal system based on message passing][10] to control xplr session
  using:
  - [Keyboard inputs][11]
  - [Shell Commands][12]
  - [Lua Functions][13]
  - [Hooks][22]
- Easy, typesafe message passing with `-m MSG` or `-M MSG` subcommands.
- [Readline-like input buffer][9] with customizable behavior to read user
  inputs.
- [Switchable recover mode][7] that saves you from doing unwanted things when
  in a hurry.
- [Customizable layouts][1] with built-in panels. For e.g.
  - **Selection list** to show you the selected paths in real-time.
  - **Help menu** to show you the available keys bindings in each mode.
  - **Input & logs** to read input and display logs.
  - **Filter and sort pipeline** to show you the applied filters and sorters.
- [Custom file properties][17] with custom colors can be displayed in the table.
- [FIFO manager][19] to manage a FIFO file that can be used to
  [integrate with previewers][6].
- [Virtual root][21] with `--vroot` and `:v` key bindings.
- **Different quit options:**
  - Quit with success without any output (`q`).
  - Quit with success and the result printed on stdout (`enter`).
  - Quit with success and the present working directory printed on stdout
    (`:` `q` `p`).
  - Quit with success and the path under focus printed on stdout
    (`:` `q` `f`).
  - Quit with success and the selection printed on stdout
    (`:` `q` `s`).
  - Quit with failure (`ctrl-c`).

[1]: layouts.md
[2]: configure-key-bindings.md
[3]: awesome-plugins.md
[4]: https://github.com/sayanarijit/xplr/tree/main/benches
[5]: https://github.com/sayanarijit/xplr/discussions/183
[6]: https://github.com/sayanarijit/xplr/pull/229
[7]: modes.md#xplrconfigmodesbuiltinrecover
[8]: default-key-bindings.md
[9]: https://github.com/sayanarijit/xplr/pull/397
[10]: messages.md
[11]: configure-key-bindings.md
[12]: mode.md#input-pipe
[13]: lua-function-calls.md
[14]: awesome-plugins.md#integration
[15]: awesome-integrations.md
[16]: awesome-hacks.md
[17]: node_types.md
[18]: https://github.com/sayanarijit/xplr/blob/main/src/init.lua
[19]: messages.md#startfifo
[21]: messages.md#virtual-root
[22]: configuration.md#hooks
