# things-3-report
A small script to generate bulleted reports from my Things 3 tasks

## Usage
The environment should be provided by `nix-shell`. Alternatively you can install the requirements directly with
```bash
$ pip install -r requirements.txt
```
To run the script, pass in the tag or list of tags to filter on. Projects and Tasks lacking one of the specified tags will be excluded from the generated report. I have a `Report` tag which I use for this. If a project is tagged with the tag, all tasks are considered tagged even if not explicitly done so.

### Specific functionality
I have also included a few pieces of additional functionality:
- The project is headed with a set of 3 randomly selected emojis from `emojis.txt`
- If you have a person's name in the task or project title, you can add an `@<Persons Name>` tag to the task or project and the script will automatically replace the vowels in their name with unicode that looks similar. This allows you to post the list on slack without generating mentions.
- Adding a code block that begins with `report` will cause the script to dump the contents into the end of the generated message. This can be used to add notes to the generated output.
- Be default the script draws from today's current list of tasks. If you specify `--signoff` it will instead draw on completed tasks from your logbook. (This is useful for an end of the day message).

## Example Usage:
Given a set of tasks tagged `Report`
```txt
Project named "Some Project" tagged as Report:
  containing two tasks (not necessarily tagged) "Task 1" and "Task 2"

A top level task called some task with a notes filed containing
```report
(#9999)[https://example.com]
`` ` <-- No space there but it's hard to escape these ticks
```

```
$ python dump.py Report
```
will generate:

```txt
:emoji: :emoji: :emoji:
  - Some Project
    - Task 1
    - Task 2
  - Some Task (#9999)(https://example.com)
```
