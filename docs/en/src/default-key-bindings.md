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

| key    | remaps | action              |
| ------ | ------ | ------------------- |
| .      |        | show hidden         |
| /      | ctrl-f | search              |
| :      |        | action              |
| ?      |        | global help menu    |
| G      |        | go to bottom        |
| V      | ctrl-a | select/unselect all |
| ctrl-d |        | duplicate as        |
| ctrl-i | tab    | next visited path   |
| ctrl-o |        | last visited path   |
| ctrl-r |        | refresh screen      |
| ctrl-u |        | clear selection     |
| ctrl-w |        | switch layout       |
| d      |        | delete              |
| down   | j      | down                |
| enter  |        | quit with result    |
| f      |        | filter              |
| g      |        | go to               |
| h      | left   | back                |
| k      | up     | up                  |
| l      | right  | enter               |
| q      |        | quit                |
| r      |        | rename              |
| s      |        | sort                |
| space  | v      | toggle selection    |
| ~      |        | go home             |
| [0-9]  |        | input               |

### go_to_path

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### relative_path_does_not_match_regex

| key   | remaps | action |
| ----- | ------ | ------ |
| enter |        | submit |

### number

| key   | remaps | action   |
| ----- | ------ | -------- |
| down  | j      | to down  |
| enter |        | to index |
| k     | up     | to up    |
| [0-9] |        | input    |

### selection_ops

| key | remaps | action      |
| --- | ------ | ----------- |
| c   |        | copy here   |
| m   |        | move here   |
| x   |        | open in gui |

### duplicate_as

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### action

| key   | remaps | action               |
| ----- | ------ | -------------------- |
| !     |        | shell                |
| c     |        | create               |
| e     |        | open in editor       |
| l     |        | logs                 |
| m     |        | toggle mouse         |
| q     |        | quit options         |
| s     |        | selection operations |
| [0-9] |        | go to index          |

### go_to

| key | remaps | action         |
| --- | ------ | -------------- |
| f   |        | follow symlink |
| g   |        | top            |
| p   |        | path           |
| x   |        | open in gui    |

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
| l         |        | by last modified                  |
| m         |        | by canonical mime essence         |
| n         |        | by node type                      |
| r         |        | by relative path                  |
| s         |        | by size                           |

### rename

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### debug_error

| key   | remaps | action              |
| ----- | ------ | ------------------- |
| enter |        | open logs in editor |
| q     |        | quit                |

### switch_layout

| key | remaps | action               |
| --- | ------ | -------------------- |
| 1   |        | default              |
| 2   |        | no help menu         |
| 3   |        | no selection panel   |
| 4   |        | no help or selection |

### search

| key    | remaps | action           |
| ------ | ------ | ---------------- |
| ctrl-n | down   | down             |
| ctrl-p | up     | up               |
| enter  |        | submit           |
| esc    |        | cancel           |
| left   |        | back             |
| right  |        | enter            |
| tab    |        | toggle selection |

### create_file

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### recover

| key | remaps | action |
| --- | ------ | ------ |

### create

| key | remaps | action           |
| --- | ------ | ---------------- |
| d   |        | create directory |
| f   |        | create file      |

### filter

| key       | remaps | action                             |
| --------- | ------ | ---------------------------------- |
| R         |        | relative path does not match regex |
| backspace |        | remove last filter                 |
| ctrl-r    |        | reset filters                      |
| ctrl-u    |        | clear filters                      |
| r         |        | relative path does match regex     |

### delete

| key | remaps | action       |
| --- | ------ | ------------ |
| D   |        | force delete |
| d   |        | delete       |

### create_directory

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### quit

| key   | remaps | action                  |
| ----- | ------ | ----------------------- |
| enter |        | just quit               |
| f     |        | quit printing focus     |
| p     |        | quit printing pwd       |
| r     |        | quit printing result    |
| s     |        | quit printing selection |

### relative_path_does_match_regex

| key   | remaps | action |
| ----- | ------ | ------ |
| enter |        | submit |
