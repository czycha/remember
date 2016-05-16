# remember

Command-line utility to remember things

## Dependencies

- [Rust](https://www.rust-lang.org/)

## Building from source

```bash
$ cd remember
$ cargo build
```

Binary is saved at **remember/target/debug/remember**.

## Installation from Cargo

```bash
$ cargo install remember
```

Make sure Cargo's bin has been added to your path.

## Usage

Command is `remember`. For help, use `remember help`. If any command fails, will exit with exit code `-1`.

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

### Examples

#### Add

```bash
$ remember add remember https://github.com/EVA-01/remember
# remember → https://github.com/EVA-01/remember
```

If key already exists, will warn user.

#### Find

```bash
$ remember find remember
# Outputs: https://github.com/EVA-01/remember
```

If key doesn't exist, returns nothing.

#### List

```bash
$ remember list
# Outputs:
#	remember → https://github.com/EVA-01/remember
```

#### Change

```bash
$ remember change remember ~/.remember
# remember → /Users/USER/.remember
```

If key doesn't exist, adds the key and value.

#### Remove

```bash
$ remember remove remember
```

#### Wipe

```bash
$ remember wipe
This will remove all information. Are you sure? (y/n) yes
```

**Note:** Using `remember wipe --force` will skip confirmation.

## Note about security

remember stores all information in plain-text for fast writing and retrieval. **Do not store passwords or other private information using remember.** In a future version, a secure option may be available, but until then **don't do it**.