mod storage;

use std::{env::Args, iter::Skip};
use storage::Database;

fn main() {
	// Extract arguments
	let mut args: Skip<Args> = std::env::args().skip(1);
	let key = args.next().expect("Invalid key input");
	let value = args.next().expect("Invalid value input");

	// Store imput data
	let _result = Database::insert(key, value);

	// Read entries
	Database::all()
}
