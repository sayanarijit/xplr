# Default Key Bindings

The default key binding is inspired by [vim][1] and slightly
overlaps with [nnn][2], but it's supposed to be customized as per user
requirements.

When you press `?` in [default mode][3], you can see the complete list
of [modes][4] and the key mappings for each mode.

[1]: https://www.vim.org/
[2]: https://github.com/jarun/nnn/
[3]: #default
[4]: modes.md

### default

| key       | remaps | action              |
| --------- | ------ | ------------------- |
| (         |        | prev deep branch    |
| )         |        | next deep branch    |
| .         |        | show hidden         |
| /         | ctrl-f | search              |
| :         |        | action              |
| ?         | f1     | global help menu    |
| G         |        | go to bottom        |
| V         | ctrl-a | select/unselect all |
| ctrl-d    |        | duplicate as        |
| ctrl-i    | tab    | next visited path   |
| ctrl-n    |        | next selection      |
| ctrl-o    |        | last visited path   |
| ctrl-p    |        | prev selection      |
| ctrl-r    |        | refresh screen      |
| ctrl-u    |        | clear selection     |
| ctrl-w    |        | switch layout       |
| d         |        | delete              |
| down      | j      | down                |
| enter     |        | quit with result    |
| f         |        | filter              |
| g         |        | go to               |
| h         | left   | back                |
| k         | up     | up                  |
| l         | right  | enter               |
| page-down |        | scroll down         |
| page-up   |        | scroll up           |
| q         |        | quit                |
| r         |        | rename              |
| s         |        | sort                |
| space     | v      | toggle selection    |
| {         |        | scroll up half      |
| }         |        | scroll down half    |
| ~         |        | go home             |
| [0-9]     |        | input               |

### duplicate_as

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| enter |        | submit           |
| f1    |        | global help menu |
| tab   |        | try complete     |

### filter

| key       | remaps | action                             |
| --------- | ------ | ---------------------------------- |
| R         |        | relative path does not match regex |
| backspace |        | remove last filter                 |
| ctrl-r    |        | reset filters                      |
| ctrl-u    |        | clear filters                      |
| f1        |        | global help menu                   |
| r         |        | relative path does match regex     |

### action

| key   | remaps | action               |
| ----- | ------ | -------------------- |
| !     |        | shell                |
| c     |        | create               |
| e     |        | open in editor       |
| f1    |        | global help menu     |
| l     |        | logs                 |
| m     |        | toggle mouse         |
| p     |        | edit permissions     |
| q     |        | quit options         |
| s     |        | selection operations |
| v     |        | vroot                |
| [0-9] |        | go to index          |

### create

| key | remaps | action           |
| --- | ------ | ---------------- |
| d   |        | create directory |
| f   |        | create file      |
| f1  |        | global help menu |

### switch_layout

| key | remaps | action               |
| --- | ------ | -------------------- |
| 1   |        | default              |
| 2   |        | no help menu         |
| 3   |        | no selection panel   |
| 4   |        | no help or selection |
| f1  |        | global help menu     |

### selection_ops

| key | remaps | action           |
| --- | ------ | ---------------- |
| c   |        | copy here        |
| e   |        | edit selection   |
| f1  |        | global help menu |
| h   |        | hardlink here    |
| l   |        | list selection   |
| m   |        | move here        |
| s   |        | softlink here    |
| u   |        | clear selection  |

### delete

| key | remaps | action           |
| --- | ------ | ---------------- |
| D   |        | force delete     |
| d   |        | delete           |
| f1  |        | global help menu |

### number

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| down  | j      | to down          |
| enter |        | to index         |
| f1    |        | global help menu |
| k     | up     | to up            |
| [0-9] |        | input            |

### create_directory

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| enter |        | submit           |
| f1    |        | global help menu |
| tab   |        | try complete     |

### recover

| key | remaps | action           |
| --- | ------ | ---------------- |
| f1  |        | global help menu |

### rename

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| enter |        | submit           |
| f1    |        | global help menu |
| tab   |        | try complete     |

### vroot

| key    | remaps | action           |
| ------ | ------ | ---------------- |
| .      |        | vroot $PWD       |
| /      |        | vroot /          |
| ctrl-r |        | reset vroot      |
| ctrl-u |        | unset vroot      |
| f1     |        | global help menu |
| v      |        | toggle vroot     |
| ~      |        | vroot $HOME      |

### relative_path_does_match_regex

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| enter |        | submit           |
| f1    |        | global help menu |

### relative_path_does_not_match_regex

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| enter |        | submit           |
| f1    |        | global help menu |

### debug_error

| key   | remaps | action              |
| ----- | ------ | ------------------- |
| enter |        | open logs in editor |
| f1    |        | global help menu    |
| q     |        | quit                |

### edit_permissions

| key    | remaps | action           |
| ------ | ------ | ---------------- |
| G      |        | -group           |
| M      |        | min              |
| O      |        | -other           |
| U      |        | -user            |
| ctrl-r |        | reset            |
| enter  |        | submit           |
| f1     |        | global help menu |
| g      |        | +group           |
| m      |        | max              |
| o      |        | +other           |
| u      |        | +user            |

### sort

| key       | remaps | action                            |
| --------- | ------ | --------------------------------- |
| !         |        | reverse sorters                   |
| C         |        | by created reverse                |
| E         |        | by canonical extension reverse    |
| L         |        | by last modified reverse          |
| M         |        | by canonical mime essence reverse |
| N         |        | by node type reverse              |
| R         |        | by relative path reverse          |
| S         |        | by size reverse                   |
| backspace |        | remove last sorter                |
| c         |        | by created                        |
| ctrl-r    |        | reset sorters                     |
| ctrl-u    |        | clear sorters                     |
| e         |        | by canonical extension            |
| enter     |        | submit                            |
| f1        |        | global help menu                  |
| l         |        | by last modified                  |
| m         |        | by canonical mime essence         |
| n         |        | by node type                      |
| r         |        | by relative path                  |
| s         |        | by size                           |

### go_to

| key | remaps | action           |
| --- | ------ | ---------------- |
| f   |        | follow symlink   |
| f1  |        | global help menu |
| g   |        | top              |
| i   |        | initial $PWD     |
| p   |        | path             |
| x   |        | open in gui      |

### quit

| key   | remaps | action                  |
| ----- | ------ | ----------------------- |
| enter |        | just quit               |
| f     |        | quit printing focus     |
| f1    |        | global help menu        |
| p     |        | quit printing pwd       |
| r     |        | quit printing result    |
| s     |        | quit printing selection |

### search

| key    | remaps | action                  |
| ------ | ------ | ----------------------- |
| ctrl-a |        | toggle search algorithm |
| ctrl-f |        | fuzzy search            |
| ctrl-n | down   | down                    |
| ctrl-p | up     | up                      |
| ctrl-r |        | regex search            |
| ctrl-s |        | sort (no search order)  |
| ctrl-z |        | toggle ordering         |
| enter  |        | submit                  |
| esc    |        | cancel                  |
| f1     |        | global help menu        |
| left   |        | back                    |
| right  |        | enter                   |
| tab    |        | toggle selection        |

### go_to_path

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| enter |        | submit           |
| f1    |        | global help menu |
| tab   |        | try complete     |

### create_file

| key   | remaps | action           |
| ----- | ------ | ---------------- |
| enter |        | submit           |
| f1    |        | global help menu |
| tab   |        | try complete     |
