# Basic key-value store

This is a basic key-value store written in Rust that utilises the file system to persist data.

## Usage

The data store supports `insert`, `select`, `list`, `delete` and `flush` operations. 

To use the program, simply run the following command with the key-value pair you wish to store:

```bash
$ cargo run -- insert key 'this is an example value'
```

Run the following command with the `list` operation to view all the persisted entries - 

```bash
$ cargo run -- list
```

The `select`, `delete` and `flush` operations can be used as follows -

```bash
$ cargo run -- select key
$ cargo run -- delete key
$ cargo run -- flush
```

Alternatively, you can build and run the binary as follows -

```bash
$ cargo build --release 
$ cd target/release/
$ ./simple_storage insert key 'this is an example value'
$ ./simple_storage list
$ ./simple_storage select key
$ ./simple_storage delete key
$ ./simple_storage flush
```
