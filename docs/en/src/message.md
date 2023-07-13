# Message

You can think of xplr as a server. Just like web servers listen to HTTP
requests, xplr listens to messages.

A message is a [sum type][9] that can have [these possible values][1].

You can send these messages to an xplr session in the following ways:

- Via command-line (currently during start-up only, using `--on-load`)
- Via [key bindings][2]
- Via [Lua function calls][3]
- Via shell command using the [input pipe][4]
- Via socket (coming soon)

### Format

To send messages using the [key bindings][2] or [Lua function calls][3],
messages are represented in [Lua][5] syntax.

For example:

- `"Quit"`
- `{ FocusPath = "/path/to/file" }`
- `{ Call = { command = "bash", args = { "-c", "read -p test" } } }`

However, to send messages using the [input pipe][4], they need to be
represented using [YAML][6] (or [JSON][7]) syntax.

For example:

- `Quit`
- `FocusPath: "/path/to/file"`
- `Call: { command: bash, args: ["-c", "read -p test"] }`

Use `"$XPLR" -m TEMPLATE [VALUE]...` command-line option to safely format
`TEMPLATE` into a valid message. If uses [jf][8] to parse and render the
template. And `$XPLR` (rather than `xplr`) makes sure that the correct version
of the binary is being used.

For example:

- `"$XPLR" -m Quit`
- `"$XPLR" -m 'FocusPath: %q' "/path/to/file"`
- `"$XPLR" -m 'Call: { command: %q, args: [%*q] }' bash -c "read -p test"`

## Also See:

- [Full List of Messages][1]

[1]: messages.md
[2]: key-bindings.md
[3]: lua-function-calls.md
[4]: environment-variables-and-pipes.md#input-pipe
[5]: https://www.lua.org/
[6]: http://yaml.org/
[7]: https://www.json.org
[8]: https://github.com/sayanarijit/jf
[9]: sum-type.md
