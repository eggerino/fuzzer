# A in-file fuzzy finder

A CLI program to search for files within a directory by fuzzy matching the file's content to a given search term.
It supports searching in different directories than the current one.
You can also search in git files only to reduce the amount of files being searched.

# Build from source

Run `cargo build --release` to compile a release build of the program.
Add it to the path environment variable `export PATH="$PWD/target/release:$PATH"`

# Test

Run `cargo test` to execute the unit tests.
