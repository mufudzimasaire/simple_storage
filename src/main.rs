mod database;
use database::Database;
use std::{env::Args, iter::Skip};

fn main() {
  // Extract the operation, key and value arguments
  // from the command eg `simple_storage insert example_key 'the example value'`
  let args: Skip<Args> = std::env::args().skip(1);

  // Initialize the DB
  let mut db = Database::new().expect("Could not initialize database");

  // Perform operation
  db.call(args)
}
