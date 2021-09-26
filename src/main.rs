mod storage;

use std::{env::Args, iter::Skip};
use storage::Database;

fn main() {
  // Extract arguments
  let mut args: Skip<Args> = std::env::args().skip(1);
  let key = args.next().expect("Invalid key input");
  let value = args.next().expect("Invalid value input");

  // Initialize DB
  let mut db = Database::new().expect("Could not initialize database");

  // Store input data
  db.insert(key, value);

  // Read all entries
  db.read_all()
}
