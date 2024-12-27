# things-3-report
A small program written in rust that digests my Things tasks into a format for Slack reporting.

Tasks are pulled out of Things using AppleScript over a JavaScript OSA Bridge. Tasks are then filtered
and formatted based on the selected format schema.

## Options
See `day-reporter -h` for a current list of options

## Basic Task format
A task is returned as a markdown list element containing its title and optionally a list of information parsed from
related tag sections in the notes portion of the task.

Tag specific additional bullets are included using a triple backtick (like GitHub code block markdown). Then instead of
specifying the coding style as you would (for example `typescript`) you specify the relevant tag: `MyTag`. Running the
program with a given `--tags` argument will cause related tag blocks to be included as sub-bullets.

### Example

<pre>
```MyTag
Some notes
```
</pre>

will result in "Some notes" being included with the task

## Name Sanitization
Name tags found on projects and tasks are automatically sanitized by replacing vowel characters with lookalikes. This
behavior can be disabled with the `--no-sanitize` argument.

## Project Formatting
Projects make up a bullet level above tasks. Any tasks in the project will be nested under the project title following
and project specific matched tag block comments in the project notes.

## Area Formatting
Areas collect up projects, but instead of indenting them further, Area's are returned as section titles above the
generated output. If only one area contains all of the reported tasks, then the area name is omitted entirely.
