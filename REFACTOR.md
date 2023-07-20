# Refactor Goals
1. Speed things up. Relying on things.py (and python in general) makes the code slow. Add in the fact that the
`nix-shell` logic tends to run every day and pulls down the universe, the current day reporter is quite slow
2. Clean up the code. Using a better language will help some, but in particular using a bonafide templating library
should improve the code.
3. Stability. Using `osascript` with the Things AppleScript API will avoid future problems that `things.py` is
constantly running into because they are overly coupled to the Things sqlite database schema.

# Ideas
- Move to rust using `osascript` to drive interactions with things
- Write a small library of scripts in JS that rely on the OSA infrastructure to call out to things, and package up
content into JSON for ease of use in rust.

# Resources
I found a really great gists for working with osascript and it even has Things examples: https://gist.github.com/tommorris/99bcbbcf445bb6475797
