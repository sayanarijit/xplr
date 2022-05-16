# Message

You can think of xplr as a server. Just like web servers listen to HTTP
requests, xplr listens to [messages][1].

You can send these messages to an xplr session in the following ways:

- Via command-line (currently during start-up only, using `--on-load`)
- Via [key bindings][2]
- Via [Lua function calls][3]
- Via shell command using the [input pipe][4]
- Via socket (coming soon)

### Format

To send messages using the [key bindings][2] or
[Lua function calls][3], messages are represented in
[Lua][5] syntax. For example:

- `"Quit"`
- `{ FocusPath = "/path/to/file" }`
- `{ Call = { command = "bash", args = { "-c", "read -p test" } } }`

However, to send messages using the [input pipe][4], they need to be
represented using
[YAML][6] (or [JSON][7]) syntax. For example:

- `Quit`
- `FocusPath: "/path/to/file"`
- `Call: { command: bash, args: ["-c", "read -p test"] }`

## Also See:

- [Full List of Messages][1]

[1]: messages.md
[2]: key-bindings.md
[3]: lua-function-calls.md
[4]: environment-variables-and-pipes.md#input-pipe
[5]: https://www.lua.org/
[6]: http://yaml.org/
[7]: https://www.json.org
