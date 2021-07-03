Filtering
=========

xplr supports filtering paths by different properties. The filtering mechanism
works like a pipeline, which in visible in the `Sort & filter` panel.

Example:

```
rel!^. › [i]abs=~abc › [i]rel!~xyz
```

This line means that the nodes visible on the table will first be filtered by
the condition: *relative path does not start with `.`*, then by the condition:
*absolute path contains `abc` (case insensitive)*, and finally by the
condition: *relative path does not contain `xyz`* (case insensitive).

Each part of this pipeline is called [Node Filter Applicable][1].


Node Filter Applicable
----------------------

It contains the following information:

- [filter][2]
- [input][3]

### filter

A filter can be one of the following:

- "RelativePathIs"
- "RelativePathIsNot"
- "IRelativePathIs"
- "IRelativePathIsNot"
- "RelativePathDoesStartWith"
- "RelativePathDoesNotStartWith"
- "IRelativePathDoesStartWith"
- "IRelativePathDoesNotStartWith"
- "RelativePathDoesContain"
- "RelativePathDoesNotContain"
- "IRelativePathDoesContain"
- "IRelativePathDoesNotContain"
- "RelativePathDoesEndWith"
- "RelativePathDoesNotEndWith"
- "IRelativePathDoesEndWith"
- "IRelativePathDoesNotEndWith"
- "AbsolutePathIs"
- "AbsolutePathIsNot"
- "IAbsolutePathIs"
- "IAbsolutePathIsNot"
- "AbsolutePathDoesStartWith"
- "AbsolutePathDoesNotStartWith"
- "IAbsolutePathDoesStartWith"
- "IAbsolutePathDoesNotStartWith"
- "AbsolutePathDoesContain"
- "AbsolutePathDoesNotContain"
- "IAbsolutePathDoesContain"
- "IAbsolutePathDoesNotContain"
- "AbsolutePathDoesEndWith"
- "AbsolutePathDoesNotEndWith"
- "IAbsolutePathDoesEndWith"
- "IAbsolutePathDoesNotEndWith"


TODO: document each

### input

Type: string

The input for the condition.


Example:
--------

```lua
ToggleNodeFilter = {
  filter = "RelativePathDoesNotStartWith",
  input = "."
}
```

Here, `ToggleNodeFilter` is a [message][4] that adds or removes
(toggles) the filter applied.


[1]:#node-filter-applicable
[2]:#filter
[3]:#input
[4]:message.md