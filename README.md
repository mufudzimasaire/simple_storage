# Basic key/value store

This is a basic key/value store written in Rust.

**Note:** The program is not intended for use in any apps, rather it is a means of applying theory into practice whilst undertaking this journey of learning the Rust language.

## Getting started

To use the program, simply run the following command with the key/value pair you wish to store:

```bash
$ cargo run -- key 'this is an example value'
```

If the data is successfully stored, it will be returned as part of the output along with all the DB entries.

Alternatively, you can build and run the binary as follows -

```bash
$ cargo build

$ ./target/debug/simple_storage key 'this is an example value'
```

## TODO

It would be nice to have the ability to specify the operation you want to perform, eg. 

```bash
$ cargo run -- insert key 'this is an example value'

$ cargo run -- update key 'updated value'

$ cargo run -- read key

$ cargo run -- read_all

$ cargo run -- delete key

$ cargo run -- delete_all
```

This can theoratically be achieved through pattern matching or via a macro.
