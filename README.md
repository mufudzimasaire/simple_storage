# Basic key-value store

This is a basic key-value store written in Rust that utilises the file system to persist data.

## Getting started

To use the program, simply run the following command with the key-value pair you wish to store:

```bash
$ cargo run -- insert key 'this is an example value'
```

Run the following command with the `list` operation to view all the persisted entries - 

```bash
$ cargo run -- list
```

Alternatively, you can build and run the binary as follows -

```bash
$ cargo build --release 
$ cd target/release/
$ ./simple_storage insert key 'this is an example value'
$ ./simple_storage list
```

## TODO

In addition to the `insert` and `list` operations, it would be great to extend the DB API to include the ability to `read` or `delete` a specific `key`, eg. 

```bash
$ ./simple_storage read key
$ ./simple_storage delete key
$ ./simple_storage flush
```

The pattern matching used for dynamically calling operations will be converted into a macro.
