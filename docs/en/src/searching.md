# Searching

xplr supports searching paths using different algorithm. The search mechanism
generally appears between filters and sorters in the `Sort & filter` panel.

Example:

```
fzy:foo↓
```

This line means that the nodes visible on the table are being filtered using the
[fuzzy matching][1] algorithm on the input `foo`. The arrow means that ranking based
ordering is being applied, i.e. [sorters][2] are being ignored.

## Node Searcher Applicable

Node Searcher contains the following fields:

- [pattern][3]
- [recoverable_focus][4]
- [algorithm][5]
- [unordered][7]
- [exact_mode][10]
- [rank_criteria][11]

### pattern

The patterns used to search.

Type: string

### recoverable_focus

Where to focus when search is cancelled.

Type: nullable string

### algorithm

Search algorithm to use. Defaults to the value set in
[xplr.config.general.search.algorithm][8].

It can be one of the following:

- Fuzzy
- Regex

### unordered

Whether to skip ordering the search result by algorithm based ranking. Defaults
to the value set in [xplr.config.general.search.unordered][9].

Type: boolean

### exact_mode

Whether to search in exact mode. Defaults to `false`.

Type: boolean

### rank_criteria

Ranking criteria to use. Defaults to `nil`.

Type: nullable list of the following options:

- Score
- NegScore
- Begin
- NegBegin
- End
- NegEnd
- Length
- NegLength
- Index
- NegIndex

## Example:

```lua
local searcher = {
  pattern = "pattern to search",
  recoverable_focus = "/path/to/focus/on/cancel",
  algorithm = "Fuzzy",
  unordered = false,
  exact_mode = false,
  rank_criteria = { "Score", "Begin", "End", "Length" },
}

xplr.util.explore({ searcher = searcher })
```

See [xplr.util.explore][6].

[1]: https://en.wikipedia.org/wiki/Approximate_string_matching
[2]: sorting.md
[3]: #pattern
[4]: #recoverable_focus
[5]: #algorithm
[6]: xplr.util.md#xplrutilexplore
[7]: #unordered
[8]: general-config.md#xplrconfiggeneralsearchalgorithm
[9]: general-config.md#xplrconfiggeneralsearchunordered
[10]: #exact_mode
[11]: #rank_criteria
