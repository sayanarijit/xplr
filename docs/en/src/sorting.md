Sorting
=======

xplr supports sorting paths by different properties. The sorting mechanism
works like a pipeline, which in visible in the `Sort & filter` panel.

Example:

```
size↑ › [i]rel↓ › [c]dir↑ › [c]file↑ › sym↑
```

This line means that the nodes visible in the table will be first sorted by
it's size, then by case insensitive relative path, then by the
canonical (symlink resolved) type of the node, and finally by whether or not
the node is a symlink.

The arrows denote the order.

Each part of this pipeline is called [Node Sorter Applicable](#node-sorter-applicable).


Node Sorter Applicable
----------------------

It contains the following information:

- [sorter](#sorter)
- [reverse](#reverse)


### sorter

A sorter can be one of the following:

- "ByRelativePath"
- "ByIRelativePath"
- "ByExtension"
- "ByIsDir"
- "ByIsFile"
- "ByIsSymlink"
- "ByIsBroken"
- "ByIsReadonly"
- "ByMimeEssence"
- "BySize"
- "ByCanonicalAbsolutePath"
- "ByICanonicalAbsolutePath"
- "ByCanonicalExtension"
- "ByCanonicalIsDir"
- "ByCanonicalIsFile"
- "ByCanonicalIsReadonly"
- "ByCanonicalMimeEssence"
- "ByCanonicalSize"
- "BySymlinkAbsolutePath"
- "ByISymlinkAbsolutePath"
- "BySymlinkExtension"
- "BySymlinkIsDir"
- "BySymlinkIsFile"
- "BySymlinkIsReadonly"
- "BySymlinkMimeEssence"
- "BySymlinkSize"

TODO: document each


### reverse

Type: boolean

It defined the direction of the order.


Example
-------

```lua
xplr.config.general.initial_sorting = {
    { sorter = "ByCanonicalIsDir", reverse = true },
    { sorter = "ByIRelativePath", reverse = false },
}
```

This snippet defines the initial sorting logic to be applied when xplr loads.
