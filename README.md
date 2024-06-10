# GPA CALCULATOR
Simple CLI application for easier time calculating stuff around GPA for my uni. This apps basically
works as a repl with error handeling and other sweet stuff.

## Avalible commands
- `add <points> <credit>` adds subject result
- `gpa` shows the current GPA value for the save records
- `expect <target_gpa> <subject_credit>` returns the minimum required points  you have to achive, from a subject, in order to get the target GPA
- `show` shows the saved subject result records
- `remove <index>` removes the suject result record at the given index
- `import <filename>` loads records from the given file, records in the file have to be in this format `<points> <index>`, one record for one line
- `save <filename>` saves all the records into given file
- `drop` delets all the records
</br>
- `quit` or `q` will exit the app
- `history` or `h` will show all previous commands

![expample](./images/example.png)


