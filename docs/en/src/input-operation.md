# Input Operation

Cursor based input operation is a [sum type][3] can be one of the following:

- { SetCursor = int }
- { InsertCharacter = str }
- "GoToPreviousCharacter"
- "GoToNextCharacter"
- "GoToPreviousWord"
- "GoToNextWord"
- "GoToStart"
- "GoToEnd"
- "DeletePreviousCharacter"
- "DeleteNextCharacter"
- "DeletePreviousWord"
- "DeleteNextWord"
- "DeleteLine"
- "DeleteTillEnd"

## Also See:

- [Message][1]
- [Full List of Messages][2]

[1]: message.md
[2]: messages.md
[3]: sum-type.md
