# Minigrep

- Minified version of the grep command.
- Developed while reading the `The Rust programming language` book, with some additions in order to proof some contents. You can find this project in the chapter 12: An I/O project: building a command line program.

## Pre-requisites

- Cargo

## Testing

- run command bellow passing the query text and the filename as arguments:

```ssh
cargo run $(query) $(filename)

```

## TODOs

- Create structure to show the file name and the line number of the occurence in the result.
- Scan directories
    - Accept a depth option to determine how deep should we search in directories.  

