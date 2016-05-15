# remember

Command-line utility to remember things

## Dependencies

- [Rust](https://www.rust-lang.org/)

## Building

```
cd remember
cargo build
```

Binary is saved at **remember/target/debug/remember**.

## Usage

```
remember 1.0.0
Remember things from your command line

USAGE:
    remember [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Adds to memory bank
    change    Change value of key. Adds key if none exists.
    find      Find value for key
    help      Prints this message or the help of the given subcommand(s)
    list      List all saved information
    remove    Removes key
    wipe      This will completely remove all information
```