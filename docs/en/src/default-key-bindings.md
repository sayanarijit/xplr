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
| .         |        | show hidden         |
| /         | ctrl-f | search              |
| :         |        | action              |
| ?         |        | global help menu    |
| G         |        | go to bottom        |
| V         | ctrl-a | select/unselect all |
| ctrl-d    |        | duplicate as        |
| ctrl-i    | tab    | next visited path   |
| ctrl-o    |        | last visited path   |
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

### filter

| key       | remaps | action                             |
| --------- | ------ | ---------------------------------- |
| R         |        | relative path does not match regex |
| backspace |        | remove last filter                 |
| ctrl-r    |        | reset filters                      |
| ctrl-u    |        | clear filters                      |
| r         |        | relative path does match regex     |

### vroot

| key    | remaps | action       |
| ------ | ------ | ------------ |
| .      |        | vroot $PWD   |
| /      |        | vroot /      |
| ~      |        | vroot $HOME  |
| v      |        | toggle vroot |
| ctrl-r |        | reset vroot  |
| ctrl-u |        | unset vroot  |

### create_file

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### selection_ops

| key | remaps | action          |
| --- | ------ | --------------- |
| c   |        | copy here       |
| m   |        | move here       |
| u   |        | clear selection |

### create

| key | remaps | action           |
| --- | ------ | ---------------- |
| d   |        | create directory |
| f   |        | create file      |

### quit

| key   | remaps | action                  |
| ----- | ------ | ----------------------- |
| enter |        | just quit               |
| f     |        | quit printing focus     |
| p     |        | quit printing pwd       |
| r     |        | quit printing result    |
| s     |        | quit printing selection |

### switch_layout

| key | remaps | action               |
| --- | ------ | -------------------- |
| 1   |        | default              |
| 2   |        | no help menu         |
| 3   |        | no selection panel   |
| 4   |        | no help or selection |

### delete

| key | remaps | action       |
| --- | ------ | ------------ |
| D   |        | force delete |
| d   |        | delete       |

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

### relative_path_does_match_regex

| key   | remaps | action |
| ----- | ------ | ------ |
| enter |        | submit |

### create_directory

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### duplicate_as

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### rename

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

### go_to_path

| key   | remaps | action       |
| ----- | ------ | ------------ |
| enter |        | submit       |
| tab   |        | try complete |

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

### debug_error

| key   | remaps | action              |
| ----- | ------ | ------------------- |
| enter |        | open logs in editor |
| q     |        | quit                |

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
| v     |        | vroot                |
| [0-9] |        | go to index          |

### recover

| key | remaps | action |
| --- | ------ | ------ |

### go_to

| key | remaps | action         |
| --- | ------ | -------------- |
| f   |        | follow symlink |
| g   |        | top            |
| i   |        | initial $PWD   |
| p   |        | path           |
| x   |        | open in gui    |
