Default Key Bindings
====================

The default key binding is inspired by [vim](https://www.vim.org/) and slightly
overlaps with [nnn](https://github.com/jarun/nnn/), but it's supposed to be
customized as per user requirements.

When you press `?` in [default mode](#default), you can see the complete list
of [modes](modes.md) and the key mappings for each mode.


### default

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 .               |                           | show hidden
 /               | ctrl-f                    | search
 :               |                           | action
 ?               |                           | global help menu
 G               |                           | go to bottom
 V               | ctrl-a                    | select/unselect all
 ctrl-c          |                           | terminate
 ctrl-i          | tab                       | next visited path
 ctrl-o          |                           | last visited path
 ctrl-r          |                           | refresh screen
 ctrl-u          |                           | clear selection
 ctrl-w          |                           | switch layout
 d               |                           | delete
 down            | j                         | down
 enter           |                           | quit with result
 f               |                           | filter
 g               |                           | go to
 h               | left                      | back
 k               | up                        | up
 l               | right                     | enter
 q               |                           | quit
 r               |                           | rename
 s               |                           | sort
 space           | v                         | toggle selection
 ~               |                           | go home
 [0-9]           |                           | input


### recover

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 ctrl-c          |                           | terminate
 esc             |                           | escape


### filter

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 R               |                           | relative does not contain
 backspace       |                           | remove last filter
 ctrl-c          |                           | terminate
 ctrl-r          |                           | reset filters
 ctrl-u          |                           | clear filters
 enter           | esc                       | done
 r               |                           | relative does contain


### number

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 backspace       |                           | remove last character
 ctrl-c          |                           | terminate
 ctrl-u          |                           | remove line
 ctrl-w          |                           | remove last word
 down            | j                         | to down
 enter           |                           | to index
 esc             |                           | cancel
 k               | up                        | to up
 [0-9]           |                           | input


### go to

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 ctrl-c          |                           | terminate
 esc             |                           | cancel
 f               |                           | follow symlink
 g               |                           | top
 x               |                           | open in gui


### search

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 backspace       |                           | remove last character
 ctrl-c          |                           | terminate
 ctrl-n          | down                      | down
 ctrl-p          | up                        | up
 ctrl-u          |                           | remove line
 ctrl-w          |                           | remove last word
 enter           | esc                       | focus
 left            |                           | back
 right           |                           | enter
 tab             |                           | toggle selection


### selection ops

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 c               |                           | copy here
 ctrl-c          |                           | terminate
 esc             |                           | cancel
 m               |                           | move here
 x               |                           | open in gui


### action to

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 !               |                           | shell
 c               |                           | create
 ctrl-c          |                           | terminate
 e               |                           | open in editor
 esc             |                           | cancel
 l               |                           | logs
 m               |                           | toggle mouse
 q               |                           | quit options
 s               |                           | selection operations
 [0-9]           |                           | go to index


### create

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 ctrl-c          |                           | terminate
 d               |                           | create directory
 esc             |                           | cancel
 f               |                           | create file


### create file

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 backspace       |                           | remove last character
 ctrl-c          |                           | terminate
 ctrl-u          |                           | remove line
 ctrl-w          |                           | remove last word
 enter           |                           | create file
 esc             |                           | cancel


### create directory

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 backspace       |                           | remove last character
 ctrl-c          |                           | terminate
 ctrl-u          |                           | remove line
 ctrl-w          |                           | remove last word
 enter           |                           | create directory
 esc             |                           | cancel


### rename

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 backspace       |                           | remove last character
 ctrl-c          |                           | terminate
 ctrl-u          |                           | remove line
 ctrl-w          |                           | remove last word
 enter           |                           | rename
 esc             |                           | cancel


### delete

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 D               |                           | force delete
 ctrl-c          |                           | terminate
 d               |                           | delete
 esc             |                           | cancel


### sort

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 !               |                           | reverse sorters
 E               |                           | by canonical extension reverse
 M               |                           | by canonical mime essence reverse
 N               |                           | by node type reverse
 R               |                           | by relative path reverse
 S               |                           | by size reverse
 backspace       |                           | remove last sorter
 ctrl-c          |                           | terminate
 ctrl-r          |                           | reset sorters
 ctrl-u          |                           | clear sorters
 e               |                           | by canonical extension
 enter           | esc                       | done
 m               |                           | by canonical mime essence
 n               |                           | by node type
 r               |                           | by relative path
 s               |                           | by size


### filter

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 R               |                           | relative does not contain
 backspace       |                           | remove last filter
 ctrl-c          |                           | terminate
 ctrl-r          |                           | reset filters
 ctrl-u          |                           | clear filters
 enter           | esc                       | done
 r               |                           | relative does contain


### relative path does contain

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 backspace       |                           | remove last character
 ctrl-c          |                           | terminate
 ctrl-u          |                           | remove line
 ctrl-w          |                           | remove last word
 enter           |                           | apply filter
 esc             |                           | cancel


### relative path does not contain

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 backspace       |                           | remove last character
 ctrl-c          |                           | terminate
 ctrl-u          |                           | remove line
 ctrl-w          |                           | remove last word
 enter           |                           | apply filter
 esc             |                           | cancel


### switch layout

 key             | remaps                    | action
 --------------- | ------------------------- | ------
 1               |                           | default
 2               |                           | no help menu
 3               |                           | no selection panel
 4               |                           | no help or selection
 ctrl-c          |                           | terminate
 esc             |                           | cancel
