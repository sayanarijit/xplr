# Default Key Bindings

The default key binding is inspired by [vim][1] and slightly
overlaps with [nnn][2], but it's supposed to be customized as per user
requirements.

When you press `?` in [default mode][3], you can see the complete list
of [modes][4] and the key mappings for each mode.

### create_directory

| key    | remaps | action       |
| ------ | ------ | ------------ |
| ctrl-c |        | terminate    |
| enter  |        | submit       |
| esc    |        | cancel       |
| tab    |        | try complete |

### default

| key    | remaps | action              |
| ------ | ------ | ------------------- |
| .      |        | show hidden         |
| /      | ctrl-f | search              |
| :      |        | action              |
| ?      |        | global help menu    |
| G      |        | go to bottom        |
| V      | ctrl-a | select/unselect all |
| ctrl-c |        | terminate           |
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

### duplicate_as

| key    | remaps | action    |
| ------ | ------ | --------- |
| ctrl-c |        | terminate |
| enter  |        | duplicate |
| esc    |        | cancel    |

### relative_path_does_not_match_regex

| key    | remaps | action    |
| ------ | ------ | --------- |
| ctrl-c |        | terminate |
| enter  |        | submit    |
| esc    |        | cancel    |

### number

| key    | remaps | action    |
| ------ | ------ | --------- |
| ctrl-c |        | terminate |
| down   | j      | to down   |
| enter  |        | to index  |
| esc    |        | cancel    |
| k      | up     | to up     |
| [0-9]  |        | input     |

### search

| key    | remaps | action           |
| ------ | ------ | ---------------- |
| ctrl-c |        | terminate        |
| ctrl-n | down   | down             |
| ctrl-p | up     | up               |
| enter  | esc    | focus            |
| left   |        | back             |
| right  |        | enter            |
| tab    |        | toggle selection |

### quit

| key    | remaps | action                  |
| ------ | ------ | ----------------------- |
| ctrl-c |        | terminate               |
| enter  |        | just quit               |
| esc    |        | cancel                  |
| f      |        | quit printing focus     |
| p      |        | quit printing pwd       |
| r      |        | quit printing result    |
| s      |        | quit printing selection |

### create

| key    | remaps | action           |
| ------ | ------ | ---------------- |
| ctrl-c |        | terminate        |
| d      |        | create directory |
| esc    |        | cancel           |
| f      |        | create file      |

### filter

| key       | remaps | action                             |
| --------- | ------ | ---------------------------------- |
| R         |        | relative path does not match regex |
| backspace |        | remove last filter                 |
| ctrl-c    |        | terminate                          |
| ctrl-r    |        | reset filters                      |
| ctrl-u    |        | clear filters                      |
| enter     | esc    | submit                             |
| r         |        | relative path does match regex     |

### delete

| key    | remaps | action       |
| ------ | ------ | ------------ |
| D      |        | force delete |
| ctrl-c |        | terminate    |
| d      |        | delete       |
| esc    |        | cancel       |

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
| ctrl-c    |        | terminate                         |
| ctrl-r    |        | reset sorters                     |
| ctrl-u    |        | clear sorters                     |
| e         |        | by canonical extension            |
| enter     | esc    | submit                            |
| l         |        | by last modified                  |
| m         |        | by canonical mime essence         |
| n         |        | by node type                      |
| r         |        | by relative path                  |
| s         |        | by size                           |

### go_to

| key    | remaps | action         |
| ------ | ------ | -------------- |
| ctrl-c |        | terminate      |
| esc    |        | cancel         |
| f      |        | follow symlink |
| g      |        | top            |
| p      |        | path           |
| x      |        | open in gui    |

### create_file

| key    | remaps | action       |
| ------ | ------ | ------------ |
| ctrl-c |        | terminate    |
| enter  |        | submit       |
| esc    |        | cancel       |
| tab    |        | try complete |

### recover

| key    | remaps | action    |
| ------ | ------ | --------- |
| ctrl-c |        | terminate |
| esc    |        | escape    |

### action

| key    | remaps | action               |
| ------ | ------ | -------------------- |
| !      |        | shell                |
| c      |        | create               |
| ctrl-c |        | terminate            |
| e      |        | open in editor       |
| esc    |        | cancel               |
| l      |        | logs                 |
| m      |        | toggle mouse         |
| q      |        | quit options         |
| s      |        | selection operations |
| [0-9]  |        | go to index          |

### switch_layout

| key    | remaps | action               |
| ------ | ------ | -------------------- |
| 1      |        | default              |
| 2      |        | no help menu         |
| 3      |        | no selection panel   |
| 4      |        | no help or selection |
| ctrl-c |        | terminate            |
| esc    |        | cancel               |

### relative_path_does_match_regex

| key    | remaps | action    |
| ------ | ------ | --------- |
| ctrl-c |        | terminate |
| enter  |        | submit    |
| esc    |        | cancel    |

### debug_error

| key    | remaps | action              |
| ------ | ------ | ------------------- |
| ctrl-c |        | terminate           |
| enter  |        | open logs in editor |
| esc    |        | escape              |
| q      |        | quit                |

### go_to_path

| key    | remaps | action       |
| ------ | ------ | ------------ |
| ctrl-c |        | terminate    |
| enter  |        | submit       |
| esc    |        | cancel       |
| tab    |        | try complete |

### selection_ops

| key    | remaps | action      |
| ------ | ------ | ----------- |
| c      |        | copy here   |
| ctrl-c |        | terminate   |
| esc    |        | cancel      |
| m      |        | move here   |
| x      |        | open in gui |

### rename

| key    | remaps | action       |
| ------ | ------ | ------------ |
| ctrl-c |        | terminate    |
| enter  |        | rename       |
| esc    |        | cancel       |
| tab    |        | try complete |

[1]: https://www.vim.org/
[2]: https://github.com/jarun/nnn/
[3]: #default
[4]: modes.md
